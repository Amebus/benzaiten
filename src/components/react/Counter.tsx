'use client';

import { useState } from 'react';

export default function Counter() {
  const [count, setCount] = useState(0);

  return (
    <button type="button" onClick={() => setCount((currentCount) => currentCount + 1)}>
      Click: {count}
    </button>
  );
}