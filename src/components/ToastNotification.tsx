import React, { useEffect } from 'react';
import { CheckCircle2, AlertCircle, X } from 'lucide-react';

export interface ToastProps {
  mensaje: string;
  tipo: 'EXITO' | 'ERROR';
  onCerrar: () => void;
}

export const ToastNotification: React.FC<ToastProps> = ({ mensaje, tipo, onCerrar }) => {
  useEffect(() => {
    const timer = setTimeout(() => {
      onCerrar();
    }, 3500);
    return () => clearTimeout(timer);
  }, [onCerrar]);

  const esExito = tipo === 'EXITO';

  return (
    <div
      style={{
        position: 'fixed',
        top: '24px',
        right: '24px',
        zIndex: 2000,
        display: 'flex',
        alignItems: 'center',
        gap: '12px',
        padding: '14px 20px',
        borderRadius: '12px',
        backgroundColor: esExito ? '#064E3B' : '#7F1D1D',
        color: esExito ? '#A7F3D0' : '#FECACA',
        border: `1px solid ${esExito ? '#059669' : '#DC2626'}`,
        boxShadow: '0 10px 25px rgba(0, 0, 0, 0.5)',
        animation: 'fadeIn 0.3s ease-out',
      }}
    >
      {esExito ? (
        <CheckCircle2 size={20} color="#34D399" />
      ) : (
        <AlertCircle size={20} color="#F87171" />
      )}
      <span style={{ fontSize: '14px', fontWeight: '600' }}>{mensaje}</span>
      <button
        onClick={onCerrar}
        style={{
          background: 'none',
          border: 'none',
          color: 'inherit',
          cursor: 'pointer',
          padding: '2px',
          display: 'flex',
          alignItems: 'center',
        }}
      >
        <X size={16} />
      </button>
    </div>
  );
};
