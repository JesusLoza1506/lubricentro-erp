import React from 'react';
import { Eye, PackageCheck, Sliders, Pencil, Trash2 } from 'lucide-react';
import { Producto, ProductoAdminDTO } from '../../../types/inventario';
import { getBadgeStyles } from '../inventarioStyles';

interface InventarioTablaProps {
  cargando: boolean;
  productos: Producto[];
  esAdmin: boolean;
  onVer: (p: Producto) => void;
  onDespachar: (p: Producto) => void;
  onAjustar: (p: Producto) => void;
  onEditar: (p: Producto) => void;
  onEliminar: (id: number, nombre: string) => void;
}

export const InventarioTabla: React.FC<InventarioTablaProps> = ({
  cargando,
  productos,
  esAdmin,
  onVer,
  onDespachar,
  onAjustar,
  onEditar,
  onEliminar,
}) => {
  return (
    <div
      style={{
        flex: 1,
        backgroundColor: 'rgba(15, 23, 42, 0.6)',
        border: '1px solid #334155',
        borderRadius: '16px',
        overflowY: 'auto',
        maxHeight: 'calc(100vh - 280px)',
      }}
    >
      <table
        style={{ width: '100%', borderCollapse: 'collapse', textAlign: 'left', fontSize: '14px' }}
      >
        <thead
          style={{
            position: 'sticky',
            top: 0,
            zIndex: 10,
            backgroundColor: '#1E293B',
            boxShadow: '0 2px 8px rgba(0,0,0,0.3)',
          }}
        >
          <tr style={{ borderBottom: '1px solid #334155', color: '#94A3B8' }}>
            <th style={{ padding: '16px' }}>CÓDIGO</th>
            <th style={{ padding: '16px' }}>PRODUCTO</th>
            <th style={{ padding: '16px' }}>UNIDAD</th>
            {esAdmin && <th style={{ padding: '16px' }}>P. COMPRA</th>}
            <th style={{ padding: '16px' }}>P. VENTA</th>
            <th style={{ padding: '16px' }}>STOCK</th>
            <th style={{ padding: '16px' }}>ESTADO</th>
            <th style={{ padding: '16px', textAlign: 'center' }}>ACCIONES</th>
          </tr>
        </thead>
        <tbody>
          {cargando ? (
            <tr>
              <td
                colSpan={esAdmin ? 8 : 7}
                style={{ padding: '32px', textAlign: 'center', color: '#94A3B8' }}
              >
                Cargando productos...
              </td>
            </tr>
          ) : productos.length === 0 ? (
            <tr>
              <td
                colSpan={esAdmin ? 8 : 7}
                style={{ padding: '32px', textAlign: 'center', color: '#94A3B8' }}
              >
                No se encontraron productos en el inventario.
              </td>
            </tr>
          ) : (
            productos.map((p) => {
              const esCritico = p.es_critico ?? p.stock_actual <= p.stock_minimo;
              const precioCompraCentimos =
                'precio_compra' in p ? (p as ProductoAdminDTO).precio_compra : null;

              return (
                <tr key={p.id_producto} style={{ borderBottom: '1px solid rgba(51, 65, 85, 0.4)' }}>
                  <td style={{ padding: '16px', color: '#38BDF8', fontFamily: 'monospace' }}>
                    {p.codigo_barras || 'N/A'}
                  </td>
                  <td style={{ padding: '16px', fontWeight: '600', color: '#F8FAFC' }}>
                    {p.nombre}
                  </td>
                  <td style={{ padding: '16px', color: '#94A3B8' }}>
                    {p.unidad_medida || 'UNIDAD'}
                  </td>
                  {esAdmin && (
                    <td style={{ padding: '16px', color: '#CBD5E1' }}>
                      {precioCompraCentimos !== null
                        ? `S/ ${(precioCompraCentimos / 100).toFixed(2)}`
                        : 'N/A'}
                    </td>
                  )}
                  <td style={{ padding: '16px', fontWeight: '700', color: '#34D399' }}>
                    S/ {(p.precio_venta / 100).toFixed(2)}
                  </td>
                  <td
                    style={{
                      padding: '16px',
                      fontWeight: '700',
                      color: esCritico ? '#EF4444' : '#F8FAFC',
                    }}
                  >
                    {p.stock_actual}
                  </td>
                  <td style={{ padding: '16px' }}>
                    <span style={getBadgeStyles(esCritico)}>
                      {esCritico ? 'STOCK CRÍTICO' : 'NORMAL'}
                    </span>
                  </td>
                  <td style={{ padding: '16px', textAlign: 'center' }}>
                    <div style={{ display: 'flex', justifyContent: 'center', gap: '8px' }}>
                      <button onClick={() => onVer(p)} title="Ver Detalle" style={btnAccionStyle}>
                        <Eye size={16} color="#38BDF8" />
                      </button>

                      <button
                        onClick={() => onDespachar(p)}
                        title="Despachar / Usar en taller"
                        style={{
                          ...btnAccionStyle,
                          backgroundColor: 'rgba(16, 185, 129, 0.15)',
                          border: '1px solid rgba(16, 185, 129, 0.3)',
                        }}
                      >
                        <PackageCheck size={16} color="#10B981" />
                      </button>

                      {esAdmin && (
                        <>
                          <button
                            onClick={() => onAjustar(p)}
                            title="Ajuste Manual de Stock"
                            style={{
                              ...btnAccionStyle,
                              backgroundColor: 'rgba(168, 85, 247, 0.15)',
                              border: '1px solid rgba(168, 85, 247, 0.3)',
                            }}
                          >
                            <Sliders size={16} color="#C084FC" />
                          </button>
                          <button
                            onClick={() => onEditar(p)}
                            title="Editar Producto"
                            style={btnAccionStyle}
                          >
                            <Pencil size={16} color="#FBBF24" />
                          </button>
                          <button
                            onClick={() => onEliminar(p.id_producto, p.nombre)}
                            title="Eliminar Producto"
                            style={{
                              ...btnAccionStyle,
                              backgroundColor: 'rgba(239, 68, 68, 0.15)',
                              border: '1px solid rgba(239, 68, 68, 0.3)',
                            }}
                          >
                            <Trash2 size={16} color="#EF4444" />
                          </button>
                        </>
                      )}
                    </div>
                  </td>
                </tr>
              );
            })
          )}
        </tbody>
      </table>
    </div>
  );
};

const btnAccionStyle: React.CSSProperties = {
  padding: '6px 10px',
  backgroundColor: 'rgba(30, 41, 59, 0.8)',
  border: '1px solid #334155',
  borderRadius: '8px',
  cursor: 'pointer',
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
};
