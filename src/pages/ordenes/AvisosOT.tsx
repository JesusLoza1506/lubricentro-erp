import React, { createContext, useCallback, useContext, useRef, useState } from 'react';
import { createPortal } from 'react-dom';
import { ToastNotification } from '../../components/ToastNotification';

export type TipoAvisoOT = 'EXITO' | 'ERROR';
export type NotificarOT = (mensaje: string, tipo: TipoAvisoOT) => void;
interface AvisoOT { id: number; mensaje: string; tipo: TipoAvisoOT; }

// Un único aviso para la página y sus ventanas: mismo componente que Ficha Vehicular.
export const AvisosOTContext = createContext<NotificarOT>(() => undefined);
export const useNotificarOT = () => useContext(AvisosOTContext);

export const useAvisosOT = () => {
  const [aviso, setAviso] = useState<AvisoOT | null>(null);
  const secuencia = useRef(0);
  const notificar = useCallback<NotificarOT>((mensaje, tipo) => {
    setAviso({ id: ++secuencia.current, mensaje, tipo });
  }, []);
  const cerrar = useCallback(() => setAviso(null), []);
  return { aviso, notificar, cerrar };
};

export const AvisoOTVisible: React.FC<{ aviso: AvisoOT | null; onCerrar: () => void }> = ({ aviso, onCerrar }) => aviso ? createPortal(
  <div className="ot-notification" role={aviso.tipo === 'ERROR' ? 'alert' : 'status'} aria-live={aviso.tipo === 'ERROR' ? 'assertive' : 'polite'} aria-atomic="true">
    <ToastNotification key={aviso.id} mensaje={aviso.mensaje} tipo={aviso.tipo} onCerrar={onCerrar} />
  </div>, document.body
) : null;

export const mensajeErrorOT = (error: unknown, alternativa: string): string => {
  if (typeof error === 'string' && error.trim()) return error;
  if (error && typeof error === 'object' && 'message' in error && typeof error.message === 'string' && error.message.trim()) return error.message;
  return alternativa;
};
