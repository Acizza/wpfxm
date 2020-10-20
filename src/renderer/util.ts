import { useState } from "react";

/** Generates a change that will trigger a component rerender
 * when the returned function is called. */
export function useForceUpdate(): () => void {
  const [, setTick] = useState(0);
  return () => setTick((cur) => ++cur);
}

export default {};
