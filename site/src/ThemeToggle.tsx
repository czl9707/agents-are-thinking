import { useCallback, useEffect, useState } from 'react';
import { SunIcon, MoonIcon } from '@radix-ui/react-icons';
import s from './ThemeToggle.module.css';

function getInitialTheme(): 'light' | 'dark' {
  const stored = localStorage.getItem('theme');
  if (stored === 'light' || stored === 'dark') return stored;
  return 'dark';
}

export function ThemeToggle() {
  const [theme, setTheme] = useState(getInitialTheme);

  useEffect(() => {
    document.body.setAttribute('data-theme', theme);
    localStorage.setItem('theme', theme);
  }, [theme]);

  const toggle = useCallback(() => {
    setTheme(t => t === 'dark' ? 'light' : 'dark');
  }, []);

  return (
    <button className={s.iconButton} onClick={toggle} aria-label="Toggle theme">
      {theme === 'dark' ? <MoonIcon /> : <SunIcon />}
    </button>
  );
}
