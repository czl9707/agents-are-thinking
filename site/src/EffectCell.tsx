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
  scale: number;
}

export function EffectCell({ index, EffectCls, onClick, scale }: EffectCellProps) {
  const frame = useEffectAnimation(EffectCls);
  const name = EffectCls.name();

  return (
    <div
      className="effect-cell"
      onClick={onClick}
      style={{ transform: `scale(${1})` }}
    >
      <span className="effect-number">{String(index + 1).padStart(2, '0')}</span>
      <pre className="effect-frame" style={{ fontSize: `${14 * Math.max(scale, 0.6)}px` }}>{frame}</pre>
      <span className="effect-name">{name}</span>
    </div>
  );
}
