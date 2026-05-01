import { useState } from 'react';
import { EFFECTS } from '@zane-chen/agents-are-thinking';
import { InfiniteCanvas } from './InfiniteCanvas';
import { Modal } from './Modal';

function App() {
  const [selected, setSelected] = useState<number | null>(null);

  return (
    <>
      <InfiniteCanvas onSelect={setSelected} />
      <Modal
        effectIndex={selected ?? 0}
        effectName={selected !== null ? EFFECTS[selected].name() : ''}
        open={selected !== null}
        onClose={() => setSelected(null)}
      />
    </>
  );
}

export default App;
