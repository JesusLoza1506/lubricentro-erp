import React from 'react';
import { Producto } from '../../../types/inventario';
import { modalInputStyle } from '../inventarioStyles';

interface ModalDespachoProps {
  abierto: boolean;
  producto: Producto | null;
  cantidad: number;
  setCantidad: (val: number) => void;
  motivo: string;
  setMotivo: (val: string) => void;
  onClose: () => void;
  onSubmit: (e: React.FormEvent) => void;
}

export const ModalDespacho: React.FC<ModalDespachoProps> = ({
  abierto,
  producto,
  cantidad,
  setCantidad,
  motivo,
  setMotivo,
  onClose,
  onSubmit,
}) => {
  if (!abierto || !producto) return null;

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
          width: '440px',
          backgroundColor: '#0F172A',
          border: '1px solid #10B981',
          borderRadius: '16px',
          padding: '24px',
          boxShadow: '0 20px 40px rgba(16, 185, 129, 0.2)',
        }}
      >
        <h3 style={{ fontSize: '18px', fontWeight: '800', color: '#F8FAFC', margin: '0 0 16px 0' }}>
          Despachar / Usar en Taller
        </h3>
        <p style={{ color: '#94A3B8', fontSize: '13px', marginBottom: '16px' }}>
          Producto: <strong style={{ color: '#F8FAFC' }}>{producto.nombre}</strong> <br />
          Stock Actual: <strong style={{ color: '#10B981' }}>{producto.stock_actual}</strong>
        </p>
        <form onSubmit={onSubmit} style={{ display: 'flex', flexDirection: 'column', gap: '14px' }}>
          <div>
            <label
              style={{ display: 'block', fontSize: '12px', color: '#94A3B8', marginBottom: '6px' }}
            >
              Cantidad a Despachar *
            </label>
            <input
              required
              type="number"
              step="0.01"
              min="0.01"
              max={producto.stock_actual}
              value={cantidad}
              onChange={(e) => setCantidad(parseFloat(e.target.value) || 0)}
              style={modalInputStyle}
            />
          </div>
          <div>
            <label
              style={{ display: 'block', fontSize: '12px', color: '#94A3B8', marginBottom: '6px' }}
            >
              Motivo / Referencia (Opcional)
            </label>
            <input
              type="text"
              placeholder="Ej. Usado en OT-2026-0045"
              value={motivo}
              onChange={(e) => setMotivo(e.target.value)}
              style={modalInputStyle}
            />
          </div>
          <div
            style={{ display: 'flex', justifyContent: 'flex-end', gap: '12px', marginTop: '8px' }}
          >
            <button
              type="button"
              onClick={onClose}
              style={{
                padding: '10px 16px',
                backgroundColor: 'transparent',
                border: '1px solid #334155',
                color: '#CBD5E1',
                borderRadius: '8px',
                cursor: 'pointer',
              }}
            >
              Cancelar
            </button>
            <button
              type="submit"
              style={{
                padding: '10px 20px',
                backgroundColor: '#10B981',
                color: '#FFFFFF',
                border: 'none',
                borderRadius: '8px',
                fontWeight: '700',
                cursor: 'pointer',
              }}
            >
              Confirmar Despacho
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};
