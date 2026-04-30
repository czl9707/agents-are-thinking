import * as Dialog from '@radix-ui/react-dialog';

interface ModalProps {
  effectIndex: number;
  effectName: string;
  onClose: () => void;
}

export function Modal({ effectIndex, effectName, onClose }: ModalProps) {
  return (
    <Dialog.Root open onOpenChange={(open) => { if (!open) onClose(); }}>
      <Dialog.Portal>
        <Dialog.Overlay className="modal-backdrop" />
        <Dialog.Content className="modal-content">
          <div className="modal-header">
            <Dialog.Title className="modal-title">
              <span className="modal-number">{String(effectIndex + 1).padStart(2, '0')}</span>
              {' '}{effectName}
            </Dialog.Title>
            <Dialog.Close asChild>
              <button className="modal-close">✕</button>
            </Dialog.Close>
          </div>
          <div className="modal-body">
          </div>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
