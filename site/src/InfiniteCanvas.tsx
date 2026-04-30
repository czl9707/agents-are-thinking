import { useEffect, useState } from 'react';
import { TransformWrapper, TransformComponent, useTransformEffect } from 'react-zoom-pan-pinch';
import { EFFECTS } from '@zane-chen/agents-are-thinking';
import { EffectCell } from './EffectCell';

const CELL_W = 130;
const CELL_H = 90;
const GAP = 2;
const COLS = 8;
const ROWS = Math.ceil(EFFECTS.length / COLS);
const TOTAL = EFFECTS.length;
const MAX_SCALE = 3;

function getMinScale(vw: number, vh: number) {
  const cw = CELL_W + GAP;
  const ch = CELL_H + GAP;
  const minByW = vw / (COLS * cw * 2);
  const minByH = vh / (ROWS * ch * 2);
  return Math.max(minByW, minByH);
}

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

interface VirtualGridProps {
  onSelect: (index: number) => void;
}

function VirtualGrid({ onSelect }: VirtualGridProps) {
  const [transform, setTransform] = useState({ x: 0, y: 0, scale: 1 });
  const [viewport, setViewport] = useState({ w: window.innerWidth, h: window.innerHeight });

  useTransformEffect(({ state }) => {
    setTransform({ x: state.positionX, y: state.positionY, scale: state.scale });
  });

  useEffect(() => {
    const onResize = () => setViewport({ w: window.innerWidth, h: window.innerHeight });
    window.addEventListener('resize', onResize);
    return () => window.removeEventListener('resize', onResize);
  }, []);

  const { x, y, scale } = transform;
  const cells = getVisibleCells(x, y, scale, viewport.w, viewport.h);

  return (
    <div style={{ position: 'relative', width: 0, height: 0 }}>
      {cells.map(({ col, row, effectIdx }) => (
        <div
          key={`${col},${row}`}
          style={{
            position: 'absolute',
            left: col * (CELL_W + GAP) * scale,
            top: row * (CELL_H + GAP) * scale,
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

interface InfiniteCanvasProps {
  onSelect: (index: number) => void;
}

export function InfiniteCanvas({ onSelect }: InfiniteCanvasProps) {
  const [viewport, setViewport] = useState({ w: window.innerWidth, h: window.innerHeight });

  useEffect(() => {
    const onResize = () => setViewport({ w: window.innerWidth, h: window.innerHeight });
    window.addEventListener('resize', onResize);
    return () => window.removeEventListener('resize', onResize);
  }, []);

  const minScale = getMinScale(viewport.w, viewport.h);

  return (
    <TransformWrapper
      minScale={minScale}
      maxScale={MAX_SCALE}
      initialScale={1}
      centerOnInit
      smooth
    >
      <TransformComponent
        wrapperStyle={{ width: '100vw', height: '100vh', background: '#0a0a0a', cursor: 'grab' }}
      >
        <VirtualGrid onSelect={onSelect} />
      </TransformComponent>
    </TransformWrapper>
  );
}
