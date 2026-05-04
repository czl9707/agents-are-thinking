import { getEffectFontCSS } from './effectFont';

const EFFECT_FRAME_CSS = (name: string) => `\
    display: inline-block;
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
  
  return `<style>
@keyframes ${keyframeName} {
${keyframeSteps}
  100% { content: "${frames[0]}"; }
}

.${name} {
  &::before {
${EFFECT_FRAME_CSS(name)}
    animation: ${keyframeName} ${duration}ms steps(1) infinite;
  }
}

</style>
<span class="${name}"/>`;
}
