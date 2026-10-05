import React, { useState, useEffect, useRef } from 'react';
import { useAuth } from '../../context/AuthContext';
import { vehiculoService } from '../../services/vehiculoService';
import {
  VehiculoCompletoDTO,
  OrdenTrabajoHistorialDTO,
  CrearVehiculoDTO,
  EditarVehiculoDTO,
} from '../../types/vehiculo';
import { ModalVehiculo, ModoModalVehiculo } from './components/ModalVehiculo';
import { ModalDetalleOT } from './components/ModalDetalleOT';
import { ConfirmModal } from '../../components/ConfirmModal';
import { ToastNotification } from '../../components/ToastNotification';

// Importación de componentes refactorizados
import { FichaVehicularHeader } from './components/FichaVehicularHeader';
import { FichaVehicularInfo } from './components/FichaVehicularInfo';
import { FichaVehicularHistorial } from './components/FichaVehicularHistorial';
import { FichaVehicularEmptyState } from './components/FichaVehicularEmptyState';
import { pageContainerStyle } from './vehiculoStyles';

// Helper para extraer el mensaje de error sin usar 'any'
const extraerMensaje = (err: unknown): string => {
  if (typeof err === 'string') return err;
  if (err && typeof err === 'object' && 'message' in err) {
    return String((err as { message: unknown }).message);
  }
  return 'Ocurrió un error inesperado al procesar la solicitud.';
};

export const FichaVehicularPage: React.FC = () => {
  const { tienePermiso, usuario } = useAuth();

  const puedeCrear = tienePermiso('FICHA_VEHICULAR', 'crear');
  const puedeEditar = tienePermiso('FICHA_VEHICULAR', 'editar');
  const puedeEliminar = tienePermiso('FICHA_VEHICULAR', 'eliminar');

  const [placaBusqueda, setPlacaBusqueda] = useState<string>('');
  const [vehiculo, setVehiculo] = useState<VehiculoCompletoDTO | null>(null);
  const [historial, setHistorial] = useState<OrdenTrabajoHistorialDTO[]>([]);
  const [cargando, setCargando] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [buscado, setBuscado] = useState<boolean>(false);

  const [notificacion, setNotificacion] = useState<{
    mensaje: string;
    tipo: 'EXITO' | 'ERROR';
  } | null>(null);

  // Referencia numérica para limpiar el timeout del toast en el navegador
  const timeoutRef = useRef<number | null>(null);

  const [modalVehiculoAbierto, setModalVehiculoAbierto] = useState<boolean>(false);
  const [modoModalVehiculo, setModoModalVehiculo] = useState<ModoModalVehiculo>('CREAR');
  const [modalOTAbierto, setModalOTAbierto] = useState<boolean>(false);
  const [otSeleccionada, setOtSeleccionada] = useState<OrdenTrabajoHistorialDTO | null>(null);
  const [confirmModalEliminar, setConfirmModalEliminar] = useState<boolean>(false);

  const mostrarNotificacion = (mensaje: string, tipo: 'EXITO' | 'ERROR') => {
    if (timeoutRef.current !== null) {
      clearTimeout(timeoutRef.current);
    }
    setNotificacion({ mensaje, tipo });
    timeoutRef.current = window.setTimeout(() => {
      setNotificacion(null);
    }, 4500);
  };

  // Cleanup effect al desmontar el componente
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
      const fichaCompleta = await vehiculoService.obtenerFichaVehicularCompleta(
        placaClean,
        usuario?.id_usuario
      );
      setVehiculo(fichaCompleta.vehiculo);
      setHistorial(fichaCompleta.historial);
    } catch (err: unknown) {
      setError(extraerMensaje(err));
    } finally {
      setCargando(false);
    }
  };

  useEffect(() => {
    const searchParams = new URLSearchParams(window.location.search);
    const queryPlaca = searchParams.get('placa');

    if (queryPlaca) {
      setPlacaBusqueda(queryPlaca.toUpperCase());
      ejecutarBusqueda(queryPlaca);
    }
  }, []);

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

          <FichaVehicularHistorial
            historial={historial}
            onVerDetalleOT={(ot) => {
              setOtSeleccionada(ot);
              setModalOTAbierto(true);
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

      <ModalDetalleOT
        modalAbierto={modalOTAbierto}
        setModalAbierto={setModalOTAbierto}
        ordenTrabajo={otSeleccionada}
      />

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
