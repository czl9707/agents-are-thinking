import { useEffect, useState } from 'react';
import { createHighlighter, createCssVariablesTheme, type Highlighter } from 'shiki';

const LANGUAGES = ['html', 'css', 'javascript', 'shellscript'] as const;
const theme = createCssVariablesTheme();

let highlighterPromise: Promise<Highlighter> | null = null;

function getHighlighter() {
  if (!highlighterPromise) {
    highlighterPromise = createHighlighter({
      themes: [theme],
      langs: [...LANGUAGES],
    });
  }
  return highlighterPromise;
}

export function useShikiHighlight(code: string, lang: string): string {
  const [html, setHtml] = useState('');

  useEffect(() => {
    if (!code) return;

    let cancelled = false;
    getHighlighter().then((highlighter) => {
      if (cancelled) return;
      const loadedLangs = highlighter.getLoadedLanguages();
      const resolvedLang = (loadedLangs as readonly string[]).includes(lang) ? lang : 'text';
      const result = highlighter.codeToHtml(code, {
        lang: resolvedLang,
        theme: 'css-variables',
      });
      if (!cancelled) setHtml(result);
    });
    return () => { cancelled = true; };
  }, [code, lang]);

  return code ? html : '';
}
