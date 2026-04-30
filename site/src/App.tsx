import { useState } from 'react';
import { EFFECTS } from '@zane-chen/agents-are-thinking';
import { InfiniteCanvas } from './InfiniteCanvas';
import { Modal } from './Modal';

function App() {
  const [selected, setSelected] = useState<number | null>(null);

  return (
    <>
      <InfiniteCanvas onSelect={setSelected} />
      {selected !== null && (
        <Modal
          effectIndex={selected}
          effectName={EFFECTS[selected].name()}
          onClose={() => setSelected(null)}
        />
      )}
    </>
  );
}

export default App;
