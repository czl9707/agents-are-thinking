import { useRef, useEffect, useCallback, useState } from 'react';
import { useDrag, useWheel, usePinch } from '@use-gesture/react';
import { EFFECTS } from '@zane-chen/agents-are-thinking';
import { EffectCell } from './EffectCell';

const CELL_W = 130;
const CELL_H = 90;
const GAP = 2;
const COLS = 8;
const ROWS = Math.ceil(EFFECTS.length / COLS);
const TOTAL = EFFECTS.length;
const MAX_SCALE = 3;
const ZOOM_COMMIT_THRESHOLD = 0.25;
const ZOOM_TIMEOUT_MS = 400;
const LERP_SPEED = 0.15;

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

function lerp(a: number, b: number, t: number) {
  return a + (b - a) * t;
}

interface InfiniteCanvasProps {
  onSelect: (index: number) => void;
}

export function InfiniteCanvas({ onSelect }: InfiniteCanvasProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const rafRef = useRef<number>(0);

  const stateRef = useRef({
    ox: 0,
    oy: 0,
    displayScale: 1,
    targetScale: 1,
    committedScale: 1,
    vw: window.innerWidth,
    vh: window.innerHeight,
  });

  const [, setTick] = useState(0);
  const scheduleUpdate = useCallback(() => {
    cancelAnimationFrame(rafRef.current);
    rafRef.current = requestAnimationFrame(() => { setTick((t) => t + 1); });
  }, []);

  const zoomTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  const clampScale = useCallback((s: number) => {
    const { vw, vh } = stateRef.current;
    const minS = getMinScale(vw, vh);
    return Math.min(MAX_SCALE, Math.max(minS, s));
  }, []);

  const animate = useCallback(() => {
    const st = stateRef.current;
    const diff = Math.abs(st.displayScale - st.targetScale);
    if (diff > 0.001) {
      st.displayScale = lerp(st.displayScale, st.targetScale, LERP_SPEED);
      scheduleUpdate();
    } else {
      st.displayScale = st.targetScale;
    }
  }, [scheduleUpdate]);

  useEffect(() => {
    const onResize = () => {
      stateRef.current.vw = window.innerWidth;
      stateRef.current.vh = window.innerHeight;
      stateRef.current.targetScale = clampScale(stateRef.current.targetScale);
      scheduleUpdate();
    };
    window.addEventListener('resize', onResize);
    return () => window.removeEventListener('resize', onResize);
  }, [clampScale, scheduleUpdate]);

  useEffect(() => {
    const tick = () => {
      animate();
      rafRef.current = requestAnimationFrame(tick);
    };
    rafRef.current = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(rafRef.current);
  }, [animate]);

  useDrag(({ delta: [dx, dy], tap }) => {
    if (!tap) {
      stateRef.current.ox += dx;
      stateRef.current.oy += dy;
      scheduleUpdate();
    }
  }, { target: containerRef, filterTaps: true });

  useWheel(({ event, delta: [_dx, dy] }) => {
    event.preventDefault();
    const st = stateRef.current;
    const factor = dy > 0 ? 0.93 : 1.07;
    st.targetScale = clampScale(st.targetScale * factor);

    clearTimeout(zoomTimerRef.current!);
    zoomTimerRef.current = setTimeout(() => {
      const change = Math.abs(st.targetScale - st.committedScale) / st.committedScale;
      if (change < ZOOM_COMMIT_THRESHOLD) {
        st.targetScale = st.committedScale;
      } else {
        st.committedScale = st.targetScale;
      }
      scheduleUpdate();
    }, ZOOM_TIMEOUT_MS);

    scheduleUpdate();
  }, { target: containerRef, eventOptions: { passive: false } });

  usePinch(({ delta: [d] }) => {
    const st = stateRef.current;
    const factor = 1 + d * -0.01;
    st.targetScale = clampScale(st.targetScale * factor);
    st.committedScale = st.targetScale;
    scheduleUpdate();
  }, { target: containerRef });

  const st = stateRef.current;
  const scale = st.displayScale;
  const cells = getVisibleCells(st.ox, st.oy, scale, st.vw, st.vh);

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
            left: st.ox + col * (CELL_W + GAP) * scale,
            top: st.oy + row * (CELL_H + GAP) * scale,
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
