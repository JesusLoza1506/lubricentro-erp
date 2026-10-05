import React from 'react';
import { AlertTriangle, Trash2, X } from 'lucide-react';

interface ConfirmModalProps {
  abierto: boolean;
  titulo: string;
  mensaje: string;
  onConfirmar: () => void;
  onCancelar: () => void;
  cargando?: boolean;
}

export const ConfirmModal: React.FC<ConfirmModalProps> = ({
  abierto,
  titulo,
  mensaje,
  onConfirmar,
  onCancelar,
  cargando = false,
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
        backgroundColor: 'rgba(0, 0, 0, 0.8)',
        backdropFilter: 'blur(6px)',
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        zIndex: 1500,
      }}
    >
      <div
        style={{
          width: '460px',
          backgroundColor: '#0F172A',
          border: '1px solid #7F1D1D',
          borderRadius: '16px',
          padding: '28px',
          boxShadow: '0 20px 50px rgba(239, 68, 68, 0.2)',
          display: 'flex',
          flexDirection: 'column',
          gap: '20px',
        }}
      >
        {/* ENCABEZADO DE ADVERTENCIA */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '14px' }}>
          <div
            style={{
              padding: '12px',
              backgroundColor: 'rgba(239, 68, 68, 0.15)',
              borderRadius: '50%',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
            }}
          >
            <AlertTriangle size={28} color="#EF4444" />
          </div>
          <div style={{ flex: 1 }}>
            <h3 style={{ fontSize: '18px', fontWeight: '800', color: '#F8FAFC', margin: 0 }}>
              {titulo}
            </h3>
            <span style={{ fontSize: '12px', color: '#FCA5A5', fontWeight: '600' }}>
              ⚠️ Acción Irreversible
            </span>
          </div>
          <button
            onClick={onCancelar}
            style={{ background: 'none', border: 'none', color: '#64748B', cursor: 'pointer' }}
          >
            <X size={20} />
          </button>
        </div>

        {/* MENSAJE DE AVISO */}
        <div
          style={{
            padding: '14px',
            backgroundColor: 'rgba(30, 41, 59, 0.6)',
            borderRadius: '10px',
            border: '1px solid #334155',
            color: '#CBD5E1',
            fontSize: '14px',
            lineHeight: '1.5',
          }}
        >
          {mensaje}
        </div>

        {/* ACCIONES */}
        <div style={{ display: 'flex', justifyContent: 'flex-end', gap: '12px' }}>
          <button
            type="button"
            onClick={onCancelar}
            disabled={cargando}
            style={{
              padding: '10px 18px',
              backgroundColor: 'transparent',
              border: '1px solid #334155',
              color: '#CBD5E1',
              borderRadius: '8px',
              fontWeight: '600',
              cursor: 'pointer',
            }}
          >
            Cancelar
          </button>
          <button
            type="button"
            onClick={onConfirmar}
            disabled={cargando}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              padding: '10px 20px',
              backgroundColor: '#DC2626',
              color: '#FFFFFF',
              border: 'none',
              borderRadius: '8px',
              fontWeight: '700',
              cursor: 'pointer',
            }}
          >
            <Trash2 size={16} />
            {cargando ? 'Eliminando...' : 'Sí, Eliminar'}
          </button>
        </div>
      </div>
    </div>
  );
};
