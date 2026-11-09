import { useCallback, useEffect, useId, useLayoutEffect, useRef, useState, type ReactNode } from 'react';
import { createPortal } from 'react-dom';
import { CircleAlert, CircleCheck, LoaderCircle, X } from 'lucide-react';
import { mensajeError } from './adminApi';

export function useConsulta<T>(cargar: () => Promise<T>) {
  const [datos, setDatos] = useState<T | null>(null);
  const [error, setError] = useState('');
  const [cargando, setCargando] = useState(true);
  const secuencia = useRef(0);
  const recargar = useCallback(async () => {
    const id = ++secuencia.current;
    setCargando(true);
    setError('');
    try {
      const resultado = await cargar();
      if (id === secuencia.current) setDatos(resultado);
    } catch (e) {
      if (id === secuencia.current) setError(mensajeError(e));
    } finally {
      if (id === secuencia.current) setCargando(false);
    }
  }, [cargar]);
  useEffect(() => {
    void recargar();
    return () => {
      secuencia.current += 1;
    };
  }, [recargar]);
  return { datos, error, cargando, recargar };
}

export function useOperacion() {
  const [guardando, setGuardando] = useState(false);
  const [error, setError] = useState('');
  const [aviso, setAviso] = useState('');
  const bloqueo = useRef(false);
  const ejecutar = async (accion: () => Promise<void>, mensaje: string) => {
    if (bloqueo.current) return;
    bloqueo.current = true;
    setGuardando(true);
    setError('');
    setAviso('');
    try {
      await accion();
      setAviso(mensaje);
    } catch (e) {
      setError(mensajeError(e));
    } finally {
      bloqueo.current = false;
      setGuardando(false);
    }
  };
  return { ejecutar, guardando, error, aviso, setError };
}
export function confirmarDescarteAdmin(hayCambios: boolean) {
  return !hayCambios || window.confirm('¿Descartar los cambios sin guardar en Administración?');
}

export type EdicionLocal = {
  marcar: (cambio: boolean) => void;
  navegar: (destino: string) => void;
};

export function EstadoAdmin({
  error,
  aviso,
  cargando,
  reintentar,
}: {
  error?: string;
  aviso?: string;
  cargando?: boolean;
  reintentar?: () => void;
}) {
  return (
    <>
      {cargando && (
        <p className="adm-estado" role="status">
          <LoaderCircle size={17} className="gt-boot-spinner" aria-hidden="true" /> Cargando datos…
        </p>
      )}
      {error && (
        <div className="gt-notice gt-notice--error adm-estado adm-error" role="alert">
          <CircleAlert size={17} aria-hidden="true" />
          <span>{error}</span>
          {reintentar && (
            <button className="gt-button" type="button" onClick={reintentar}>
              Reintentar
            </button>
          )}
        </div>
      )}
      {aviso && (
        <p className="gt-notice gt-notice--success adm-estado" role="status">
          <CircleCheck size={17} aria-hidden="true" /> {aviso}
        </p>
      )}
    </>
  );
}

export function Botones({
  cancelar,
  guardando,
  texto = 'Guardar',
  deshabilitado,
}: {
  cancelar: () => void;
  guardando: boolean;
  texto?: string;
  deshabilitado?: boolean;
}) {
  return (
    <div className="gt-actions adm-form-acciones">
      <button className="gt-button" type="button" onClick={cancelar} disabled={guardando}>
        Cancelar
      </button>
      <button
        className="gt-button gt-button--primary"
        type="submit"
        disabled={guardando || deshabilitado}
      >
        {guardando ? 'Procesando…' : texto}
      </button>
    </div>
  );
}

export function Campo({ etiqueta, children }: { etiqueta: string; children: ReactNode }) {
  return (
    <label className="adm-campo">
      <span>{etiqueta}</span>
      {children}
    </label>
  );
}

export function DialogoAdmin({
  titulo,
  descripcion,
  cerrar,
  ocupado = false,
  children,
}: {
  titulo: string;
  descripcion?: string;
  cerrar: () => void;
  ocupado?: boolean;
  children: ReactNode;
}) {
  const dialogo = useRef<HTMLDialogElement>(null);
  const encabezado = useRef<HTMLHeadingElement>(null);
  const id = useId();
  useLayoutEffect(() => {
    const nodo = dialogo.current;
    const origen = document.activeElement;
    if (!nodo) return;
    if (typeof nodo.showModal === 'function') nodo.showModal();
    else nodo.setAttribute('open', '');
    return () => {
      if (typeof nodo.close === 'function') nodo.close();
      else nodo.removeAttribute('open');
      queueMicrotask(() => {
        if (document.activeElement === document.body && origen instanceof HTMLElement && origen.isConnected) {
          origen.focus({ preventScroll: true });
        }
      });
    };
  }, []);
  useLayoutEffect(() => {
    encabezado.current?.focus();
  }, [titulo]);
  return createPortal(
    <dialog
      ref={dialogo}
      className="gloria-theme gt-dialog adm-dialogo"
      aria-labelledby={id}
      aria-describedby={descripcion ? id + '-descripcion' : undefined}
      aria-modal="true"
      onCancel={(e) => {
        e.preventDefault();
        if (!ocupado) cerrar();
      }}
    >
      <div className="gt-dialog-header">
        <div>
          <h2 id={id} ref={encabezado} tabIndex={-1} className="gt-dialog-title">{titulo}</h2>
          {descripcion && <p id={id + '-descripcion'} className="gt-dialog-description">{descripcion}</p>}
        </div>
        <button type="button" className="gt-button gt-button--icon" aria-label="Cerrar diálogo" onClick={cerrar} disabled={ocupado}>
          <X size={19} aria-hidden="true" />
        </button>
      </div>
      {children}
    </dialog>,
    document.body
  );
}

export function EtiquetaAdmin({ children, tono }: { children: ReactNode; tono?: 'exito' | 'aviso' | 'error' }) {
  return <span className="adm-indicador" data-tono={tono}>{children}</span>;
}
