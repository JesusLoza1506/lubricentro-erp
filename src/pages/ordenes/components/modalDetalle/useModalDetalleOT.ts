import { useState, useEffect, useCallback } from 'react';
import { useNotificarOT, mensajeErrorOT } from '../../AvisosOT';
import {
  OrdenTrabajoHistorialDTO,
  TipoAceiteOT,
  TableroZanjasDTO,
} from '../../../../types/ordenTrabajo';
import { OrdenTrabajoHistorialDTO as VehiculoOTDTO } from '../../../../types/vehiculo';
import { useAuth } from '../../../../context/AuthContext';
import {
  ordenesService,
  MecanicoDTO,
  ServicioResumenDTO,
  ProductoInsumoDTO,
} from '../../../../services/ordenesService';

export interface UseModalDetalleOTProps {
  ordenTrabajo?: OrdenTrabajoHistorialDTO | VehiculoOTDTO;
  ot?: OrdenTrabajoHistorialDTO | VehiculoOTDTO;
  onIniciarZanja?: (idOt: number, idZanja: number, idMecanico?: number) => Promise<void>;
  onFinalizarOT?: (
    idOt: number,
    kmSalida: number,
    resumen: string,
    tipoAceite?: TipoAceiteOT
  ) => Promise<void>;
  onCancelarOT?: (idOt: number, motivo: string) => Promise<void>;
  onReasignarMecanico?: (idOt: number, nuevoIdMecanico: number, motivo: string) => Promise<void>;
  onActualizarOT?: () => Promise<void>;
}

export const useModalDetalleOT = ({
  ordenTrabajo,
  ot,
  onIniciarZanja,
  onFinalizarOT,
  onCancelarOT,
  onReasignarMecanico,
  onActualizarOT,
}: UseModalDetalleOTProps) => {
  const { usuario } = useAuth();
  const notificar = useNotificarOT();
  const otInicial = (ordenTrabajo || ot) as OrdenTrabajoHistorialDTO;

  const [ordenActual, setOrdenActual] = useState<OrdenTrabajoHistorialDTO>(
    otInicial || {
      id_ot: 0,
      codigo_ot: '',
      placa: '',
      estado: 'EN_ESPERA',
      kilometraje_ingreso: 0,
      fecha_ingreso: '',
      servicios: [],
      insumos: [],
    }
  );

  const [loading, setLoading] = useState(false);
  const [errorMsg, setErrorMsgState] = useState('');
  const setErrorMsg = useCallback((mensaje: string) => {
    setErrorMsgState(mensaje);
    if (mensaje) notificar(mensaje, 'ERROR');
  }, [notificar]);

  const [modoAccion, setModoAccion] = useState<
    | 'NINGUNO'
    | 'INICIAR'
    | 'FINALIZAR'
    | 'CANCELAR'
    | 'REASIGNAR'
    | 'DEVOLVER'
    | 'AGREGAR_SERVICIO'
    | 'EDITAR_SERVICIO'
    | 'AGREGAR_INSUMO'
  >('NINGUNO');

  const [tablero, setTablero] = useState<TableroZanjasDTO | null>(null);
  const [idZanjaSel, setIdZanjaSel] = useState<number>(1);
  const [idMecanicoInicio, setIdMecanicoInicio] = useState<number>(0);
  const [kmSalida, setKmSalida] = useState<number>(otInicial?.kilometraje_ingreso ?? 0);
  const [resumenPublico, setResumenPublico] = useState<string>('');
  const [tipoAceite, setTipoAceite] = useState<TipoAceiteOT | ''>('');

  const [motivoAccion, setMotivoAccion] = useState<string>('');
  const [idMecanicoNuevo, setIdMecanicoNuevo] = useState<number>(0);
  const [idDetalleDevolver, setIdDetalleDevolver] = useState<number>(0);
  const [cantDevolver, setCantDevolver] = useState<number>(1);

  // Formulario Servicios
  const [serviciosCatalogo, setServiciosCatalogo] = useState<ServicioResumenDTO[]>([]);
  const [idServicioSel, setIdServicioSel] = useState<number>(0);
  const [cantServicioSel, setCantServicioSel] = useState<number>(1);
  const [idDetalleServicioEdit, setIdDetalleServicioEdit] = useState<number>(0);

  // Formulario Insumos
  const [productosCatalogo, setProductosCatalogo] = useState<ProductoInsumoDTO[]>([]);
  const [idProductoSel, setIdProductoSel] = useState<number>(0);
  const [cantProductoSel, setCantProductoSel] = useState<number>(1);

  const [mecanicos, setMecanicos] = useState<MecanicoDTO[]>([]);

  const rol = (usuario?.rol || 'CAJERO').toUpperCase();
  const idUsuario = (usuario as any)?.id_usuario || (usuario as any)?.id || 0;
  const esTerminal = ordenActual.estado === 'FINALIZADO' || ordenActual.estado === 'CANCELADO';

  const esMecanicoAsignado =
    (rol === 'MECANICO' || rol === 'TECNICO') && ordenActual.id_mecanico === idUsuario;

  const puedeAgregarServicios =
    !esTerminal &&
    (rol === 'ADMINISTRADOR' ||
      (esMecanicoAsignado &&
        (ordenActual.estado === 'EN_ESPERA' || ordenActual.estado === 'EN_PROCESO')));

  const productoSeleccionado = (productosCatalogo || []).find(
    (p) => p.id_producto === idProductoSel
  );
  const esGranelSel =
    productoSeleccionado?.unidad_medida?.toUpperCase() === 'ML' ||
    productoSeleccionado?.categoria?.toUpperCase() === 'GRANEL';

  useEffect(() => {
    if (otInicial) {
      setOrdenActual(otInicial);
      setKmSalida(otInicial.kilometraje_ingreso ?? 0);

      if (otInicial.servicios && otInicial.servicios.length > 0) {
        const listaServiciosStr = otInicial.servicios.map((s) => s.nombre_snapshot).join(', ');
        setResumenPublico(`Servicio técnico completado: ${listaServiciosStr}.`);
      } else {
        setResumenPublico('Servicio técnico completado satisfactoriamente.');
      }
    }
  }, [ordenTrabajo, ot]);

  useEffect(() => {
    if (!esTerminal) {
      Promise.resolve(ordenesService.listarMecanicos?.())
        .then((res) => setMecanicos(res || []))
        .catch((err: unknown) => setErrorMsg(mensajeErrorOT(err, 'No se pudo cargar el catálogo de mecánicos.')));

      Promise.resolve(ordenesService.listarServicios?.())
        .then((res) => setServiciosCatalogo(res || []))
        .catch((err: unknown) => setErrorMsg(mensajeErrorOT(err, 'No se pudo cargar el catálogo de servicios.')));

      Promise.resolve(ordenesService.listarProductosInsumo?.())
        .then((res) => setProductosCatalogo(res || []))
        .catch((err: unknown) => setErrorMsg(mensajeErrorOT(err, 'No se pudo cargar el catálogo de insumos.')));

      if (ordenActual.estado === 'EN_ESPERA') {
        Promise.resolve(ordenesService.obtenerTableroZanjas?.(idUsuario, rol))
          .then((res) => setTablero(res || null))
          .catch((err: unknown) => setErrorMsg(mensajeErrorOT(err, 'No se pudo consultar la disponibilidad de zanjas.')));
      }
    }
  }, [esTerminal, ordenActual.estado, idUsuario, rol]);

  const refrescarTrasGuardar = async () => {
    try {
      if (onActualizarOT) await onActualizarOT();
    } catch (err: unknown) {
      setErrorMsg(`La operación se guardó, pero no se pudo actualizar la vista. ${mensajeErrorOT(err, 'Vuelve a consultar la orden.')}`);
    }
  };

  const handleIniciarSubmit = async () => {
    if (!onIniciarZanja) return;
    try {
      setLoading(true);
      setErrorMsg('');
      await onIniciarZanja(
        ordenActual.id_ot,
        idZanjaSel,
        idMecanicoInicio > 0 ? idMecanicoInicio : undefined
      );
      setModoAccion('NINGUNO');
      await refrescarTrasGuardar();
    } catch (err: unknown) {
      setErrorMsg(
        typeof err === 'string' ? err : (err as any)?.message || 'Error al iniciar zanja'
      );
    } finally {
      setLoading(false);
    }
  };

  const handleFinalizarSubmit = async () => {
    if (!onFinalizarOT) return;
    try {
      setLoading(true);
      setErrorMsg('');
      await onFinalizarOT(
        ordenActual.id_ot,
        kmSalida,
        resumenPublico,
        tipoAceite !== '' ? tipoAceite : undefined
      );
      setModoAccion('NINGUNO');
      await refrescarTrasGuardar();
    } catch (err: unknown) {
      setErrorMsg(typeof err === 'string' ? err : (err as any)?.message || 'Error al finalizar OT');
    } finally {
      setLoading(false);
    }
  };

  const handleCancelarSubmit = async () => {
    if (!onCancelarOT) return;
    if (motivoAccion.trim().length < 5) {
      setErrorMsg('El motivo debe tener al menos 5 caracteres.');
      return;
    }
    try {
      setLoading(true);
      setErrorMsg('');
      await onCancelarOT(ordenActual.id_ot, motivoAccion);
      setModoAccion('NINGUNO');
      setMotivoAccion('');
      await refrescarTrasGuardar();
    } catch (err: unknown) {
      setErrorMsg(typeof err === 'string' ? err : (err as any)?.message || 'Error al cancelar OT');
    } finally {
      setLoading(false);
    }
  };

  const handleReasignarSubmit = async () => {
    if (!onReasignarMecanico) return;
    if (idMecanicoNuevo <= 0) {
      setErrorMsg('Seleccione un mecánico válido.');
      return;
    }
    if (motivoAccion.trim().length < 5) {
      setErrorMsg('El motivo debe tener al menos 5 caracteres.');
      return;
    }
    try {
      setLoading(true);
      setErrorMsg('');
      await onReasignarMecanico(ordenActual.id_ot, idMecanicoNuevo, motivoAccion);
      setModoAccion('NINGUNO');
      setMotivoAccion('');
      await refrescarTrasGuardar();
    } catch (err: unknown) {
      setErrorMsg(
        typeof err === 'string' ? err : (err as any)?.message || 'Error al reasignar mecánico'
      );
    } finally {
      setLoading(false);
    }
  };

  const handleDevolverInsumoSubmit = async () => {
    if (cantDevolver <= 0) {
      setErrorMsg('Ingrese una cantidad válida a devolver.');
      return;
    }
    if (motivoAccion.trim().length < 5) {
      setErrorMsg('El motivo de devolución debe tener al menos 5 caracteres.');
      return;
    }

    try {
      setLoading(true);
      setErrorMsg('');

      const insumoADevolver = ordenActual.insumos.find((i) => i.id_detalle === idDetalleDevolver);
      const esGranelDevolucion = insumoADevolver?.unidad_medida?.toUpperCase() === 'ML';
      const cantidadADevolverFinal = esGranelDevolucion ? cantDevolver / 1000 : cantDevolver;

      // Generar UUID único de transacción para idempotencia en devolución
      const idOperacion = crypto.randomUUID();

      const act = await ordenesService.devolverInsumoOT(
        {
          id_ot: ordenActual.id_ot,
          id_detalle_producto: idDetalleDevolver,
          cantidad_a_devolver: cantidadADevolverFinal,
          motivo: motivoAccion,
          id_operacion: idOperacion,
        },
        idUsuario,
        rol
      );
      notificar('¡Devolución de insumo registrada correctamente!', 'EXITO');
      setOrdenActual(act);
      setModoAccion('NINGUNO');
      setMotivoAccion('');
      await refrescarTrasGuardar();
    } catch (err: unknown) {
      setErrorMsg(
        typeof err === 'string' ? err : (err as any)?.message || 'Error al devolver insumo'
      );
    } finally {
      setLoading(false);
    }
  };

  const handleAgregarServicioSubmit = async () => {
    if (idServicioSel <= 0) {
      setErrorMsg('Seleccione un servicio del catálogo.');
      return;
    }
    try {
      setLoading(true);
      setErrorMsg('');
      const act = await ordenesService.agregarServicioOT(
        { id_ot: ordenActual.id_ot, id_servicio: idServicioSel, cantidad: cantServicioSel },
        idUsuario,
        rol
      );
      notificar('¡Servicio agregado correctamente!', 'EXITO');
      setOrdenActual(act);
      setModoAccion('NINGUNO');
      setIdServicioSel(0);
      setCantServicioSel(1);
      await refrescarTrasGuardar();
    } catch (err: unknown) {
      setErrorMsg(
        typeof err === 'string' ? err : (err as any)?.message || 'Error al agregar servicio'
      );
    } finally {
      setLoading(false);
    }
  };

  const handleModificarServicioSubmit = async () => {
    try {
      setLoading(true);
      setErrorMsg('');
      const act = await ordenesService.modificarServicioOT(
        {
          id_ot: ordenActual.id_ot,
          id_detalle_servicio: idDetalleServicioEdit,
          nueva_cantidad: cantServicioSel,
        },
        idUsuario,
        rol
      );
      notificar('¡Servicio actualizado correctamente!', 'EXITO');
      setOrdenActual(act);
      setModoAccion('NINGUNO');
      await refrescarTrasGuardar();
    } catch (err: unknown) {
      setErrorMsg(
        typeof err === 'string' ? err : (err as any)?.message || 'Error al modificar servicio'
      );
    } finally {
      setLoading(false);
    }
  };

  const handleEliminarServicio = async (idDetalle: number) => {
    if (!window.confirm('¿Desea retirar este servicio de la Orden de Trabajo?')) return;
    try {
      setLoading(true);
      setErrorMsg('');
      const act = await ordenesService.eliminarServicioOT(
        idDetalle,
        ordenActual.id_ot,
        idUsuario,
        rol
      );
      notificar('¡Servicio retirado correctamente!', 'EXITO');
      setOrdenActual(act);
      await refrescarTrasGuardar();
    } catch (err: unknown) {
      setErrorMsg(
        typeof err === 'string' ? err : (err as any)?.message || 'Error al eliminar servicio'
      );
    } finally {
      setLoading(false);
    }
  };

  const handleAgregarInsumoSubmit = async () => {
    if (idProductoSel <= 0) {
      setErrorMsg('Seleccione un producto/insumo válido.');
      return;
    }
    if (cantProductoSel <= 0) {
      setErrorMsg('Ingrese una cantidad mayor a cero.');
      return;
    }
    try {
      setLoading(true);
      setErrorMsg('');

      const cantidadAEnviar = esGranelSel ? cantProductoSel / 1000 : cantProductoSel;

      // Generar UUID único de transacción para idempotencia en consumo
      const idOperacion = crypto.randomUUID();

      const act = await ordenesService.registrarInsumoOT(
        {
          id_ot: ordenActual.id_ot,
          id_producto: idProductoSel,
          cantidad: cantidadAEnviar,
          id_operacion: idOperacion,
        },
        idUsuario,
        rol
      );
      notificar('¡Despacho de insumo registrado correctamente!', 'EXITO');
      setOrdenActual(act);
      setModoAccion('NINGUNO');
      setIdProductoSel(0);
      setCantProductoSel(1);
      await refrescarTrasGuardar();
    } catch (err: unknown) {
      setErrorMsg(
        typeof err === 'string' ? err : (err as any)?.message || 'Error al registrar insumo'
      );
    } finally {
      setLoading(false);
    }
  };

  return {
    ordenActual,
    rol,
    loading,
    errorMsg,
    setErrorMsg,
    modoAccion,
    setModoAccion,
    esTerminal,
    puedeAgregarServicios,
    esGranelSel,
    mecanicos,
    serviciosCatalogo,
    productosCatalogo,
    tablero,
    idZanjaSel,
    setIdZanjaSel,
    idMecanicoInicio,
    setIdMecanicoInicio,
    kmSalida,
    setKmSalida,
    tipoAceite,
    setTipoAceite,
    resumenPublico,
    setResumenPublico,
    motivoAccion,
    setMotivoAccion,
    idMecanicoNuevo,
    setIdMecanicoNuevo,
    idServicioSel,
    setIdServicioSel,
    cantServicioSel,
    setCantServicioSel,
    setIdDetalleServicioEdit,
    idProductoSel,
    setIdProductoSel,
    cantProductoSel,
    setCantProductoSel,
    setIdDetalleDevolver,
    cantDevolver,
    setCantDevolver,
    handleIniciarSubmit,
    handleFinalizarSubmit,
    handleCancelarSubmit,
    handleReasignarSubmit,
    handleDevolverInsumoSubmit,
    handleAgregarServicioSubmit,
    handleModificarServicioSubmit,
    handleEliminarServicio,
    handleAgregarInsumoSubmit,
    hasIniciarHandler: Boolean(onIniciarZanja),
    hasFinalizarHandler: Boolean(onFinalizarOT),
    hasCancelarHandler: Boolean(onCancelarOT),
    hasReasignarHandler: Boolean(onReasignarMecanico),
  };
};
