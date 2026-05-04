import * as Tabs from '@radix-ui/react-tabs';
import { CopyButton } from './CopyButton';
import { useShikiHighlight } from './hooks/useShikiHighlight';
import s from './CodeTabs.module.css';

interface CodeTabsProps {
  cssSnippet: string;
  npmInstall: string;
  npmUsage: string;
}

function CodeBlock({ code, lang, label, copyText }: { code: string; lang: string; label: string; copyText?: string }) {
  const highlighted = useShikiHighlight(code, lang);
  return (
    <div className={s.block}>
      <div className={s.blockHeader}>
        <span className={s.label}>{label}</span>
        <CopyButton text={copyText ?? code} />
      </div>
      <div className={s.code} dangerouslySetInnerHTML={{ __html: highlighted }} />
    </div>
  );
}

export function CodeTabs({ cssSnippet, npmInstall, npmUsage }: CodeTabsProps) {
  return (
    <Tabs.Root defaultValue="css" className={s.container}>
      <Tabs.List className={s.tabs}>
        <Tabs.Trigger value="css" className={s.tab}>baked-css</Tabs.Trigger>
        <Tabs.Trigger value="npm" className={s.tab}>npm</Tabs.Trigger>
      </Tabs.List>
      <Tabs.Content value="css" className={s.content}>
        <CodeBlock code={cssSnippet} lang="html" label="Baked CSS — zero dependencies" />
      </Tabs.Content>
      <Tabs.Content value="npm" className={s.content}>
        <CodeBlock code={npmInstall} lang="bash" label="Install" />
        <CodeBlock code={npmUsage} lang="javascript" label="Usage" />
      </Tabs.Content>
    </Tabs.Root>
  );
}
