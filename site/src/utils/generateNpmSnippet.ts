export function generateNpmSnippet(effectName: string): { install: string; usage: string } {
  return {
    install: `npm install @zane-chen/agents-are-thinking`,
    usage: `import { ${effectName} } from '@zane-chen/agents-are-thinking';

const effect = new ${effectName}();
setInterval(() => {
  process.stdout.write('\\r' + effect.step());
}, 100);`,
  };
}
