import { TransformWrapper, TransformComponent } from 'react-zoom-pan-pinch';
import { EffectGrid } from './EffectGrid';

interface InfiniteCanvasProps {
  onSelect: (index: number) => void;
}

export function InfiniteCanvas({ onSelect }: InfiniteCanvasProps) {
  return (
    <TransformWrapper
      minScale={0.3}
      maxScale={3}
      centerOnInit
      centerZoomedOut
    >
      <TransformComponent
        wrapperStyle={{ width: '100vw', height: '100vh' }}
      >
        <EffectGrid onSelect={onSelect} />
      </TransformComponent>
    </TransformWrapper>
  );
}
