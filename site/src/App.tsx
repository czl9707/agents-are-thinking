import { useState, useEffect } from 'react';
import { EFFECTS } from '@zane-chen/agents-are-thinking';
import { Header } from './Header';
import { InfiniteCanvas } from './InfiniteCanvas';
import { Drawer } from './Drawer';

function App() {
  const [selected, setSelected] = useState<number | null>(null);

  useEffect(() => {
    const base = `${EFFECTS.length} agents are thinking`;
    let dots = 1;
    document.title = `${base}.`;
    const id = setInterval(() => {
      dots = dots % 3 + 1;
      document.title = `${base}${".".repeat(dots)}`;
    }, 500);
    return () => clearInterval(id);
  }, []);

  return (
    <>
      <Header />
      <InfiniteCanvas onSelect={setSelected} />
      <Drawer
        effectIndex={selected ?? 0}
        open={selected !== null}
        onClose={() => setSelected(null)}
        onPrev={() => setSelected(((selected ?? 0) - 1 + EFFECTS.length) % EFFECTS.length)}
        onNext={() => setSelected(((selected ?? 0) + 1) % EFFECTS.length)}
      />
    </>
  );
}

export default App;
