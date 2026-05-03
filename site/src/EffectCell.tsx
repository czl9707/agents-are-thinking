import { useRef } from 'react';
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
  const verb = VERBS[index % VERBS.length];
  const dotCount = Math.floor(frameCount / 10) % 4;
  const dots = ".".repeat(dotCount);
  const downRef = useRef({ x: 0, y: 0 });

  return (
    <div
      className={s.cell}
      onPointerDown={(e) => { downRef.current = { x: e.clientX, y: e.clientY }; }}
      onClick={(e) => {
        const dx = e.clientX - downRef.current.x;
        const dy = e.clientY - downRef.current.y;
        if (dx * dx + dy * dy < 25) onClick();
      }}
    >
      <div className={s.header}>
        <span className={s.number}>{String(index + 1).padStart(2, '0')}</span>
        <span className={s.name}>{name}</span>
      </div>
      <div className={s.frame}>
        {/* {Array.from(frame).map((char, ci) => (
          <span key={ci} className={s.char}>{char}</span>
        ))} */}
        <span className={s.effect}>{frame}</span>
        <span className={s.verb}>{verb}{dots}</span>
      </div>
    </div>
  );
}
