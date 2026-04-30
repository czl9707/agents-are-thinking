import { useEffect, useRef, useState } from 'react';

const FPS = 10;
const INTERVAL = 1000 / FPS;

interface WasmEffect {
  step(): string;
  free(): void;
}

interface EffectClass {
  new (): WasmEffect;
}

export function useEffectAnimation(EffectCls: EffectClass) {
  const [frame, setFrame] = useState('');
  const instanceRef = useRef<WasmEffect | null>(null);

  useEffect(() => {
    const instance = new EffectCls();
    instanceRef.current = instance;

    let lastTime = 0;
    let rafId: number;

    const tick = (time: number) => {
      if (time - lastTime >= INTERVAL) {
        setFrame(instance.step());
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

  return frame;
}
