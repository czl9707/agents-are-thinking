export function generateBakedCSS(
  name: string,
  frames: string[],
  cycleLength: number,
): string {
  const className = name;
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

.${className}::before {
  content: "";
  animation: ${keyframeName} ${duration}ms steps(1) infinite;
  font-family: monospace;
}
</style>
<div class="${className}"></div>`;
}
