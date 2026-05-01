interface DrawerProps {
  effectIndex: number;
  effectName: string;
  open: boolean;
  onClose: () => void;
}

export function Drawer({ effectIndex, effectName, open, onClose }: DrawerProps) {
  return (
    <div
      className="drawer-backdrop"
      data-open={open ? '' : undefined}
      onClick={onClose}
    >
      <aside
        className="drawer-panel"
        data-open={open ? '' : undefined}
        onClick={(e) => e.stopPropagation()}
      >
        <div className="drawer-header">
          <span className="drawer-title">
            <span className="drawer-number">{String(effectIndex + 1).padStart(2, '0')}</span>
            {' '}{effectName}
          </span>
          <button className="drawer-close" onClick={onClose}>✕</button>
        </div>
        <div className="drawer-body">
          <p className="drawer-soon">coming soon</p>
        </div>
      </aside>
    </div>
  );
}
