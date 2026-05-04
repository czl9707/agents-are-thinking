export interface WasmEffect {
  step(): string;
  free(): void;
}

export interface EffectClass {
  new (): WasmEffect;
  name(): string;
  description(): string;
  cycleLength(): number;
}
