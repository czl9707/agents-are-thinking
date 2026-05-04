import { useEffect, useRef, useState } from 'react';
import type { WasmEffect, EffectClass } from '../types';

const FPS = 10;
const INTERVAL = 1000 / FPS;

export function useEffectAnimation(EffectCls: EffectClass): [string, number] {
  const [frame, setFrame] = useState('');
  const [count, setCount] = useState(0);
  const instanceRef = useRef<WasmEffect | null>(null);

  useEffect(() => {
    const instance = new EffectCls();
    instanceRef.current = instance;

    let lastTime = 0;
    let n = 0;
    let rafId: number;

    const tick = (time: number) => {
      if (time - lastTime >= INTERVAL) {
        setFrame(instance.step());
        n++;
        setCount(n);
        lastTime = time;
      }
      rafId = requestAnimationFrame(tick);
    };

    rafId = requestAnimationFrame(tick);

    return () => {
      cancelAnimationFrame(rafId);
      instance.free();
    };
  }, [EffectCls]);

  return [frame, count];
}
