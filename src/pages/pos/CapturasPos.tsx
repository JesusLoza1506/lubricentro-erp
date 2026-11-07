import { createContext, useContext, useEffect, useId, useRef } from 'react';
export const CapturasPosContext = createContext<(id: string, dirty: boolean) => void>(
  () => undefined
);
export function useCapturaPos() {
  const registrar = useContext(CapturasPosContext);
  const id = useId();
  const ref = useRef(registrar);
  ref.current = registrar;
  useEffect(() => () => ref.current(id, false), [id]);
  return (dirty: boolean) => ref.current(id, dirty);
}
