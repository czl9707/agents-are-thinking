import { useState } from 'react';
import { CopyButton } from './CopyButton';
import s from './CodeTabs.module.css';

interface CodeTabsProps {
  cssSnippet: string;
  npmInstall: string;
  npmUsage: string;
}

export function CodeTabs({ cssSnippet, npmInstall, npmUsage }: CodeTabsProps) {
  const [tab, setTab] = useState<'css' | 'npm'>('css');

  return (
    <div className={s.container}>
      <div className={s.tabs}>
        <button
          className={`${s.tab} ${tab === 'css' ? s.active : ''}`}
          onClick={() => setTab('css')}
        >
          CSS
        </button>
        <button
          className={`${s.tab} ${tab === 'npm' ? s.active : ''}`}
          onClick={() => setTab('npm')}
        >
          npm
        </button>
      </div>
      <div className={s.content}>
        {tab === 'css' && (
          <div className={s.block}>
            <div className={s.blockHeader}>
              <span className={s.label}>Baked CSS — zero dependencies</span>
              <CopyButton text={cssSnippet} />
            </div>
            <pre className={s.code}><code>{cssSnippet}</code></pre>
          </div>
        )}
        {tab === 'npm' && (
          <div className={s.block}>
            <div className={s.blockHeader}>
              <span className={s.label}>Install</span>
              <CopyButton text={npmInstall} />
            </div>
            <pre className={s.code}><code>{npmInstall}</code></pre>
            <div className={s.blockHeader}>
              <span className={s.label}>Usage</span>
              <CopyButton text={npmUsage} />
            </div>
            <pre className={s.code}><code>{npmUsage}</code></pre>
          </div>
        )}
      </div>
    </div>
  );
}
