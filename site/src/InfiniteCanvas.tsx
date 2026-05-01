import { useEffect, useState } from 'react';
import { TransformWrapper, TransformComponent, useTransformEffect } from 'react-zoom-pan-pinch';
import { EFFECTS } from '@zane-chen/agents-are-thinking';
import { EffectCell } from './EffectCell';

const CELL_W = 300;
const CELL_H = 125;
const COLS = 7;
const TOTAL = EFFECTS.length;
const MAX_SCALE = 2;
const MIN_SCALE = 1;

function getVisibleCells(
  ox: number,
  oy: number,
  scale: number,
  vw: number,
  vh: number,
) {
  const cw = CELL_W * scale;
  const ch = CELL_H * scale;


  const startCol = Math.floor(-ox / cw);
  const endCol = Math.ceil((-ox + vw) / cw);
  const startRow = Math.floor(-oy / ch);
  const endRow = Math.ceil((-oy + vh) / ch);

  const cells: { col: number; row: number; effectIdx: number }[] = [];
  for (let r = startRow; r <= endRow; r++) {
    for (let c = startCol; c <= endCol; c++) {
      const linearIdx = r * COLS + c;
      const effectIdx = ((linearIdx % TOTAL) + TOTAL) % TOTAL;
      cells.push({ col: c, row: r, effectIdx });
    }
  }
  return { cells, startCol, endCol, startRow, endRow };
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
  const { cells, startCol, endCol, startRow, endRow } = getVisibleCells(x, y, scale, viewport.w, viewport.h);

  return (
    <div style={{ position: 'relative', width: 0, height: 0 }}>
      <div style={{
        position: 'absolute',
        left: startCol * CELL_W,
        top: startRow * CELL_H,
        width: (endCol - startCol + 1) * CELL_W,
        height: (endRow - startRow + 1) * CELL_H,
        backgroundImage: GRID_SVG,
        backgroundRepeat: 'repeat',
        backgroundSize: `${CELL_W}px ${CELL_H}px`,
        pointerEvents: 'none',
      }} />
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
          />
        </div>
      ))}
    </div>
  );
}

const GRID_SVG = `url("data:image/svg+xml,${encodeURIComponent(
  `<svg xmlns='http://www.w3.org/2000/svg' width='${CELL_W}' height='${CELL_H}'>` +
  `<rect x='0' y='0' width='${CELL_W}' height='${CELL_H}' fill='none' stroke='rgba(128,128,128,0.4)' stroke-width='1'/>` +
  `<path d='M0 0v6M0 0h6M${CELL_W} 0v6M${CELL_W} 0h-6M0 ${CELL_H}v-6M0 ${CELL_H}h6M${CELL_W} ${CELL_H}v-6M${CELL_W} ${CELL_H}h-6'` +
  ` stroke='rgb(128,128,128)' stroke-width='1' fill='none'/>` +
  `</svg>`
)}")`;

interface InfiniteCanvasProps {
  onSelect: (index: number) => void;
}

export function InfiniteCanvas({ onSelect }: InfiniteCanvasProps) {
  return (
    <TransformWrapper
      minScale={MIN_SCALE}
      maxScale={MAX_SCALE}
      zoomAnimation={{disabled: false, animationType: "easeOut"}}
      wheel={{ step: 0.001 }}
      limitToBounds={false}
      centerOnInit
      smooth
    >
      <TransformComponent
        wrapperStyle={{
          width: '100vw',
          height: '100vh',
          cursor: 'grab',
        }}
      >
        <VirtualGrid onSelect={onSelect} />
      </TransformComponent>
    </TransformWrapper>
  );
}
