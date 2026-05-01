import { useState } from 'react';
import { EFFECTS } from '@zane-chen/agents-are-thinking';
import { InfiniteCanvas } from './InfiniteCanvas';
import { Drawer } from './Drawer';
import { ThemeToggle } from './ThemeToggle';
import s from './Header.module.css';

function App() {
  const [selected, setSelected] = useState<number | null>(null);

  return (
    <>
      <header className={s.header}>
        <span className={s.title}>AGENTS.ARE.THINKING</span>
        <div style={{ flex: '1 1' }} />
        <ThemeToggle />
      </header>
      <InfiniteCanvas onSelect={setSelected} />
      <Drawer
        effectIndex={selected ?? 0}
        effectName={selected !== null ? EFFECTS[selected].name() : ''}
        open={selected !== null}
        onClose={() => setSelected(null)}
      />
    </>
  );
}

export default App;
