import React, { useEffect, useState } from 'react';
import { useAuth } from '../../context/AuthContext';
import { ordenesService } from '../../services/ordenesService';
import {
  OrdenTrabajoHistorialDTO,
  CrearOrdenTrabajoPayload,
  EstadoOT,
} from '../../types/ordenTrabajo';
import { styles } from './ordenesStyles';
import { TableroZanjas } from './components/TableroZanjas';
import { ModalCrearOT } from './components/ModalCrearOT';
import { ModalDetalleOT } from './components/ModalDetalleOT';

interface OrdenesPageProps {
  onNavegarAFichaVehicular?: (placa?: string) => void;
}

export const OrdenesPage: React.FC<OrdenesPageProps> = ({ onNavegarAFichaVehicular }) => {
  const { usuario, tienePermiso } = useAuth();
  const [ordenes, setOrdenes] = useState<OrdenTrabajoHistorialDTO[]>([]);
  const [loading, setLoading] = useState(true);
  const [modalCrearAbierto, setModalCrearAbierto] = useState(false);
  const [zanjaSeleccionada, setZanjaSeleccionada] = useState<number | undefined>(undefined);
  const [otSeleccionada, setOtSeleccionada] = useState<OrdenTrabajoHistorialDTO | null>(null);

  const puedeCrear = tienePermiso('OT', 'crear');

  const obtenerIdUsuario = (): number => {
    if (!usuario) return 0;
    const uAny = usuario as any;
    return uAny.id_usuario || uAny.id || uAny.idUsuario || 0;
  };

  const cargarOrdenes = async () => {
    try {
      setLoading(true);
      const userId = obtenerIdUsuario();
      if (!userId) return;
      const data = await ordenesService.listarOrdenesTrabajo(userId);
      setOrdenes(data);

      if (otSeleccionada) {
        const actualizada = data.find((o) => o.id_ot === otSeleccionada.id_ot);
        if (actualizada) setOtSeleccionada(actualizada);
      }
    } catch (err) {
      console.error('Error al cargar órdenes de trabajo:', err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    cargarOrdenes();
  }, [usuario]);

  const zanjasOcupadas = ordenes
    .filter((ot) => (ot.estado === 'EN_ESPERA' || ot.estado === 'EN_PROCESO') && ot.zanja)
    .map((ot) => Number(ot.zanja));

  const handleAbrirModalCrear = (zanja?: number) => {
    setZanjaSeleccionada(zanja);
    setModalCrearAbierto(true);
  };

  const handleCrearOT = async (payload: CrearOrdenTrabajoPayload) => {
    const userId = obtenerIdUsuario();
    await ordenesService.crearOrdenTrabajo(payload, userId);
    await cargarOrdenes();
  };

  const handleCambiarEstado = async (idOt: number, nuevoEstado: EstadoOT, zanja?: number) => {
    const userId = obtenerIdUsuario();
    await ordenesService.cambiarEstadoOT({ id_ot: idOt, nuevo_estado: nuevoEstado, zanja }, userId);
    await cargarOrdenes();
  };

  const handleCancelarOT = async (idOt: number) => {
    const userId = obtenerIdUsuario();
    await ordenesService.cambiarEstadoOT({ id_ot: idOt, nuevo_estado: 'CANCELADO' }, userId);
    await cargarOrdenes();
  };

  const handleReasignarMecanico = async (idOt: number, nuevoIdMecanico: number) => {
    const userId = obtenerIdUsuario();
    await ordenesService.reasignarMecanicoOT(
      { id_ot: idOt, nuevo_id_mecanico: nuevoIdMecanico },
      userId
    );
    await cargarOrdenes();
  };

  // Filtrado del Historial del Día usando HORA LOCAL estricta (YYYY-MM-DD)
  const ahora = new Date();
  const anioLocal = ahora.getFullYear();
  const mesLocal = String(ahora.getMonth() + 1).padStart(2, '0');
  const diaLocal = String(ahora.getDate()).padStart(2, '0');
  const hoyFechaLocalStr = `${anioLocal}-${mesLocal}-${diaLocal}`;

  const historicoCompletados = ordenes.filter((ot) => {
    const esEstadoTerminal = ot.estado === 'FINALIZADO' || ot.estado === 'CANCELADO';
    if (!esEstadoTerminal) return false;

    if (!ot.fecha_ingreso) return true;
    const fechaOtStr = ot.fecha_ingreso.split('T')[0].split(' ')[0];
    return fechaOtStr === hoyFechaLocalStr;
  });

  return (
    <div style={styles.container}>
      <div style={styles.header}>
        <div>
          <h1 style={styles.title}>Órdenes de Trabajo / Taller</h1>
          <p style={styles.subtitle}>Gestión de servicios y ocupación de zanjas en tiempo real</p>
        </div>
        {puedeCrear && (
          <button style={styles.btnNuevaOT} onClick={() => handleAbrirModalCrear()}>
            + Nueva OT
          </button>
        )}
      </div>

      {loading ? (
        <div style={{ padding: '32px', textAlign: 'center', color: '#94A3B8' }}>
          Cargando tablero de taller...
        </div>
      ) : (
        <>
          <TableroZanjas
            ordenes={ordenes}
            onVerDetalle={(ot) => setOtSeleccionada(ot)}
            onAbrirModalCrear={handleAbrirModalCrear}
            puedeCrear={puedeCrear}
          />

          <div style={styles.historicoContainer}>
            <h3 style={{ margin: '0 0 16px 0', color: '#F8FAFC', fontSize: '16px' }}>
              Historial del Día (Finalizadas / Canceladas)
            </h3>
            {historicoCompletados.length === 0 ? (
              <p style={{ color: '#64748B', fontSize: '14px', margin: 0 }}>
                No hay órdenes finalizadas o canceladas registradas hoy.
              </p>
            ) : (
              <table style={styles.tabla}>
                <thead>
                  <tr>
                    <th style={styles.th}>Código</th>
                    <th style={styles.th}>Placa</th>
                    <th style={styles.th}>Mecánico</th>
                    <th style={styles.th}>Km Ingreso</th>
                    <th style={styles.th}>Estado</th>
                    <th style={styles.th}>Acciones</th>
                  </tr>
                </thead>
                <tbody>
                  {historicoCompletados.map((ot) => (
                    <tr key={ot.id_ot}>
                      <td style={styles.td}>{ot.codigo_ot}</td>
                      <td style={styles.td}>
                        <strong>{ot.placa}</strong>
                      </td>
                      <td style={styles.td}>{ot.nombre_mecanico || '—'}</td>
                      <td style={styles.td}>{ot.kilometraje_ingreso.toLocaleString()} km</td>
                      <td style={styles.td}>{ot.estado}</td>
                      <td style={styles.td}>
                        <button
                          onClick={() => setOtSeleccionada(ot)}
                          style={{
                            padding: '4px 8px',
                            backgroundColor: '#334155',
                            color: '#F8FAFC',
                            border: 'none',
                            borderRadius: '4px',
                            cursor: 'pointer',
                            fontSize: '12px',
                          }}
                        >
                          Ver
                        </button>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            )}
          </div>
        </>
      )}

      {modalCrearAbierto && (
        <ModalCrearOT
          ordenesExistentes={ordenes}
          zanjaInicial={zanjaSeleccionada}
          zanjasOcupadas={zanjasOcupadas}
          idMecanicoDefault={obtenerIdUsuario()}
          onClose={() => setModalCrearAbierto(false)}
          onSubmit={handleCrearOT}
          onIrARegistrarVehiculo={() => {
            setModalCrearAbierto(false);
            if (onNavegarAFichaVehicular) onNavegarAFichaVehicular();
          }}
        />
      )}

      {otSeleccionada && (
        <ModalDetalleOT
          ot={otSeleccionada}
          onClose={() => setOtSeleccionada(null)}
          onCambiarEstado={handleCambiarEstado}
          onCancelarOT={handleCancelarOT}
          onReasignarMecanico={handleReasignarMecanico}
          onIrAFicha={(placa) => {
            setOtSeleccionada(null);
            if (onNavegarAFichaVehicular) onNavegarAFichaVehicular(placa);
          }}
          onActualizarOT={cargarOrdenes}
        />
      )}
    </div>
  );
};
