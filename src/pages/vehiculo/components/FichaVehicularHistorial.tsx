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
    <div style={{ display: 'flex', flexDirection: 'column', gap: '14px', marginTop: '8px' }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <h3 style={{ fontSize: '18px', fontWeight: '700', color: '#F8FAFC', margin: 0 }}>
          Historial de Mantenimientos ({historial.length})
        </h3>
        {totalPaginas > 1 && (
          <span style={{ fontSize: '12px', color: '#94A3B8', fontWeight: '600' }}>
            Página {paginaActual} de {totalPaginas}
          </span>
        )}
      </div>

      {historial.length === 0 ? (
        <div
          style={{
            padding: '32px',
            textAlign: 'center',
            backgroundColor: 'rgba(15, 23, 42, 0.4)',
            border: '1px dashed #334155',
            borderRadius: '16px',
            color: '#94A3B8',
            fontSize: '14px',
          }}
        >
          Este vehículo no tiene mantenimientos registrados.
        </div>
      ) : (
        <>
          <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
            {historialPaginado.map((ot) => (
              <div
                key={ot.id_ot}
                style={{
                  padding: '18px 22px',
                  backgroundColor: 'rgba(15, 23, 42, 0.6)',
                  border: '1px solid #334155',
                  borderRadius: '14px',
                  display: 'flex',
                  justifyContent: 'space-between',
                  alignItems: 'center',
                  flexWrap: 'wrap',
                  gap: '16px',
                }}
              >
                <div
                  style={{
                    display: 'flex',
                    flexDirection: 'column',
                    gap: '6px',
                    flex: '1',
                    minWidth: '260px',
                  }}
                >
                  <div
                    style={{ display: 'flex', alignItems: 'center', gap: '10px', flexWrap: 'wrap' }}
                  >
                    <span style={{ fontSize: '15px', fontWeight: '800', color: '#38BDF8' }}>
                      {ot.codigo_ot}
                    </span>
                    <span
                      style={{
                        fontSize: '11px',
                        fontWeight: '700',
                        padding: '2px 8px',
                        borderRadius: '8px',
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
                      }}
                    >
                      {ot.estado}
                    </span>

                    {ot.tipo_aceite && (
                      <span
                        style={{
                          display: 'inline-flex',
                          alignItems: 'center',
                          gap: '4px',
                          fontSize: '11px',
                          fontWeight: '600',
                          padding: '2px 8px',
                          borderRadius: '8px',
                          backgroundColor: 'rgba(56, 189, 248, 0.15)',
                          color: '#38BDF8',
                          border: '1px solid rgba(56, 189, 248, 0.3)',
                        }}
                      >
                        <Droplet size={12} />
                        {ot.tipo_aceite}
                      </span>
                    )}

                    {ot.corregido === 1 && (
                      <span
                        style={{
                          display: 'inline-flex',
                          alignItems: 'center',
                          gap: '4px',
                          fontSize: '11px',
                          fontWeight: '700',
                          padding: '2px 8px',
                          borderRadius: '8px',
                          backgroundColor: 'rgba(239, 68, 68, 0.15)',
                          color: '#FCA5A5',
                          border: '1px solid rgba(239, 68, 68, 0.3)',
                        }}
                      >
                        <FilePenLine size={12} />
                        CORREGIDO
                      </span>
                    )}
                  </div>

                  <p style={{ fontSize: '13px', color: '#E2E8F0', margin: 0 }}>
                    {ot.observaciones || 'Sin observaciones registradas.'}
                  </p>

                  <div
                    style={{
                      display: 'flex',
                      gap: '12px',
                      fontSize: '12px',
                      color: '#94A3B8',
                      marginTop: '2px',
                    }}
                  >
                    <span>Ingreso: {(ot.kilometraje_ingreso ?? 0).toLocaleString()} km</span>
                    <span>•</span>
                    <span>
                      Próximo servicio:{' '}
                      {ot.proximo_kilometraje !== null && ot.proximo_kilometraje !== undefined
                        ? `${ot.proximo_kilometraje.toLocaleString()} km`
                        : 'Sin cambio'}
                    </span>
                  </div>
                </div>

                <div
                  style={{ display: 'flex', alignItems: 'center', gap: '12px', flexWrap: 'wrap' }}
                >
                  <div
                    style={{
                      display: 'flex',
                      alignItems: 'center',
                      gap: '6px',
                      color: '#94A3B8',
                      fontSize: '12px',
                    }}
                  >
                    <Clock size={14} />
                    {ot.fecha_ingreso
                      ? new Date(ot.fecha_ingreso).toLocaleDateString('es-PE')
                      : 'N/A'}
                  </div>

                  <button
                    onClick={() => onVerDetalleOT(ot)}
                    title="Ver Insumos / Detalle OT"
                    style={{
                      display: 'flex',
                      alignItems: 'center',
                      gap: '6px',
                      padding: '7px 12px',
                      backgroundColor: 'rgba(30, 41, 59, 0.8)',
                      border: '1px solid #334155',
                      borderRadius: '8px',
                      color: '#38BDF8',
                      fontSize: '12px',
                      fontWeight: '600',
                      cursor: 'pointer',
                    }}
                  >
                    <Eye size={14} /> Detalle
                  </button>

                  {puedeCorregirOT && ot.estado === 'FINALIZADO' && onCorregirOT && (
                    <button
                      onClick={() => onCorregirOT(ot)}
                      title="Corregir Historial de OT (Administrador)"
                      style={{
                        display: 'flex',
                        alignItems: 'center',
                        gap: '6px',
                        padding: '7px 12px',
                        backgroundColor: 'rgba(239, 68, 68, 0.15)',
                        border: '1px solid rgba(239, 68, 68, 0.4)',
                        borderRadius: '8px',
                        color: '#FCA5A5',
                        fontSize: '12px',
                        fontWeight: '600',
                        cursor: 'pointer',
                      }}
                    >
                      <Wrench size={14} /> Corregir
                    </button>
                  )}
                </div>
              </div>
            ))}
          </div>

          {/* BARRA DE PAGINACIÓN */}
          {totalPaginas > 1 && (
            <div
              style={{
                display: 'flex',
                justifyContent: 'center',
                alignItems: 'center',
                gap: '16px',
                marginTop: '12px',
              }}
            >
              <button
                onClick={handlePaginaAnterior}
                disabled={paginaActual === 1}
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '4px',
                  padding: '6px 14px',
                  backgroundColor: 'rgba(30, 41, 59, 0.8)',
                  border: '1px solid #334155',
                  borderRadius: '8px',
                  color: paginaActual === 1 ? '#64748B' : '#F8FAFC',
                  fontSize: '12px',
                  fontWeight: '600',
                  cursor: paginaActual === 1 ? 'not-allowed' : 'pointer',
                }}
              >
                <ChevronLeft size={16} /> Anterior
              </button>
              <span style={{ fontSize: '13px', color: '#94A3B8' }}>
                Página <strong style={{ color: '#F8FAFC' }}>{paginaActual}</strong> de{' '}
                <strong style={{ color: '#F8FAFC' }}>{totalPaginas}</strong>
              </span>
              <button
                onClick={handlePaginaSiguiente}
                disabled={paginaActual === totalPaginas}
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '4px',
                  padding: '6px 14px',
                  backgroundColor: 'rgba(30, 41, 59, 0.8)',
                  border: '1px solid #334155',
                  borderRadius: '8px',
                  color: paginaActual === totalPaginas ? '#64748B' : '#F8FAFC',
                  fontSize: '12px',
                  fontWeight: '600',
                  cursor: paginaActual === totalPaginas ? 'not-allowed' : 'pointer',
                }}
              >
                Siguiente <ChevronRight size={16} />
              </button>
            </div>
          )}
        </>
      )}
    </div>
  );
};
