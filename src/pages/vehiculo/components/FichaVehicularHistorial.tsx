import React, { useState, useEffect } from 'react';
import { OrdenTrabajoHistorialDTO } from '../../../types/vehiculo';
import { Clock, Eye, Droplet, FilePenLine, Wrench, ChevronLeft, ChevronRight } from 'lucide-react';

interface Props {
  historial: OrdenTrabajoHistorialDTO[];
  puedeCorregirOT?: boolean;
  onVerDetalleOT: (ot: OrdenTrabajoHistorialDTO) => void;
  onCorregirOT?: (ot: OrdenTrabajoHistorialDTO) => void;
}

const ELEMENTOS_POR_PAGINA = 20;

export const FichaVehicularHistorial: React.FC<Props> = ({
  historial,
  puedeCorregirOT = false,
  onVerDetalleOT,
  onCorregirOT,
}) => {
  const [paginaActual, setPaginaActual] = useState(1);

  // CORRECCIÓN PUNTO 5: Reiniciar la página a 1 cuando cambia el vehículo o la lista de historial
  useEffect(() => {
    setPaginaActual(1);
  }, [historial]);

  const totalPaginas = Math.ceil(historial.length / ELEMENTOS_POR_PAGINA) || 1;
  const inicioIndice = (paginaActual - 1) * ELEMENTOS_POR_PAGINA;
  const historialPaginado = historial.slice(inicioIndice, inicioIndice + ELEMENTOS_POR_PAGINA);

  const handlePaginaAnterior = () => {
    if (paginaActual > 1) {
      setPaginaActual((prev) => prev - 1);
    }
  };

  const handlePaginaSiguiente = () => {
    if (paginaActual < totalPaginas) {
      setPaginaActual((prev) => prev + 1);
    }
  };

  return (
    <section className="fv-history">
      <div className="fv-section-title"><h3>Historial de mantenimientos <span>({historial.length})</span></h3><span className="fv-page-size">20 registros por página</span></div>
      {historial.length === 0 ? (
        <div className="fv-list-empty">Este vehículo no tiene mantenimientos registrados.</div>
      ) : (
        <>
          <div className="fv-pagination">
              <span>Mostrando {inicioIndice + 1}–{Math.min(inicioIndice + ELEMENTOS_POR_PAGINA, historial.length)} de {historial.length}</span>
              {totalPaginas > 1 && (
                <div className="fv-pagination-controls">
                  <button onClick={handlePaginaAnterior} disabled={paginaActual === 1} className="fv-button fv-small"><ChevronLeft size={16} />Anterior</button>
                  <span>Página <strong>{paginaActual}</strong> de <strong>{totalPaginas}</strong></span>
                  <button onClick={handlePaginaSiguiente} disabled={paginaActual === totalPaginas} className="fv-button fv-small">Siguiente<ChevronRight size={16} /></button>
                </div>
              )}
            </div>
          <div className="fv-table-scroll">
            <table className="fv-history-table">
              <thead><tr><th scope="col">Fecha / Orden</th><th scope="col">Observaciones / Aceite</th><th scope="col">Kilometraje</th><th scope="col">Estado</th><th scope="col">Acciones</th></tr></thead>
              <tbody>
                {historialPaginado.map((ot) => (
                  <tr key={ot.id_ot}>
                    <td><strong className="fv-order-code">{ot.codigo_ot}</strong><span className="fv-date"><Clock size={13} />{ot.fecha_ingreso ? new Date(ot.fecha_ingreso).toLocaleDateString('es-PE') : 'N/A'}</span></td>
                    <td><p className="fv-observation" title={ot.observaciones || 'Sin observaciones registradas.'}>{ot.observaciones || 'Sin observaciones registradas.'}</p>
                      <div className="fv-inline-badges">
                        {ot.tipo_aceite && (<span className="fv-oil"><Droplet size={12} />{ot.tipo_aceite}</span>)}
                        {ot.corregido === 1 && (<span className="fv-corrected"><FilePenLine size={12} />CORREGIDO</span>)}
                      </div>
                    </td>
                    <td className="fv-km-cell"><span>Ingreso: <strong>{(ot.kilometraje_ingreso ?? 0).toLocaleString()} km</strong></span><span>Próximo servicio: {ot.proximo_kilometraje !== null && ot.proximo_kilometraje !== undefined ? `${ot.proximo_kilometraje.toLocaleString()} km` : 'Sin cambio'}</span></td>
                    <td><span className="fv-status" style={{
                        backgroundColor:
                          ot.estado === 'FINALIZADO'
                            ? 'rgba(52, 211, 153, 0.2)'
                            : ot.estado === 'EN_PROCESO'
                              ? 'rgba(251, 191, 36, 0.2)'
                              : 'rgba(148, 163, 184, 0.2)',
                        color:
                          ot.estado === 'FINALIZADO'
                            ? '#34D399'
                            : ot.estado === 'EN_PROCESO'
                              ? '#FBBF24'
                              : '#94A3B8',
                    }}>{ot.estado}</span></td>
                    <td><div className="fv-row-actions">
                      <button onClick={() => onVerDetalleOT(ot)} title="Ver Insumos / Detalle OT" className="fv-button fv-small"><Eye size={14} />Detalle</button>
                      {puedeCorregirOT && ot.estado === 'FINALIZADO' && onCorregirOT && (
                        <button onClick={() => onCorregirOT(ot)} title="Corregir Historial de OT (Administrador)" className="fv-button fv-small"><Wrench size={14} />Corregir</button>
                      )}
                    </div></td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
          <div className="fv-pagination">
              <span>Mostrando {inicioIndice + 1}–{Math.min(inicioIndice + ELEMENTOS_POR_PAGINA, historial.length)} de {historial.length}</span>
              {totalPaginas > 1 && (
                <div className="fv-pagination-controls">
                  <button onClick={handlePaginaAnterior} disabled={paginaActual === 1} className="fv-button fv-small"><ChevronLeft size={16} />Anterior</button>
                  <span>Página <strong>{paginaActual}</strong> de <strong>{totalPaginas}</strong></span>
                  <button onClick={handlePaginaSiguiente} disabled={paginaActual === totalPaginas} className="fv-button fv-small">Siguiente<ChevronRight size={16} /></button>
                </div>
              )}
            </div>
        </>
      )}
    </section>
  );
};
