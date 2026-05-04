import { useState } from 'react';
import s from './Button.module.css';

interface CopyButtonProps {
  text: string;
}

export function CopyButton({ text }: CopyButtonProps) {
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    await navigator.clipboard.writeText(text);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <button className={s.btn} onClick={handleCopy}>
      {copied ? 'Copied' : 'Copy'}
    </button>
  );
}
