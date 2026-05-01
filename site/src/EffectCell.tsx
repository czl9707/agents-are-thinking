import { useRef, useCallback } from 'react';
import { useEffectAnimation } from './hooks/useEffectAnimation';
import s from './EffectCell.module.css';

const VERBS = [
  "thinking",
  "analyzing",
  "cooking",
  "brewing",
  "computing",
  "pondering",
  "dreaming",
  "processing",
  "vibing",
  "wondering",
  "imagining",
  "calculating",
  "marinating",
  "reflecting",
  "absorbing",
  "channeling",
  "meditating",
  "manifesting",
  "deciphering",
  "daydreaming",
  "ideating",
  "brainstorming",
  "philosophizing",
  "percolating",
  "simmering",
  "distilling",
  "unraveling",
  "connecting",
  "orchestrating",
  "choreographing",
  "architecting",
  "composing",
  "sculpting",
  "weaving",
  "stewing",
  "fermenting",
  "incubating",
  "crystallizing",
  "galvanizing",
  "reassembling",
  "reconfiguring",
  "calibrating",
  "harmonizing",
  "fractalizing",
  "quantizing",
  "transmuting",
  "metamorphosing",
  "osmosing",
  "photosynthesizing",
  "communing",
];

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
  const [frame, frameCount] = useEffectAnimation(EffectCls);
  const name = EffectCls.name();
  const startRef = useRef<{ x: number; y: number } | null>(null);

  const verb = VERBS[index % VERBS.length];
  const dotCount = Math.floor(frameCount / 10) % 4;
  const dots = ".".repeat(dotCount);

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
      className={s.cell}
      onPointerDown={handlePointerDown}
      onPointerUp={handlePointerUp}
    >
      <div className={s.header}>
        <span className={s.number}>{String(index + 1).padStart(2, '0')}</span>
        <span className={s.name}>{name}</span>
      </div>
      <div className={s.frame}>
        {frame.split('\n').map((line, li) => (
          <div key={li}>
            {Array.from(line).map((char, ci) => (
              <span key={ci} className={s.char}>{char}</span>
            ))}
          </div>
        ))}
        <span className={s.verb}>{verb}{dots}</span>
      </div>
    </div>
  );
}
