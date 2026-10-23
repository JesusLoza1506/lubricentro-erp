import React from 'react';
import { HistorialOTPublicoDTO } from '../../../types/vehiculo';
import {
  Clock,
  Eye,
  Droplet,
  FilePenLine,
  Pencil,
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
          <div className="fv-table-scroll" aria-busy={cargando} tabIndex={0} role="region" aria-label="Historial de mantenimientos" style={{ opacity: cargando ? 0.6 : 1 }}>
            <table className="fv-history-table">
              <thead>
                <tr>
                  <th scope="col">Atención</th>
                  <th scope="col">Trabajo realizado</th>
                  <th scope="col">Cliente atendido</th>
                  <th scope="col">Kilometraje</th>
                  <th scope="col">Consultar / editar</th>
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
                        <span className="fv-state-caption" data-state={ot.estado || 'FINALIZADO'}>{ot.estado || 'FINALIZADO'}</span>
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
                            >
                              <FilePenLine size={12} />
                              Corregido
                            </span>
                          )}
                        </div>
                      </td>
                      <td>
                        <div className="fv-history-client">
                          <span>
                            <User size={13} color="#94A3B8" /> {nombreClienteAtendido}
                          </span>
                          {docClienteAtendido && (
                            <span className="fv-client-document">
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
                          Próximo cambio:{' '}
                          {kmProx !== null && kmProx !== undefined
                            ? `${kmProx.toLocaleString()} km`
                            : 'No registrado'}
                        </span>
                      </td>
                      <td>
                        <div className="fv-row-actions">
                          <button
                            onClick={() => onVerDetalleOT(ot)}
                            title={`Ver atención ${ot.codigo_ot}`}
                            className="fv-button fv-small fv-view-attention"
                          >
                            <Eye size={14} />
                            Ver atención
                          </button>
                          {puedeCorregirOT &&
                            (ot.estado === 'FINALIZADO' || !ot.estado) &&
                            onCorregirOT && (
                              <button
                                onClick={() => onCorregirOT(ot)}
                                title="Corregir datos de esta atención (Administrador)"
                                aria-label={`Corregir datos de ${ot.codigo_ot}`}
                                className="fv-button fv-small fv-edit-attention"
                              >
                                <Pencil size={14} />
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
