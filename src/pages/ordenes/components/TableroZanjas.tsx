import React from 'react';
import { TableroZanjasDTO, OrdenTrabajoHistorialDTO } from '../../../types/ordenTrabajo';
import { styles } from '../ordenesStyles';

interface TableroZanjasProps {
  tablero: TableroZanjasDTO;
  onVerDetalle: (ot: OrdenTrabajoHistorialDTO) => void;
  onIniciarEnZanja: (idOt: number, idZanja: number) => void;
}

type ZanjaItemDTO = TableroZanjasDTO['zanjas'][number];

export const TableroZanjas: React.FC<TableroZanjasProps> = ({
  tablero,
  onVerDetalle,
  onIniciarEnZanja,
}) => {
  // Filtrar zanjas únicamente activadas en base de datos (Fase 3 Punto 2)
  const zanjasHabilitadas = tablero.zanjas.filter((z: ZanjaItemDTO) => z.activa === 1);

  return (
    <div>
      {/* COLA COMÚN EN ESPERA */}
      <div
        style={{
          marginBottom: '24px',
          backgroundColor: '#0F172A',
          padding: '16px',
          borderRadius: '8px',
          border: '1px solid #334155',
        }}
      >
        <h3
          style={{
            margin: '0 0 12px 0',
            fontSize: '16px',
            color: '#F8FAFC',
            display: 'flex',
            alignItems: 'center',
            gap: '8px',
          }}
        >
          <span>⏳</span> Cola de Espera ({tablero.cola_espera.length})
        </h3>
        {tablero.cola_espera.length === 0 ? (
          <p style={{ margin: 0, color: '#64748B', fontSize: '13px', fontStyle: 'italic' }}>
            No hay vehículos esperando atención en cola.
          </p>
        ) : (
          <div
            style={{
              display: 'grid',
              gridTemplateColumns: 'repeat(auto-fill, minmax(280px, 1fr))',
              gap: '12px',
            }}
          >
            {tablero.cola_espera.map((ot: OrdenTrabajoHistorialDTO) => (
              <div
                key={ot.id_ot}
                style={{
                  backgroundColor: '#1E293B',
                  padding: '12px',
                  borderRadius: '6px',
                  border: '1px solid #334155',
                }}
              >
                <div
                  style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '6px' }}
                >
                  <strong style={{ fontSize: '16px', color: '#F8FAFC' }}>{ot.placa}</strong>
                  <span style={{ color: '#94A3B8', fontSize: '12px' }}>{ot.codigo_ot}</span>
                </div>
                <p style={{ margin: '2px 0', fontSize: '12px', color: '#CBD5E1' }}>
                  <strong>Cliente:</strong> {ot.cliente_nombre || 'No especificado'}
                </p>
                <p style={{ margin: '2px 0', fontSize: '12px', color: '#CBD5E1' }}>
                  <strong>Técnico:</strong> {ot.nombre_mecanico || 'Pendiente (Autoasignable)'}
                </p>
                <p style={{ margin: '2px 0', fontSize: '12px', color: '#CBD5E1' }}>
                  <strong>Km Ingreso:</strong> {ot.kilometraje_ingreso.toLocaleString()} km
                </p>
                <button
                  onClick={() => onVerDetalle(ot)}
                  style={{
                    marginTop: '10px',
                    width: '100%',
                    padding: '6px',
                    backgroundColor: '#2563EB',
                    color: '#FFF',
                    border: 'none',
                    borderRadius: '4px',
                    cursor: 'pointer',
                    fontWeight: 600,
                    fontSize: '12px',
                  }}
                >
                  Atender / Gestionar OT
                </button>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* TABLERO DE ZANJAS HABILITADAS */}
      <h3 style={{ margin: '0 0 12px 0', fontSize: '16px', color: '#F8FAFC' }}>
        Zanjas de Trabajo ({zanjasHabilitadas.length} Habilitadas)
      </h3>
      <div
        style={{
          display: 'grid',
          gridTemplateColumns: 'repeat(auto-fit, minmax(300px, 1fr))',
          gap: '16px',
        }}
      >
        {zanjasHabilitadas.map((z: ZanjaItemDTO) => {
          const ot = z.ot_actual;
          const estaOcupada = z.ocupada || Boolean(ot);

          return (
            <div key={z.id_zanja} style={styles.columnaZanja}>
              <div style={styles.tituloZanja}>
                <span>Zanja {z.id_zanja}</span>
                <span
                  style={{
                    padding: '2px 8px',
                    borderRadius: '4px',
                    fontSize: '11px',
                    fontWeight: 700,
                    backgroundColor: estaOcupada ? '#1E3A8A' : '#064E3B',
                    color: estaOcupada ? '#60A5FA' : '#34D399',
                  }}
                >
                  {estaOcupada ? 'OCUPADA' : 'LIBRE'}
                </span>
              </div>

              {estaOcupada && ot ? (
                <div style={styles.tarjetaOt}>
                  <div
                    style={{
                      display: 'flex',
                      justifyContent: 'space-between',
                      marginBottom: '8px',
                    }}
                  >
                    <strong style={{ fontSize: '18px', color: '#F8FAFC' }}>{ot.placa}</strong>
                    <span style={{ color: '#94A3B8', fontSize: '13px' }}>{ot.codigo_ot}</span>
                  </div>
                  <p style={{ margin: '4px 0', fontSize: '13px', color: '#CBD5E1' }}>
                    <strong>Cliente:</strong> {ot.cliente_nombre || 'No especificado'}
                  </p>
                  <p style={{ margin: '4px 0', fontSize: '13px', color: '#CBD5E1' }}>
                    <strong>Mecánico:</strong> {ot.nombre_mecanico || 'En ejecución'}
                  </p>
                  <p style={{ margin: '4px 0', fontSize: '13px', color: '#CBD5E1' }}>
                    <strong>Ingreso:</strong> {ot.kilometraje_ingreso.toLocaleString()} km
                  </p>
                  <button
                    onClick={() => onVerDetalle(ot)}
                    style={{
                      marginTop: '12px',
                      width: '100%',
                      padding: '8px',
                      backgroundColor: '#334155',
                      color: '#F8FAFC',
                      border: 'none',
                      borderRadius: '6px',
                      cursor: 'pointer',
                      fontWeight: 600,
                    }}
                  >
                    Ver Detalle / Insumos
                  </button>
                </div>
              ) : (
                <div style={styles.tarjetaVacia}>
                  <p style={{ margin: 0, fontWeight: 500, color: '#94A3B8' }}>Zanja Disponible</p>
                  {tablero.cola_espera.length > 0 && (
                    <div style={{ marginTop: '10px' }}>
                      <p style={{ fontSize: '11px', color: '#64748B', marginBottom: '4px' }}>
                        Asignar desde cola:
                      </p>
                      <select
                        onChange={(e) => {
                          const val = Number(e.target.value);
                          if (val > 0) onIniciarEnZanja(val, z.id_zanja);
                        }}
                        defaultValue={0}
                        style={{
                          width: '100%',
                          padding: '6px',
                          backgroundColor: '#0F172A',
                          border: '1px solid #334155',
                          borderRadius: '4px',
                          color: '#38BDF8',
                          fontSize: '12px',
                        }}
                      >
                        <option value={0}>-- Seleccionar vehículo --</option>
                        {tablero.cola_espera.map((colaOt: OrdenTrabajoHistorialDTO) => (
                          <option key={colaOt.id_ot} value={colaOt.id_ot}>
                            {colaOt.placa} ({colaOt.codigo_ot})
                          </option>
                        ))}
                      </select>
                    </div>
                  )}
                </div>
              )}
            </div>
          );
        })}
      </div>
    </div>
  );
};
