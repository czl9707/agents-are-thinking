import * as Tabs from '@radix-ui/react-tabs';
import { CopyButton } from './CopyButton';
import { useShikiHighlight } from './hooks/useShikiHighlight';
import { generateBakedCSS } from './utils/generateBakedCSS';
import { generateNpmSnippet } from './utils/generateNpmSnippet';
import { useBakeEffect } from './hooks/useBakeEffect';
import type { EffectClass } from './types';
import s from './CodeTabs.module.css';

interface CodeTabsProps {
  effectName: string;
  EffectCls: EffectClass | null;
}

function toPascalCase(kebab: string): string {
  return kebab
    .split('-')
    .map(part => part.charAt(0).toUpperCase() + part.slice(1))
    .join('');
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

export function CodeTabs({ effectName, EffectCls }: CodeTabsProps) {
  const baked = useBakeEffect(EffectCls);
  const cssSnippet = baked ? generateBakedCSS(effectName, baked.frames, baked.cycleLength) : '';
  const pascalName = effectName ? toPascalCase(effectName) : '';
  const npm = pascalName ? generateNpmSnippet(effectName, pascalName) : { install: '', usage: '' };

  return (
    <Tabs.Root defaultValue="css" className={s.container}>
      <Tabs.List className={s.tabs}>
        <Tabs.Trigger value="css" className={s.tab}>baked-css</Tabs.Trigger>
        <Tabs.Trigger value="npm" className={s.tab}>npm</Tabs.Trigger>
      </Tabs.List>
      <Tabs.Content value="css" className={s.content}>
        <CodeBlock code={cssSnippet} lang="html" label="Baked CSS" />
      </Tabs.Content>
      <Tabs.Content value="npm" className={s.content}>
        <CodeBlock code={npm.install} lang="bash" label="Install" />
        <CodeBlock code={npm.usage} lang="javascript" label="Usage" />
      </Tabs.Content>
    </Tabs.Root>
  );
}
