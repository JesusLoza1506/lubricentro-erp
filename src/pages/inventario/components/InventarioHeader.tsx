import React from 'react';
import { Plus } from 'lucide-react';

interface InventarioHeaderProps {
  esAdmin: boolean;
  onAbrirModalCrear: () => void;
}

export const InventarioHeader: React.FC<InventarioHeaderProps> = ({
  esAdmin,
  onAbrirModalCrear,
}) => {
  return (
    <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
      <div>
        <h2 style={{ fontSize: '24px', fontWeight: '800', color: '#F8FAFC', margin: 0 }}>
          Inventario & Productos
        </h2>
        <p style={{ color: '#94A3B8', fontSize: '14px', margin: '4px 0 0 0' }}>
          Gestión de catálogo, precios y control de stock mínimo.
        </p>
      </div>
      {esAdmin && (
        <button
          onClick={onAbrirModalCrear}
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: '8px',
            padding: '12px 20px',
            backgroundColor: '#2563EB',
            color: '#FFFFFF',
            border: 'none',
            borderRadius: '10px',
            fontWeight: '700',
            cursor: 'pointer',
          }}
        >
          <Plus size={18} /> Nuevo Producto
        </button>
      )}
    </div>
  );
};
