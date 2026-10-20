import React, { useState, useEffect, useRef } from 'react';
import { useAuth } from '../../context/AuthContext';
import { vehiculoService } from '../../services/vehiculoService';
import { ordenesService } from '../../services/ordenesService';
import { clienteService } from '../../services/clienteService';
import {
  VehiculoCompletoDTO,
  OrdenTrabajoHistorialDTO as VehiculoOTDTO,
  CrearVehiculoDTO,
  EditarVehiculoDTO,
  CorregirHistorialOtPayloadDTO,
} from '../../types/vehiculo';
import { ClienteDTO, CrearClienteDTO, EditarClienteDTO } from '../../types/cliente';
import { CrearOrdenTrabajoPayload, OrdenTrabajoHistorialDTO } from '../../types/ordenTrabajo';
import { ModalVehiculo, ModoModalVehiculo } from './components/ModalVehiculo';

import { ModalDetalleOT } from './components/ModalDetalleOT';
import { ModalCorregirHistorialOT } from './components/ModalCorregirHistorialOT';
import { ModalCliente, ModoModalCliente } from './components/ModalCliente';
import { ModalCrearOT } from '../ordenes/components/ModalCrearOT';
import { ConfirmModal } from '../../components/ConfirmModal';
import { ToastNotification } from '../../components/ToastNotification';

import { FichaVehicularHeader } from './components/FichaVehicularHeader';
import { FichaVehicularInfo } from './components/FichaVehicularInfo';
import { FichaVehicularHistorial } from './components/FichaVehicularHistorial';
import { FichaVehicularEmptyState } from './components/FichaVehicularEmptyState';
import { FichaVehicularProximosCambios } from './components/FichaVehicularProximosCambios';
import { pageContainerStyle } from './vehiculoStyles';
import { Car, Bell } from 'lucide-react';

interface FichaVehicularPageProps {
  placaInicial?: string;
  onVolver?: () => void;
  onNavegarAOrdenes?: () => void;
}

type PestanaVehiculo = 'FICHA' | 'PROXIMOS_CAMBIOS';

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

  const esMecanico = usuario?.rol === 'MECANICO';
  const esAdmin = usuario?.rol === 'ADMINISTRADOR';

  const puedeCrear = !esMecanico && tienePermiso('FICHA_VEHICULAR', 'crear');
  const puedeEditar = !esMecanico && tienePermiso('FICHA_VEHICULAR', 'editar');
  const puedeEliminar = !esMecanico && tienePermiso('FICHA_VEHICULAR', 'eliminar');
  const puedeCrearOT = !esMecanico && tienePermiso('OT', 'crear');

  const [pestanaActiva, setPestanaActiva] = useState<PestanaVehiculo>('FICHA');
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
  const busquedaRequestIdRef = useRef<number>(0);

  const [modalVehiculoAbierto, setModalVehiculoAbierto] = useState<boolean>(false);
  const [modoModalVehiculo, setModoModalVehiculo] = useState<ModoModalVehiculo>('CREAR');
  const [modalCrearOTAbierto, setModalCrearOTAbierto] = useState<boolean>(false);
  const [otSeleccionada, setOtSeleccionada] = useState<VehiculoOTDTO | null>(null);
  const [otCorregirSeleccionada, setOtCorregirSeleccionada] = useState<VehiculoOTDTO | null>(null);

  const [modalClienteAbierto, setModalClienteAbierto] = useState<boolean>(false);
  const [modoModalCliente, setModoModalCliente] = useState<ModoModalCliente>('EDITAR');
  const [clienteEditar, setClienteEditar] = useState<ClienteDTO | null>(null);

  const [confirmModalEliminar, setConfirmModalEliminar] = useState<boolean>(false);
  const [confirmModalReactivar, setConfirmModalReactivar] = useState<boolean>(false);
  const [motivoAccion, setMotivoAccion] = useState<string>('');

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

    const currentRequestId = ++busquedaRequestIdRef.current;

    setCargando(true);
    setError(null);
    setVehiculo(null);
    setHistorial([]);
    setBuscado(true);

    const placaClean = placa.replace(/[^a-zA-Z0-9]/g, '').toUpperCase();
    try {
      const [fichaCompleta, listaOTs] = await Promise.all([
        vehiculoService.obtenerFichaVehicularCompleta(placaClean),
        ordenesService.listarOrdenesTrabajo().catch(() => []),
      ]);

      if (currentRequestId !== busquedaRequestIdRef.current) return;

      setVehiculo(fichaCompleta.vehiculo);
      setHistorial(fichaCompleta.historial);
      setOrdenesTodas(listaOTs);
    } catch (err: unknown) {
      if (currentRequestId !== busquedaRequestIdRef.current) return;
      const msg = extraerMensaje(err);

      // Si el error es de "No encontrado", mostramos el EmptyState limpio
      if (msg.toLowerCase().includes('no se encontró vehículo registrado')) {
        setError(null);
      } else {
        setError(msg);
      }
    } finally {
      if (currentRequestId === busquedaRequestIdRef.current) {
        setCargando(false);
      }
    }
  };

  useEffect(() => {
    if (placaInicial) {
      setPlacaBusqueda(placaInicial.toUpperCase());
      ejecutarBusqueda(placaInicial);
    }
  }, [placaInicial]);

  const handleGuardarVehiculo = async (data: CrearVehiculoDTO | EditarVehiculoDTO) => {
    try {
      if (modoModalVehiculo === 'CREAR') {
        const placaReg = await vehiculoService.registrarVehiculo(data as CrearVehiculoDTO);
        mostrarNotificacion(`¡Vehículo con placa ${placaReg} registrado correctamente!`, 'EXITO');
      } else {
        await vehiculoService.actualizarVehiculo(data as EditarVehiculoDTO);
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
      await ordenesService.crearOrdenTrabajo(payload);
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

  const handleCorregirHistorialOT = async (payload: CorregirHistorialOtPayloadDTO) => {
    try {
      await vehiculoService.corregirHistorialOt(payload);
      mostrarNotificacion('¡Historial de la OT corregido con éxito!', 'EXITO');
      setOtCorregirSeleccionada(null);
      if (vehiculo) {
        await ejecutarBusqueda(vehiculo.placa);
      }
    } catch (err: unknown) {
      const msgError = extraerMensaje(err);
      mostrarNotificacion(msgError, 'ERROR');
      throw err;
    }
  };

  const handleAbrirCrearCliente = () => {
    if (esMecanico) return;
    setClienteEditar(null);
    setModoModalCliente('CREAR');
    setModalClienteAbierto(true);
  };

  const handleAbrirModalCliente = async () => {
    if (!vehiculo || esMecanico) return;
    try {
      const tipoDoc = vehiculo.cliente?.tipo_documento || vehiculo.tipo_documento_cliente || 'DNI';
      const numDoc = vehiculo.cliente?.numero_documento || vehiculo.documento_cliente;

      if (numDoc) {
        const dataCliente = await clienteService.buscarClientePorDocumento(tipoDoc, numDoc);
        setClienteEditar(dataCliente);
      } else {
        setClienteEditar(null);
      }
      setModoModalCliente('EDITAR');
      setModalClienteAbierto(true);
    } catch (err: unknown) {
      const msg = extraerMensaje(err);
      mostrarNotificacion(msg, 'ERROR');
    }
  };

  const handleGuardarCliente = async (data: CrearClienteDTO | EditarClienteDTO) => {
    try {
      if (modoModalCliente === 'CREAR') {
        await clienteService.registrarCliente(data as CrearClienteDTO);
        mostrarNotificacion('¡Nuevo cliente registrado con éxito!', 'EXITO');
      } else {
        await clienteService.actualizarCliente(data as EditarClienteDTO);
        mostrarNotificacion('¡Datos del cliente actualizados con éxito!', 'EXITO');
      }
      setModalClienteAbierto(false);
      if (vehiculo) {
        await ejecutarBusqueda(vehiculo.placa);
      }
    } catch (err: unknown) {
      const msgError = extraerMensaje(err);
      mostrarNotificacion(msgError, 'ERROR');
      throw err;
    }
  };

  const ejecutarDesactivacion = async () => {
    if (!vehiculo || esMecanico) return;
    if (!motivoAccion.trim() || motivoAccion.trim().length < 5) {
      mostrarNotificacion(
        'Debe ingresar un motivo válido (mínimo 5 caracteres) para desactivar el vehículo.',
        'ERROR'
      );
      return;
    }

    try {
      await vehiculoService.cambiarEstadoVehiculo({
        placa: vehiculo.placa,
        activo: 0,
        motivo: motivoAccion.trim(),
      });
      mostrarNotificacion(
        `El vehículo con placa ${vehiculo.placa} fue desactivado correctamente.`,
        'EXITO'
      );
      setConfirmModalEliminar(false);
      setMotivoAccion('');
      await ejecutarBusqueda(vehiculo.placa);
    } catch (err: unknown) {
      const msgError = extraerMensaje(err);
      mostrarNotificacion(msgError, 'ERROR');
    }
  };

  const ejecutarReactivacion = async () => {
    if (!vehiculo || esMecanico) return;
    if (!motivoAccion.trim() || motivoAccion.trim().length < 5) {
      mostrarNotificacion(
        'Debe ingresar un motivo válido (mínimo 5 caracteres) para reactivar el vehículo.',
        'ERROR'
      );
      return;
    }

    try {
      await vehiculoService.cambiarEstadoVehiculo({
        placa: vehiculo.placa,
        activo: 1,
        motivo: motivoAccion.trim(),
      });
      mostrarNotificacion(
        `El vehículo con placa ${vehiculo.placa} fue reactivado correctamente.`,
        'EXITO'
      );
      setConfirmModalReactivar(false);
      setMotivoAccion('');
      await ejecutarBusqueda(vehiculo.placa);
    } catch (err: unknown) {
      const msgError = extraerMensaje(err);
      mostrarNotificacion(msgError, 'ERROR');
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

      {/* NAVEGACIÓN ENTRE PESTAÑAS */}
      <div
        style={{
          display: 'flex',
          gap: '12px',
          marginBottom: '16px',
          borderBottom: '1px solid #334155',
          paddingBottom: '8px',
        }}
      >
        <button
          onClick={() => setPestanaActiva('FICHA')}
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: '8px',
            padding: '10px 18px',
            backgroundColor: pestanaActiva === 'FICHA' ? '#2563EB' : 'transparent',
            color: pestanaActiva === 'FICHA' ? '#FFFFFF' : '#94A3B8',
            border: 'none',
            borderRadius: '8px',
            fontWeight: '700',
            fontSize: '13px',
            cursor: 'pointer',
          }}
        >
          <Car size={16} /> Ficha del Vehículo
        </button>

        {!esMecanico && (
          <button
            onClick={() => setPestanaActiva('PROXIMOS_CAMBIOS')}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              padding: '10px 18px',
              backgroundColor: pestanaActiva === 'PROXIMOS_CAMBIOS' ? '#2563EB' : 'transparent',
              color: pestanaActiva === 'PROXIMOS_CAMBIOS' ? '#FFFFFF' : '#94A3B8',
              border: 'none',
              borderRadius: '8px',
              fontWeight: '700',
              fontSize: '13px',
              cursor: 'pointer',
            }}
          >
            <Bell size={16} /> Próximos Cambios de Aceite
          </button>
        )}
      </div>

      {pestanaActiva === 'PROXIMOS_CAMBIOS' && !esMecanico ? (
        <FichaVehicularProximosCambios
          onSeleccionarPlaca={(placa: string) => {
            setPlacaBusqueda(placa);
            setPestanaActiva('FICHA');
            ejecutarBusqueda(placa);
          }}
        />
      ) : (
        <>
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
            onAbrirCrearCliente={handleAbrirCrearCliente}
          />

          {error && <div style={errorStyle}>⚠️ Error del Sistema: {error}</div>}

          {!vehiculo && !cargando && !error && (
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
                  flexDirection: 'column',
                  gap: '16px',
                  marginBottom: '16px',
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
                  onAbrirReactivar={() => setConfirmModalReactivar(true)}
                  onAbrirModalCliente={handleAbrirModalCliente}
                />
              </div>

              {puedeCrearOT && vehiculo.activo === 1 && (
                <div style={{ display: 'flex', justifyContent: 'flex-end', margin: '4px 0 8px 0' }}>
                  <button onClick={() => setModalCrearOTAbierto(true)} style={btnCrearOTStyle}>
                    + Crear OT para {vehiculo.placa}
                  </button>
                </div>
              )}

              <FichaVehicularHistorial
                historial={historial}
                puedeCorregirOT={esAdmin}
                onVerDetalleOT={(ot) => setOtSeleccionada(ot)}
                onCorregirOT={(ot) => setOtCorregirSeleccionada(ot)}
              />
            </>
          )}
        </>
      )}

      {/* MODAL REGISTRAR / EDITAR VEHÍCULO */}
      {modalVehiculoAbierto && (
        <ModalVehiculo
          modalAbierto={modalVehiculoAbierto}
          setModalAbierto={setModalVehiculoAbierto}
          modo={modoModalVehiculo}
          vehiculoEditar={vehiculo}
          handleGuardar={handleGuardarVehiculo}
          esAdmin={esAdmin}
        />
      )}

      {/* MODAL DETALLE DE OT */}
      <ModalDetalleOT
        modalAbierto={!!otSeleccionada}
        setModalAbierto={(abierto) => {
          if (!abierto) setOtSeleccionada(null);
        }}
        ordenTrabajo={otSeleccionada}
      />

      {/* MODAL CORREGIR HISTORIAL OT */}
      {otCorregirSeleccionada && (
        <ModalCorregirHistorialOT
          modalAbierto={!!otCorregirSeleccionada}
          setModalAbierto={(abierto) => {
            if (!abierto) setOtCorregirSeleccionada(null);
          }}
          ordenTrabajo={otCorregirSeleccionada}
          handleGuardar={handleCorregirHistorialOT}
        />
      )}

      {/* MODAL EDICIÓN / REGISTRO DE CLIENTE */}
      {modalClienteAbierto && (
        <ModalCliente
          modalAbierto={modalClienteAbierto}
          setModalAbierto={setModalClienteAbierto}
          modo={modoModalCliente}
          clienteEditar={clienteEditar}
          onClienteGuardado={handleGuardarCliente}
        />
      )}

      {/* MODAL CREAR ORDEN DE TRABAJO */}
      {modalCrearOTAbierto && vehiculo && (
        <ModalCrearOT
          ordenesExistentes={ordenesTodas}
          zanjasOcupadas={zanjasOcupadas}
          idMecanicoDefault={0}
          placaInicial={vehiculo.placa}
          onClose={() => setModalCrearOTAbierto(false)}
          onSubmit={handleCrearOT}
        />
      )}

      {/* CONFIRM MODAL PARA DESACTIVAR VEHÍCULO */}
      <ConfirmModal
        abierto={confirmModalEliminar}
        titulo="⚠ ADVERTENCIA: DESACTIVAR VEHÍCULO"
        mensaje={`¿Estás seguro de desactivar el vehículo con placa "${vehiculo?.placa}"? Ingrese un motivo obligatorio para proceder:`}
        requiereMotivo={true}
        motivoValue={motivoAccion}
        onMotivoChange={setMotivoAccion}
        onConfirmar={ejecutarDesactivacion}
        onCancelar={() => {
          setConfirmModalEliminar(false);
          setMotivoAccion('');
        }}
      />

      {/* CONFIRM MODAL PARA REACTIVAR VEHÍCULO */}
      <ConfirmModal
        abierto={confirmModalReactivar}
        titulo="✅ REACTIVAR VEHÍCULO"
        mensaje={`¿Desea reactivar el vehículo con placa "${vehiculo?.placa}" para permitir nuevas Órdenes de Trabajo? Ingrese un motivo:`}
        requiereMotivo={true}
        motivoValue={motivoAccion}
        onMotivoChange={setMotivoAccion}
        onConfirmar={ejecutarReactivacion}
        onCancelar={() => {
          setConfirmModalReactivar(false);
          setMotivoAccion('');
        }}
      />
    </div>
  );
};

const errorStyle: React.CSSProperties = {
  padding: '16px',
  backgroundColor: 'rgba(239, 68, 68, 0.1)',
  border: '1px solid rgba(239, 68, 68, 0.3)',
  borderRadius: '12px',
  color: '#FCA5A5',
  fontSize: '14px',
};

const btnCrearOTStyle: React.CSSProperties = {
  padding: '10px 18px',
  backgroundColor: '#2563EB',
  color: '#FFFFFF',
  border: 'none',
  borderRadius: '8px',
  fontWeight: '700',
  fontSize: '13px',
  cursor: 'pointer',
  boxShadow: '0 4px 12px rgba(37, 99, 235, 0.3)',
};
