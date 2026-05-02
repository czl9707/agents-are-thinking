import { useState, useEffect } from 'react';
import { EFFECTS } from '@zane-chen/agents-are-thinking';
import { GitHubLogoIcon } from '@radix-ui/react-icons';
import { InfiniteCanvas } from './InfiniteCanvas';
import { Drawer } from './Drawer';
import { ThemeToggle } from './ThemeToggle';
import s from './Header.module.css';
import t from './ThemeToggle.module.css';

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
      <header className={s.header}>
        <span className={s.title}>AGENTS.ARE.THINKING</span>
        <div style={{ flex: '1 1' }} />
        <a
           className={t.iconButton}
          href="https://github.com/czl9707/agents-are-thinking"
          target="_blank"
          rel="noopener noreferrer"
          aria-label="GitHub"
        >
          <GitHubLogoIcon />
        </a>
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
