const FONT_OVERRIDES: Record<string, string> = {
  dot: "'Fira Code Variable', monospace",
};

export function getEffectFamily(name: string): string | null {
  const prefix = name.split('-')[0];
  return FONT_OVERRIDES[prefix] ? prefix : null;
}

export function getEffectFontCSS(name: string): string {
  const prefix = name.split('-')[0];
  return FONT_OVERRIDES[prefix] ?? "'Cascadia Code Variable', monospace";
}
