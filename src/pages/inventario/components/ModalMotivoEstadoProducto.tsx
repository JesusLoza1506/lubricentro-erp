import { useNotificarInventario } from '../AvisosInventario';
import React, { useEffect, useRef, useState } from 'react';
import { modalInputStyle } from '../inventarioStyles';

interface Props {
  abierto: boolean;
  nombre: string;
  accion: 'DESACTIVAR' | 'REACTIVAR';
  onConfirmar: (motivo: string) => Promise<void>;
  onCancelar: () => void;
}

export const ModalMotivoEstadoProducto: React.FC<Props> = ({
  abierto,
  nombre,
  accion,
  onConfirmar,
  onCancelar,
}) => {
  const notificar = useNotificarInventario();
  const [motivo, setMotivo] = useState('');
  const [error, setError] = useState('');
  const [guardando, setGuardando] = useState(false);
  const pendiente = useRef(false);
  useEffect(() => {
    if (abierto) {
      setMotivo('');
      setError('');
    }
  }, [abierto, nombre, accion]);
  if (!abierto) return null;
  const confirmar = async (e: React.FormEvent) => {
    e.preventDefault();
    if (pendiente.current) return;
    const limpio = motivo.trim();
    if (Array.from(limpio).length < 5 || Array.from(limpio).length > 250) {
      setError('Escribe un motivo de 5 a 250 caracteres.');
      return;
    }
    pendiente.current = true;
    setGuardando(true);
    setError('');
    try {
      await onConfirmar(limpio);
    } catch (err) {
      const mensaje =
        typeof err === 'string'
          ? err
          : err instanceof Error
            ? err.message
            : 'No se pudo cambiar el estado. Puedes reintentar.';
      setError(mensaje);
      notificar(mensaje, 'ERROR');
    } finally {
      pendiente.current = false;
      setGuardando(false);
    }
  };
  return (
    <div
      className="gloria-theme inv-overlay"
      style={{
        position: 'fixed',
        inset: 0,
        background: 'rgba(0,0,0,.75)',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        zIndex: 1200,
        padding: 16,
      }}
    >
      <form
        className="inv-dialog inv-dialog-compact"
        onSubmit={confirmar}
        role="dialog"
        aria-modal="true"
        aria-labelledby="estado-producto-titulo"
        style={{
          width: 460,
          maxWidth: '100%',
          padding: 24,
          borderRadius: 12,
          background: '#0F172A',
          color: '#F8FAFC',
          border: '1px solid #334155',
        }}
      >
        <header className="inv-dialog-header">
          <div><span className="inv-dialog-kicker">Estado del catálogo</span>
            <h3 id="estado-producto-titulo">
              {accion === 'DESACTIVAR' ? 'Dar de Baja Producto' : 'Reactivar Producto'}
            </h3>
            <p>{nombre}. Se conservarán sus movimientos e historial.</p>
          </div>
        </header>
        <label htmlFor="motivo-estado-producto">Motivo administrativo</label>
        <textarea
          id="motivo-estado-producto"
          autoFocus
          required
          rows={3}
          value={motivo}
          disabled={guardando}
          onChange={(e) => setMotivo(e.target.value)}
          style={{ ...modalInputStyle, width: '100%', boxSizing: 'border-box', marginTop: 8 }}
          aria-describedby="motivo-producto-ayuda"
        />
        <small id="motivo-producto-ayuda" className="inv-help">
          De 5 a 250 caracteres · {Array.from(motivo.trim()).length}/250
        </small>
        {error && (
          <p role="alert" className="inv-form-error">
            {error}
          </p>
        )}
        <footer className="inv-form-footer inv-confirm-footer">
          <button type="button" className="inv-button" onClick={onCancelar} disabled={guardando}>Cancelar</button>
          <button type="submit" disabled={guardando} aria-busy={guardando}
            className={`inv-button ${accion === 'DESACTIVAR' ? 'inv-danger' : 'inv-primary'}`}>
            {guardando ? 'Guardando…' : 'Confirmar'}
          </button>
        </footer>
      </form>
    </div>
  );
};
