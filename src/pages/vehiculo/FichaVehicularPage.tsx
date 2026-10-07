import React, { useState, useEffect, useRef } from 'react';
import { useAuth } from '../../context/AuthContext';
import { vehiculoService } from '../../services/vehiculoService';
import { ordenesService } from '../../services/ordenesService';
import {
  VehiculoCompletoDTO,
  OrdenTrabajoHistorialDTO as VehiculoOTDTO,
  CrearVehiculoDTO,
  EditarVehiculoDTO,
} from '../../types/vehiculo';
import { CrearOrdenTrabajoPayload, OrdenTrabajoHistorialDTO } from '../../types/ordenTrabajo';
import { ModalVehiculo, ModoModalVehiculo } from './components/ModalVehiculo';

// Modal de consulta informativa exclusivo de Ficha Vehicular
import { ModalDetalleOT } from './components/ModalDetalleOT';
import { ModalCrearOT } from '../ordenes/components/ModalCrearOT';
import { ConfirmModal } from '../../components/ConfirmModal';
import { ToastNotification } from '../../components/ToastNotification';

// Importación de componentes refactorizados
import { FichaVehicularHeader } from './components/FichaVehicularHeader';
import { FichaVehicularInfo } from './components/FichaVehicularInfo';
import { FichaVehicularHistorial } from './components/FichaVehicularHistorial';
import { FichaVehicularEmptyState } from './components/FichaVehicularEmptyState';
import { pageContainerStyle } from './vehiculoStyles';

interface FichaVehicularPageProps {
  placaInicial?: string;
  onVolver?: () => void;
  onNavegarAOrdenes?: () => void;
}

// Helper para extraer el mensaje de error sin usar 'any'
const extraerMensaje = (err: unknown): string => {
  if (typeof err === 'string') return err;
  if (err && typeof err === 'object' && 'message' in err) {
    return String((err as { message: unknown }).message);
  }
  return 'Ocurrió un error inesperado al procesar la solicitud.';
};

export const FichaVehicularPage: React.FC<FichaVehicularPageProps> = ({
  placaInicial,
  onNavegarAOrdenes,
}) => {
  const { tienePermiso, usuario } = useAuth();

  const puedeCrear = tienePermiso('FICHA_VEHICULAR', 'crear');
  const puedeEditar = tienePermiso('FICHA_VEHICULAR', 'editar');
  const puedeEliminar = tienePermiso('FICHA_VEHICULAR', 'eliminar');
  const puedeCrearOT = tienePermiso('OT', 'crear');

  const [placaBusqueda, setPlacaBusqueda] = useState<string>(placaInicial || '');
  const [vehiculo, setVehiculo] = useState<VehiculoCompletoDTO | null>(null);
  const [historial, setHistorial] = useState<VehiculoOTDTO[]>([]);
  const [ordenesTodas, setOrdenesTodas] = useState<OrdenTrabajoHistorialDTO[]>([]);
  const [cargando, setCargando] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [buscado, setBuscado] = useState<boolean>(false);

  const [notificacion, setNotificacion] = useState<{
    mensaje: string;
    tipo: 'EXITO' | 'ERROR';
  } | null>(null);

  const timeoutRef = useRef<number | null>(null);

  const [modalVehiculoAbierto, setModalVehiculoAbierto] = useState<boolean>(false);
  const [modoModalVehiculo, setModoModalVehiculo] = useState<ModoModalVehiculo>('CREAR');
  const [modalCrearOTAbierto, setModalCrearOTAbierto] = useState<boolean>(false);
  const [otSeleccionada, setOtSeleccionada] = useState<VehiculoOTDTO | null>(null);
  const [confirmModalEliminar, setConfirmModalEliminar] = useState<boolean>(false);

  const obtenerIdUsuario = (): number => {
    if (!usuario) return 0;
    const uAny = usuario as any;
    return uAny.id_usuario || uAny.id || uAny.idUsuario || 0;
  };

  const mostrarNotificacion = (mensaje: string, tipo: 'EXITO' | 'ERROR') => {
    if (timeoutRef.current !== null) {
      clearTimeout(timeoutRef.current);
    }
    setNotificacion({ mensaje, tipo });
    timeoutRef.current = window.setTimeout(() => {
      setNotificacion(null);
    }, 4500);
  };

  useEffect(() => {
    return () => {
      if (timeoutRef.current !== null) {
        clearTimeout(timeoutRef.current);
      }
    };
  }, []);

  const ejecutarBusqueda = async (placa: string) => {
    if (!placa.trim()) return;
    setCargando(true);
    setError(null);
    setVehiculo(null);
    setHistorial([]);
    setBuscado(true);

    const placaClean = placa.trim().toUpperCase();
    try {
      const uId = obtenerIdUsuario();
      const [fichaCompleta, listaOTs] = await Promise.all([
        vehiculoService.obtenerFichaVehicularCompleta(placaClean, uId),
        ordenesService.listarOrdenesTrabajo(uId).catch(() => []),
      ]);
      setVehiculo(fichaCompleta.vehiculo);
      setHistorial(fichaCompleta.historial);
      setOrdenesTodas(listaOTs);
    } catch (err: unknown) {
      setError(extraerMensaje(err));
    } finally {
      setCargando(false);
    }
  };

  useEffect(() => {
    if (placaInicial) {
      setPlacaBusqueda(placaInicial.toUpperCase());
      ejecutarBusqueda(placaInicial);
    } else {
      const searchParams = new URLSearchParams(window.location.search);
      const queryPlaca = searchParams.get('placa');
      if (queryPlaca) {
        setPlacaBusqueda(queryPlaca.toUpperCase());
        ejecutarBusqueda(queryPlaca);
      }
    }
  }, [placaInicial]);

  const handleGuardarVehiculo = async (data: CrearVehiculoDTO | EditarVehiculoDTO) => {
    try {
      if (modoModalVehiculo === 'CREAR') {
        await vehiculoService.registrarVehiculo(data as CrearVehiculoDTO, usuario?.id_usuario);
        mostrarNotificacion(`¡Vehículo con placa ${data.placa} registrado correctamente!`, 'EXITO');
      } else {
        await vehiculoService.actualizarVehiculo(data as EditarVehiculoDTO, usuario?.id_usuario);
        mostrarNotificacion('¡Ficha técnica y propietario actualizados correctamente!', 'EXITO');
      }
      setPlacaBusqueda(data.placa);
      await ejecutarBusqueda(data.placa);
    } catch (err: unknown) {
      const msgError = extraerMensaje(err);
      mostrarNotificacion(msgError, 'ERROR');
      throw err;
    }
  };

  const handleCrearOT = async (payload: CrearOrdenTrabajoPayload) => {
    try {
      const uId = obtenerIdUsuario();
      await ordenesService.crearOrdenTrabajo(payload, uId);
      mostrarNotificacion('¡Orden de Trabajo creada correctamente!', 'EXITO');
      setModalCrearOTAbierto(false);

      if (onNavegarAOrdenes) {
        onNavegarAOrdenes();
      } else if (vehiculo) {
        await ejecutarBusqueda(vehiculo.placa);
      }
    } catch (err: unknown) {
      const msgError = extraerMensaje(err);
      mostrarNotificacion(msgError, 'ERROR');
      throw err;
    }
  };

  const ejecutarEliminacion = async () => {
    if (!vehiculo) return;
    try {
      await vehiculoService.eliminarVehiculo(vehiculo.placa, usuario?.id_usuario);
      mostrarNotificacion(
        `El vehículo con placa ${vehiculo.placa} fue eliminado del sistema.`,
        'EXITO'
      );
      setVehiculo(null);
      setHistorial([]);
      setConfirmModalEliminar(false);
      setBuscado(false);
    } catch (err: unknown) {
      const msgError = extraerMensaje(err);
      mostrarNotificacion(msgError, 'ERROR');
      setConfirmModalEliminar(false);
    }
  };

  const zanjasOcupadas = ordenesTodas
    .filter((ot) => (ot.estado === 'EN_ESPERA' || ot.estado === 'EN_PROCESO') && ot.zanja)
    .map((ot) => Number(ot.zanja));

  return (
    <div style={pageContainerStyle}>
      {notificacion && (
        <ToastNotification
          mensaje={notificacion.mensaje}
          tipo={notificacion.tipo}
          onCerrar={() => setNotificacion(null)}
        />
      )}

      <FichaVehicularHeader
        placaBusqueda={placaBusqueda}
        setPlacaBusqueda={setPlacaBusqueda}
        cargando={cargando}
        puedeCrear={puedeCrear}
        onBuscar={(e) => {
          e.preventDefault();
          ejecutarBusqueda(placaBusqueda);
        }}
        onAbrirCrear={() => {
          setModoModalVehiculo('CREAR');
          setModalVehiculoAbierto(true);
        }}
      />

      {error && (
        <div
          style={{
            padding: '16px',
            backgroundColor: 'rgba(239, 68, 68, 0.1)',
            border: '1px solid rgba(239, 68, 68, 0.3)',
            borderRadius: '12px',
            color: '#FCA5A5',
            fontSize: '14px',
          }}
        >
          {error}
        </div>
      )}

      {!vehiculo && !cargando && (
        <FichaVehicularEmptyState
          buscado={buscado}
          placaBusqueda={placaBusqueda}
          puedeCrear={puedeCrear}
          onEjecutarBusqueda={(placa) => {
            setPlacaBusqueda(placa);
            ejecutarBusqueda(placa);
          }}
          onAbrirCrear={() => {
            setModoModalVehiculo('CREAR');
            setModalVehiculoAbierto(true);
          }}
        />
      )}

      {vehiculo && (
        <>
          <div
            style={{
              display: 'flex',
              justifyContent: 'space-between',
              alignItems: 'center',
              marginBottom: '12px',
            }}
          >
            <FichaVehicularInfo
              vehiculo={vehiculo}
              puedeEditar={puedeEditar}
              puedeEliminar={puedeEliminar}
              onAbrirEditar={() => {
                setModoModalVehiculo('EDITAR');
                setModalVehiculoAbierto(true);
              }}
              onAbrirEliminar={() => setConfirmModalEliminar(true)}
            />
          </div>

          {/* Botón flotante/superior para Crear OT directamente para este vehículo */}
          {puedeCrearOT && (
            <div style={{ display: 'flex', justifyContent: 'flex-end', marginBottom: '16px' }}>
              <button
                onClick={() => setModalCrearOTAbierto(true)}
                style={{
                  padding: '10px 18px',
                  backgroundColor: '#2563EB',
                  color: '#FFFFFF',
                  border: 'none',
                  borderRadius: '8px',
                  fontWeight: '700',
                  fontSize: '13px',
                  cursor: 'pointer',
                  boxShadow: '0 4px 12px rgba(37, 99, 235, 0.3)',
                }}
              >
                + Crear OT para {vehiculo.placa}
              </button>
            </div>
          )}

          <FichaVehicularHistorial
            historial={historial}
            onVerDetalleOT={(ot) => {
              setOtSeleccionada(ot);
            }}
          />
        </>
      )}

      <ModalVehiculo
        modalAbierto={modalVehiculoAbierto}
        setModalAbierto={setModalVehiculoAbierto}
        modo={modoModalVehiculo}
        vehiculoEditar={vehiculo}
        handleGuardar={handleGuardarVehiculo}
      />

      {/* Modal de consulta de detalle exclusivo de Ficha */}
      <ModalDetalleOT
        modalAbierto={!!otSeleccionada}
        setModalAbierto={(abierto) => {
          if (!abierto) setOtSeleccionada(null);
        }}
        ordenTrabajo={otSeleccionada}
      />

      {/* Modal de creación de OT con placa precargada */}
      {modalCrearOTAbierto && vehiculo && (
        <ModalCrearOT
          ordenesExistentes={ordenesTodas}
          zanjasOcupadas={zanjasOcupadas}
          idMecanicoDefault={obtenerIdUsuario()}
          placaInicial={vehiculo.placa}
          onClose={() => setModalCrearOTAbierto(false)}
          onSubmit={handleCrearOT}
        />
      )}

      <ConfirmModal
        abierto={confirmModalEliminar}
        titulo="⚠ ADVERTENCIA: ELIMINAR VEHÍCULO"
        mensaje={`¿Estás completamente seguro de eliminar el vehículo con placa "${vehiculo?.placa}"? Esta acción desactivará el vehículo en el sistema.`}
        onConfirmar={ejecutarEliminacion}
        onCancelar={() => setConfirmModalEliminar(false)}
      />
    </div>
  );
};
