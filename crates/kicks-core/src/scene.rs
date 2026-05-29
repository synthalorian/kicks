use serde::{Deserialize, Serialize};

use crate::signal_chain::SignalChain;

/// A single performance scene — a named snapshot of the full signal chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub name: String,
    pub signal_chain: SignalChain,
}

/// Ordered collection of scenes for live performance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneCollection {
    pub scenes: Vec<Scene>,
    pub active_scene: Option<usize>,
}

impl SceneCollection {
    pub fn new() -> Self {
        Self {
            scenes: Vec::new(),
            active_scene: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.scenes.is_empty()
    }

    pub fn len(&self) -> usize {
        self.scenes.len()
    }

    /// Add a new scene at the end.
    pub fn add_scene(&mut self, name: String, signal_chain: SignalChain) {
        self.scenes.push(Scene { name, signal_chain });
    }

    /// Insert a scene at a specific index.
    pub fn insert_scene(&mut self, index: usize, name: String, signal_chain: SignalChain) {
        let idx = index.min(self.scenes.len());
        self.scenes.insert(idx, Scene { name, signal_chain });
    }

    /// Remove a scene by index. Returns the removed scene, or None.
    pub fn remove_scene(&mut self, index: usize) -> Option<Scene> {
        if index < self.scenes.len() {
            let removed = self.scenes.remove(index);
            // Adjust active_scene if it pointed past the removal
            if let Some(active) = self.active_scene {
                if active == index {
                    self.active_scene = if self.scenes.is_empty() {
                        None
                    } else {
                        Some(active.min(self.scenes.len() - 1))
                    };
                } else if active > index {
                    self.active_scene = Some(active - 1);
                }
            }
            Some(removed)
        } else {
            None
        }
    }

    /// Rename a scene at the given index.
    pub fn rename_scene(&mut self, index: usize, new_name: String) -> Option<()> {
        let scene = self.scenes.get_mut(index)?;
        scene.name = new_name;
        Some(())
    }

    /// Move a scene from one index to another.
    pub fn reorder_scene(&mut self, from: usize, to: usize) -> Option<()> {
        let len = self.scenes.len();
        if from >= len || to >= len || from == to {
            return None;
        }
        let scene = self.scenes.remove(from);
        let insert_at = to;
        self.scenes.insert(insert_at, scene);
        Some(())
    }

    /// Get a reference to a scene by index.
    pub fn get_scene(&self, index: usize) -> Option<&Scene> {
        self.scenes.get(index)
    }

    /// Get the active scene index.
    pub fn active_index(&self) -> Option<usize> {
        self.active_scene.filter(|&i| i < self.scenes.len())
    }

    /// Get the active scene.
    pub fn active_scene(&self) -> Option<&Scene> {
        self.active_index().and_then(|i| self.scenes.get(i))
    }

    /// Set the active scene index (clamped to valid range).
    pub fn set_active(&mut self, index: usize) {
        if index < self.scenes.len() {
            self.active_scene = Some(index);
        }
    }

    /// Move to the next scene (wraps around).
    pub fn next_scene(&mut self) -> Option<usize> {
        if self.scenes.is_empty() {
            return None;
        }
        let next = self
            .active_scene
            .map(|i| (i + 1) % self.scenes.len())
            .unwrap_or(0);
        self.active_scene = Some(next);
        Some(next)
    }

    /// Move to the previous scene (wraps around).
    pub fn prev_scene(&mut self) -> Option<usize> {
        if self.scenes.is_empty() {
            return None;
        }
        let prev = self
            .active_scene
            .map(|i| if i == 0 { self.scenes.len() - 1 } else { i - 1 })
            .unwrap_or(0);
        self.active_scene = Some(prev);
        Some(prev)
    }
}

impl Default for SceneCollection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signal_chain::SignalChain;

    fn test_chain() -> SignalChain {
        SignalChain::default()
    }

    #[test]
    fn test_empty_collection() {
        let coll = SceneCollection::new();
        assert!(coll.is_empty());
        assert_eq!(coll.len(), 0);
        assert!(coll.active_scene().is_none());
    }

    #[test]
    fn test_add_and_get_scene() {
        let mut coll = SceneCollection::new();
        coll.add_scene("Clean".into(), test_chain());
        coll.add_scene("Crunch".into(), test_chain());
        assert_eq!(coll.len(), 2);
        assert_eq!(coll.get_scene(0).unwrap().name, "Clean");
        assert_eq!(coll.get_scene(1).unwrap().name, "Crunch");
    }

    #[test]
    fn test_remove_scene_adjusts_active() {
        let mut coll = SceneCollection::new();
        coll.add_scene("A".into(), test_chain());
        coll.add_scene("B".into(), test_chain());
        coll.add_scene("C".into(), test_chain());
        coll.set_active(1); // B

        let removed = coll.remove_scene(1);
        assert_eq!(removed.unwrap().name, "B");
        assert_eq!(coll.len(), 2);
        // Active was 1, we removed at 1 → active should stay at 1 (now C)
        assert_eq!(coll.active_index(), Some(1));
        assert_eq!(coll.get_scene(1).unwrap().name, "C");
    }

    #[test]
    fn test_remove_last_scene_clears_active() {
        let mut coll = SceneCollection::new();
        coll.add_scene("Only".into(), test_chain());
        coll.set_active(0);
        coll.remove_scene(0);
        assert!(coll.active_scene().is_none());
        assert!(coll.is_empty());
    }

    #[test]
    fn test_rename_scene() {
        let mut coll = SceneCollection::new();
        coll.add_scene("Old".into(), test_chain());
        coll.rename_scene(0, "New".into());
        assert_eq!(coll.get_scene(0).unwrap().name, "New");
    }

    #[test]
    fn test_reorder_scene() {
        let mut coll = SceneCollection::new();
        coll.add_scene("A".into(), test_chain());
        coll.add_scene("B".into(), test_chain());
        coll.add_scene("C".into(), test_chain());

        coll.reorder_scene(0, 2); // Move A to position 2
        assert_eq!(coll.get_scene(0).unwrap().name, "B");
        assert_eq!(coll.get_scene(1).unwrap().name, "C");
        assert_eq!(coll.get_scene(2).unwrap().name, "A");
    }

    #[test]
    fn test_next_prev_scene() {
        let mut coll = SceneCollection::new();
        coll.add_scene("A".into(), test_chain());
        coll.add_scene("B".into(), test_chain());
        coll.add_scene("C".into(), test_chain());

        assert_eq!(coll.next_scene(), Some(0)); // no active → 0
        assert_eq!(coll.next_scene(), Some(1)); // 0 → 1
        assert_eq!(coll.next_scene(), Some(2)); // 1 → 2
        assert_eq!(coll.next_scene(), Some(0)); // 2 → 0 (wrap)
        assert_eq!(coll.prev_scene(), Some(2)); // 0 → 2 (wrap back)

        let mut empty: SceneCollection = SceneCollection::new();
        assert!(empty.next_scene().is_none());
        assert!(empty.prev_scene().is_none());
    }

    #[test]
    fn test_insert_scene() {
        let mut coll = SceneCollection::new();
        coll.add_scene("A".into(), test_chain());
        coll.add_scene("C".into(), test_chain());
        coll.insert_scene(1, "B".into(), test_chain());

        assert_eq!(coll.len(), 3);
        assert_eq!(coll.get_scene(0).unwrap().name, "A");
        assert_eq!(coll.get_scene(1).unwrap().name, "B");
        assert_eq!(coll.get_scene(2).unwrap().name, "C");
    }

    #[test]
    fn test_set_active_clamped() {
        let mut coll = SceneCollection::new();
        coll.add_scene("A".into(), test_chain());
        coll.set_active(5); // out of bounds → ignored
        assert_eq!(coll.active_index(), None);

        coll.set_active(0);
        assert_eq!(coll.active_index(), Some(0));
    }
}
