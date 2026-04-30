import { useEffect, useState } from 'react';
import { EFFECTS } from '@zane-chen/agents-are-thinking';

function App() {
  const [loaded, setLoaded] = useState(false);

  useEffect(() => {
    if (EFFECTS.length > 0) setLoaded(true);
  }, []);

  return (
    <div>
      {loaded ? `Loaded ${EFFECTS.length} effects` : 'Loading...'}
    </div>
  );
}

export default App;
