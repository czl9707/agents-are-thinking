import { useEffect, useCallback } from 'react';

interface ModalProps {
  effectIndex: number;
  effectName: string;
  onClose: () => void;
}

export function Modal({ effectIndex, effectName, onClose }: ModalProps) {
  const handleKeyDown = useCallback(
    (e: KeyboardEvent) => {
      if (e.key === 'Escape') onClose();
    },
    [onClose]
  );

  useEffect(() => {
    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [handleKeyDown]);

  return (
    <div className="modal-backdrop" onClick={onClose}>
      <div className="modal-content" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <h2>
            <span className="modal-number">{String(effectIndex + 1).padStart(2, '0')}</span>
            {' '}{effectName}
          </h2>
          <button className="modal-close" onClick={onClose}>✕</button>
        </div>
        <div className="modal-body">
        </div>
      </div>
    </div>
  );
}
