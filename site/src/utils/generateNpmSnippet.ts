import { getEffectFontCSS } from './effectFont';

export function generateNpmSnippet(effectName: string, pascalName: string): { install: string; usage: string } {
  const fontFamily = getEffectFontCSS(effectName);
  return {
    install: `npm install @zane-chen/agents-are-thinking`,
    usage: `import { useEffect, useRef, useState } from 'react';
import { ${pascalName} } from '@zane-chen/agents-are-thinking';

export function ThinkingIndicator() {
  const [frame, setFrame] = useState('');
  const effectRef = useRef<${pascalName} | null>(null);

  useEffect(() => {
    const effect = new ${pascalName}();
    effectRef.current = effect;
    let last = 0;
    let raf: number;
    const tick = (t: number) => {
      if (t - last >= 100) {
        setFrame(effect.step());
        last = t;
      }
      raf = requestAnimationFrame(tick);
    };
    raf = requestAnimationFrame(tick);
    return () => {
      cancelAnimationFrame(raf);
      effect.free();
    };
  }, []);

  return (
    <span style={{
      fontFamily: ${fontFamily},
      display: 'inline-block',
      whiteSpace: 'pre-wrap',
      lineHeight: '1',
      letterSpacing: '0px',
      minWidth: '9ch',
    }}>
      {frame}
    </span>
  );
}`,
  };
}
