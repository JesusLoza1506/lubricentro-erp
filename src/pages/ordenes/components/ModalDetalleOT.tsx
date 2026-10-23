import React, { useState, useEffect } from 'react';
import { OrdenTrabajoHistorialDTO, TipoAceiteOT } from '../../../types/ordenTrabajo';
import { OrdenTrabajoHistorialDTO as VehiculoOTDTO } from '../../../types/vehiculo';
import { useAuth } from '../../../context/AuthContext';
import { ordenesService, MecanicoDTO } from '../../../services/ordenesService';

export interface ModalDetalleOTProps {
  ordenTrabajo?: OrdenTrabajoHistorialDTO | VehiculoOTDTO;
  ot?: OrdenTrabajoHistorialDTO | VehiculoOTDTO;
  modalAbierto?: boolean;
  setModalAbierto?: (abierto: boolean) => void;
  onClose: () => void;
  onIniciarZanja?: (idOt: number, idZanja: number) => Promise<void>;
  onFinalizarOT?: (
    idOt: number,
    kmSalida: number,
    resumen: string,
    tipoAceite?: TipoAceiteOT
  ) => Promise<void>;
  onCancelarOT?: (idOt: number, motivo: string) => Promise<void>;
  onReasignarMecanico?: (idOt: number, nuevoIdMecanico: number, motivo: string) => Promise<void>;
  onActualizarOT?: () => Promise<void>;
}

export const ModalDetalleOT: React.FC<ModalDetalleOTProps> = ({
  ordenTrabajo,
  ot,
  onClose,
  onIniciarZanja,
  onFinalizarOT,
  onCancelarOT,
  onReasignarMecanico,
  onActualizarOT,
}) => {
  const { usuario } = useAuth();
  const otInicial = (ordenTrabajo || ot) as OrdenTrabajoHistorialDTO;

  const [ordenActual, setOrdenActual] = useState<OrdenTrabajoHistorialDTO>(
    otInicial || {
      id_ot: 0,
      codigo_ot: '',
      placa: '',
      estado: 'EN_ESPERA',
      kilometraje_ingreso: 0,
      fecha_ingreso: '',
      servicios: [],
      insumos: [],
    }
  );

  const [loading, setLoading] = useState(false);
  const [errorMsg, setErrorMsg] = useState('');

  const [modoAccion, setModoAccion] = useState<
    'NINGUNO' | 'INICIAR' | 'FINALIZAR' | 'CANCELAR' | 'REASIGNAR' | 'DEVOLVER'
  >('NINGUNO');

  const [idZanjaSel, setIdZanjaSel] = useState<number>(1);
  const [kmSalida, setKmSalida] = useState<number>(otInicial?.kilometraje_ingreso ?? 0);
  const [resumenPublico, setResumenPublico] = useState<string>(
    'Servicio técnico completado satisfactoriamente.'
  );
  const [tipoAceite, setTipoAceite] = useState<TipoAceiteOT | ''>('');

  const [motivoAccion, setMotivoAccion] = useState<string>('');
  const [idMecanicoNuevo, setIdMecanicoNuevo] = useState<number>(0);
  const [idDetalleDevolver, setIdDetalleDevolver] = useState<number>(0);
  const [cantDevolver, setCantDevolver] = useState<number>(1);

  const [mecanicos, setMecanicos] = useState<MecanicoDTO[]>([]);

  const rol = usuario?.rol || 'CAJERO';
  const idUsuario = (usuario as any)?.id_usuario || (usuario as any)?.id || 0;
  const esTerminal = ordenActual.estado === 'FINALIZADO' || ordenActual.estado === 'CANCELADO';

  useEffect(() => {
    if (otInicial) {
      setOrdenActual(otInicial);
      setKmSalida(otInicial.kilometraje_ingreso ?? 0);
    }
  }, [ordenTrabajo, ot]);

  useEffect(() => {
    if (!esTerminal) {
      ordenesService.listarMecanicos().then(setMecanicos).catch(console.error);
    }
  }, [esTerminal]);

  const handleIniciarSubmit = async () => {
    if (!onIniciarZanja) return;
    try {
      setLoading(true);
      setErrorMsg('');
      await onIniciarZanja(ordenActual.id_ot, idZanjaSel);
      setModoAccion('NINGUNO');
      if (onActualizarOT) await onActualizarOT();
    } catch (err: unknown) {
      setErrorMsg(
        typeof err === 'string' ? err : (err as any)?.message || 'Error al iniciar zanja'
      );
    } finally {
      setLoading(false);
    }
  };

  const handleFinalizarSubmit = async () => {
    if (!onFinalizarOT) return;
    try {
      setLoading(true);
      setErrorMsg('');
      await onFinalizarOT(
        ordenActual.id_ot,
        kmSalida,
        resumenPublico,
        tipoAceite !== '' ? tipoAceite : undefined
      );
      setModoAccion('NINGUNO');
      if (onActualizarOT) await onActualizarOT();
    } catch (err: unknown) {
      setErrorMsg(typeof err === 'string' ? err : (err as any)?.message || 'Error al finalizar OT');
    } finally {
      setLoading(false);
    }
  };

  const handleCancelarSubmit = async () => {
    if (!onCancelarOT) return;
    if (motivoAccion.trim().length < 5) {
      setErrorMsg('El motivo debe tener al menos 5 caracteres.');
      return;
    }
    try {
      setLoading(true);
      setErrorMsg('');
      await onCancelarOT(ordenActual.id_ot, motivoAccion);
      setModoAccion('NINGUNO');
      setMotivoAccion('');
      if (onActualizarOT) await onActualizarOT();
    } catch (err: unknown) {
      setErrorMsg(typeof err === 'string' ? err : (err as any)?.message || 'Error al cancelar OT');
    } finally {
      setLoading(false);
    }
  };

  const handleReasignarSubmit = async () => {
    if (!onReasignarMecanico) return;
    if (idMecanicoNuevo <= 0) {
      setErrorMsg('Seleccione un mecánico válido.');
      return;
    }
    if (motivoAccion.trim().length < 5) {
      setErrorMsg('El motivo debe tener al menos 5 caracteres.');
      return;
    }
    try {
      setLoading(true);
      setErrorMsg('');
      await onReasignarMecanico(ordenActual.id_ot, idMecanicoNuevo, motivoAccion);
      setModoAccion('NINGUNO');
      setMotivoAccion('');
      if (onActualizarOT) await onActualizarOT();
    } catch (err: unknown) {
      setErrorMsg(
        typeof err === 'string' ? err : (err as any)?.message || 'Error al reasignar mecánico'
      );
    } finally {
      setLoading(false);
    }
  };

  const handleDevolverInsumoSubmit = async () => {
    if (motivoAccion.trim().length < 5) {
      setErrorMsg('El motivo de devolución debe tener al menos 5 caracteres.');
      return;
    }
    try {
      setLoading(true);
      setErrorMsg('');
      const act = await ordenesService.devolverInsumoOT(
        {
          id_ot: ordenActual.id_ot,
          id_detalle_producto: idDetalleDevolver,
          cantidad_a_devolver: cantDevolver,
          motivo: motivoAccion,
        },
        idUsuario,
        rol
      );
      setOrdenActual(act);
      setModoAccion('NINGUNO');
      setMotivoAccion('');
      if (onActualizarOT) await onActualizarOT();
    } catch (err: unknown) {
      setErrorMsg(
        typeof err === 'string' ? err : (err as any)?.message || 'Error al devolver insumo'
      );
    } finally {
      setLoading(false);
    }
  };

  return (
    <div
      style={{
        position: 'fixed',
        top: 0,
        left: 0,
        right: 0,
        bottom: 0,
        backgroundColor: 'rgba(15, 23, 42, 0.85)',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        zIndex: 1000,
      }}
    >
      <div
        style={{
          backgroundColor: '#1E293B',
          borderRadius: '12px',
          width: '100%',
          maxWidth: '760px',
          padding: '24px',
          border: '1px solid #334155',
          color: '#F8FAFC',
          maxHeight: '90vh',
          overflowY: 'auto',
        }}
      >
        <div
          style={{
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
            marginBottom: '16px',
          }}
        >
          <h2 style={{ margin: 0, fontSize: '18px' }}>
            OT: <span style={{ color: '#38BDF8' }}>{ordenActual.codigo_ot}</span> (
            {ordenActual.placa})
          </h2>
          <span
            style={{
              padding: '4px 10px',
              borderRadius: '6px',
              fontSize: '12px',
              fontWeight: 'bold',
              backgroundColor: '#334155',
            }}
          >
            {ordenActual.estado}
          </span>
        </div>

        {errorMsg && (
          <div
            style={{
              padding: '10px',
              backgroundColor: '#7F1D1D',
              color: '#FCA5A5',
              borderRadius: '6px',
              marginBottom: '16px',
              fontSize: '13px',
            }}
          >
            {errorMsg}
          </div>
        )}

        {ordenActual.nota_interna && (
          <div
            style={{
              padding: '10px',
              backgroundColor: '#0F172A',
              borderLeft: '4px solid #F59E0B',
              borderRadius: '4px',
              marginBottom: '16px',
              fontSize: '13px',
            }}
          >
            🔒 <strong>Nota Interna:</strong> {ordenActual.nota_interna}
          </div>
        )}

        {/* ACCIONES DE ESTADO DE ORDEN */}
        {!esTerminal && (
          <div
            style={{
              display: 'flex',
              gap: '8px',
              marginBottom: '16px',
              flexWrap: 'wrap',
            }}
          >
            {ordenActual.estado === 'EN_ESPERA' && onIniciarZanja && (
              <button
                disabled={loading}
                onClick={() => setModoAccion('INICIAR')}
                style={{
                  padding: '6px 12px',
                  backgroundColor: '#2563EB',
                  color: '#FFF',
                  border: 'none',
                  borderRadius: '6px',
                  cursor: 'pointer',
                  fontSize: '12px',
                  fontWeight: '600',
                }}
              >
                ▶ Iniciar Trabajo en Zanja
              </button>
            )}

            {ordenActual.estado === 'EN_PROCESO' && onFinalizarOT && (
              <button
                disabled={loading}
                onClick={() => setModoAccion('FINALIZAR')}
                style={{
                  padding: '6px 12px',
                  backgroundColor: '#16A34A',
                  color: '#FFF',
                  border: 'none',
                  borderRadius: '6px',
                  cursor: 'pointer',
                  fontSize: '12px',
                  fontWeight: '600',
                }}
              >
                ✔ Finalizar OT
              </button>
            )}

            {rol === 'ADMINISTRADOR' && onReasignarMecanico && (
              <button
                disabled={loading}
                onClick={() => setModoAccion('REASIGNAR')}
                style={{
                  padding: '6px 12px',
                  backgroundColor: '#0284C7',
                  color: '#FFF',
                  border: 'none',
                  borderRadius: '6px',
                  cursor: 'pointer',
                  fontSize: '12px',
                  fontWeight: '600',
                }}
              >
                👤 Reasignar Mecánico
              </button>
            )}

            {onCancelarOT && (
              <button
                disabled={loading}
                onClick={() => setModoAccion('CANCELAR')}
                style={{
                  padding: '6px 12px',
                  backgroundColor: '#DC2626',
                  color: '#FFF',
                  border: 'none',
                  borderRadius: '6px',
                  cursor: 'pointer',
                  fontSize: '12px',
                  fontWeight: '600',
                }}
              >
                ✖ Cancelar OT
              </button>
            )}
          </div>
        )}

        {/* FORMULARIOS DE ACCIÓN */}
        {modoAccion === 'INICIAR' && (
          <div
            style={{
              padding: '12px',
              backgroundColor: '#0F172A',
              borderRadius: '6px',
              marginBottom: '16px',
              border: '1px solid #2563EB',
            }}
          >
            <h5 style={{ margin: '0 0 8px 0', color: '#38BDF8' }}>Asignar Zanja de Trabajo</h5>
            <select
              value={idZanjaSel}
              onChange={(e) => setIdZanjaSel(Number(e.target.value))}
              style={{
                width: '100%',
                padding: '6px',
                marginBottom: '8px',
                backgroundColor: '#1E293B',
                border: '1px solid #334155',
                color: '#FFF',
              }}
            >
              {[1, 2, 3, 4, 5, 6].map((z) => (
                <option key={z} value={z}>
                  Zanja N° {z}
                </option>
              ))}
            </select>
            <div style={{ display: 'flex', gap: '8px', justifyContent: 'flex-end' }}>
              <button
                onClick={() => setModoAccion('NINGUNO')}
                style={{
                  padding: '4px 10px',
                  backgroundColor: '#475569',
                  color: '#FFF',
                  border: 'none',
                  borderRadius: '4px',
                }}
              >
                Cancelar
              </button>
              <button
                onClick={handleIniciarSubmit}
                style={{
                  padding: '4px 10px',
                  backgroundColor: '#2563EB',
                  color: '#FFF',
                  border: 'none',
                  borderRadius: '4px',
                }}
              >
                Confirmar Inicio
              </button>
            </div>
          </div>
        )}

        {modoAccion === 'FINALIZAR' && (
          <div
            style={{
              padding: '12px',
              backgroundColor: '#0F172A',
              borderRadius: '6px',
              marginBottom: '16px',
              border: '1px solid #16A34A',
            }}
          >
            <h5 style={{ margin: '0 0 8px 0', color: '#4ADE80' }}>Finalizar Orden de Trabajo</h5>
            <label style={{ fontSize: '12px', color: '#94A3B8' }}>Km Salida *</label>
            <input
              type="number"
              value={kmSalida}
              onChange={(e) => setKmSalida(Number(e.target.value))}
              style={{
                width: '100%',
                padding: '6px',
                marginBottom: '8px',
                backgroundColor: '#1E293B',
                border: '1px solid #334155',
                color: '#FFF',
              }}
            />
            <label style={{ fontSize: '12px', color: '#94A3B8' }}>Tipo de Aceite Utilizado</label>
            <select
              value={tipoAceite}
              onChange={(e) => setTipoAceite(e.target.value as TipoAceiteOT | '')}
              style={{
                width: '100%',
                padding: '6px',
                marginBottom: '8px',
                backgroundColor: '#1E293B',
                border: '1px solid #334155',
                color: '#FFF',
              }}
            >
              <option value="">No aplica / Sin cambio de aceite</option>
              <option value="MINERAL">MINERAL (5,000 km)</option>
              <option value="SINTETICO">SINTETICO (10,000 km)</option>
            </select>
            <label style={{ fontSize: '12px', color: '#94A3B8' }}>Resumen Público *</label>
            <textarea
              rows={2}
              value={resumenPublico}
              onChange={(e) => setResumenPublico(e.target.value)}
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
                onClick={() => setModoAccion('NINGUNO')}
                style={{
                  padding: '4px 10px',
                  backgroundColor: '#475569',
                  color: '#FFF',
                  border: 'none',
                  borderRadius: '4px',
                }}
              >
                Cancelar
              </button>
              <button
                onClick={handleFinalizarSubmit}
                style={{
                  padding: '4px 10px',
                  backgroundColor: '#16A34A',
                  color: '#FFF',
                  border: 'none',
                  borderRadius: '4px',
                }}
              >
                Confirmar Finalización
              </button>
            </div>
          </div>
        )}

        {modoAccion === 'CANCELAR' && (
          <div
            style={{
              padding: '12px',
              backgroundColor: '#0F172A',
              borderRadius: '6px',
              marginBottom: '16px',
              border: '1px solid #DC2626',
            }}
          >
            <h5 style={{ margin: '0 0 8px 0', color: '#FCA5A5' }}>Cancelar Orden de Trabajo</h5>
            <textarea
              rows={2}
              value={motivoAccion}
              onChange={(e) => setMotivoAccion(e.target.value)}
              placeholder="Motivo de cancelación (mínimo 5 caracteres)..."
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
                onClick={() => setModoAccion('NINGUNO')}
                style={{
                  padding: '4px 10px',
                  backgroundColor: '#475569',
                  color: '#FFF',
                  border: 'none',
                  borderRadius: '4px',
                }}
              >
                Volver
              </button>
              <button
                onClick={handleCancelarSubmit}
                style={{
                  padding: '4px 10px',
                  backgroundColor: '#DC2626',
                  color: '#FFF',
                  border: 'none',
                  borderRadius: '4px',
                }}
              >
                Confirmar Cancelación
              </button>
            </div>
          </div>
        )}

        {modoAccion === 'REASIGNAR' && (
          <div
            style={{
              padding: '12px',
              backgroundColor: '#0F172A',
              borderRadius: '6px',
              marginBottom: '16px',
              border: '1px solid #0284C7',
            }}
          >
            <h5 style={{ margin: '0 0 8px 0', color: '#38BDF8' }}>Reasignar Mecánico</h5>
            <select
              value={idMecanicoNuevo}
              onChange={(e) => setIdMecanicoNuevo(Number(e.target.value))}
              style={{
                width: '100%',
                padding: '6px',
                marginBottom: '8px',
                backgroundColor: '#1E293B',
                border: '1px solid #334155',
                color: '#FFF',
              }}
            >
              <option value={0}>Seleccione Nuevo Mecánico...</option>
              {mecanicos.map((m) => (
                <option key={m.id_usuario} value={m.id_usuario}>
                  {m.nombre_completo}
                </option>
              ))}
            </select>
            <textarea
              rows={2}
              value={motivoAccion}
              onChange={(e) => setMotivoAccion(e.target.value)}
              placeholder="Motivo de reasignación (mínimo 5 caracteres)..."
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
                onClick={() => setModoAccion('NINGUNO')}
                style={{
                  padding: '4px 10px',
                  backgroundColor: '#475569',
                  color: '#FFF',
                  border: 'none',
                  borderRadius: '4px',
                }}
              >
                Cancelar
              </button>
              <button
                onClick={handleReasignarSubmit}
                style={{
                  padding: '4px 10px',
                  backgroundColor: '#0284C7',
                  color: '#FFF',
                  border: 'none',
                  borderRadius: '4px',
                }}
              >
                Confirmar Reasignación
              </button>
            </div>
          </div>
        )}

        {/* SECCIÓN SERVICIOS */}
        <h4 style={{ color: '#38BDF8', marginBottom: '8px' }}>Servicios Aplicados</h4>
        <table
          style={{
            width: '100%',
            borderCollapse: 'collapse',
            marginBottom: '16px',
            fontSize: '13px',
          }}
        >
          <thead>
            <tr style={{ borderBottom: '1px solid #334155', color: '#94A3B8' }}>
              <th style={{ textAlign: 'left', padding: '6px' }}>Servicio</th>
              <th style={{ padding: '6px' }}>Cant.</th>
              <th style={{ padding: '6px' }}>P. Unit</th>
              <th style={{ padding: '6px' }}>Subtotal</th>
            </tr>
          </thead>
          <tbody>
            {ordenActual.servicios && ordenActual.servicios.length > 0 ? (
              ordenActual.servicios.map((s) => (
                <tr key={s.id_detalle} style={{ borderBottom: '1px solid #0F172A' }}>
                  <td style={{ padding: '6px' }}>{s.nombre_snapshot}</td>
                  <td style={{ padding: '6px', textAlign: 'center' }}>{s.cantidad}</td>
                  <td style={{ padding: '6px', textAlign: 'right' }}>
                    S/ {s.precio_unitario_soles.toFixed(2)}
                  </td>
                  <td style={{ padding: '6px', textAlign: 'right', fontWeight: 'bold' }}>
                    S/ {s.subtotal_soles.toFixed(2)}
                  </td>
                </tr>
              ))
            ) : (
              <tr>
                <td colSpan={4} style={{ padding: '8px', textAlign: 'center', color: '#94A3B8' }}>
                  Sin servicios registrados
                </td>
              </tr>
            )}
          </tbody>
        </table>

        {/* SECCIÓN INSUMOS */}
        <h4 style={{ color: '#38BDF8', marginBottom: '8px' }}>Insumos Consumidos</h4>
        <table
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
                    {i.devuelto_ml_und}
                  </td>
                  <td style={{ padding: '6px', textAlign: 'center', fontWeight: 'bold' }}>
                    {i.neto_ml_und}
                  </td>
                  <td style={{ padding: '6px', textAlign: 'right' }}>
                    S/ {i.subtotal_soles.toFixed(2)}
                  </td>
                  {!esTerminal && (
                    <td style={{ padding: '6px', textAlign: 'center' }}>
                      <button
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
                          cursor: 'pointer',
                          fontSize: '11px',
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
        </table>

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
              type="number"
              min={1}
              value={cantDevolver}
              onChange={(e) => setCantDevolver(Number(e.target.value))}
              placeholder="Cantidad"
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
                onClick={() => setModoAccion('NINGUNO')}
                style={{
                  padding: '4px 10px',
                  backgroundColor: '#475569',
                  color: '#FFF',
                  border: 'none',
                  borderRadius: '4px',
                }}
              >
                Cancelar
              </button>
              <button
                onClick={handleDevolverInsumoSubmit}
                style={{
                  padding: '4px 10px',
                  backgroundColor: '#D97706',
                  color: '#FFF',
                  border: 'none',
                  borderRadius: '4px',
                }}
              >
                Confirmar Devolución
              </button>
            </div>
          </div>
        )}

        <div
          style={{
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
            marginTop: '20px',
            paddingTop: '12px',
            borderTop: '1px solid #334155',
          }}
        >
          <strong style={{ fontSize: '16px', color: '#10B981' }}>
            Total General: S/ {(ordenActual.total_general_soles ?? 0).toFixed(2)}
          </strong>
          <button
            onClick={onClose}
            style={{
              padding: '8px 16px',
              backgroundColor: '#475569',
              color: '#FFF',
              border: 'none',
              borderRadius: '6px',
              cursor: 'pointer',
            }}
          >
            Cerrar
          </button>
        </div>
      </div>
    </div>
  );
};
