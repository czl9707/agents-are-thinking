const FONT_OVERRIDES: Record<string, string> = {
  dot: "'Fira Code', monospace",
};

export function getEffectFamily(name: string): string | null {
  const prefix = name.split('-')[0];
  return FONT_OVERRIDES[prefix] ? prefix : null;
}

export function getEffectFontCSS(name: string): string {
  const prefix = name.split('-')[0];
  return FONT_OVERRIDES[prefix] ?? "'Cascadia Code', monospace";
}

export function getEffectFontNote(name: string): string {
  const prefix = name.split('-')[0];
  switch (prefix) {
    case 'dot':
      return 'Uses dot-family elements (· ∘ • ○ ●). Rendered best with Fira Code font familiy — most other monospace fonts misalign.';
    case 'vblock':
      return 'Uses vertical block elements (▏▎▍▌▋▊▉█). Cascadia Code font familiy handles them well. (Fira Code misaligns).';
    case 'bar':
      return 'Uses bar characters (▁▂▃▄▅▆▇█). Cascadia Code font familiy handles them well.';
    case 'braille':
      return 'Uses braille patterns. Cascadia Code font familiy handles them well.';
    case 'shade':
      return 'Uses shade characters (░▒▓█). Cascadia Code font familiy handles them well.';
    case 'square':
      return 'Uses geometric shapes (·□■). Cascadia Code font familiy handles them well.';
    default:
      return 'Use a monospace font for consistent character alignment.';
  }
}
