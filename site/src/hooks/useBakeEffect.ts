import { useMemo } from 'react';

interface WasmEffect {
  step(): string;
  free(): void;
}

interface EffectClass {
  new (): WasmEffect;
  cycleLength(): number;
}

interface BakedEffect {
  frames: string[];
  cycleLength: number;
}

export function useBakeEffect(EffectCls: EffectClass | null): BakedEffect | null {
  return useMemo(() => {
    if (!EffectCls) return null;

    const instance = new EffectCls();
    const len = EffectCls.cycleLength();
    const frames: string[] = [];
    for (let i = 0; i < len; i++) {
      frames.push(instance.step());
    }
    instance.free();
    return { frames, cycleLength: len };
  }, [EffectCls]);
}
