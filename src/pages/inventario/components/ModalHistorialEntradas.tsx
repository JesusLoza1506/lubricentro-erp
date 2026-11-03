import React, { useEffect, useState } from 'react';
import {
  ChevronDown,
  ChevronRight,
  ArrowLeft,
  ArrowRight,
  Filter,
  AlertCircle,
} from 'lucide-react';
import { IngresoStockResponseDTO } from '../../../types/inventario';
import { inventarioService } from '../../../services/inventarioService';

interface ModalHistorialEntradasProps {
  abierto: boolean;
  onClose: () => void;
  onError: (msg: string) => void;
  onActualizado: () => void;
}

export const ModalHistorialEntradas: React.FC<ModalHistorialEntradasProps> = ({
  abierto,
  onClose,
  onError,
  onActualizado,
}) => {
  const [ingresos, setIngresos] = useState<IngresoStockResponseDTO[]>([]);
  const [cargando, setCargando] = useState(false);
  const [expandidoId, setExpandidoId] = useState<number | null>(null);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);

  // Filtros
  const [filtroEstado, setFiltroEstado] = useState('TODOS');
  const [filtroOrigen, setFiltroOrigen] = useState('TODOS');

  // Paginación
  const [pagina, setPagina] = useState(0);
  const limite = 20;

  // Modal para anulación con motivo
  const [anularId, setAnularId] = useState<number | null>(null);
  const [motivoAnulacion, setMotivoAnulacion] = useState('');
  const [anulando, setAnulando] = useState(false);

  const cargarIngresos = async (numPagina: number) => {
    setCargando(true);
    setErrorMsg(null);
    try {
      const data = await inventarioService.listarIngresosStock(
        limite,
        numPagina * limite,
        filtroEstado === 'TODOS' ? undefined : filtroEstado,
        filtroOrigen === 'TODOS' ? undefined : filtroOrigen
      );
      setIngresos(data || []);
      setPagina(numPagina);
    } catch (err: unknown) {
      const msg =
        typeof err === 'string'
          ? err
          : (err as { message?: string })?.message || 'Error al cargar entradas de stock';
      setErrorMsg(msg);
      onError(msg);
    } finally {
      setCargando(false);
    }
  };

  useEffect(() => {
    if (abierto) {
      setPagina(0);
      cargarIngresos(0);
    }
  }, [abierto, filtroEstado, filtroOrigen]);

  if (!abierto) return null;

  const handleAnular = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!anularId || motivoAnulacion.trim().length < 5) return;
    try {
      setAnulando(true);
      await inventarioService.anularIngresoCompra({
        id_ingreso: anularId,
        motivo: motivoAnulacion.trim(),
      });
      setAnularId(null);
      setMotivoAnulacion('');
      cargarIngresos(pagina);
      onActualizado();
    } catch (err: unknown) {
      const msg =
        typeof err === 'string'
          ? err
          : (err as { message?: string })?.message || 'Error al anular entrada';
      onError(msg);
    } finally {
      setAnulando(false);
    }
  };

  return (
    <div
      style={{
        position: 'fixed',
        top: 0,
        left: 0,
        width: '100vw',
        height: '100vh',
        backgroundColor: 'rgba(0,0,0,0.85)',
        backdropFilter: 'blur(4px)',
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        zIndex: 1100,
      }}
    >
      <div
        style={{
          width: '950px',
          backgroundColor: '#0F172A',
          border: '1px solid #334155',
          borderRadius: '16px',
          padding: '24px',
          boxShadow: '0 20px 40px rgba(0,0,0,0.5)',
          maxHeight: '90vh',
          display: 'flex',
          flexDirection: 'column',
          gap: '16px',
        }}
      >
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
          <h3 style={{ fontSize: '20px', fontWeight: '800', color: '#F8FAFC', margin: 0 }}>
            Historial de Ingresos de Stock y Compras
          </h3>
          <button
            onClick={onClose}
            style={{
              backgroundColor: 'transparent',
              border: 'none',
              color: '#94A3B8',
              cursor: 'pointer',
              fontSize: '16px',
            }}
          >
            ✕ Cerrar
          </button>
        </div>

        {/* Barra de Filtros (Estado y Origen) */}
        <div
          style={{
            display: 'flex',
            gap: '16px',
            alignItems: 'center',
            backgroundColor: '#1E293B',
            padding: '12px 16px',
            borderRadius: '10px',
            flexWrap: 'wrap',
          }}
        >
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
            <Filter size={14} color="#94A3B8" />
            <label style={{ fontSize: '12px', color: '#CBD5E1' }}>Estado:</label>
            <select
              value={filtroEstado}
              onChange={(e) => setFiltroEstado(e.target.value)}
              style={selectFiltroStyle}
            >
              <option value="TODOS">TODOS LOS ESTADOS</option>
              <option value="REGISTRADO">REGISTRADOS</option>
              <option value="ANULADO">ANULADOS</option>
            </select>
          </div>

          <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
            <label style={{ fontSize: '12px', color: '#CBD5E1' }}>Origen:</label>
            <select
              value={filtroOrigen}
              onChange={(e) => setFiltroOrigen(e.target.value)}
              style={selectFiltroStyle}
            >
              <option value="TODOS">TODOS LOS ORIGENES</option>
              <option value="COMPRA">COMPRA</option>
              <option value="INICIAL">STOCK INICIAL</option>
              <option value="AJUSTE">AJUSTE</option>
            </select>
          </div>
        </div>

        {errorMsg && (
          <div
            style={{
              backgroundColor: 'rgba(239, 68, 68, 0.15)',
              border: '1px solid #EF4444',
              color: '#F87171',
              padding: '10px 14px',
              borderRadius: '8px',
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              fontSize: '13px',
            }}
          >
            <AlertCircle size={16} />
            <span>{errorMsg}</span>
          </div>
        )}

        <div
          style={{
            flex: 1,
            overflowY: 'auto',
            border: '1px solid #334155',
            borderRadius: '12px',
            backgroundColor: '#1E293B',
          }}
        >
          <table
            style={{
              width: '100%',
              borderCollapse: 'collapse',
              fontSize: '13px',
              textAlign: 'left',
            }}
          >
            <thead>
              <tr style={{ borderBottom: '1px solid #334155', color: '#94A3B8' }}>
                <th style={{ padding: '12px' }}></th>
                <th style={{ padding: '12px' }}>CÓDIGO</th>
                <th style={{ padding: '12px' }}>ORIGEN</th>
                <th style={{ padding: '12px' }}>PROVEEDOR</th>
                <th style={{ padding: '12px' }}>COMPROBANTE</th>
                <th style={{ padding: '12px' }}>COSTO TOTAL</th>
                <th style={{ padding: '12px' }}>ESTADO</th>
                <th style={{ padding: '12px', textAlign: 'center' }}>ACCIONES</th>
              </tr>
            </thead>
            <tbody>
              {cargando ? (
                <tr>
                  <td
                    colSpan={8}
                    style={{ padding: '24px', textAlign: 'center', color: '#94A3B8' }}
                  >
                    Cargando entradas de stock...
                  </td>
                </tr>
              ) : ingresos.length === 0 ? (
                <tr>
                  <td
                    colSpan={8}
                    style={{ padding: '24px', textAlign: 'center', color: '#94A3B8' }}
                  >
                    No se encontraron registros de ingreso con los filtros aplicados.
                  </td>
                </tr>
              ) : (
                ingresos.map((ing) => {
                  const esExpandido = expandidoId === ing.id_ingreso;
                  const totalSoles = (ing.total_costo_micro / 1_000_000).toFixed(2);
                  const esAnulado = ing.estado === 'ANULADO';

                  return (
                    <React.Fragment key={ing.id_ingreso}>
                      <tr
                        style={{
                          borderBottom: '1px solid #334155',
                          opacity: esAnulado ? 0.6 : 1,
                          backgroundColor: esExpandido ? 'rgba(51, 65, 85, 0.4)' : 'transparent',
                        }}
                      >
                        <td style={{ padding: '12px', textAlign: 'center' }}>
                          <button
                            onClick={() => setExpandidoId(esExpandido ? null : ing.id_ingreso)}
                            style={{
                              backgroundColor: 'transparent',
                              border: 'none',
                              color: '#38BDF8',
                              cursor: 'pointer',
                            }}
                          >
                            {esExpandido ? <ChevronDown size={16} /> : <ChevronRight size={16} />}
                          </button>
                        </td>
                        <td style={{ padding: '12px', fontFamily: 'monospace', color: '#38BDF8' }}>
                          {ing.codigo}
                        </td>
                        <td style={{ padding: '12px', color: '#F8FAFC' }}>{ing.origen}</td>
                        <td style={{ padding: '12px', color: '#CBD5E1' }}>
                          {ing.proveedor_nombre || 'Interno / N/A'}
                        </td>
                        <td style={{ padding: '12px', color: '#CBD5E1' }}>
                          {ing.tipo_documento}: {ing.numero_documento || '-'}
                        </td>
                        <td style={{ padding: '12px', fontWeight: '700', color: '#34D399' }}>
                          S/ {totalSoles}
                        </td>
                        <td style={{ padding: '12px' }}>
                          <span
                            style={{
                              padding: '4px 8px',
                              borderRadius: '6px',
                              fontSize: '11px',
                              fontWeight: '700',
                              backgroundColor: esAnulado
                                ? 'rgba(239, 68, 68, 0.15)'
                                : 'rgba(16, 185, 129, 0.15)',
                              color: esAnulado ? '#EF4444' : '#10B981',
                              border: `1px solid ${esAnulado ? '#EF4444' : '#10B981'}`,
                            }}
                          >
                            {ing.estado}
                          </span>
                        </td>
                        <td style={{ padding: '12px', textAlign: 'center' }}>
                          {!esAnulado && (
                            <button
                              onClick={() => {
                                setAnularId(ing.id_ingreso);
                                setMotivoAnulacion('');
                              }}
                              style={{
                                padding: '6px 10px',
                                backgroundColor: 'rgba(239, 68, 68, 0.15)',
                                border: '1px solid #EF4444',
                                color: '#EF4444',
                                borderRadius: '6px',
                                fontSize: '11px',
                                fontWeight: '700',
                                cursor: 'pointer',
                              }}
                            >
                              Anular
                            </button>
                          )}
                        </td>
                      </tr>

                      {esExpandido && (
                        <tr>
                          <td
                            colSpan={8}
                            style={{ padding: '12px 24px', backgroundColor: '#0F172A' }}
                          >
                            <div
                              style={{ marginBottom: '8px', fontSize: '12px', color: '#94A3B8' }}
                            >
                              Detalle de Productos Ingresados (Registrado por:{' '}
                              {ing.usuario_nombre || 'N/A'}):
                            </div>
                            <table
                              style={{
                                width: '100%',
                                borderCollapse: 'collapse',
                                fontSize: '12px',
                              }}
                            >
                              <thead>
                                <tr style={{ color: '#64748B', textAlign: 'left' }}>
                                  <th style={{ padding: '6px' }}>CÓDIGO</th>
                                  <th style={{ padding: '6px' }}>PRODUCTO</th>
                                  <th style={{ padding: '6px' }}>CANTIDAD</th>
                                  <th style={{ padding: '6px' }}>COSTO U. (S/)</th>
                                  <th style={{ padding: '6px' }}>SUBTOTAL</th>
                                </tr>
                              </thead>
                              <tbody>
                                {ing.detalles.map((d) => {
                                  const cantUI =
                                    d.unidad_medida === 'ML'
                                      ? `${(d.cantidad / 1000).toFixed(2)} L`
                                      : `${d.cantidad} UND`;
                                  const costoU = (d.costo_efectivo_micro / 1_000_000).toFixed(2);
                                  const subtotal = (d.valor_aporte_micro / 1_000_000).toFixed(2);
                                  return (
                                    <tr
                                      key={d.id_detalle}
                                      style={{
                                        borderBottom: '1px solid #1E293B',
                                        color: '#CBD5E1',
                                      }}
                                    >
                                      <td style={{ padding: '6px', fontFamily: 'monospace' }}>
                                        {d.codigo_snapshot}
                                      </td>
                                      <td style={{ padding: '6px', color: '#F8FAFC' }}>
                                        {d.nombre_snapshot}
                                      </td>
                                      <td style={{ padding: '6px' }}>{cantUI}</td>
                                      <td style={{ padding: '6px' }}>S/ {costoU}</td>
                                      <td
                                        style={{
                                          padding: '6px',
                                          fontWeight: '700',
                                          color: '#34D399',
                                        }}
                                      >
                                        S/ {subtotal}
                                      </td>
                                    </tr>
                                  );
                                })}
                              </tbody>
                            </table>
                          </td>
                        </tr>
                      )}
                    </React.Fragment>
                  );
                })
              )}
            </tbody>
          </table>
        </div>

        {/* Controles de Paginación */}
        <div
          style={{
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
            marginTop: '4px',
          }}
        >
          <button
            disabled={pagina === 0 || cargando}
            onClick={() => cargarIngresos(pagina - 1)}
            style={{
              padding: '8px 12px',
              backgroundColor: '#1E293B',
              border: '1px solid #334155',
              color: '#F8FAFC',
              borderRadius: '6px',
              cursor: pagina === 0 || cargando ? 'not-allowed' : 'pointer',
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              fontSize: '12px',
              opacity: pagina === 0 || cargando ? 0.5 : 1,
            }}
          >
            <ArrowLeft size={14} /> Anterior
          </button>
          <span style={{ fontSize: '13px', color: '#94A3B8' }}>Página {pagina + 1}</span>
          <button
            disabled={ingresos.length < limite || cargando}
            onClick={() => cargarIngresos(pagina + 1)}
            style={{
              padding: '8px 12px',
              backgroundColor: '#1E293B',
              border: '1px solid #334155',
              color: '#F8FAFC',
              borderRadius: '6px',
              cursor: ingresos.length < limite || cargando ? 'not-allowed' : 'pointer',
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              fontSize: '12px',
              opacity: ingresos.length < limite || cargando ? 0.5 : 1,
            }}
          >
            Siguiente <ArrowRight size={14} />
          </button>
        </div>
      </div>

      {/* Modal Confirmar Anulación */}
      {anularId && (
        <div
          style={{
            position: 'fixed',
            top: 0,
            left: 0,
            width: '100vw',
            height: '100vh',
            backgroundColor: 'rgba(0,0,0,0.85)',
            display: 'flex',
            justifyContent: 'center',
            alignItems: 'center',
            zIndex: 1300,
          }}
        >
          <div
            style={{
              width: '420px',
              backgroundColor: '#0F172A',
              border: '1px solid #EF4444',
              borderRadius: '16px',
              padding: '24px',
            }}
          >
            <h3 style={{ color: '#F8FAFC', fontSize: '18px', marginTop: 0 }}>
              Anular Ingreso / Compra
            </h3>
            <form
              onSubmit={handleAnular}
              style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}
            >
              <label style={{ fontSize: '12px', color: '#94A3B8' }}>
                Motivo de anulación (mínimo 5 caracteres) *
              </label>
              <input
                required
                minLength={5}
                maxLength={250}
                type="text"
                placeholder="Ej. Error en la factura del proveedor o lote devuelto"
                value={motivoAnulacion}
                onChange={(e) => setMotivoAnulacion(e.target.value)}
                style={{
                  width: '100%',
                  backgroundColor: '#1E293B',
                  border: '1px solid #334155',
                  color: '#F8FAFC',
                  padding: '10px',
                  borderRadius: '8px',
                  boxSizing: 'border-box',
                }}
              />
              <div
                style={{
                  display: 'flex',
                  justifyContent: 'flex-end',
                  gap: '10px',
                  marginTop: '10px',
                }}
              >
                <button
                  type="button"
                  onClick={() => setAnularId(null)}
                  style={{
                    padding: '8px 14px',
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
                  disabled={anulando || motivoAnulacion.trim().length < 5}
                  style={{
                    padding: '8px 18px',
                    backgroundColor: '#EF4444',
                    color: '#FFFFFF',
                    border: 'none',
                    borderRadius: '8px',
                    fontWeight: '700',
                    cursor: 'pointer',
                  }}
                >
                  {anulando ? 'Anulando...' : 'Confirmar Anulación'}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
};

const selectFiltroStyle: React.CSSProperties = {
  backgroundColor: '#0F172A',
  border: '1px solid #334155',
  color: '#F8FAFC',
  borderRadius: '6px',
  padding: '6px 10px',
  fontSize: '12px',
  outline: 'none',
};
