import { ArrowLeftIcon, ArrowRightIcon, Cross2Icon } from '@radix-ui/react-icons';
import { useEffectAnimation } from './hooks/useEffectAnimation';
import { getEffectFamily, getEffectFontNote } from './utils/effectFont';
import { CodeTabs } from './CodeTabs';
import { EFFECTS } from '@zane-chen/agents-are-thinking';
import type { EffectClass } from './types';
import { useMemo } from 'react';
import b from './Button.module.css';
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
  const family = getEffectFamily(effectName);
  const fontNote = getEffectFontNote(effectName);

  const tabs = useMemo(() => (
    <div className={s.body}>
      <CodeTabs
        effectName={effectName}
        EffectCls={EffectCls}
      />
    </div>
  ), [effectIndex])

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
            <button className={`${b.btn} ${b.square}`} onClick={onPrev}><ArrowLeftIcon /></button>
            <button className={`${b.btn} ${b.square}`} onClick={onNext}><ArrowRightIcon /></button>
            <button className={`${b.btn} ${b.square}`} onClick={onClose}><Cross2Icon /></button>
          </div>
        </div>
        <div className={s.content}>
          <div className={s.preview}>
            <span className={`effect-frame ${s.effect}`} data-family={family ?? undefined}>{frame}</span>
          </div>
          <p className={s.note}>{fontNote}</p>
          {tabs}
        </div>
      </aside>
    </div>
  );
}
