import { useRef, useCallback } from 'react';
import { useEffectAnimation } from './hooks/useEffectAnimation';

interface WasmEffect {
  step(): string;
  free(): void;
}

interface EffectClass {
  new (): WasmEffect;
  name(): string;
  description(): string;
}

interface EffectCellProps {
  index: number;
  EffectCls: EffectClass;
  onClick: () => void;
}

export function EffectCell({ index, EffectCls, onClick }: EffectCellProps) {
  const frame = useEffectAnimation(EffectCls);
  const name = EffectCls.name();
  const startRef = useRef<{ x: number; y: number } | null>(null);

  const handlePointerDown = useCallback((e: React.PointerEvent) => {
    startRef.current = { x: e.clientX, y: e.clientY };
  }, []);

  const handlePointerUp = useCallback((e: React.PointerEvent) => {
    if (!startRef.current) return;
    const dx = e.clientX - startRef.current.x;
    const dy = e.clientY - startRef.current.y;
    startRef.current = null;
    if (Math.abs(dx) < 5 && Math.abs(dy) < 5) {
      onClick();
    }
  }, [onClick]);

  return (
    <div
      className="effect-cell"
      onPointerDown={handlePointerDown}
      onPointerUp={handlePointerUp}
      style={{ transform: `scale(${1})` }}
    >
      <span className="effect-number">{String(index + 1).padStart(2, '0')}</span>
      <pre className="effect-frame">{frame}</pre>
      <span className="effect-name">{name}</span>
    </div>
  );
}
