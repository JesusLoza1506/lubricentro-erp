import React, { useEffect, useState } from 'react';
import { useAuth } from '../../context/AuthContext';
import { ordenesService } from '../../services/ordenesService';
import {
  TableroZanjasDTO,
  OrdenTrabajoHistorialDTO,
  CrearOrdenTrabajoPayload,
  TipoAceiteOT,
} from '../../types/ordenTrabajo';
import { styles } from './ordenesStyles';
import { TableroZanjas } from './components/TableroZanjas';
import { ModalCrearOT } from './components/ModalCrearOT';
import { ModalDetalleOT } from './components/ModalDetalleOT';

interface OrdenesPageProps {
  onNavegarAFichaVehicular?: (placa?: string) => void;
}

export const OrdenesPage: React.FC<OrdenesPageProps> = () => {
  const { usuario, tienePermiso } = useAuth();
  const [tablero, setTablero] = useState<TableroZanjasDTO | null>(null);
  const [loading, setLoading] = useState(true);
  const [modalCrearAbierto, setModalCrearAbierto] = useState(false);
  const [otSeleccionada, setOtSeleccionada] = useState<OrdenTrabajoHistorialDTO | null>(null);

  const rol = usuario?.rol || 'CAJERO';
  const idUsuario = (usuario as any)?.id_usuario || (usuario as any)?.id || 0;

  // Validación de permisos para la creación de OT / Recepción de vehículo
  const puedeCrearOT = usuario?.rol !== 'MECANICO' && tienePermiso('OT', 'crear');

  const cargarTablero = async () => {
    try {
      setLoading(true);
      if (!idUsuario) return;
      const data = await ordenesService.obtenerTableroZanjas(idUsuario, rol);
      setTablero(data);

      if (otSeleccionada) {
        const act = await ordenesService.obtenerOtPorId(otSeleccionada.id_ot, idUsuario, rol);
        setOtSeleccionada(act);
      }
    } catch (err) {
      console.error('Error al cargar tablero de zanjas:', err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    cargarTablero();
  }, [usuario]);

  const handleCrearOT = async (payload: CrearOrdenTrabajoPayload) => {
    await ordenesService.crearOrdenTrabajo(payload, idUsuario, rol);
    await cargarTablero();
  };

  const handleIniciarEnZanja = async (idOt: number, idZanja: number) => {
    await ordenesService.iniciarTrabajoOT(
      { id_ot: idOt, id_zanja_ejecucion: idZanja },
      idUsuario,
      rol
    );
    await cargarTablero();
  };

  const handleFinalizarOT = async (
    idOt: number,
    kmSalida: number,
    resumen: string,
    tipoAceite?: TipoAceiteOT
  ) => {
    await ordenesService.finalizarOT(
      {
        id_ot: idOt,
        kilometraje_salida: kmSalida,
        resumen_publico: resumen,
        tipo_aceite: tipoAceite,
      },
      idUsuario,
      rol
    );
    await cargarTablero();
  };

  const handleCancelarOT = async (idOt: number, motivo: string) => {
    await ordenesService.cancelarOT({ id_ot: idOt, motivo }, idUsuario, rol);
    await cargarTablero();
  };

  const handleReasignarMecanico = async (idOt: number, nuevoIdMecanico: number, motivo: string) => {
    await ordenesService.reasignarMecanicoOT(
      { id_ot: idOt, nuevo_id_mecanico: nuevoIdMecanico, motivo },
      idUsuario,
      rol
    );
    await cargarTablero();
  };

  return (
    <div style={styles.container}>
      <div style={styles.header}>
        <div>
          <h1 style={styles.title}>Órdenes de Trabajo / Taller v7.0</h1>
          <p style={styles.subtitle}>
            Gestión transaccional de zanjas 1 a 6 y cola común de espera
          </p>
        </div>
        {puedeCrearOT && (
          <button style={styles.btnNuevaOT} onClick={() => setModalCrearAbierto(true)}>
            + Recepcionar Vehículo
          </button>
        )}
      </div>

      {loading || !tablero ? (
        <div style={{ padding: '32px', textAlign: 'center', color: '#94A3B8' }}>
          Cargando tablero en tiempo real...
        </div>
      ) : (
        <TableroZanjas
          tablero={tablero}
          onVerDetalle={(ot) => setOtSeleccionada(ot)}
          onIniciarEnZanja={handleIniciarEnZanja}
        />
      )}

      {modalCrearAbierto && (
        <ModalCrearOT
          idMecanicoDefault={idUsuario}
          onClose={() => setModalCrearAbierto(false)}
          onSubmit={handleCrearOT}
        />
      )}

      {otSeleccionada && (
        <ModalDetalleOT
          ot={otSeleccionada}
          onClose={() => setOtSeleccionada(null)}
          onIniciarZanja={handleIniciarEnZanja}
          onFinalizarOT={handleFinalizarOT}
          onCancelarOT={handleCancelarOT}
          onReasignarMecanico={handleReasignarMecanico}
          onActualizarOT={cargarTablero}
        />
      )}
    </div>
  );
};
