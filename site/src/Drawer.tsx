import { ArrowLeftIcon, ArrowRightIcon, Cross2Icon } from '@radix-ui/react-icons';
import { useEffectAnimation } from './hooks/useEffectAnimation';
import { getEffectFamily, getEffectFontNote } from './utils/effectFont';
import { CodeTabs } from './CodeTabs';
import { EFFECTS } from '@zane-chen/agents-are-thinking';
import { useMemo } from 'react';
import b from './Button.module.css';
import s from './Drawer.module.css';

interface DrawerProps {
  effectIndex: number;
  open: boolean;
  onClose: () => void;
  onPrev: () => void;
  onNext: () => void;
}

export function Drawer({ effectIndex, open, onClose, onPrev, onNext }: DrawerProps) {
  const activeEffect = EFFECTS[effectIndex];
  const effectName = activeEffect.name();
  const fontNote = getEffectFontNote(effectName);

  const tabs = useMemo(() => (
    <div className={s.body}>
      <CodeTabs
        effectName={effectName}
        EffectCls={EFFECTS[effectIndex]}
      />
    </div>
  ), [effectIndex])

  console.log(effectName);


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
            <Effect effectIndex={effectIndex}/>
          </div>
          <p className={s.note}>{fontNote}</p>
          {tabs}
        </div>
      </aside>
    </div>
  );
}

function Effect({effectIndex}: {effectIndex: number}){
  const activeEffect = EFFECTS[effectIndex];
  const effectName = activeEffect.name();
  const [frame] = useEffectAnimation(activeEffect);
  const family = getEffectFamily(effectName);

  return (
    <span className={`effect-frame ${s.effect}`} data-family={family ?? undefined}>{frame}</span>
  )
}