import { GitHubLogoIcon } from '@radix-ui/react-icons';
import { ThemeToggle } from './ThemeToggle';
import b from './Button.module.css';
import s from './Header.module.css';

export function Header() {
  return (
    <header className={s.header}>
      <span className={s.title}>AGENTS.ARE.THINKING</span>
      <div style={{ flex: '1 1' }} />
      <a
        className={`${b.btn} ${b.icon}`}
        href="https://github.com/czl9707/agents-are-thinking"
        target="_blank"
        rel="noopener noreferrer"
        aria-label="GitHub"
      >
        <GitHubLogoIcon />
      </a>
      <ThemeToggle />
    </header>
  );
}
