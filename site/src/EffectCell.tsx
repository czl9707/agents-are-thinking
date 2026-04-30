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

  return (
    <div
      className="effect-cell"
      onClick={onClick}
    >
      <span className="effect-number">{String(index + 1).padStart(2, '0')}</span>
      <pre className="effect-frame">{frame}</pre>
      <span className="effect-name">{name}</span>
    </div>
  );
}
