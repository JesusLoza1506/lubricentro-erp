import React from 'react';
import { OrdenTrabajoHistorialDTO } from '../../../types/vehiculo';
import { Clock, Eye, Droplet } from 'lucide-react';

interface Props {
  historial: OrdenTrabajoHistorialDTO[];
  onVerDetalleOT: (ot: OrdenTrabajoHistorialDTO) => void;
}

export const FichaVehicularHistorial: React.FC<Props> = ({ historial, onVerDetalleOT }) => {
  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
      <h3 style={{ fontSize: '18px', fontWeight: '700', color: '#F8FAFC', margin: 0 }}>
        Historial de Mantenimientos ({historial.length})
      </h3>

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
        <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
          {historial.map((ot) => (
            <div
              key={ot.id_ot}
              style={{
                padding: '20px',
                backgroundColor: 'rgba(15, 23, 42, 0.6)',
                border: '1px solid #334155',
                borderRadius: '14px',
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
              }}
            >
              <div style={{ display: 'flex', flexDirection: 'column', gap: '6px' }}>
                <div
                  style={{ display: 'flex', alignItems: 'center', gap: '10px', flexWrap: 'wrap' }}
                >
                  <span style={{ fontSize: '16px', fontWeight: '800', color: '#38BDF8' }}>
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

                  {/* Insignia visual para Tipo de Aceite */}
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
                </div>

                <p style={{ fontSize: '13px', color: '#E2E8F0', margin: 0 }}>
                  {ot.observaciones || 'Sin observaciones registradas.'}
                </p>

                <div
                  style={{
                    display: 'flex',
                    gap: '16px',
                    fontSize: '12px',
                    color: '#94A3B8',
                    marginTop: '4px',
                  }}
                >
                  <span>Ingreso: {(ot.kilometraje_ingreso ?? 0).toLocaleString()} km</span>
                  <span>•</span>
                  <span>Próximo servicio: {(ot.proximo_kilometraje ?? 0).toLocaleString()} km</span>
                </div>
              </div>

              <div style={{ display: 'flex', alignItems: 'center', gap: '16px' }}>
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
                    padding: '6px 12px',
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
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};
