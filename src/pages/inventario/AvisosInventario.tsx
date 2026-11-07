import { createContext, useCallback, useContext, useRef, useState } from 'react';
import { createPortal } from 'react-dom';
import { ToastNotification } from '../../components/ToastNotification';

type TipoAviso = 'EXITO' | 'ERROR';
type Notificar = (mensaje: string, tipo: TipoAviso) => void;
interface Aviso {
  id: number;
  mensaje: string;
  tipo: TipoAviso;
}
export const AvisosInventarioContext = createContext<Notificar>(() => undefined);
export const useNotificarInventario = () => useContext(AvisosInventarioContext);

export function useAvisosInventario() {
  const [aviso, setAviso] = useState<Aviso | null>(null);
  const secuencia = useRef(0);
  const notificar = useCallback<Notificar>((mensaje, tipo) => {
    setAviso({ id: ++secuencia.current, mensaje, tipo });
  }, []);
  const cerrar = useCallback(() => setAviso(null), []);
  return { aviso, notificar, cerrar };
}

export function AvisoInventarioVisible({
  aviso,
  onCerrar,
}: {
  aviso: Aviso | null;
  onCerrar: () => void;
}) {
  return aviso
    ? createPortal(
        <div
          className="gloria-theme inv-notification"
          data-kind={aviso.tipo}
          role={aviso.tipo === 'ERROR' ? 'alert' : 'status'}
          aria-live={aviso.tipo === 'ERROR' ? 'assertive' : 'polite'}
          aria-atomic="true"
        >
          <ToastNotification
            key={aviso.id}
            mensaje={aviso.mensaje}
            tipo={aviso.tipo}
            onCerrar={onCerrar}
          />
        </div>,
        document.body
      )
    : null;
}
