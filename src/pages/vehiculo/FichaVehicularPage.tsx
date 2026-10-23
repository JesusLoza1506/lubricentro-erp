import './vehiculoVisual.css';
import React, { useState, useEffect, useRef } from 'react';
import { useAuth } from '../../context/AuthContext';
import { vehiculoService } from '../../services/vehiculoService';
import { ordenesService } from '../../services/ordenesService';
import { clienteService } from '../../services/clienteService';
import {
  VehiculoCompletoDTO,
  HistorialOTPublicoDTO as VehiculoOTDTO,
  CrearVehiculoDTO,
  EditarVehiculoDTO,
  CorregirHistorialOtPayloadDTO,
} from '../../types/vehiculo';
import { ClienteDTO, CrearClienteDTO, EditarClienteDTO } from '../../types/cliente';
import { CrearOrdenTrabajoPayload } from '../../types/ordenTrabajo';
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
import {
  FichaVehicularProximosCambios,
  FiltrosProximosCambiosState,
} from './components/FichaVehicularProximosCambios';
import { pageContainerStyle } from './vehiculoStyles';
import { Car, Bell, RotateCcw } from 'lucide-react';

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
  const { tienePermiso, usuario, setHayCambiosSinGuardar } = useAuth();

  const idUsuario = usuario?.id_usuario ?? 0;
  const rolUsuario = usuario?.rol ?? '';

  const esMecanico = usuario?.rol === 'MECANICO';
  const esAdmin = usuario?.rol === 'ADMINISTRADOR';

  const puedeCrear = !esMecanico && tienePermiso('FICHA_VEHICULAR', 'crear');
  const puedeEditar = !esMecanico && tienePermiso('FICHA_VEHICULAR', 'editar');
  const puedeEliminar = !esMecanico && tienePermiso('FICHA_VEHICULAR', 'eliminar');
  const puedeCrearOT = !esMecanico && tienePermiso('OT', 'crear');

  const [pestanaActiva, setPestanaActiva] = useState<PestanaVehiculo>('FICHA');
  const [placaBusqueda, setPlacaBusqueda] = useState<string>(placaInicial || '');
  const [vehiculo, setVehiculo] = useState<VehiculoCompletoDTO | null>(null);

  // Historial Paginado Global
  const [historial, setHistorial] = useState<VehiculoOTDTO[]>([]);
  const [totalRegistrosHistorial, setTotalRegistrosHistorial] = useState<number>(0);
  const [paginaActualHistorial, setPaginaActualHistorial] = useState<number>(1);
  const [totalPaginasHistorial, setTotalPaginasHistorial] = useState<number>(1);

  // Tarjetas de Resumen Rápido (Estáticas e independientes de la página)
  const [ultimoServicio, setUltimoServicio] = useState<VehiculoOTDTO | null>(null);
  const [ultimoCambioAceite, setUltimoCambioAceite] = useState<VehiculoOTDTO | null>(null);

  const [cargando, setCargando] = useState<boolean>(false);
  const [errorSistema, setErrorSistema] = useState<string | null>(null);
  const [buscado, setBuscado] = useState<boolean>(false);

  const [filtrosProximos, setFiltrosProximos] = useState<FiltrosProximosCambiosState>({
    busqueda: '',
    filtroCausa: 'TODOS',
    paginaActual: 1,
  });

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

  const ejecutarBusqueda = async (placa: string, pagina: number = 1) => {
    if (!placa.trim()) return;

    const currentRequestId = ++busquedaRequestIdRef.current;

    setCargando(true);
    setErrorSistema(null);
    setBuscado(true);

    if (pagina === 1) {
      setVehiculo(null);
      setHistorial([]);
      setUltimoServicio(null);
      setUltimoCambioAceite(null);
    }

    const placaClean = placa.replace(/[^a-zA-Z0-9]/g, '').toUpperCase();
    try {
      const fichaCompleta = await vehiculoService.obtenerFichaVehicularCompleta(
        placaClean,
        pagina,
        20
      );

      if (currentRequestId !== busquedaRequestIdRef.current) return;

      setVehiculo(fichaCompleta.vehiculo);
      const items = fichaCompleta.historial?.items || [];
      setHistorial(items);
      setTotalRegistrosHistorial(fichaCompleta.historial?.total_registros || 0);
      setPaginaActualHistorial(fichaCompleta.historial?.pagina_actual || 1);
      setTotalPaginasHistorial(fichaCompleta.historial?.total_paginas || 1);

      // Tarjetas independientes del backend
      setUltimoServicio(
        (fichaCompleta as any).ultimo_servicio || (items.length > 0 ? items[0] : null)
      );
      setUltimoCambioAceite((fichaCompleta as any).ultimo_cambio_aceite || null);
    } catch (err: unknown) {
      if (currentRequestId !== busquedaRequestIdRef.current) return;
      const msg = extraerMensaje(err);
      const msgLower = msg.toLowerCase();

      if (
        msgLower.includes('no se encontró vehículo registrado') ||
        msgLower.includes('notfound')
      ) {
        setErrorSistema(null);
      } else {
        setErrorSistema(msg);
      }
    } finally {
      if (currentRequestId === busquedaRequestIdRef.current) {
        setCargando(false);
      }
    }
  };

  const handleLimpiarBusqueda = () => {
    busquedaRequestIdRef.current++;
    setPlacaBusqueda('');
    setVehiculo(null);
    setHistorial([]);
    setTotalRegistrosHistorial(0);
    setPaginaActualHistorial(1);
    setTotalPaginasHistorial(1);
    setUltimoServicio(null);
    setUltimoCambioAceite(null);
    setErrorSistema(null);
    setBuscado(false);
  };

  useEffect(() => {
    if (placaInicial) {
      setPlacaBusqueda(placaInicial.toUpperCase());
      ejecutarBusqueda(placaInicial, 1);
    }
  }, [placaInicial]);

  const handleAbrirCrearOTDirecta = async () => {
    if (!vehiculo) return;

    if (vehiculo.activo !== 1) {
      mostrarNotificacion(
        `El vehículo con placa ${vehiculo.placa} se encuentra desactivado. Debe ser reactivado antes de recepcionar.`,
        'ERROR'
      );
      return;
    }

    try {
      setCargando(true);
      const ordenesActivas = await ordenesService.listarOrdenesTrabajo(idUsuario, rolUsuario);
      const otAbierta = ordenesActivas.find(
        (ot) =>
          ot.placa.toUpperCase() === vehiculo.placa.toUpperCase() &&
          (ot.estado === 'EN_ESPERA' || ot.estado === 'EN_PROCESO')
      );

      if (otAbierta) {
        mostrarNotificacion(
          `El vehículo ya cuenta con una Orden de Trabajo abierta (${otAbierta.codigo_ot} en estado ${otAbierta.estado}).`,
          'ERROR'
        );
        return;
      }

      setHayCambiosSinGuardar(true);
      setModalCrearOTAbierto(true);
    } catch (err: unknown) {
      const msg = extraerMensaje(err);
      mostrarNotificacion(msg, 'ERROR');
    } finally {
      setCargando(false);
    }
  };

  const handleGuardarVehiculo = async (data: CrearVehiculoDTO | EditarVehiculoDTO) => {
    try {
      if (modoModalVehiculo === 'CREAR') {
        const placaReg = await vehiculoService.registrarVehiculo(data as CrearVehiculoDTO);
        mostrarNotificacion(`¡Vehículo con placa ${placaReg} registrado correctamente!`, 'EXITO');
      } else {
        await vehiculoService.actualizarVehiculo(data as EditarVehiculoDTO);
        mostrarNotificacion('¡Ficha técnica y propietario actualizados correctamente!', 'EXITO');
      }
      setHayCambiosSinGuardar(false);
      setModalVehiculoAbierto(false);
      setPlacaBusqueda(data.placa);
      await ejecutarBusqueda(data.placa, 1);
    } catch (err: unknown) {
      const msgError = extraerMensaje(err);
      mostrarNotificacion(msgError, 'ERROR');
      throw err;
    }
  };

  const handleCrearOT = async (payload: CrearOrdenTrabajoPayload) => {
    try {
      await ordenesService.crearOrdenTrabajo(payload, idUsuario, rolUsuario);
      mostrarNotificacion('¡Orden de Trabajo creada correctamente!', 'EXITO');
      setHayCambiosSinGuardar(false);
      setModalCrearOTAbierto(false);

      if (onNavegarAOrdenes) {
        onNavegarAOrdenes();
      } else if (vehiculo) {
        await ejecutarBusqueda(vehiculo.placa, 1);
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
      setHayCambiosSinGuardar(false);
      setOtCorregirSeleccionada(null);
      if (vehiculo) {
        await ejecutarBusqueda(vehiculo.placa, paginaActualHistorial);
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
    setHayCambiosSinGuardar(true);
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
      setHayCambiosSinGuardar(true);
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
      setHayCambiosSinGuardar(false);
      setModalClienteAbierto(false);
      if (vehiculo) {
        await ejecutarBusqueda(vehiculo.placa, paginaActualHistorial);
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
      await ejecutarBusqueda(vehiculo.placa, paginaActualHistorial);
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
      await ejecutarBusqueda(vehiculo.placa, paginaActualHistorial);
    } catch (err: unknown) {
      const msgError = extraerMensaje(err);
      mostrarNotificacion(msgError, 'ERROR');
    }
  };

  return (
    <div className="fv-page" style={pageContainerStyle}>
      {notificacion && (
        <ToastNotification
          mensaje={notificacion.mensaje}
          tipo={notificacion.tipo}
          onCerrar={() => setNotificacion(null)}
        />
      )}

      <div
        style={{
          display: 'flex',
          gap: '6px',
          marginBottom: '0px',
          borderBottom: '1px solid #334155',
          paddingBottom: '0px',
        }}
      >
        <button
          onClick={() => setPestanaActiva('FICHA')}
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: '8px',
            padding: '10px 18px',
            backgroundColor: 'transparent',
            borderBottom: pestanaActiva === 'FICHA' ? '2px solid #6DA5F0' : '2px solid transparent',
            color: pestanaActiva === 'FICHA' ? '#FFFFFF' : '#94A3B8',
            borderTop: 'none',
            borderLeft: 'none',
            borderRight: 'none',
            borderRadius: '0px',
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
              backgroundColor: 'transparent',
              borderBottom:
                pestanaActiva === 'PROXIMOS_CAMBIOS'
                  ? '2px solid #6DA5F0'
                  : '2px solid transparent',
              color: pestanaActiva === 'PROXIMOS_CAMBIOS' ? '#FFFFFF' : '#94A3B8',
              borderTop: 'none',
              borderLeft: 'none',
              borderRight: 'none',
              borderRadius: '0px',
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
          filtrosIniciales={filtrosProximos}
          onFiltrosChange={setFiltrosProximos}
          onSeleccionarPlaca={(placa: string) => {
            setPlacaBusqueda(placa);
            setPestanaActiva('FICHA');
            ejecutarBusqueda(placa, 1);
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
              ejecutarBusqueda(placaBusqueda, 1);
            }}
            onLimpiar={handleLimpiarBusqueda}
            onAbrirCrear={() => {
              setModoModalVehiculo('CREAR');
              setHayCambiosSinGuardar(true);
              setModalVehiculoAbierto(true);
            }}
            onAbrirCrearCliente={handleAbrirCrearCliente}
          />

          {errorSistema && (
            <div style={errorBannerStyle}>
              <div style={{ flex: 1 }}>
                <strong>⚠️ Error al consultar la base de datos:</strong>
                <p style={{ margin: '4px 0 0 0', fontSize: '13px' }}>{errorSistema}</p>
              </div>
              <button
                onClick={() => ejecutarBusqueda(placaBusqueda, paginaActualHistorial)}
                style={btnReintentarStyle}
              >
                <RotateCcw size={14} /> Reintentar
              </button>
            </div>
          )}

          {!vehiculo && !cargando && !errorSistema && (
            <FichaVehicularEmptyState
              buscado={buscado}
              placaBusqueda={placaBusqueda}
              puedeCrear={puedeCrear}
              onEjecutarBusqueda={(placa) => {
                setPlacaBusqueda(placa);
                ejecutarBusqueda(placa, 1);
              }}
              onAbrirCrear={() => {
                setModoModalVehiculo('CREAR');
                setHayCambiosSinGuardar(true);
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
                  accionesVehiculo={
                    <>
                      {puedeCrearOT && (
                        <div className="fv-create-order">
                          <button onClick={handleAbrirCrearOTDirecta} style={btnCrearOTStyle}>
                            + Crear OT
                          </button>
                        </div>
                      )}
                    </>
                  }
                  vehiculo={vehiculo}
                  ultimoServicio={ultimoServicio}
                  ultimoCambioAceite={ultimoCambioAceite}
                  puedeEditar={puedeEditar}
                  puedeEliminar={puedeEliminar}
                  onAbrirEditar={() => {
                    setModoModalVehiculo('EDITAR');
                    setHayCambiosSinGuardar(true);
                    setModalVehiculoAbierto(true);
                  }}
                  onAbrirEliminar={() => setConfirmModalEliminar(true)}
                  onAbrirReactivar={() => setConfirmModalReactivar(true)}
                  onAbrirModalCliente={handleAbrirModalCliente}
                />
              </div>

              <FichaVehicularHistorial
                historial={historial as any}
                totalRegistros={totalRegistrosHistorial}
                paginaActual={paginaActualHistorial}
                totalPaginas={totalPaginasHistorial}
                cargando={cargando}
                puedeCorregirOT={esAdmin}
                onVerDetalleOT={(ot) => setOtSeleccionada(ot as any)}
                onCorregirOT={(ot) => {
                  setOtCorregirSeleccionada(ot as any);
                  setHayCambiosSinGuardar(true);
                }}
                onCambiarPagina={(nuevaPagina) => {
                  ejecutarBusqueda(vehiculo.placa, nuevaPagina);
                }}
              />
            </>
          )}
        </>
      )}

      {modalVehiculoAbierto && (
        <ModalVehiculo
          modalAbierto={modalVehiculoAbierto}
          setModalAbierto={(abierto) => {
            setModalVehiculoAbierto(abierto);
            if (!abierto) setHayCambiosSinGuardar(false);
          }}
          modo={modoModalVehiculo}
          vehiculoEditar={vehiculo}
          handleGuardar={handleGuardarVehiculo}
          esAdmin={esAdmin}
        />
      )}

      {otSeleccionada && (
        <ModalDetalleOT
          modalAbierto={!!otSeleccionada}
          setModalAbierto={(abierto) => {
            if (!abierto) setOtSeleccionada(null);
          }}
          ordenTrabajo={otSeleccionada}
        />
      )}

      {otCorregirSeleccionada && (
        <ModalCorregirHistorialOT
          modalAbierto={!!otCorregirSeleccionada}
          setModalAbierto={(abierto) => {
            if (!abierto) {
              setOtCorregirSeleccionada(null);
              setHayCambiosSinGuardar(false);
            }
          }}
          ordenTrabajo={otCorregirSeleccionada as any}
          handleGuardar={handleCorregirHistorialOT}
        />
      )}

      {modalClienteAbierto && (
        <ModalCliente
          modalAbierto={modalClienteAbierto}
          setModalAbierto={(abierto) => {
            setModalClienteAbierto(abierto);
            if (!abierto) setHayCambiosSinGuardar(false);
          }}
          modo={modoModalCliente}
          clienteEditar={clienteEditar}
          onClienteGuardado={handleGuardarCliente}
        />
      )}

      {modalCrearOTAbierto && vehiculo && (
        <ModalCrearOT
          idMecanicoDefault={0}
          placaInicial={vehiculo.placa}
          vehiculoDirecto={vehiculo}
          onClose={() => {
            setModalCrearOTAbierto(false);
            setHayCambiosSinGuardar(false);
          }}
          onSubmit={handleCrearOT}
        />
      )}

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

const errorBannerStyle: React.CSSProperties = {
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'space-between',
  gap: '12px',
  padding: '14px 18px',
  backgroundColor: 'rgba(239, 68, 68, 0.12)',
  border: '1px solid rgba(239, 68, 68, 0.35)',
  borderRadius: '10px',
  color: '#FCA5A5',
  fontSize: '14px',
  marginBottom: '16px',
};

const btnReintentarStyle: React.CSSProperties = {
  display: 'flex',
  alignItems: 'center',
  gap: '6px',
  padding: '8px 14px',
  backgroundColor: '#DC2626',
  color: '#FFFFFF',
  border: 'none',
  borderRadius: '6px',
  fontSize: '12px',
  fontWeight: '600',
  cursor: 'pointer',
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
  boxShadow: 'none',
};
