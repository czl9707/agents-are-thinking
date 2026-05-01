import { useEffect, useState } from 'react';
import { TransformWrapper, TransformComponent, useTransformEffect } from 'react-zoom-pan-pinch';
import { EFFECTS } from '@zane-chen/agents-are-thinking';
import { EffectCell } from './EffectCell';

const CELL_W = 160;
const CELL_H = 120;
const COLS = 7;
const TOTAL = EFFECTS.length;
const MAX_SCALE = 2;
const MIN_SCALE = 0.8;

function getVisibleCells(
  ox: number,
  oy: number,
  scale: number,
  vw: number,
  vh: number,
) {
  const cw = CELL_W * scale;
  const ch = CELL_H * scale;

  const BUFFER = 3;

  const startCol = Math.floor(-ox / cw) - BUFFER;
  const endCol = Math.ceil((-ox + vw) / cw) + BUFFER;
  const startRow = Math.floor(-oy / ch) - BUFFER;
  const endRow = Math.ceil((-oy + vh) / ch) + BUFFER;

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
            left: col * CELL_W,
            top: row * CELL_H,
            width: CELL_W,
            height: CELL_H,
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
  return (
    <TransformWrapper
      minScale={MIN_SCALE}
      maxScale={MAX_SCALE}
      zoomAnimation={{disabled: false, animationType: "easeOut"}}
      initialScale={1}
      wheel={{ step: 0.001 }}
      limitToBounds={false}
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
