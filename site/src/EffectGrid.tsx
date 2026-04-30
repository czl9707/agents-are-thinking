import { EFFECTS } from '@zane-chen/agents-are-thinking';
import { EffectCell } from './EffectCell';

interface EffectGridProps {
  onSelect: (index: number) => void;
}

export function EffectGrid({ onSelect }: EffectGridProps) {
  return (
    <div className="effect-grid">
      {EFFECTS.map((EffectCls, i) => (
        <EffectCell
          key={i}
          index={i}
          EffectCls={EffectCls}
          onClick={() => onSelect(i)}
        />
      ))}
    </div>
  );
}
