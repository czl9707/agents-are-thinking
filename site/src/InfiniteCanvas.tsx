import { useRef, useState, useEffect } from 'react';
import { useDrag, useWheel, usePinch } from '@use-gesture/react';
import { EFFECTS } from '@zane-chen/agents-are-thinking';
import { EffectCell } from './EffectCell';

const CELL_W = 130;
const CELL_H = 90;
const GAP = 2;
const COLS = 8;
const TOTAL = EFFECTS.length;

function getVisibleCells(
  ox: number,
  oy: number,
  scale: number,
  vw: number,
  vh: number,
) {
  const cw = (CELL_W + GAP) * scale;
  const ch = (CELL_H + GAP) * scale;

  const startCol = Math.floor(-ox / cw) - 1;
  const endCol = Math.ceil((-ox + vw) / cw) + 1;
  const startRow = Math.floor(-oy / ch) - 1;
  const endRow = Math.ceil((-oy + vh) / ch) + 1;

  const cells: { col: number; row: number; effectIdx: number }[] = [];
  for (let r = startRow; r <= endRow; r++) {
    for (let c = startCol; c <= endCol; c++) {
      const linearIdx = r * COLS + c;
      const effectIdx = ((linearIdx % TOTAL) + TOTAL) % TOTAL;
      cells.push({ col: c, row: r, effectIdx });
    }
  }
  return cells;
}

interface InfiniteCanvasProps {
  onSelect: (index: number) => void;
}

export function InfiniteCanvas({ onSelect }: InfiniteCanvasProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const [offset, setOffset] = useState({ x: 0, y: 0 });
  const [scale, setScale] = useState(1);
  const [size, setSize] = useState({ w: window.innerWidth, h: window.innerHeight });

  useEffect(() => {
    const onResize = () => setSize({ w: window.innerWidth, h: window.innerHeight });
    window.addEventListener('resize', onResize);
    return () => window.removeEventListener('resize', onResize);
  }, []);

  useDrag(({ delta: [dx, dy], tap }) => {
    if (!tap) setOffset((o) => ({ x: o.x + dx, y: o.y + dy }));
  }, { target: containerRef, filterTaps: true });

  useWheel(({ event, delta: [_dx, dy] }) => {
    event.preventDefault();
    const factor = dy > 0 ? 0.9 : 1.1;
    setScale((s) => Math.min(3, Math.max(0.3, s * factor)));
  }, { target: containerRef, eventOptions: { passive: false } });

  usePinch(({ delta: [d] }) => {
    const factor = 1 + d * -0.01;
    setScale((s) => Math.min(3, Math.max(0.3, s * factor)));
  }, { target: containerRef });

  const cells = getVisibleCells(offset.x, offset.y, scale, size.w, size.h);

  return (
    <div
      ref={containerRef}
      style={{
        width: '100vw',
        height: '100vh',
        overflow: 'hidden',
        cursor: 'grab',
        position: 'relative',
        background: '#0a0a0a',
      }}
    >
      {cells.map(({ col, row, effectIdx }) => (
        <div
          key={`${col},${row}`}
          style={{
            position: 'absolute',
            left: offset.x + col * (CELL_W + GAP) * scale,
            top: offset.y + row * (CELL_H + GAP) * scale,
            width: CELL_W * scale,
            height: CELL_H * scale,
          }}
        >
          <EffectCell
            index={effectIdx}
            EffectCls={EFFECTS[effectIdx]}
            onClick={() => onSelect(effectIdx)}
            scale={scale}
          />
        </div>
      ))}
    </div>
  );
}
