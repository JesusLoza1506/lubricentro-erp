import React from 'react';
import { OrdenTrabajoHistorialDTO } from '../../../../../types/ordenTrabajo';
import { ProductoInsumoDTO } from '../../../../../services/ordenesService';

interface TablaInsumosOTProps {
  ordenActual: OrdenTrabajoHistorialDTO;
  esTerminal: boolean;
  modoAccion: string;
  setModoAccion: (modo: any) => void;
  productosCatalogo: ProductoInsumoDTO[];
  idProductoSel: number;
  setIdProductoSel: (v: number) => void;
  cantProductoSel: number;
  setCantProductoSel: (v: number) => void;
  esGranelSel: boolean;
  cantDevolver: number;
  setCantDevolver: (v: number) => void;
  motivoAccion: string;
  setMotivoAccion: (v: string) => void;
  setIdDetalleDevolver: (v: number) => void;
  onAgregarInsumoSubmit: () => void;
  onDevolverInsumoSubmit: () => void;
  procesandoDespacho?: boolean;
  procesandoDevolucion?: boolean;
}

export const TablaInsumosOT: React.FC<TablaInsumosOTProps> = ({
  ordenActual,
  esTerminal,
  modoAccion,
  setModoAccion,
  productosCatalogo,
  idProductoSel,
  setIdProductoSel,
  cantProductoSel,
  setCantProductoSel,
  esGranelSel,
  cantDevolver,
  setCantDevolver,
  motivoAccion,
  setMotivoAccion,
  setIdDetalleDevolver,
  onAgregarInsumoSubmit,
  onDevolverInsumoSubmit,
  procesandoDespacho = false,
  procesandoDevolucion = false,
}) => {
  return (
    <section className="ot-detail-section ot-supplies-section">
      <div
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          marginBottom: '8px',
        }}
      >
        <h4 style={{ color: '#38BDF8', margin: 0 }}>Insumos Consumidos</h4>
        {!esTerminal && ordenActual.estado === 'EN_PROCESO' && (
          <button
            disabled={procesandoDespacho}
            onClick={() => setModoAccion('AGREGAR_INSUMO')}
            style={{
              padding: '4px 8px',
              backgroundColor: '#16A34A',
              color: '#FFF',
              border: 'none',
              borderRadius: '4px',
              fontSize: '12px',
              cursor: procesandoDespacho ? 'not-allowed' : 'pointer',
              opacity: procesandoDespacho ? 0.6 : 1,
            }} data-tone="primary"
          >
            + Despachar Insumo
          </button>
        )}
      </div>

      {modoAccion === 'AGREGAR_INSUMO' && (
        <div
          style={{
            padding: '10px',
            backgroundColor: '#0F172A',
            borderRadius: '6px',
            marginBottom: '12px',
          }}
        >
          <label style={{ fontSize: '12px', color: '#94A3B8' }}>Seleccionar Producto/Insumo</label>
          <select
            disabled={procesandoDespacho}
            value={idProductoSel}
            onChange={(e) => setIdProductoSel(Number(e.target.value))}
            style={{
              width: '100%',
              padding: '6px',
              marginBottom: '8px',
              backgroundColor: '#1E293B',
              border: '1px solid #334155',
              color: '#FFF',
            }}
          >
            <option value={0}>-- Seleccionar Producto --</option>
            {(productosCatalogo || []).map((p) => (
              <option key={p.id_producto} value={p.id_producto}>
                {p.nombre} ({p.unidad_medida}) - Stock: {p.stock_actual} - S/{' '}
                {(p.precio_venta_soles ?? 0).toFixed(2)}
              </option>
            ))}
          </select>
          <label style={{ fontSize: '12px', color: '#94A3B8' }}>
            Cantidad {esGranelSel ? 'en ML (Mililitros)' : 'en Unidades (UND)'}
          </label>
          <input
            disabled={procesandoDespacho}
            type="number"
            min={0.001}
            step="any"
            value={cantProductoSel}
            onChange={(e) => setCantProductoSel(Number(e.target.value))}
            placeholder={esGranelSel ? 'Ej. 3500 (3.5 Litros = 3500 ML)' : 'Ej. 1'}
            style={{
              width: '100%',
              padding: '6px',
              marginBottom: '8px',
              backgroundColor: '#1E293B',
              border: '1px solid #334155',
              color: '#FFF',
            }}
          />
          <div style={{ display: 'flex', gap: '8px', justifyContent: 'flex-end' }}>
            <button
              disabled={procesandoDespacho}
              onClick={() => setModoAccion('NINGUNO')}
              style={{
                padding: '4px 8px',
                backgroundColor: '#475569',
                color: '#FFF',
                border: 'none',
                borderRadius: '4px',
                cursor: procesandoDespacho ? 'not-allowed' : 'pointer',
              }}
            >
              Cancelar
            </button>
            <button
              disabled={procesandoDespacho}
              onClick={onAgregarInsumoSubmit}
              style={{
                padding: '4px 8px',
                backgroundColor: '#16A34A',
                color: '#FFF',
                border: 'none',
                borderRadius: '4px',
                cursor: procesandoDespacho ? 'not-allowed' : 'pointer',
                opacity: procesandoDespacho ? 0.6 : 1,
              }} data-tone="primary"
            >
              {procesandoDespacho ? 'Despachando...' : 'Confirmar Despacho'}
            </button>
          </div>
        </div>
      )}

      <div className="ot-detail-table-scroll"><table
        style={{
          width: '100%',
          borderCollapse: 'collapse',
          marginBottom: '20px',
          fontSize: '13px',
        }}
      >
        <thead>
          <tr style={{ borderBottom: '1px solid #334155', color: '#94A3B8' }}>
            <th style={{ textAlign: 'left', padding: '6px' }}>Insumo</th>
            <th style={{ padding: '6px' }}>Consumido</th>
            <th style={{ padding: '6px' }}>Devuelto</th>
            <th style={{ padding: '6px' }}>Neto</th>
            <th style={{ padding: '6px' }}>Subtotal</th>
            {!esTerminal && <th style={{ padding: '6px' }}>Acción</th>}
          </tr>
        </thead>
        <tbody>
          {ordenActual.insumos && ordenActual.insumos.length > 0 ? (
            ordenActual.insumos.map((i) => (
              <tr key={i.id_detalle} style={{ borderBottom: '1px solid #0F172A' }}>
                <td style={{ padding: '6px' }}>{i.nombre_snapshot}</td>
                <td style={{ padding: '6px', textAlign: 'center' }}>
                  {i.cantidad_formateada} {i.unidad_medida}
                </td>
                <td style={{ padding: '6px', textAlign: 'center', color: '#F59E0B' }}>
                  {i.devuelto_ml_und} {i.unidad_medida === 'ML' ? 'ML' : 'UND'}
                </td>
                <td style={{ padding: '6px', textAlign: 'center', fontWeight: 'bold' }}>
                  {i.neto_ml_und} {i.unidad_medida === 'ML' ? 'ML' : 'UND'}
                </td>
                <td style={{ padding: '6px', textAlign: 'right' }}>
                  S/ {(i.subtotal_soles ?? 0).toFixed(2)}
                </td>
                {!esTerminal && (
                  <td style={{ padding: '6px', textAlign: 'center' }}>
                    <button
                      disabled={procesandoDevolucion}
                      onClick={() => {
                        setIdDetalleDevolver(i.id_detalle);
                        setModoAccion('DEVOLVER');
                      }}
                      style={{
                        padding: '2px 6px',
                        backgroundColor: '#D97706',
                        color: '#FFF',
                        border: 'none',
                        borderRadius: '4px',
                        cursor: procesandoDevolucion ? 'not-allowed' : 'pointer',
                        fontSize: '11px',
                        opacity: procesandoDevolucion ? 0.6 : 1,
                      }}
                    >
                      Devolver
                    </button>
                  </td>
                )}
              </tr>
            ))
          ) : (
            <tr>
              <td
                colSpan={!esTerminal ? 6 : 5}
                style={{ padding: '8px', textAlign: 'center', color: '#94A3B8' }}
              >
                Sin insumos registrados
              </td>
            </tr>
          )}
        </tbody>
      </table></div>

      {modoAccion === 'DEVOLVER' && (
        <div
          style={{
            padding: '12px',
            backgroundColor: '#0F172A',
            borderRadius: '6px',
            marginBottom: '16px',
            border: '1px solid #D97706',
          }}
        >
          <h5 style={{ margin: '0 0 8px 0', color: '#F59E0B' }}>
            Devolución de Insumo a Inventario
          </h5>
          <input
            disabled={procesandoDevolucion}
            type="number"
            min={0.001}
            step="any"
            value={cantDevolver}
            onChange={(e) => setCantDevolver(Number(e.target.value))}
            placeholder="Cantidad en ML o UND"
            style={{
              width: '100%',
              padding: '6px',
              marginBottom: '8px',
              backgroundColor: '#1E293B',
              border: '1px solid #334155',
              color: '#FFF',
            }}
          />
          <textarea
            disabled={procesandoDevolucion}
            rows={2}
            value={motivoAccion}
            onChange={(e) => setMotivoAccion(e.target.value)}
            placeholder="Motivo obligatorio de devolución (min 5 caracteres)..."
            style={{
              width: '100%',
              padding: '6px',
              marginBottom: '8px',
              backgroundColor: '#1E293B',
              border: '1px solid #334155',
              color: '#FFF',
            }}
          />
          <div style={{ display: 'flex', gap: '8px', justifyContent: 'flex-end' }}>
            <button
              disabled={procesandoDevolucion}
              onClick={() => setModoAccion('NINGUNO')}
              style={{
                padding: '4px 10px',
                backgroundColor: '#475569',
                color: '#FFF',
                border: 'none',
                borderRadius: '4px',
                cursor: procesandoDevolucion ? 'not-allowed' : 'pointer',
              }}
            >
              Cancelar
            </button>
            <button
              disabled={procesandoDevolucion}
              onClick={onDevolverInsumoSubmit}
              style={{
                padding: '4px 10px',
                backgroundColor: '#D97706',
                color: '#FFF',
                border: 'none',
                borderRadius: '4px',
                cursor: procesandoDevolucion ? 'not-allowed' : 'pointer',
                opacity: procesandoDevolucion ? 0.6 : 1,
              }}
            >
              {procesandoDevolucion ? 'Devolviendo...' : 'Confirmar Devolución'}
            </button>
          </div>
        </div>
      )}
    </section>
  );
};
