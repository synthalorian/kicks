import { useState, useCallback, useEffect } from 'react';
import { useEngineStore } from '../../stores/engineStore';
import { PedalSlot } from './PedalSlot';

export function PedalBoard() {
  const { chain, loading, fetchChain, toggleSlot, updateSlot, moveSlot } = useEngineStore();
  const [draggingIndex, setDraggingIndex] = useState<number | null>(null);
  const [dropTargetIndex, setDropTargetIndex] = useState<number | null>(null);

  useEffect(() => {
    fetchChain();
    // Listen for amp presets being applied and refetch chain
    const handler = () => fetchChain();
    window.addEventListener('amp-preset-applied', handler);
    return () => window.removeEventListener('amp-preset-applied', handler);
  }, [fetchChain]);

  const handleToggle = useCallback(
    async (slotId: string) => {
      await toggleSlot(slotId);
    },
    [toggleSlot],
  );

  const handleParamChange = useCallback(
    async (slotId: string, param: string, value: number) => {
      await updateSlot(slotId, { parameters: { [param]: value } });
    },
    [updateSlot],
  );

  const handleDragStart = useCallback(
    (index: number) => (e: React.DragEvent) => {
      e.dataTransfer.effectAllowed = 'move';
      e.dataTransfer.setData('text/plain', String(index));
      setDraggingIndex(index);
    },
    [],
  );

  const handleDragOver = useCallback(
    (index: number) => (e: React.DragEvent) => {
      e.preventDefault();
      e.dataTransfer.dropEffect = 'move';
      setDropTargetIndex(index);
    },
    [],
  );

  const handleDrop = useCallback(
    (targetIndex: number) => async (e: React.DragEvent) => {
      e.preventDefault();
      setDraggingIndex(null);
      setDropTargetIndex(null);
      const fromIdx = parseInt(e.dataTransfer.getData('text/plain'), 10);
      if (isNaN(fromIdx) || fromIdx === targetIndex) return;

      try {
        await moveSlot(fromIdx, targetIndex);
      } catch (err) {
        console.error('Failed to move slot:', err);
      }
    },
    [moveSlot],
  );

  const handleDragEnd = useCallback(() => {
    setDraggingIndex(null);
    setDropTargetIndex(null);
  }, []);

  const slots = chain?.slots ?? [];

  if (loading && slots.length === 0) {
    return <div className="text-[var(--text-muted)] text-sm">Loading signal chain...</div>;
  }

  return (
    <div
      className="flex gap-3 overflow-x-auto pb-2"
      onDragEnd={handleDragEnd}
      onDragLeave={() => setDropTargetIndex(null)}
    >
      {slots.map((slot, i) => (
        <div key={slot.id} className="relative flex-shrink-0">
          {/* Drop indicator */}
          {dropTargetIndex === i && draggingIndex !== null && draggingIndex !== i && (
            <div className="absolute -left-1 top-0 bottom-0 w-0.5 bg-[var(--accent)] z-10 rounded" />
          )}
          <PedalSlot
            slot={slot}
            index={i}
            isDragging={draggingIndex === i}
            onToggle={() => handleToggle(slot.id)}
            onParamChange={(param, value) => handleParamChange(slot.id, param, value)}
            onDragStart={handleDragStart(i)}
            onDragOver={handleDragOver(i)}
            onDrop={handleDrop(i)}
          />
        </div>
      ))}
    </div>
  );
}
