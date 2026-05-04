import { getEffectFontCSS } from './effectFont';

const EFFECT_FRAME_CSS = (name: string) => `  display: inline-block;
  min-width: 9ch;
  max-width: 9ch;
  white-space: pre-wrap;
  line-height: 1;
  letter-spacing: 0px;
  font-family: ${getEffectFontCSS(name)};`;

export function generateBakedCSS(
  name: string,
  frames: string[],
  cycleLength: number,
): string {
  const keyframeName = name;
  const duration = cycleLength * 100;

  const keyframeSteps = frames.map((frame, i) => {
    const pct = ((i / frames.length) * 100).toFixed(2);
    return `  ${pct}% { content: "${frame}"; }`;
  }).join('\n');

  const fontFamily = getEffectFontCSS(name);
  const fontNote = fontFamily.includes('Fira Code')
    ? `\n/* Note: dot-family effects render best with "Fira Code".\n   Or any monospace font with good Unicode dot glyph coverage (· ∘ • ○ ●). */`
    : `\n/* Font: "Cascadia Code" font render best\n   Or any monospace font with good coverage over this char sets. */`;

  return `<style>${fontNote}
@keyframes ${keyframeName} {
${keyframeSteps}
  100% { content: "${frames[0]}"; }
}

.${name} {
  &::before {
    ${EFFECT_FRAME_CSS(name)}
    content: "";
    animation: ${keyframeName} ${duration}ms steps(1) infinite;
}

</style>
<span class="${name}"></span>`;
}
