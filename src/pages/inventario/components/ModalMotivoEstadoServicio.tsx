import React, { useState, useEffect } from 'react';
import { ServicioDTO } from '../../../types/inventario';
import { modalInputStyle } from '../inventarioStyles';

interface ModalMotivoEstadoServicioProps {
  error?: string;
  abierto: boolean;
  servicio: ServicioDTO | null;
  motivo: string;
  setMotivo: (val: string) => void;
  onClose: () => void;
  onSubmit: (e: React.FormEvent) => Promise<void>;
}

export const ModalMotivoEstadoServicio: React.FC<ModalMotivoEstadoServicioProps> = ({
  abierto,
  error,
  servicio,
  motivo,
  setMotivo,
  onClose,
  onSubmit,
}) => {
  const [procesando, setProcesando] = useState(false);

  useEffect(() => {
    if (abierto) {
      setProcesando(false);
    }
  }, [abierto]);

  if (!abierto || !servicio) return null;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (procesando || motivo.trim().length < 5) return;
    try {
      setProcesando(true);
      await onSubmit(e);
    } finally {
      setProcesando(false);
    }
  };

  return (
    <div
      className="gloria-theme inv-overlay"
      style={{
        position: 'fixed',
        top: 0,
        left: 0,
        width: '100vw',
        height: '100vh',
        backgroundColor: 'rgba(0,0,0,0.8)',
        backdropFilter: 'blur(4px)',
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        zIndex: 1250,
      }}
    >
      <div
        className="inv-dialog inv-dialog-compact"
        role="dialog"
        aria-modal="true"
        aria-label="Estado del servicio"
        style={{
          width: '440px',
          backgroundColor: '#0F172A',
          border: '1px solid #334155',
          borderRadius: '16px',
          padding: '24px',
          boxShadow: '0 20px 40px rgba(0,0,0,0.5)',
        }}
      >
        <header className="inv-dialog-header">
          <div>
            <span className="inv-dialog-kicker">Estado del catálogo</span>
            <h3>{servicio.activo ? 'Desactivar Servicio' : 'Reactivar Servicio'}</h3>
            <p>
              Servicio: <strong>{servicio.nombre}</strong>
            </p>
          </div>
        </header>
        {error && (
          <p role="alert" className="inv-form-error">
            {error}
          </p>
        )}
        <form onSubmit={handleSubmit} className="inv-form">
          <label
            htmlFor="inv-service-status-reason"
            style={{
              display: 'block',
              fontSize: '11px',
              color: '#94A3B8',
              marginBottom: '6px',
            }}
          >
            Motivo Administrativo (5-250 caracteres) *
          </label>
          <textarea
            id="inv-service-status-reason"
            required
            disabled={procesando}
            rows={3}
            minLength={5}
            maxLength={250}
            value={motivo}
            onChange={(e) => setMotivo(e.target.value)}
            style={{
              ...modalInputStyle,
              width: '100%',
              boxSizing: 'border-box',
              opacity: procesando ? 0.6 : 1,
            }}
          />
          <footer className="inv-form-footer">
            <button type="button" className="inv-button" disabled={procesando} onClick={onClose}>
              Cancelar
            </button>
            <button
              type="submit"
              aria-busy={procesando}
              disabled={procesando || motivo.trim().length < 5}
              className={`inv-button ${servicio.activo ? 'inv-danger' : 'inv-primary'}`}
            >
              {procesando ? 'Guardando...' : 'Confirmar'}
            </button>
          </footer>
        </form>
      </div>
    </div>
  );
};
