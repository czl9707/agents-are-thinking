import s from './Drawer.module.css';

interface DrawerProps {
  effectIndex: number;
  effectName: string;
  open: boolean;
  onClose: () => void;
}

export function Drawer({ effectIndex, effectName, open, onClose }: DrawerProps) {
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
        <div className={s.body}>
          <p className={s.soon}>coming soon</p>
        </div>
      </aside>
    </div>
  );
}
