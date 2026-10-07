import React, { useState, useEffect } from 'react';
import { OrdenTrabajoHistorialDTO, EstadoOT } from '../../../types/ordenTrabajo';
import { useAuth } from '../../../context/AuthContext';
import { ordenesService, MecanicoDTO, ServicioResumenDTO } from '../../../services/ordenesService';
import { inventarioService } from '../../../services/inventarioService';
import { Producto } from '../../../types/inventario';

interface ModalDetalleOTProps {
  ot: OrdenTrabajoHistorialDTO;
  onClose: () => void;
  onCambiarEstado: (idOt: number, nuevoEstado: EstadoOT, zanja?: number) => Promise<void>;
  onCancelarOT?: (idOt: number) => Promise<void>;
  onReasignarMecanico?: (idOt: number, nuevoIdMecanico: number) => Promise<void>;
  onIrAFicha?: (placa: string) => void;
  onActualizarOT?: () => Promise<void>;
}

export const ModalDetalleOT: React.FC<ModalDetalleOTProps> = ({
  ot,
  onClose,
  onCambiarEstado,
  onCancelarOT,
  onReasignarMecanico,
  onIrAFicha,
  onActualizarOT,
}) => {
  const { usuario } = useAuth();
  const [ordenActual, setOrdenActual] = useState<OrdenTrabajoHistorialDTO>(ot);
  const [loading, setLoading] = useState(false);
  const [errorMsg, setErrorMsg] = useState('');
  const [exitoMsg, setExitoMsg] = useState('');

  // Reasignación de mecánico (Solo Admin)
  const [mecanicos, setMecanicos] = useState<MecanicoDTO[]>([]);
  const [idMecanicoSeleccionado, setIdMecanicoSeleccionado] = useState<number>(ot.id_mecanico);
  const [reasignando, setReasignando] = useState(false);

  // Estados para Insumos y Servicios
  const [productos, setProductos] = useState<Producto[]>([]);
  const [servicios, setServicios] = useState<ServicioResumenDTO[]>([]);
  const [tipoItem, setTipoItem] = useState<'PRODUCTO' | 'SERVICIO'>('PRODUCTO');
  const [idProductoSel, setIdProductoSel] = useState<number>(0);
  const [idServicioSel, setIdServicioSel] = useState<number>(0);
  const [cantidadProd, setCantidadProd] = useState<number>(1);
  const [agregandoItem, setAgregandoItem] = useState(false);

  const idUsuarioActual = usuario ? (usuario as any).id_usuario || (usuario as any).id || 0 : 0;
  const esAdmin = usuario?.rol === 'ADMINISTRADOR';
  const esMecanico = usuario?.rol === 'MECANICO';
  const esMecanicoAsignado = esMecanico && ordenActual.id_mecanico === idUsuarioActual;
  const puedeOperarInsumos = esAdmin || esMecanicoAsignado;
  const esTerminal = ordenActual.estado === 'FINALIZADO' || ordenActual.estado === 'CANCELADO';

  const obtenerIdUsuario = (): number => {
    if (!usuario) return 0;
    const uAny = usuario as any;
    return uAny.id_usuario || uAny.id || uAny.idUsuario || 0;
  };

  useEffect(() => {
    setOrdenActual(ot);
    setIdMecanicoSeleccionado(ot.id_mecanico);
  }, [ot]);

  useEffect(() => {
    if (esAdmin && !esTerminal) {
      ordenesService.listarMecanicos().then(setMecanicos).catch(console.error);
    }
  }, [esAdmin, esTerminal]);

  useEffect(() => {
    if (!esTerminal && puedeOperarInsumos) {
      inventarioService
        .listarProductosPublicos()
        .then((prods: Producto[]) => {
          setProductos(prods);
          if (prods.length > 0) setIdProductoSel(prods[0].id_producto);
        })
        .catch(console.error);

      ordenesService
        .listarServicios()
        .then((servs: ServicioResumenDTO[]) => {
          setServicios(servs);
          if (servs.length > 0) setIdServicioSel(servs[0].id_servicio);
        })
        .catch(console.error);
    }
  }, [esTerminal, puedeOperarInsumos]);

  const handleTransicion = async (nuevoEstado: EstadoOT) => {
    try {
      setLoading(true);
      setErrorMsg('');
      await onCambiarEstado(ordenActual.id_ot, nuevoEstado, ordenActual.zanja || 1);

      if (nuevoEstado === 'FINALIZADO') {
        setExitoMsg(
          '✅ Orden finalizada con éxito. La zanja ha sido liberada. Notifique al Cajero para la emisión del comprobante.'
        );
        setTimeout(() => onClose(), 3000);
      } else {
        onClose();
      }
    } catch (err: any) {
      setErrorMsg(typeof err === 'string' ? err : err?.message || 'Error al cambiar estado.');
    } finally {
      setLoading(false);
    }
  };

  const handleCambioMecanico = async (nuevoId: number) => {
    if (!onReasignarMecanico || nuevoId === ordenActual.id_mecanico) return;
    try {
      setReasignando(true);
      setErrorMsg('');
      await onReasignarMecanico(ordenActual.id_ot, nuevoId);
      setIdMecanicoSeleccionado(nuevoId);
    } catch (err: any) {
      setErrorMsg(typeof err === 'string' ? err : err?.message || 'Error al reasignar mecánico.');
      setIdMecanicoSeleccionado(ordenActual.id_mecanico);
    } finally {
      setReasignando(false);
    }
  };

  const handleAgregarItem = async () => {
    try {
      setAgregandoItem(true);
      setErrorMsg('');
      const uId = obtenerIdUsuario();

      let otActualizada: OrdenTrabajoHistorialDTO;
      if (tipoItem === 'PRODUCTO') {
        if (!idProductoSel) return;
        otActualizada = await ordenesService.agregarProductoOT(
          { id_ot: ordenActual.id_ot, id_producto: idProductoSel, cantidad: cantidadProd },
          uId
        );
      } else {
        if (!idServicioSel) return;
        otActualizada = await ordenesService.agregarServicioOT(
          { id_ot: ordenActual.id_ot, id_servicio: idServicioSel },
          uId
        );
      }

      setOrdenActual(otActualizada);
      if (onActualizarOT) await onActualizarOT();
    } catch (err: any) {
      setErrorMsg(typeof err === 'string' ? err : err?.message || 'Error al agregar ítem.');
    } finally {
      setAgregandoItem(false);
    }
  };

  const handleEliminarItem = async (idDetalle: number, tipo: string) => {
    try {
      setLoading(true);
      setErrorMsg('');
      const uId = obtenerIdUsuario();

      let otActualizada: OrdenTrabajoHistorialDTO;
      if (tipo === 'PRODUCTO') {
        otActualizada = await ordenesService.eliminarProductoOT(idDetalle, ordenActual.id_ot, uId);
      } else {
        otActualizada = await ordenesService.eliminarServicioOT(idDetalle, ordenActual.id_ot, uId);
      }

      setOrdenActual(otActualizada);
      if (onActualizarOT) await onActualizarOT();
    } catch (err: any) {
      setErrorMsg(typeof err === 'string' ? err : err?.message || 'Error al eliminar ítem.');
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
        backgroundColor: 'rgba(15, 23, 42, 0.8)',
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
          maxWidth: '720px',
          padding: '24px',
          border: '1px solid #334155',
          color: '#F8FAFC',
          maxHeight: '90vh',
          overflowY: 'auto',
          overflowX: 'hidden',
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
          <h2 style={{ margin: 0, fontSize: '20px' }}>
            Detalle de Orden: <span style={{ color: '#38BDF8' }}>{ordenActual.codigo_ot}</span>
          </h2>
          <button
            onClick={() => {
              onClose();
              if (onIrAFicha) onIrAFicha(ordenActual.placa);
            }}
            style={{
              backgroundColor: '#334155',
              color: '#38BDF8',
              border: 'none',
              padding: '6px 12px',
              borderRadius: '6px',
              fontSize: '12px',
              cursor: 'pointer',
              fontWeight: 600,
            }}
          >
            Ver Ficha Vehicular ({ordenActual.placa}) ↗
          </button>
        </div>

        {errorMsg && (
          <div
            style={{
              padding: '10px',
              backgroundColor: '#7F1D1D',
              color: '#FCA5A5',
              borderRadius: '6px',
              marginBottom: '16px',
              fontSize: '14px',
            }}
          >
            {errorMsg}
          </div>
        )}

        {exitoMsg && (
          <div
            style={{
              padding: '12px',
              backgroundColor: '#064E3B',
              border: '1px solid #059669',
              color: '#A7F3D0',
              borderRadius: '6px',
              marginBottom: '16px',
              fontSize: '14px',
              fontWeight: 600,
            }}
          >
            {exitoMsg}
          </div>
        )}

        <div
          style={{
            display: 'grid',
            gridTemplateColumns: 'repeat(2, 1fr)',
            gap: '12px',
            backgroundColor: '#0F172A',
            padding: '16px',
            borderRadius: '8px',
            marginBottom: '20px',
          }}
        >
          <div>
            <p style={{ margin: '4px 0', fontSize: '13px', color: '#94A3B8' }}>Estado Actual</p>
            <strong style={{ color: '#F8FAFC' }}>{ordenActual.estado.replace('_', ' ')}</strong>
          </div>
          <div>
            <p style={{ margin: '4px 0', fontSize: '13px', color: '#94A3B8' }}>Zanja Asignada</p>
            <strong style={{ color: '#F8FAFC' }}>
              {ordenActual.zanja ? `Zanja ${ordenActual.zanja}` : 'Ninguna'}
            </strong>
          </div>
          <div>
            <p style={{ margin: '4px 0', fontSize: '13px', color: '#94A3B8' }}>Mecánico Asignado</p>
            {esAdmin && !esTerminal ? (
              <select
                disabled={reasignando}
                value={idMecanicoSeleccionado}
                onChange={(e) => handleCambioMecanico(Number(e.target.value))}
                style={{
                  width: '100%',
                  padding: '6px 10px',
                  backgroundColor: '#1E293B',
                  border: '1px solid #38BDF8',
                  borderRadius: '6px',
                  color: '#F8FAFC',
                  fontSize: '13px',
                  marginTop: '2px',
                }}
              >
                {mecanicos.map((m) => (
                  <option key={m.id_usuario} value={m.id_usuario}>
                    {m.nombre_completo}
                  </option>
                ))}
              </select>
            ) : (
              <strong style={{ color: '#F8FAFC' }}>
                {ordenActual.nombre_mecanico || 'No asignado'}
              </strong>
            )}
          </div>
          <div>
            <p style={{ margin: '4px 0', fontSize: '13px', color: '#94A3B8' }}>
              Kilometraje Ingreso / Próximo
            </p>
            <strong style={{ color: '#F8FAFC' }}>
              {ordenActual.kilometraje_ingreso.toLocaleString()} /{' '}
              {ordenActual.proximo_kilometraje.toLocaleString()} km
            </strong>
          </div>
        </div>

        {/* Formulario para Agregar Ítem (Solo para Admin o Mecánico asignado a la OT) */}
        {!esTerminal && puedeOperarInsumos && (
          <div
            style={{
              backgroundColor: '#0F172A',
              padding: '14px',
              borderRadius: '8px',
              marginBottom: '20px',
              border: '1px solid #334155',
            }}
          >
            <h4 style={{ margin: '0 0 10px 0', fontSize: '14px', color: '#38BDF8' }}>
              + Agregar Insumo o Servicio
            </h4>
            <div
              style={{
                display: 'flex',
                flexWrap: 'wrap',
                gap: '8px',
                alignItems: 'center',
              }}
            >
              <select
                value={tipoItem}
                onChange={(e) => setTipoItem(e.target.value as any)}
                style={{
                  flex: '0 0 110px',
                  padding: '8px',
                  backgroundColor: '#1E293B',
                  border: '1px solid #334155',
                  borderRadius: '6px',
                  color: '#F8FAFC',
                  fontSize: '12px',
                }}
              >
                <option value="PRODUCTO">Producto</option>
                <option value="SERVICIO">Servicio</option>
              </select>

              <div style={{ flex: '1 1 200px', minWidth: '180px' }}>
                {tipoItem === 'PRODUCTO' ? (
                  <select
                    value={idProductoSel}
                    onChange={(e) => setIdProductoSel(Number(e.target.value))}
                    style={{
                      width: '100%',
                      padding: '8px',
                      backgroundColor: '#1E293B',
                      border: '1px solid #334155',
                      borderRadius: '6px',
                      color: '#F8FAFC',
                      fontSize: '12px',
                    }}
                  >
                    {productos.map((p) => (
                      <option key={p.id_producto} value={p.id_producto}>
                        {p.nombre} (S/ {(p.precio_venta / 100.0).toFixed(2)}) - Stock:{' '}
                        {p.stock_actual}
                      </option>
                    ))}
                  </select>
                ) : (
                  <select
                    value={idServicioSel}
                    onChange={(e) => setIdServicioSel(Number(e.target.value))}
                    style={{
                      width: '100%',
                      padding: '8px',
                      backgroundColor: '#1E293B',
                      border: '1px solid #334155',
                      borderRadius: '6px',
                      color: '#F8FAFC',
                      fontSize: '12px',
                    }}
                  >
                    {servicios.map((s) => (
                      <option key={s.id_servicio} value={s.id_servicio}>
                        {s.descripcion} (S/ {s.precio_base.toFixed(2)})
                      </option>
                    ))}
                  </select>
                )}
              </div>

              <div style={{ flex: '0 0 70px' }}>
                {tipoItem === 'PRODUCTO' ? (
                  <input
                    type="number"
                    min={1}
                    value={cantidadProd}
                    onChange={(e) => setCantidadProd(Number(e.target.value))}
                    style={{
                      width: '100%',
                      padding: '8px',
                      backgroundColor: '#1E293B',
                      border: '1px solid #334155',
                      borderRadius: '6px',
                      color: '#F8FAFC',
                      fontSize: '12px',
                      textAlign: 'center',
                    }}
                  />
                ) : (
                  <span
                    style={{
                      fontSize: '12px',
                      color: '#94A3B8',
                      textAlign: 'center',
                      display: 'block',
                    }}
                  >
                    1 unid
                  </span>
                )}
              </div>

              <button
                type="button"
                onClick={handleAgregarItem}
                disabled={agregandoItem}
                style={{
                  flex: '0 0 75px',
                  padding: '8px 12px',
                  backgroundColor: '#2563EB',
                  color: '#FFF',
                  border: 'none',
                  borderRadius: '6px',
                  cursor: 'pointer',
                  fontWeight: 600,
                  fontSize: '12px',
                }}
              >
                {agregandoItem ? '...' : 'Añadir'}
              </button>
            </div>
          </div>
        )}

        <h3 style={{ fontSize: '15px', color: '#CBD5E1', marginBottom: '8px' }}>
          Insumos y Servicios Aplicados
        </h3>
        {ordenActual.detalles.length === 0 ? (
          <p style={{ fontSize: '13px', color: '#64748B', fontStyle: 'italic' }}>
            No se registraron productos ni servicios adicionales.
          </p>
        ) : (
          <table style={{ width: '100%', borderCollapse: 'collapse', marginBottom: '20px' }}>
            <thead>
              <tr
                style={{
                  borderBottom: '1px solid #334155',
                  textAlign: 'left',
                  fontSize: '12px',
                  color: '#94A3B8',
                }}
              >
                <th style={{ padding: '8px' }}>Ítem</th>
                <th style={{ padding: '8px' }}>Tipo</th>
                <th style={{ padding: '8px' }}>Cant.</th>
                <th style={{ padding: '8px' }}>P. Unit</th>
                <th style={{ padding: '8px' }}>Subtotal</th>
                {!esTerminal && puedeOperarInsumos && (
                  <th style={{ padding: '8px', textAlign: 'center' }}>Acción</th>
                )}
              </tr>
            </thead>
            <tbody>
              {ordenActual.detalles.map((d) => (
                <tr
                  key={`${d.tipo}-${d.id_detalle}`}
                  style={{ borderBottom: '1px solid #1E293B', fontSize: '13px' }}
                >
                  <td style={{ padding: '8px' }}>{d.descripcion}</td>
                  <td style={{ padding: '8px', color: '#94A3B8' }}>{d.tipo}</td>
                  <td style={{ padding: '8px' }}>{d.cantidad}</td>
                  <td style={{ padding: '8px' }}>S/ {d.precio_unitario.toFixed(2)}</td>
                  <td style={{ padding: '8px', fontWeight: '600' }}>S/ {d.subtotal.toFixed(2)}</td>
                  {!esTerminal && puedeOperarInsumos && (
                    <td style={{ padding: '8px', textAlign: 'center' }}>
                      <button
                        onClick={() => handleEliminarItem(d.id_detalle, d.tipo)}
                        style={{
                          backgroundColor: '#7F1D1D',
                          color: '#FCA5A5',
                          border: 'none',
                          padding: '2px 8px',
                          borderRadius: '4px',
                          cursor: 'pointer',
                          fontSize: '11px',
                        }}
                      >
                        ✕
                      </button>
                    </td>
                  )}
                </tr>
              ))}
            </tbody>
          </table>
        )}

        <div
          style={{
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
            marginTop: '24px',
            paddingTop: '16px',
            borderTop: '1px solid #334155',
          }}
        >
          <div>
            {/* Cancelación restringida estrictamente al ADMINISTRADOR */}
            {esAdmin && !esTerminal && onCancelarOT && (
              <button
                type="button"
                onClick={async () => {
                  try {
                    setLoading(true);
                    await onCancelarOT(ordenActual.id_ot);
                    onClose();
                  } catch (err: any) {
                    setErrorMsg(
                      typeof err === 'string' ? err : err?.message || 'Error al cancelar la OT.'
                    );
                  } finally {
                    setLoading(false);
                  }
                }}
                disabled={loading || !!exitoMsg}
                style={{
                  padding: '8px 14px',
                  backgroundColor: '#7F1D1D',
                  color: '#FCA5A5',
                  border: 'none',
                  borderRadius: '6px',
                  cursor: 'pointer',
                  fontSize: '13px',
                }}
              >
                Cancelar OT
              </button>
            )}
          </div>

          <div style={{ display: 'flex', gap: '10px' }}>
            <button
              type="button"
              onClick={onClose}
              style={{
                padding: '8px 16px',
                backgroundColor: 'transparent',
                color: '#94A3B8',
                border: '1px solid #334155',
                borderRadius: '6px',
                cursor: 'pointer',
              }}
            >
              Cerrar
            </button>

            {(esAdmin || esMecanicoAsignado) && ordenActual.estado === 'EN_ESPERA' && (
              <button
                type="button"
                onClick={() => handleTransicion('EN_PROCESO')}
                disabled={loading || !!exitoMsg}
                style={{
                  padding: '8px 16px',
                  backgroundColor: '#2563EB',
                  color: '#FFF',
                  border: 'none',
                  borderRadius: '6px',
                  fontWeight: '600',
                  cursor: 'pointer',
                }}
              >
                Iniciar Trabajo (EN PROCESO)
              </button>
            )}

            {(esAdmin || esMecanicoAsignado) && ordenActual.estado === 'EN_PROCESO' && (
              <button
                type="button"
                onClick={() => handleTransicion('FINALIZADO')}
                disabled={loading || !!exitoMsg}
                style={{
                  padding: '8px 16px',
                  backgroundColor: '#059669',
                  color: '#FFF',
                  border: 'none',
                  borderRadius: '6px',
                  fontWeight: '600',
                  cursor: 'pointer',
                }}
              >
                Finalizar Trabajo
              </button>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};
