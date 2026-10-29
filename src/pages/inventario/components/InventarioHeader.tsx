import React from 'react';
import { Plus, ShoppingCart } from 'lucide-react';

interface InventarioHeaderProps {
  esAdmin: boolean;
  onAbrirModalCrear: () => void;
  onAbrirModalCompra?: () => void;
}

export const InventarioHeader: React.FC<InventarioHeaderProps> = ({
  esAdmin,
  onAbrirModalCrear,
  onAbrirModalCompra,
}) => {
  return (
    <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
      <div>
        <h2 style={{ fontSize: '24px', fontWeight: '800', color: '#F8FAFC', margin: 0 }}>
          Gestión de Inventario y Stock
        </h2>
        <p style={{ color: '#94A3B8', fontSize: '14px', margin: '4px 0 0 0' }}>
          Control de productos, stock crítico y despachos a taller.
        </p>
      </div>

      {esAdmin && (
        <div style={{ display: 'flex', gap: '12px' }}>
          {onAbrirModalCompra && (
            <button
              onClick={onAbrirModalCompra}
              style={{
                padding: '10px 18px',
                backgroundColor: 'rgba(16, 185, 129, 0.15)',
                border: '1px solid #10B981',
                color: '#34D399',
                borderRadius: '10px',
                fontWeight: '700',
                cursor: 'pointer',
                display: 'flex',
                alignItems: 'center',
                gap: '8px',
              }}
            >
              <ShoppingCart size={18} />
              Registrar Compra
            </button>
          )}
          <button
            onClick={onAbrirModalCrear}
            style={{
              padding: '10px 18px',
              backgroundColor: '#2563EB',
              border: 'none',
              color: '#FFFFFF',
              borderRadius: '10px',
              fontWeight: '700',
              cursor: 'pointer',
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
            }}
          >
            <Plus size={18} />
            Nuevo Producto
          </button>
        </div>
      )}
    </div>
  );
};
