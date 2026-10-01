import React from 'react';
import { AlertTriangle } from 'lucide-react';

interface ConfirmModalProps {
  abierto: boolean;
  titulo: string;
  mensaje: string;
  onConfirmar: () => void;
  onCancelar: () => void;
}

export const ConfirmModal: React.FC<ConfirmModalProps> = ({
  abierto,
  titulo,
  mensaje,
  onConfirmar,
  onCancelar,
}) => {
  if (!abierto) return null;

  return (
    <div
      style={{
        position: 'fixed',
        top: 0,
        left: 0,
        width: '100vw',
        height: '100vh',
        backgroundColor: 'rgba(0,0,0,0.75)',
        backdropFilter: 'blur(4px)',
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        zIndex: 1100,
      }}
    >
      <div
        style={{
          width: '420px',
          backgroundColor: '#0F172A',
          border: '1px solid #334155',
          borderRadius: '16px',
          padding: '24px',
          boxShadow: '0 20px 40px rgba(0,0,0,0.6)',
          display: 'flex',
          flexDirection: 'column',
          gap: '16px',
        }}
      >
        <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
          <div
            style={{
              padding: '10px',
              backgroundColor: 'rgba(239, 68, 68, 0.15)',
              borderRadius: '10px',
            }}
          >
            <AlertTriangle size={24} color="#EF4444" />
          </div>
          <div>
            <h3 style={{ fontSize: '18px', fontWeight: '800', color: '#F8FAFC', margin: 0 }}>
              {titulo}
            </h3>
          </div>
        </div>

        <p style={{ fontSize: '14px', color: '#CBD5E1', margin: 0, lineHeight: '1.5' }}>
          {mensaje}
        </p>

        <div style={{ display: 'flex', justifyContent: 'flex-end', gap: '12px', marginTop: '8px' }}>
          <button
            onClick={onCancelar}
            style={{
              padding: '10px 16px',
              backgroundColor: 'transparent',
              border: '1px solid #334155',
              color: '#CBD5E1',
              borderRadius: '8px',
              cursor: 'pointer',
              fontWeight: '600',
            }}
          >
            Cancelar
          </button>
          <button
            onClick={onConfirmar}
            style={{
              padding: '10px 20px',
              backgroundColor: '#DC2626',
              color: '#FFFFFF',
              border: 'none',
              borderRadius: '8px',
              fontWeight: '700',
              cursor: 'pointer',
            }}
          >
            Eliminar
          </button>
        </div>
      </div>
    </div>
  );
};
