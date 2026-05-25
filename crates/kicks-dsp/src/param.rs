// Kicks — Lock-free Parameter Channel
//
// An SPSC (single-producer, single-consumer) channel for lock-free
// parameter changes from the main thread to the audio callback.
// Uses ringbuf's atomic-index-based heap ring buffer internally —
// the Producer and Consumer share state through atomics, not locks.
//
// Real-time safety:
//   - send()  (main thread side)   — NOT real-time safe (brief `Mutex` lock
//     for borrow checker; uncontended in practice)
//   - receive (audio callback side) — real-time safe (lock-free `try_pop`
//     in the consumer closure, no mutex involved)

use ringbuf::traits::{Producer, Split};
use ringbuf::{HeapCons, HeapProd, HeapRb};

/// Maximum number of pending parameter changes.
///
/// This should be large enough to handle bulk operations such as
/// scene loading (~50+ parameter pushes) without dropping changes.
/// 256 entries at ~24 bytes each ≈ 6 KB — negligible overhead.
pub const PARAM_QUEUE_CAPACITY: usize = 256;

/// Error returned when the parameter queue is at capacity.
#[derive(Debug, Clone, Copy)]
pub struct ParamQueueFull;

/// Thread-safe sender for parameter changes.
///
/// Internally wraps `HeapProd` in a `std::sync::Mutex` only because
/// the ringbuf traits require `&mut self` on some operations. The ring
/// buffer's index operations are atomic (lock-free), so the inner
/// `Mutex` is uncontended — it purely satisfies the borrow checker.
///
/// `ParamSender` is `Send` + `Sync`, so it can be shared via `&` across
/// threads (e.g., in Tauri's `State`).
pub struct ParamSender {
    inner: std::sync::Mutex<HeapProd<(String, f32)>>,
}

impl ParamSender {
    /// Enqueue a parameter change.
    ///
    /// Returns `Err(ParamQueueFull)` if the ring buffer is full. The
    /// audio callback drains the queue before every `process_all` call,
    /// so this should only fill up during extreme bulk operations.
    pub fn send(&self, id: String, value: f32) -> Result<(), ParamQueueFull> {
        let mut inner = self.inner.lock().expect("ParamSender lock");
        inner.try_push((id, value)).map_err(|_| ParamQueueFull)
    }
}

/// The consumer end of the parameter channel, moved into the audio
/// callback and drained before each processing cycle.
pub type ParamConsumer = HeapCons<(String, f32)>;

/// Create a new parameter channel.
///
/// Returns a `(ParamSender, ParamConsumer)` pair. The sender is stored
/// in `AppState` for main-thread access; the consumer is passed into
/// `CpalAudioIO::start()` and moved into the audio callback closure.
pub fn param_channel() -> (ParamSender, ParamConsumer) {
    let rb = HeapRb::<(String, f32)>::new(PARAM_QUEUE_CAPACITY);
    let (producer, consumer) = rb.split();
    (ParamSender { inner: std::sync::Mutex::new(producer) }, consumer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ringbuf::traits::Consumer;

    #[test]
    fn send_and_receive() {
        let (tx, mut rx) = param_channel();
        tx.send("gain".into(), 0.5).unwrap();
        tx.send("master".into(), 0.8).unwrap();
        assert_eq!(rx.try_pop(), Some(("gain".into(), 0.5)));
        assert_eq!(rx.try_pop(), Some(("master".into(), 0.8)));
        assert_eq!(rx.try_pop(), None);
    }

    #[test]
    fn queue_full_error() {
        let small_capacity = 2;
        let rb = HeapRb::<(String, f32)>::new(small_capacity);
        let (producer, _consumer) = rb.split();
        let tx = ParamSender { inner: std::sync::Mutex::new(producer) };

        assert!(tx.send("a".into(), 1.0).is_ok());
        assert!(tx.send("b".into(), 2.0).is_ok());
        assert!(tx.send("c".into(), 3.0).is_err()); // full
    }

    #[test]
    fn param_sender_is_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<ParamSender>();
    }
}
