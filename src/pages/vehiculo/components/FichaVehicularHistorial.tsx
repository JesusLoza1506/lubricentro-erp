import React from 'react';
import { HistorialOTPublicoDTO } from '../../../types/vehiculo';
import {
  Clock,
  Eye,
  Droplet,
  FilePenLine,
  Wrench,
  ChevronLeft,
  ChevronRight,
  User,
} from 'lucide-react';

interface Props {
  historial: HistorialOTPublicoDTO[] | any[];
  totalRegistros: number;
  paginaActual: number;
  totalPaginas: number;
  cargando?: boolean;
  puedeCorregirOT?: boolean;
  onVerDetalleOT: (ot: HistorialOTPublicoDTO) => void;
  onCorregirOT?: (ot: HistorialOTPublicoDTO) => void;
  onCambiarPagina: (nuevaPagina: number) => void;
}

const ELEMENTOS_POR_PAGINA = 20;

export const FichaVehicularHistorial: React.FC<Props> = ({
  historial,
  totalRegistros,
  paginaActual,
  totalPaginas,
  cargando = false,
  puedeCorregirOT = false,
  onVerDetalleOT,
  onCorregirOT,
  onCambiarPagina,
}) => {
  const inicioIndice = (paginaActual - 1) * ELEMENTOS_POR_PAGINA;

  const handlePaginaAnterior = () => {
    if (paginaActual > 1 && !cargando) {
      onCambiarPagina(paginaActual - 1);
    }
  };

  const handlePaginaSiguiente = () => {
    if (paginaActual < totalPaginas && !cargando) {
      onCambiarPagina(paginaActual + 1);
    }
  };

  const formatearFechaHora = (fechaStr?: string) => {
    if (!fechaStr) return 'N/A';
    const f = new Date(fechaStr);
    if (isNaN(f.getTime())) return 'N/A';
    return `${f.toLocaleDateString('es-PE')} ${f.toLocaleTimeString('es-PE', {
      hour: '2-digit',
      minute: '2-digit',
    })}`;
  };

  return (
    <section className="fv-history">
      <div className="fv-section-title">
        <h3>
          Historial de mantenimientos <span>({totalRegistros})</span>
        </h3>
        <span className="fv-page-size">20 registros por página</span>
      </div>
      {historial.length === 0 ? (
        <div className="fv-list-empty">Este vehículo no tiene mantenimientos registrados.</div>
      ) : (
        <>
          <div className="fv-pagination">
            <span>
              Mostrando {totalRegistros === 0 ? 0 : inicioIndice + 1}–
              {Math.min(inicioIndice + historial.length, totalRegistros)} de {totalRegistros}
            </span>
            {totalPaginas > 1 && (
              <div className="fv-pagination-controls">
                <button
                  onClick={handlePaginaAnterior}
                  disabled={paginaActual === 1 || cargando}
                  className="fv-button fv-small"
                >
                  <ChevronLeft size={16} />
                  Anterior
                </button>
                <span>
                  Página <strong>{paginaActual}</strong> de <strong>{totalPaginas}</strong>
                </span>
                <button
                  onClick={handlePaginaSiguiente}
                  disabled={paginaActual === totalPaginas || cargando}
                  className="fv-button fv-small"
                >
                  Siguiente
                  <ChevronRight size={16} />
                </button>
              </div>
            )}
          </div>
          <div className="fv-table-scroll" style={{ opacity: cargando ? 0.6 : 1 }}>
            <table className="fv-history-table">
              <thead>
                <tr>
                  <th scope="col">Fecha Fin / Orden</th>
                  <th scope="col">Resumen Público / Aceite</th>
                  <th scope="col">Cliente Atendido</th>
                  <th scope="col">Kilometraje</th>
                  <th scope="col">Estado</th>
                  <th scope="col">Acciones</th>
                </tr>
              </thead>
              <tbody>
                {historial.map((ot) => {
                  const fechaAtencion = ot.fecha_fin || ot.fecha_servicio || ot.fecha_ingreso;
                  const resumenTexto =
                    ot.resumen_publico || ot.observaciones || 'Sin resumen registrado.';
                  const kmSalida = ot.kilometraje_salida ?? ot.kilometraje_ingreso ?? 0;
                  const kmProx = ot.km_proximo ?? ot.proximo_kilometraje;
                  const nombreClienteAtendido =
                    ot.nombre_cliente || ot.cliente_nombre || 'Cliente Varios / No especificado';
                  const docClienteAtendido = ot.documento_cliente || ot.cliente_documento;

                  return (
                    <tr key={ot.id_ot}>
                      <td>
                        <strong className="fv-order-code">{ot.codigo_ot}</strong>
                        <span className="fv-date">
                          <Clock size={13} />
                          {formatearFechaHora(fechaAtencion)}
                        </span>
                      </td>
                      <td>
                        <p className="fv-observation" title={resumenTexto}>
                          {resumenTexto}
                        </p>
                        <div className="fv-inline-badges">
                          {ot.tipo_aceite && (
                            <span className="fv-oil">
                              <Droplet size={12} />
                              {ot.tipo_aceite}
                            </span>
                          )}
                          {ot.corregido === 1 && (
                            <span
                              className="fv-corrected"
                              style={{ color: '#F59E0B', fontWeight: '700' }}
                            >
                              <FilePenLine size={12} />
                              CORREGIDO
                            </span>
                          )}
                        </div>
                      </td>
                      <td>
                        <div
                          style={{
                            display: 'flex',
                            flexDirection: 'column',
                            gap: '2px',
                            fontSize: '12px',
                          }}
                        >
                          <span
                            style={{
                              fontWeight: '600',
                              color: '#F8FAFC',
                              display: 'flex',
                              alignItems: 'center',
                              gap: '4px',
                            }}
                          >
                            <User size={13} color="#94A3B8" /> {nombreClienteAtendido}
                          </span>
                          {docClienteAtendido && (
                            <span
                              style={{ color: '#94A3B8', fontSize: '11px', paddingLeft: '17px' }}
                            >
                              Doc: {docClienteAtendido}
                            </span>
                          )}
                        </div>
                      </td>
                      <td className="fv-km-cell">
                        <span>
                          Salida: <strong>{kmSalida.toLocaleString()} km</strong>
                        </span>
                        <span>
                          Próximo servicio:{' '}
                          {kmProx !== null && kmProx !== undefined
                            ? `${kmProx.toLocaleString()} km`
                            : 'Sin cambio'}
                        </span>
                      </td>
                      <td>
                        <span
                          className="fv-status"
                          style={{
                            backgroundColor:
                              ot.estado === 'FINALIZADO' || !ot.estado
                                ? 'rgba(52, 211, 153, 0.2)'
                                : ot.estado === 'EN_PROCESO'
                                  ? 'rgba(251, 191, 36, 0.2)'
                                  : 'rgba(148, 163, 184, 0.2)',
                            color:
                              ot.estado === 'FINALIZADO' || !ot.estado
                                ? '#34D399'
                                : ot.estado === 'EN_PROCESO'
                                  ? '#FBBF24'
                                  : '#94A3B8',
                          }}
                        >
                          {ot.estado || 'FINALIZADO'}
                        </span>
                      </td>
                      <td>
                        <div className="fv-row-actions">
                          <button
                            onClick={() => onVerDetalleOT(ot)}
                            title="Ver Insumos / Detalle Público OT"
                            className="fv-button fv-small"
                          >
                            <Eye size={14} />
                            Detalle
                          </button>
                          {puedeCorregirOT &&
                            (ot.estado === 'FINALIZADO' || !ot.estado) &&
                            onCorregirOT && (
                              <button
                                onClick={() => onCorregirOT(ot)}
                                title="Corregir Historial de OT (Administrador)"
                                className="fv-button fv-small"
                              >
                                <Wrench size={14} />
                                Corregir
                              </button>
                            )}
                        </div>
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
          <div className="fv-pagination">
            <span>
              Mostrando {totalRegistros === 0 ? 0 : inicioIndice + 1}–
              {Math.min(inicioIndice + historial.length, totalRegistros)} de {totalRegistros}
            </span>
            {totalPaginas > 1 && (
              <div className="fv-pagination-controls">
                <button
                  onClick={handlePaginaAnterior}
                  disabled={paginaActual === 1 || cargando}
                  className="fv-button fv-small"
                >
                  <ChevronLeft size={16} />
                  Anterior
                </button>
                <span>
                  Página <strong>{paginaActual}</strong> de <strong>{totalPaginas}</strong>
                </span>
                <button
                  onClick={handlePaginaSiguiente}
                  disabled={paginaActual === totalPaginas || cargando}
                  className="fv-button fv-small"
                >
                  Siguiente
                  <ChevronRight size={16} />
                </button>
              </div>
            )}
          </div>
        </>
      )}
    </section>
  );
};
