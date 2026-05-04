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

export function getEffectFontNote(name: string): string {
  const prefix = name.split('-')[0];
  switch (prefix) {
    case 'dot':
      return 'Dot-family effects (· ∘ • ○ ●) render best with Fira Code — most other monospace fonts misalign or substitute these glyphs.';
    case 'vblock':
      return 'Uses vertical block elements (▏▎▍▌▋▊▉█). Cascadia Code handles them well; Fira Code does not.';
    case 'bar':
      return 'Uses bar characters (▁▂▃▄▅▆▇█). Most monospace fonts render these correctly.';
    case 'braille':
      return 'Uses braille patterns. Any monospace font with Unicode braille support works.';
    case 'shade':
      return 'Uses shade characters (░▒▓█). Most monospace fonts support these.';
    case 'square':
      return 'Uses geometric shapes (·□■). Most monospace fonts support these.';
    default:
      return 'Use a monospace font for consistent character alignment.';
  }
}
