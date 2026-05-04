import { useEffectAnimation } from './hooks/useEffectAnimation';
import { useBakeEffect } from './hooks/useBakeEffect';
import { generateBakedCSS } from './utils/generateBakedCSS';
import { generateNpmSnippet } from './utils/generateNpmSnippet';
import { getEffectFamily } from './utils/effectFont';
import { CodeTabs } from './CodeTabs';
import { EFFECTS } from '@zane-chen/agents-are-thinking';
import type { EffectClass } from './types';
import s from './Drawer.module.css';

interface DrawerProps {
  EffectCls: EffectClass | null;
  effectIndex: number;
  effectName: string;
  open: boolean;
  onClose: () => void;
  onPrev: () => void;
  onNext: () => void;
}

export function Drawer({ EffectCls, effectIndex, effectName, open, onClose, onPrev, onNext }: DrawerProps) {
  const activeEffect = EffectCls ?? EFFECTS[0];
  const [frame] = useEffectAnimation(activeEffect);
  const baked = useBakeEffect(open ? EffectCls : null);
  const family = getEffectFamily(effectName);
  const pascalName = effectName ? toPascalCase(effectName) : '';
  const npm = pascalName ? generateNpmSnippet(effectName, pascalName) : { install: '', usage: '' };
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
          <div className={s.actions}>
            <button className={s.action} onClick={onPrev}>&larr;</button>
            <button className={s.action} onClick={onNext}>&rarr;</button>
            <button className={s.action} onClick={onClose}>&times;</button>
          </div>
        </div>
        <div className={s.scroll}>
          <div className={s.preview}>
            <span className={`effect-frame ${s.effect}`} data-family={family ?? undefined}>{frame}</span>
          </div>
          <div className={s.body}>
            <CodeTabs
              cssSnippet={cssSnippet}
              npmInstall={npm.install}
              npmUsage={npm.usage}
            />
          </div>
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
