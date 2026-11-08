import { useNotificarInventario } from '../AvisosInventario';
import { GestionProveedores } from './GestionProveedores';
import { solicitudPendiente } from '../../../services/operacionInventario';
import { AnularIngresoDTO } from '../../../types/inventario';
import React, { useEffect, useState, useRef } from 'react';
import { ChevronDown, ChevronRight, ArrowLeft, ArrowRight, RefreshCw } from 'lucide-react';
import { IngresoStockResponseDTO } from '../../../types/inventario';
import { inventarioService } from '../../../services/inventarioService';

interface ModalHistorialEntradasProps {
  integrado?: boolean;
  abierto: boolean;
  onClose: () => void;
  onError: (msg: string) => void;
  onActualizado: () => void;
}

export const ModalHistorialEntradas: React.FC<ModalHistorialEntradasProps> = ({
  abierto,
  integrado = false,
  onClose,
  onError,
  onActualizado,
}) => {
  const notificar = useNotificarInventario();
  const enviando = useRef(false);
  const consulta = useRef(0);
  const [incierta, setIncierta] = useState(false);
  const [fechaInicio, setFechaInicio] = useState('');
  const [fechaFin, setFechaFin] = useState('');
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
    const turno = ++consulta.current;
    setCargando(true);
    setErrorMsg(null);
    try {
      const data = await inventarioService.listarIngresosStock(
        limite,
        numPagina * limite,
        filtroEstado === 'TODOS' ? undefined : filtroEstado,
        filtroOrigen === 'TODOS' ? undefined : filtroOrigen,
        fechaInicio || undefined,
        fechaFin || undefined
      );
      if (turno !== consulta.current) return;
      setIngresos(data || []);
      setPagina(numPagina);
    } catch (err: unknown) {
      const msg =
        typeof err === 'string'
          ? err
          : (err as { message?: string })?.message || 'Error al cargar entradas de stock';
      if (turno !== consulta.current) return;
      setErrorMsg(msg);
      onError(msg);
    } finally {
      if (turno === consulta.current) setCargando(false);
    }
  };

  useEffect(() => {
    if (abierto) {
      setPagina(0);
      cargarIngresos(0);
      try {
        const pendiente = solicitudPendiente('anular_ingreso_compra_cmd')?.payload as
          | AnularIngresoDTO
          | undefined;
        setIncierta(!!pendiente);
        if (pendiente) {
          setAnularId(pendiente.id_ingreso);
          setMotivoAnulacion(pendiente.motivo);
        }
      } catch (error) {
        setErrorMsg(String(error));
      }
    }
  }, [abierto, filtroEstado, filtroOrigen]);

  if (!abierto) return null;

  const handleAnular = async (e: React.FormEvent) => {
    e.preventDefault();
    if (enviando.current) return;
    if (!anularId || motivoAnulacion.trim().length < 5) return;
    try {
      enviando.current = true;
      setAnulando(true);
      const pendiente = solicitudPendiente('anular_ingreso_compra_cmd');
      if (pendiente && (pendiente.payload as AnularIngresoDTO).id_ingreso !== anularId)
        throw new Error('Reabre la anulación pendiente antes de iniciar otra.');
      await inventarioService.anularIngresoCompra(
        (pendiente?.payload as AnularIngresoDTO) ?? {
          id_ingreso: anularId,
          motivo: motivoAnulacion.trim(),
        }
      );
      setIncierta(false);
      notificar('Entrada anulada correctamente', 'EXITO');
      setAnularId(null);
      setMotivoAnulacion('');
      cargarIngresos(pagina);
      onActualizado();
    } catch (err: unknown) {
      const msg =
        typeof err === 'string'
          ? err
          : (err as { message?: string })?.message || 'Error al anular entrada';
      setErrorMsg(msg);
      setIncierta(!!solicitudPendiente('anular_ingreso_compra_cmd'));
      onError(msg);
    } finally {
      enviando.current = false;
      setAnulando(false);
    }
  };

  const moneda = (micro: number) =>
    (micro / 1000000).toLocaleString('es-PE', {
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    });
  const abrirAnulacion = (id: number) => {
    const pendiente = solicitudPendiente('anular_ingreso_compra_cmd')?.payload as
      | AnularIngresoDTO
      | undefined;
    setAnularId(pendiente?.id_ingreso ?? id);
    if (pendiente) {
      setMotivoAnulacion(pendiente.motivo);
      setIncierta(true);
      return;
    }
    setMotivoAnulacion('');
  };
  return (
    <div
      className={integrado ? 'inv-entry-panel' : 'gloria-theme inv-overlay'}
      style={{
        position: integrado ? 'relative' : 'fixed',
        inset: integrado ? undefined : 0,
        display: 'flex',
        flex: 1,
        minHeight: 0,
        alignItems: integrado ? 'stretch' : 'center',
        justifyContent: 'center',
        zIndex: integrado ? undefined : 1100,
      }}
    >
      <div
        className={integrado ? 'inv-entry-content' : 'inv-dialog inv-dialog-wide'}
        style={{
          display: 'flex',
          flexDirection: 'column',
          gap: 14,
          minHeight: 0,
          flex: integrado ? 1 : undefined,
        }}
      >
        <header className={integrado ? 'inv-list-heading inv-entry-heading' : 'inv-dialog-header'}>
          <div>
            <h3>Entradas de inventario</h3>
            <p>Consulta ingresos y documentos del almacén.</p>
          </div>
          {!integrado && (
            <button className="inv-button" disabled={anulando} onClick={onClose}>
              Cerrar
            </button>
          )}
        </header>
        <div className="inv-entry-toolbar">
          <label>
            Desde
            <input
              aria-label="Entradas desde"
              type="date"
              value={fechaInicio}
              onChange={(e) => setFechaInicio(e.target.value)}
            />
          </label>
          <label>
            Hasta
            <input
              aria-label="Entradas hasta"
              type="date"
              value={fechaFin}
              onChange={(e) => setFechaFin(e.target.value)}
            />
          </label>
          <button className="inv-button" disabled={cargando} onClick={() => cargarIngresos(0)}>
            Aplicar fechas
          </button>
          <label>
            Estado
            <select
              aria-label="Estado de entrada"
              value={filtroEstado}
              onChange={(e) => setFiltroEstado(e.target.value)}
            >
              <option value="TODOS">Todos los estados</option>
              <option value="REGISTRADO">Registrado</option>
              <option value="ANULADO">Anulado</option>
            </select>
          </label>
          <label>
            Origen
            <select
              aria-label="Origen de entrada"
              value={filtroOrigen}
              onChange={(e) => setFiltroOrigen(e.target.value)}
            >
              <option value="TODOS">Todos los orígenes</option>
              <option value="COMPRA">Compra</option>
              <option value="INICIAL">Stock inicial</option>
              <option value="AJUSTE">Ajuste</option>
            </select>
          </label>
          <button
            className="inv-icon-button"
            title="Actualizar entradas"
            aria-label="Actualizar entradas"
            disabled={cargando}
            onClick={() => cargarIngresos(pagina)}
          >
            <RefreshCw size={16} />
          </button>
        </div>
        <GestionProveedores />
        {errorMsg && !anularId && (
          <p role="alert" className="inv-form-error">
            {errorMsg}
          </p>
        )}
        <section className="inv-table-section" aria-label="Entradas de inventario">
          <div
            className="inv-table-scroll"
            tabIndex={0}
            role="region"
            aria-label="Resultados de entradas"
            aria-busy={cargando}
          >
            <table className="inv-table inv-entry-table">
              <thead>
                <tr>
                  <th></th>
                  <th>Código / fecha</th>
                  <th>Origen</th>
                  <th>Proveedor</th>
                  <th>Comprobante</th>
                  <th className="inv-number">Costo total</th>
                  <th>Estado</th>
                  <th>Acciones</th>
                </tr>
              </thead>
              <tbody>
                {cargando ? (
                  <tr>
                    <td colSpan={8} className="inv-empty">
                      Cargando entradas de stock...
                    </td>
                  </tr>
                ) : !ingresos.length ? (
                  <tr>
                    <td colSpan={8} className="inv-empty">
                      No se encontraron registros de ingreso con los filtros aplicados.
                    </td>
                  </tr>
                ) : (
                  ingresos.map((ing) => {
                    const expandido = expandidoId === ing.id_ingreso;
                    const anulado = ing.estado === 'ANULADO';
                    return (
                      <React.Fragment key={ing.id_ingreso}>
                        <tr style={{ opacity: anulado ? 0.65 : 1 }}>
                          <td>
                            <button
                              className="inv-icon-button"
                              aria-label={`Ver detalle de ${ing.codigo}`}
                              aria-expanded={expandido}
                              onClick={() => setExpandidoId(expandido ? null : ing.id_ingreso)}
                            >
                              {expandido ? <ChevronDown size={15} /> : <ChevronRight size={15} />}
                            </button>
                          </td>
                          <td>
                            <span className="inv-code">{ing.codigo}</span>
                            <span className="inv-muted-line">{ing.fecha_registro}</span>
                          </td>
                          <td className="inv-meta">{ing.origen}</td>
                          <td>{ing.proveedor_nombre || 'Ingreso interno'}</td>
                          <td>
                            {ing.tipo_documento && ing.tipo_documento !== 'SIN_DOCUMENTO' ? (
                              <>
                                <span className="inv-meta">{ing.tipo_documento}</span>
                                <span className="inv-muted-line">
                                  {ing.numero_documento || '—'}
                                </span>
                              </>
                            ) : (
                              <span className="inv-meta">Sin documento</span>
                            )}
                          </td>
                          <td className="inv-number">S/ {moneda(ing.total_costo_micro)}</td>
                          <td>
                            <span className={`inv-badge ${anulado ? 'is-muted' : ''}`}>
                              {anulado ? 'Anulado' : 'Registrado'}
                            </span>
                          </td>
                          <td>
                            {!anulado && (
                              <button
                                className="inv-button inv-annul"
                                onClick={() => abrirAnulacion(ing.id_ingreso)}
                              >
                                Anular
                              </button>
                            )}
                          </td>
                        </tr>
                        {expandido && (
                          <tr>
                            <td colSpan={8}>
                              <p className="inv-help" style={{ marginBottom: 12 }}>
                                Registrado por {ing.usuario_nombre || 'N/A'} · {ing.fecha_registro}
                              </p>
                              {ing.observacion && <p className="inv-help">{ing.observacion}</p>}
                              {anulado && (
                                <p className="inv-help">
                                  Anulación: {ing.motivo_anulacion} · {ing.fecha_anulacion}
                                </p>
                              )}
                              <table className="inv-table">
                                <thead>
                                  <tr>
                                    <th>Código</th>
                                    <th>Producto</th>
                                    <th className="inv-number">Cantidad</th>
                                    <th className="inv-number">Costo / UND o L</th>
                                    <th className="inv-number">Subtotal</th>
                                  </tr>
                                </thead>
                                <tbody>
                                  {ing.detalles.map((d) => (
                                    <tr key={d.id_detalle}>
                                      <td className="inv-code">{d.codigo_snapshot}</td>
                                      <td>{d.nombre_snapshot}</td>
                                      <td className="inv-number">
                                        {d.unidad_medida === 'ML'
                                          ? `${(d.cantidad / 1000).toFixed(3)} L`
                                          : `${d.cantidad} UND`}
                                      </td>
                                      <td className="inv-number">
                                        S/ {moneda(d.costo_efectivo_micro)}
                                      </td>
                                      <td className="inv-number">
                                        S/ {moneda(d.valor_aporte_micro)}
                                      </td>
                                    </tr>
                                  ))}
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
          <footer className="inv-table-footer">
            <span>Entradas · {ingresos.length} registros en esta página</span>
            <div className="inv-pagination">
              <span>
                Página <strong>{pagina + 1}</strong>
              </span>
              <button
                className="inv-button"
                disabled={pagina === 0 || cargando}
                onClick={() => cargarIngresos(pagina - 1)}
              >
                <ArrowLeft size={14} />
                Anterior
              </button>
              <button
                className="inv-button"
                disabled={ingresos.length < limite || cargando}
                onClick={() => cargarIngresos(pagina + 1)}
              >
                Siguiente
                <ArrowRight size={14} />
              </button>
            </div>
          </footer>
        </section>
      </div>
      {!!anularId && (
        <div
          className="gloria-theme inv-overlay"
          style={{
            position: 'fixed',
            inset: 0,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            zIndex: 1300,
          }}
        >
          <div
            className="inv-dialog inv-dialog-compact"
            role="dialog"
            aria-modal="true"
            aria-labelledby="inv-annul-title"
          >
            <header className="inv-dialog-header">
              <div>
                <h3 id="inv-annul-title">Anular Ingreso / Compra</h3>
                <p>Indica por qué debe revertirse esta entrada.</p>
              </div>
            </header>
            <form onSubmit={handleAnular} className="inv-form">
              <label className="inv-field">
                Motivo de anulación (mínimo 5 caracteres) *
                <input
                  required
                  minLength={5}
                  maxLength={250}
                  disabled={anulando || incierta}
                  placeholder="Ej. Error en la factura del proveedor o lote devuelto"
                  value={motivoAnulacion}
                  onChange={(e) => setMotivoAnulacion(e.target.value)}
                />
              </label>
              {errorMsg && (
                <p role="alert" className="inv-form-error">
                  {errorMsg}
                </p>
              )}
              {incierta && (
                <p className="inv-help">
                  Se conserva la solicitud original para comprobar su resultado.
                </p>
              )}
              <footer className="inv-form-footer">
                <button
                  type="button"
                  className="inv-button"
                  disabled={anulando}
                  onClick={() => setAnularId(null)}
                >
                  Cancelar
                </button>
                <button
                  type="submit"
                  className="inv-button inv-danger"
                  disabled={anulando || motivoAnulacion.trim().length < 5}
                >
                  {anulando
                    ? 'Anulando...'
                    : incierta
                      ? 'Reintentar anulación pendiente'
                      : 'Confirmar Anulación'}
                </button>
              </footer>
            </form>
          </div>
        </div>
      )}
    </div>
  );
};
