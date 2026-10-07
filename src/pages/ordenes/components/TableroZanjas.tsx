import React from 'react';
import { OrdenTrabajoHistorialDTO } from '../../../types/ordenTrabajo';
import { styles } from '../ordenesStyles';

interface TableroZanjasProps {
  ordenes: OrdenTrabajoHistorialDTO[];
  onVerDetalle: (ot: OrdenTrabajoHistorialDTO) => void;
  onAbrirModalCrear: (zanja: number) => void;
  puedeCrear: boolean;
}

export const TableroZanjas: React.FC<TableroZanjasProps> = ({
  ordenes,
  onVerDetalle,
  onAbrirModalCrear,
  puedeCrear,
}) => {
  const getOtActivaZanja = (numZanja: number) => {
    return ordenes.find(
      (ot) => ot.zanja === numZanja && (ot.estado === 'EN_ESPERA' || ot.estado === 'EN_PROCESO')
    );
  };

  const renderBadgeEstado = (estado: string) => {
    let style = { ...styles.badgeEstado };
    if (estado === 'EN_ESPERA') {
      style = { ...style, backgroundColor: '#334155', color: '#94A3B8' };
    } else if (estado === 'EN_PROCESO') {
      style = { ...style, backgroundColor: '#1E3A8A', color: '#60A5FA' };
    } else if (estado === 'FINALIZADO') {
      style = { ...style, backgroundColor: '#064E3B', color: '#34D399' };
    } else {
      style = { ...style, backgroundColor: '#7F1D1D', color: '#FCA5A5' };
    }

    return <span style={style}>{estado.replace('_', ' ')}</span>;
  };

  return (
    <div style={styles.tableroGrid}>
      {[1, 2].map((numZanja) => {
        const otActiva = getOtActivaZanja(numZanja);

        return (
          <div key={numZanja} style={styles.columnaZanja}>
            <div style={styles.tituloZanja}>
              <span>Zanja {numZanja}</span>
              {otActiva && renderBadgeEstado(otActiva.estado)}
            </div>

            {otActiva ? (
              <div style={styles.tarjetaOt}>
                <div
                  style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '8px' }}
                >
                  <strong style={{ fontSize: '18px', color: '#F8FAFC' }}>{otActiva.placa}</strong>
                  <span style={{ color: '#94A3B8', fontSize: '14px' }}>{otActiva.codigo_ot}</span>
                </div>

                <p style={{ margin: '4px 0', fontSize: '14px', color: '#CBD5E1' }}>
                  <strong>Mecánico:</strong> {otActiva.nombre_mecanico || 'No asignado'}
                </p>
                <p style={{ margin: '4px 0', fontSize: '14px', color: '#CBD5E1' }}>
                  <strong>Km Ingreso:</strong> {otActiva.kilometraje_ingreso.toLocaleString()} km
                </p>

                {otActiva.observaciones && (
                  <p
                    style={{
                      margin: '8px 0 0 0',
                      fontSize: '13px',
                      color: '#94A3B8',
                      fontStyle: 'italic',
                    }}
                  >
                    "{otActiva.observaciones}"
                  </p>
                )}

                <button
                  onClick={() => onVerDetalle(otActiva)}
                  style={{
                    marginTop: '16px',
                    width: '100%',
                    padding: '8px',
                    backgroundColor: '#334155',
                    color: '#F8FAFC',
                    border: 'none',
                    borderRadius: '6px',
                    cursor: 'pointer',
                    fontWeight: '600',
                  }}
                >
                  Ver Detalle / Gestionar
                </button>
              </div>
            ) : (
              <div style={styles.tarjetaVacia}>
                <p style={{ margin: 0, fontWeight: '500' }}>Zanja Disponible</p>
                {puedeCrear && (
                  <button
                    onClick={() => onAbrirModalCrear(numZanja)}
                    style={{ ...styles.btnNuevaOT, marginTop: '12px' }}
                  >
                    + Asignar OT a Zanja {numZanja}
                  </button>
                )}
              </div>
            )}
          </div>
        );
      })}
    </div>
  );
};
