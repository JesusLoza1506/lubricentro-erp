import React from 'react';
import { Producto } from '../../../types/inventario';
import { modalInputStyle } from '../inventarioStyles';

interface ModalAjusteStockProps {
  abierto: boolean;
  producto: Producto | null;
  tipoMovimiento: 'ENTRADA' | 'AJUSTE';
  setTipoMovimiento: (val: 'ENTRADA' | 'AJUSTE') => void;
  cantidadNueva: number;
  setCantidadNueva: (val: number) => void;
  motivo: string;
  setMotivo: (val: string) => void;
  onClose: () => void;
  onSubmit: (e: React.FormEvent) => void;
}

export const ModalAjusteStock: React.FC<ModalAjusteStockProps> = ({
  abierto,
  producto,
  tipoMovimiento,
  setTipoMovimiento,
  cantidadNueva,
  setCantidadNueva,
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
          width: '460px',
          backgroundColor: '#0F172A',
          border: '1px solid #A855F7',
          borderRadius: '16px',
          padding: '24px',
          boxShadow: '0 20px 40px rgba(168, 85, 247, 0.2)',
        }}
      >
        <h3 style={{ fontSize: '18px', fontWeight: '800', color: '#F8FAFC', margin: '0 0 16px 0' }}>
          Ajuste Manual de Stock (Admin)
        </h3>
        <p style={{ color: '#94A3B8', fontSize: '13px', marginBottom: '16px' }}>
          Producto: <strong style={{ color: '#F8FAFC' }}>{producto.nombre}</strong> <br />
          Stock Actual en BD: <strong style={{ color: '#A855F7' }}>{producto.stock_actual}</strong>
        </p>
        <form onSubmit={onSubmit} style={{ display: 'flex', flexDirection: 'column', gap: '14px' }}>
          <div>
            <label
              style={{ display: 'block', fontSize: '12px', color: '#94A3B8', marginBottom: '6px' }}
            >
              Tipo de Movimiento *
            </label>
            <select
              required
              value={tipoMovimiento}
              onChange={(e) => setTipoMovimiento(e.target.value as 'ENTRADA' | 'AJUSTE')}
              style={{
                ...modalInputStyle,
                backgroundColor: '#1E293B',
                color: '#F8FAFC',
                cursor: 'pointer',
              }}
            >
              <option value="AJUSTE">AJUSTE (Conteo físico / Corrección)</option>
              <option value="ENTRADA">ENTRADA (Ingreso de nueva mercadería)</option>
            </select>
          </div>
          <div>
            <label
              style={{ display: 'block', fontSize: '12px', color: '#94A3B8', marginBottom: '6px' }}
            >
              Nueva Cantidad Resultante *
            </label>
            <input
              required
              type="number"
              step="0.01"
              min="0"
              value={cantidadNueva}
              onChange={(e) => setCantidadNueva(parseFloat(e.target.value) || 0)}
              style={modalInputStyle}
            />
          </div>
          <div>
            <label
              style={{ display: 'block', fontSize: '12px', color: '#94A3B8', marginBottom: '6px' }}
            >
              Motivo Obligatorio *
            </label>
            <input
              required
              type="text"
              placeholder="Ej. Conteo físico de inventario de cierre de mes"
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
                backgroundColor: '#A855F7',
                color: '#FFFFFF',
                border: 'none',
                borderRadius: '8px',
                fontWeight: '700',
                cursor: 'pointer',
              }}
            >
              Guardar Ajuste
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};
