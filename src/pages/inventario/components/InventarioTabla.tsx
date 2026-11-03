import React from 'react';
import {
  Eye,
  PackageCheck,
  Sliders,
  Pencil,
  Trash2,
  History,
  ChevronLeft,
  ChevronRight,
} from 'lucide-react';
import { Producto, ProductoAdminDTO } from '../../../types/inventario';
import { getBadgeStyles } from '../inventarioStyles';

export interface InventarioTablaProps {
  cargando: boolean;
  productos: Producto[];
  esAdmin: boolean;
  paginaActual: number;
  totalPaginas: number;
  totalRegistros: number;
  busqueda?: string;
  esVistaCriticos?: boolean;
  onCambiarPagina: (nuevaPagina: number) => void;
  onVer: (p: Producto) => void;
  onDespachar: (p: Producto) => void;
  onAjustar: (p: Producto) => void;
  onVerKardex: (p: Producto) => void;
  onEditar: (p: Producto) => void;
  onEliminar: (id: number, nombre: string, activo: boolean) => void;
}

export const InventarioTabla: React.FC<InventarioTablaProps> = ({
  cargando,
  productos,
  esAdmin,
  paginaActual,
  totalPaginas,
  totalRegistros,
  busqueda,
  esVistaCriticos,
  onCambiarPagina,
  onVer,
  onDespachar,
  onAjustar,
  onVerKardex,
  onEditar,
  onEliminar,
}) => {
  return (
    <div style={{ flex: 1, display: 'flex', flexDirection: 'column', gap: '12px' }}>
      <div
        style={{
          flex: 1,
          backgroundColor: 'rgba(15, 23, 42, 0.6)',
          border: '1px solid #334155',
          borderRadius: '16px',
          overflowY: 'auto',
          maxHeight: 'calc(100vh - 330px)',
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
              <th style={{ padding: '16px' }}>CATEGORÍA</th>
              <th style={{ padding: '16px' }}>UNIDAD</th>
              {esAdmin && <th style={{ padding: '16px' }}>ÚLTIMO COSTO</th>}
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
                  colSpan={esAdmin ? 9 : 8}
                  style={{ padding: '32px', textAlign: 'center', color: '#94A3B8' }}
                >
                  Cargando productos...
                </td>
              </tr>
            ) : productos.length === 0 ? (
              <tr>
                <td
                  colSpan={esAdmin ? 9 : 8}
                  style={{ padding: '32px', textAlign: 'center', color: '#94A3B8' }}
                >
                  {esVistaCriticos
                    ? 'No se encontraron productos en estado crítico de stock.'
                    : busqueda && busqueda.trim() !== ''
                      ? `No se encontraron productos que coincidan con "${busqueda}".`
                      : 'No se encontraron productos en el catálogo.'}
                </td>
              </tr>
            ) : (
              productos.map((p) => {
                const esCritico = p.es_critico ?? p.stock_actual <= p.stock_minimo;
                const pAdmin = p as ProductoAdminDTO;
                const ultimoCostoSol =
                  esAdmin && pAdmin.ultimo_costo_micro
                    ? (pAdmin.ultimo_costo_micro / 1_000_000).toFixed(2)
                    : null;

                const estaActivo = Boolean(p.activo);

                return (
                  <tr
                    key={p.id_producto}
                    style={{
                      borderBottom: '1px solid rgba(51, 65, 85, 0.4)',
                      opacity: estaActivo ? 1 : 0.6,
                    }}
                  >
                    <td style={{ padding: '16px', color: '#38BDF8', fontFamily: 'monospace' }}>
                      {p.codigo}
                    </td>
                    <td style={{ padding: '16px', fontWeight: '600', color: '#F8FAFC' }}>
                      {p.nombre}{' '}
                      {!estaActivo && <small style={{ color: '#EF4444' }}>(Inactivo)</small>}
                    </td>
                    <td style={{ padding: '16px', color: '#94A3B8' }}>{p.categoria}</td>
                    <td style={{ padding: '16px', color: '#94A3B8' }}>{p.unidad_medida}</td>
                    {esAdmin && (
                      <td style={{ padding: '16px', color: '#CBD5E1' }}>
                        {ultimoCostoSol ? `S/ ${ultimoCostoSol}` : 'Sin costo'}
                      </td>
                    )}
                    <td style={{ padding: '16px', fontWeight: '700', color: '#34D399' }}>
                      S/ {(p.precio_venta_cent / 100).toFixed(2)}
                    </td>
                    <td
                      style={{
                        padding: '16px',
                        fontWeight: '700',
                        color: esCritico ? '#EF4444' : '#F8FAFC',
                      }}
                    >
                      {p.unidad_medida === 'ML'
                        ? `${(p.stock_actual / 1000).toFixed(2)} L`
                        : `${p.stock_actual} UND`}
                    </td>
                    <td style={{ padding: '16px' }}>
                      <span style={getBadgeStyles(esCritico)}>
                        {esCritico ? 'CRÍTICO' : 'NORMAL'}
                      </span>
                    </td>
                    <td style={{ padding: '16px', textAlign: 'center' }}>
                      <div style={{ display: 'flex', justifyContent: 'center', gap: '8px' }}>
                        <button onClick={() => onVer(p)} title="Ver Detalle" style={btnAccionStyle}>
                          <Eye size={16} color="#38BDF8" />
                        </button>

                        {p.categoria === 'GRANEL' && estaActivo && (
                          <button
                            onClick={() => onDespachar(p)}
                            title="Despachar Granel (Taller)"
                            style={{
                              ...btnAccionStyle,
                              backgroundColor: 'rgba(16, 185, 129, 0.15)',
                              border: '1px solid rgba(16, 185, 129, 0.3)',
                            }}
                          >
                            <PackageCheck size={16} color="#10B981" />
                          </button>
                        )}

                        {esAdmin && (
                          <>
                            <button
                              onClick={() => onVerKardex(p)}
                              title="Ver Historial Kárdex"
                              style={{
                                ...btnAccionStyle,
                                backgroundColor: 'rgba(56, 189, 248, 0.15)',
                                border: '1px solid rgba(56, 189, 248, 0.3)',
                              }}
                            >
                              <History size={16} color="#38BDF8" />
                            </button>

                            <button
                              onClick={() => onAjustar(p)}
                              title="Conteo Físico Reconciliado"
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
                              onClick={() => onEliminar(p.id_producto, p.nombre, estaActivo)}
                              title={estaActivo ? 'Dar de Baja Producto' : 'Reactivar Producto'}
                              style={{
                                ...btnAccionStyle,
                                backgroundColor: estaActivo
                                  ? 'rgba(239, 68, 68, 0.15)'
                                  : 'rgba(16, 185, 129, 0.15)',
                                border: estaActivo
                                  ? '1px solid rgba(239, 68, 68, 0.3)'
                                  : '1px solid rgba(16, 185, 129, 0.3)',
                              }}
                            >
                              <Trash2 size={16} color={estaActivo ? '#EF4444' : '#10B981'} />
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

      {/* Barra de Paginación */}
      <div
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          padding: '12px 16px',
          backgroundColor: '#1E293B',
          borderRadius: '12px',
          border: '1px solid #334155',
          fontSize: '13px',
          color: '#CBD5E1',
        }}
      >
        <span>
          Total de productos: <strong style={{ color: '#F8FAFC' }}>{totalRegistros}</strong>
        </span>
        <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
          <span>
            Página <strong style={{ color: '#F8FAFC' }}>{paginaActual}</strong> de{' '}
            <strong style={{ color: '#F8FAFC' }}>{totalPaginas || 1}</strong>
          </span>
          <div style={{ display: 'flex', gap: '6px' }}>
            <button
              disabled={paginaActual <= 1}
              onClick={() => onCambiarPagina(paginaActual - 1)}
              style={{
                padding: '6px 12px',
                backgroundColor: paginaActual <= 1 ? 'rgba(51, 65, 85, 0.3)' : '#0F172A',
                border: '1px solid #334155',
                color: paginaActual <= 1 ? '#64748B' : '#F8FAFC',
                borderRadius: '8px',
                cursor: paginaActual <= 1 ? 'not-allowed' : 'pointer',
                display: 'flex',
                alignItems: 'center',
                gap: '4px',
              }}
            >
              <ChevronLeft size={16} /> Anterior
            </button>
            <button
              disabled={paginaActual >= totalPaginas}
              onClick={() => onCambiarPagina(paginaActual + 1)}
              style={{
                padding: '6px 12px',
                backgroundColor: paginaActual >= totalPaginas ? 'rgba(51, 65, 85, 0.3)' : '#0F172A',
                border: '1px solid #334155',
                color: paginaActual >= totalPaginas ? '#64748B' : '#F8FAFC',
                borderRadius: '8px',
                cursor: paginaActual >= totalPaginas ? 'not-allowed' : 'pointer',
                display: 'flex',
                alignItems: 'center',
                gap: '4px',
              }}
            >
              Siguiente <ChevronRight size={16} />
            </button>
          </div>
        </div>
      </div>
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
