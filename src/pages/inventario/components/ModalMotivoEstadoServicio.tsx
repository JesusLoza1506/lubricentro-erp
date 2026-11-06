import React, { useState, useEffect } from 'react';
import { ServicioDTO } from '../../../types/inventario';
import { modalInputStyle } from '../inventarioStyles';

interface ModalMotivoEstadoServicioProps {
  abierto: boolean;
  servicio: ServicioDTO | null;
  motivo: string;
  setMotivo: (val: string) => void;
  onClose: () => void;
  onSubmit: (e: React.FormEvent) => Promise<void>;
}

export const ModalMotivoEstadoServicio: React.FC<ModalMotivoEstadoServicioProps> = ({
  abierto,
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
        style={{
          width: '440px',
          backgroundColor: '#0F172A',
          border: '1px solid #334155',
          borderRadius: '16px',
          padding: '24px',
          boxShadow: '0 20px 40px rgba(0,0,0,0.5)',
        }}
      >
        <h3
          style={{
            fontSize: '18px',
            fontWeight: '800',
            color: '#F8FAFC',
            margin: '0 0 12px 0',
          }}
        >
          {servicio.activo ? 'Desactivar Servicio' : 'Reactivar Servicio'}
        </h3>
        <p style={{ fontSize: '13px', color: '#94A3B8', marginBottom: '16px' }}>
          Servicio: <strong>{servicio.nombre}</strong>
        </p>
        <form onSubmit={handleSubmit}>
          <label
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
          <div
            style={{
              display: 'flex',
              justifyContent: 'flex-end',
              gap: '10px',
              marginTop: '16px',
            }}
          >
            <button
              type="button"
              disabled={procesando}
              onClick={onClose}
              style={{
                padding: '8px 14px',
                backgroundColor: 'transparent',
                border: '1px solid #334155',
                color: '#CBD5E1',
                borderRadius: '8px',
                cursor: procesando ? 'not-allowed' : 'pointer',
                opacity: procesando ? 0.6 : 1,
              }}
            >
              Cancelar
            </button>
            <button
              type="submit"
              disabled={procesando || motivo.trim().length < 5}
              style={{
                padding: '8px 18px',
                backgroundColor: servicio.activo ? '#EF4444' : '#10B981',
                color: '#FFFFFF',
                border: 'none',
                borderRadius: '8px',
                fontWeight: '700',
                cursor: procesando || motivo.trim().length < 5 ? 'not-allowed' : 'pointer',
                opacity: procesando || motivo.trim().length < 5 ? 0.6 : 1,
              }}
            >
              {procesando ? 'Guardando...' : 'Confirmar'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};
