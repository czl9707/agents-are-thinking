import { useEffectAnimation } from './hooks/useEffectAnimation';
import { useBakeEffect } from './hooks/useBakeEffect';
import { generateBakedCSS } from './utils/generateBakedCSS';
import { generateNpmSnippet } from './utils/generateNpmSnippet';
import { CodeTabs } from './CodeTabs';
import { EFFECTS } from '@zane-chen/agents-are-thinking';
import s from './Drawer.module.css';

interface WasmEffect {
  step(): string;
  free(): void;
}

interface EffectClass {
  new (): WasmEffect;
  name(): string;
  cycleLength(): number;
}

interface DrawerProps {
  EffectCls: EffectClass | null;
  effectIndex: number;
  effectName: string;
  open: boolean;
  onClose: () => void;
}

export function Drawer({ EffectCls, effectIndex, effectName, open, onClose }: DrawerProps) {
  const activeEffect = EffectCls ?? EFFECTS[0];
  const [frame] = useEffectAnimation(activeEffect);
  const baked = useBakeEffect(open ? EffectCls : null);
  const pascalName = effectName ? toPascalCase(effectName) : '';
  const npm = pascalName ? generateNpmSnippet(pascalName) : { install: '', usage: '' };
  const cssSnippet = baked ? generateBakedCSS(effectName, baked.frames, baked.cycleLength) : '';

  return (
    <div
      className={s.backdrop}
      data-open={open ? '' : undefined}
      onClick={onClose}
    >
      <aside
        className={s.panel}
        data-open={open ? '' : undefined}
        onClick={(e) => e.stopPropagation()}
      >
        <div className={s.header}>
          <span className={s.title}>
            <span className={s.number}>{String(effectIndex + 1).padStart(2, '0')}</span>
            {' '}{effectName}
          </span>
          <button className={s.close} onClick={onClose}>✕</button>
        </div>
        <div className={s.preview}>
          <div className={s.previewFrame}>
            {Array.from(frame).map((char, i) => (
              <span key={i} className={s.char}>{char}</span>
            ))}
          </div>
        </div>
        <div className={s.body}>
          <CodeTabs
            cssSnippet={cssSnippet}
            npmInstall={npm.install}
            npmUsage={npm.usage}
          />
        </div>
      </aside>
    </div>
  );
}

function toPascalCase(kebab: string): string {
  return kebab
    .split('-')
    .map(part => part.charAt(0).toUpperCase() + part.slice(1))
    .join('');
}
