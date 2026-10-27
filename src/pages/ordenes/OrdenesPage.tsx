import { AvisosOTContext, AvisoOTVisible, useAvisosOT, mensajeErrorOT } from './AvisosOT';
import './ordenesVisual.css';
import { LayoutGrid, ClipboardList, ArrowRight, Filter, ChevronLeft, ChevronRight } from 'lucide-react';
import React, { useEffect, useState } from 'react';
import { useAuth } from '../../context/AuthContext';
import { ordenesService, MecanicoDTO } from '../../services/ordenesService';
import {
  TableroZanjasDTO,
  OrdenTrabajoHistorialDTO,
  CrearOrdenTrabajoPayload,
  TipoAceiteOT,
  EstadoOT,
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
  const { aviso, notificar, cerrar } = useAvisosOT();
  const [errorTablero, setErrorTablero] = useState('');
  const [errorTabla, setErrorTabla] = useState('');
  const [errorMecanicos, setErrorMecanicos] = useState('');
  const [tablero, setTablero] = useState<TableroZanjasDTO | null>(null);
  const [loading, setLoading] = useState(true);
  const [modalCrearAbierto, setModalCrearAbierto] = useState(false);
  const [otSeleccionada, setOtSeleccionada] = useState<OrdenTrabajoHistorialDTO | null>(null);
  const [mecanicos, setMecanicos] = useState<MecanicoDTO[]>([]);

  // Formateador explícito en hora de Perú / Lima (America/Lima)
  const obtenerFechaHoyPeru = () => {
    const formatter = new Intl.DateTimeFormat('en-CA', {
      timeZone: 'America/Lima',
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
    });
    return formatter.format(new Date());
  };

  // Estados para Tabla Paginada
  const [vistaActiva, setVistaActiva] = useState<'TABLERO' | 'HISTORIAL_PAGINADO'>('TABLERO');
  const [subVistaHistorial, setSubVistaHistorial] = useState<'TODAS' | 'PROPIAS' | 'LIBRES'>(
    'TODAS'
  );
  const [paginaActual, setPaginaActual] = useState<number>(1);
  const [busquedaPlaca, setBusquedaPlaca] = useState<string>('');
  const [busquedaCodigoOT, setBusquedaCodigoOT] = useState<string>('');
  const [estadoFiltro, setEstadoFiltro] = useState<string>('');
  const [mecanicoFiltroId, setMecanicoFiltroId] = useState<number>(0);
  const [fechaInicio, setFechaInicio] = useState<string>(obtenerFechaHoyPeru());
  const [fechaFin, setFechaFin] = useState<string>(obtenerFechaHoyPeru());
  const [resultadoPaginado, setResultadoPaginado] = useState<any>(null);
  const [cargandoTabla, setCargandoTabla] = useState<boolean>(false);

  const rol = usuario?.rol || 'CAJERO';
  const idUsuario = (usuario as any)?.id_usuario || (usuario as any)?.id || 0;

  const puedeCrearOT = usuario?.rol !== 'MECANICO' && tienePermiso('OT', 'crear');

  const cargarTablero = async (operacionGuardada = false) => {
    try {
      setLoading(true);
      setErrorTablero('');
      if (!idUsuario) return;
      const data = await ordenesService.obtenerTableroZanjas(idUsuario, rol);
      setTablero(data);

      if (otSeleccionada) {
        const act = await ordenesService.obtenerOtPorId(otSeleccionada.id_ot, idUsuario, rol);
        setOtSeleccionada(act);
      }
    } catch (err) {
      const mensaje = `${operacionGuardada ? 'La operación se guardó, pero no se pudo actualizar el tablero.' : 'No se pudo cargar el tablero.'} ${mensajeErrorOT(err, 'Inténtalo nuevamente.')}`;
      setErrorTablero(mensaje);
      notificar(mensaje, 'ERROR');
    } finally {
      setLoading(false);
    }
  };

  const cargarTablaPaginada = async (pagina = 1, operacionGuardada = false) => {
    try {
      setCargandoTabla(true);
      setErrorTabla('');
      const res = await ordenesService.listarOrdenesTrabajoPaginado({
        pagina,
        limite: 20,
        placa: busquedaPlaca.trim() || undefined,
        codigo_ot: busquedaCodigoOT.trim() || undefined,
        estado: (estadoFiltro as EstadoOT) || undefined,
        id_mecanico: mecanicoFiltroId > 0 ? mecanicoFiltroId : undefined,
        fecha_inicio: fechaInicio || undefined,
        fecha_fin: fechaFin || undefined,
        solo_propias: subVistaHistorial === 'PROPIAS' ? true : undefined,
        solo_libres: subVistaHistorial === 'LIBRES' ? true : undefined,
      } as any);
      setResultadoPaginado(res);
      setPaginaActual(pagina);
    } catch (err) {
      const mensaje = `${operacionGuardada ? 'La operación se guardó, pero no se pudo actualizar el historial.' : 'No se pudo consultar el historial.'} ${mensajeErrorOT(err, 'Inténtalo nuevamente.')}`;
      setErrorTabla(mensaje);
      notificar(mensaje, 'ERROR');
    } finally {
      setCargandoTabla(false);
    }
  };

  const cargarMecanicos = async () => {
    setErrorMecanicos('');
    try {
      setMecanicos(await ordenesService.listarMecanicos());
    } catch (err: unknown) {
      const mensaje = `No se pudo cargar la lista de mecánicos. ${mensajeErrorOT(err, 'Inténtalo nuevamente.')}`;
      setErrorMecanicos(mensaje);
      notificar(mensaje, 'ERROR');
    }
  };

  useEffect(() => {
    cargarTablero();
    cargarMecanicos();
  }, [usuario]);

  useEffect(() => {
    if (vistaActiva === 'HISTORIAL_PAGINADO') {
      cargarTablaPaginada(1);
    }
  }, [vistaActiva, estadoFiltro, subVistaHistorial]);

  const handleCrearOT = async (payload: CrearOrdenTrabajoPayload) => {
    try {
      await ordenesService.crearOrdenTrabajo(payload, idUsuario, rol);
    } catch (err: unknown) {
      notificar(mensajeErrorOT(err, 'No se pudo crear la orden. Inténtalo nuevamente.'), 'ERROR');
      throw err;
    }
    notificar('¡Orden de trabajo creada correctamente!', 'EXITO');
    await cargarTablero(true);
    if (vistaActiva === 'HISTORIAL_PAGINADO') await cargarTablaPaginada(paginaActual, true);
  };

  const handleIniciarEnZanja = async (idOt: number, idZanja: number, idMecanico?: number) => {
    await ordenesService.iniciarTrabajoOT(
      { id_ot: idOt, id_zanja_ejecucion: idZanja, id_mecanico: idMecanico },
      idUsuario,
      rol
    );
    notificar('¡Atención iniciada en la zanja seleccionada!', 'EXITO');
    await cargarTablero(true);
    if (vistaActiva === 'HISTORIAL_PAGINADO') await cargarTablaPaginada(paginaActual, true);
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
    notificar('¡Orden de trabajo finalizada correctamente!', 'EXITO');
    await cargarTablero(true);
    if (vistaActiva === 'HISTORIAL_PAGINADO') await cargarTablaPaginada(paginaActual, true);
  };

  const handleCancelarOT = async (idOt: number, motivo: string) => {
    await ordenesService.cancelarOT({ id_ot: idOt, motivo }, idUsuario, rol);
    notificar('¡Orden de trabajo cancelada correctamente!', 'EXITO');
    await cargarTablero(true);
    if (vistaActiva === 'HISTORIAL_PAGINADO') await cargarTablaPaginada(paginaActual, true);
  };

  const handleReasignarMecanico = async (idOt: number, nuevoIdMecanico: number, motivo: string) => {
    await ordenesService.reasignarMecanicoOT(
      { id_ot: idOt, nuevo_id_mecanico: nuevoIdMecanico, motivo },
      idUsuario,
      rol
    );
    notificar('¡Mecánico reasignado correctamente!', 'EXITO');
    await cargarTablero(true);
    if (vistaActiva === 'HISTORIAL_PAGINADO') await cargarTablaPaginada(paginaActual, true);
  };

  return (
    <AvisosOTContext.Provider value={notificar}>
    <div className="ot-page" style={styles.container}>
      <AvisoOTVisible aviso={aviso} onCerrar={cerrar} />
      <header className="ot-page-header">
        <div><span className="ot-eyebrow">Gestión del taller</span><h1>Órdenes de trabajo / Taller</h1><p>Organiza las atenciones y consulta los trabajos del taller.</p></div>
        {puedeCrearOT && <button className="ot-button ot-primary" onClick={() => setModalCrearAbierto(true)}>+ Recepcionar vehículo</button>}
      </header>
      <nav className="ot-tabs" aria-label="Vistas de órdenes de trabajo">
        <button className="ot-tab" aria-current={vistaActiva === 'TABLERO' ? 'page' : undefined} onClick={() => setVistaActiva('TABLERO')}><LayoutGrid size={16} />Taller</button>
        <button className="ot-tab" aria-current={vistaActiva === 'HISTORIAL_PAGINADO' ? 'page' : undefined} onClick={() => setVistaActiva('HISTORIAL_PAGINADO')}><ClipboardList size={16} />Historial de órdenes</button>
      </nav>
      {errorMecanicos && <div className="ot-feedback-error" role="alert"><span>{errorMecanicos}</span><button className="ot-button ot-small" onClick={() => void cargarMecanicos()}>Reintentar mecánicos</button></div>}
      {vistaActiva === 'TABLERO' && errorTablero && <div className="ot-feedback-error" role="alert"><span>{errorTablero}</span><button className="ot-button ot-small" onClick={() => void cargarTablero()}>Reintentar tablero</button></div>}
      {vistaActiva === 'TABLERO' ? (
        loading ? <div className="ot-empty" role="status">Cargando tablero en tiempo real...</div> : !tablero ? <div className="ot-empty">{errorTablero ? 'El tablero no está disponible. Reintenta la consulta.' : 'Cargando tablero en tiempo real...'}</div> : (
          <TableroZanjas tablero={tablero} onVerDetalle={(ot) => setOtSeleccionada(ot)} onIniciarEnZanja={(idOt, idZanja) => {
              // Esta acción directa no tiene un formulario que capture el rechazo.
              void handleIniciarEnZanja(idOt, idZanja).catch((err: unknown) => {
                notificar(mensajeErrorOT(err, 'No se pudo iniciar la atención. Inténtalo nuevamente.'), 'ERROR');
              });
            }} />
        )
      ) : (
        <section className="ot-history" aria-label="Historial de órdenes">
          <div className="ot-history-toolbar">
            <div className="ot-segments" aria-label="Grupo de órdenes">
              <button aria-pressed={subVistaHistorial === 'TODAS'} onClick={() => setSubVistaHistorial('TODAS')}>Todas las órdenes</button>
              <button aria-pressed={subVistaHistorial === 'PROPIAS'} onClick={() => setSubVistaHistorial('PROPIAS')}>Mis órdenes</button>
              <button aria-pressed={subVistaHistorial === 'LIBRES'} onClick={() => setSubVistaHistorial('LIBRES')}>Libres en espera</button>
            </div>
            <span className="ot-muted">20 registros por página</span>
          </div>
          <div className="ot-filter-panel">
            <div className="ot-filter-main">
              <label className="ot-field"><span>Placa</span><input type="text" placeholder="Buscar por placa..." value={busquedaPlaca} onChange={(e) => setBusquedaPlaca(e.target.value)} onKeyDown={(e) => e.key === 'Enter' && cargarTablaPaginada(1)} /></label>
              <label className="ot-field"><span>Código de orden</span><input type="text" placeholder="Código OT..." value={busquedaCodigoOT} onChange={(e) => setBusquedaCodigoOT(e.target.value)} onKeyDown={(e) => e.key === 'Enter' && cargarTablaPaginada(1)} /></label>
              <label className="ot-field"><span>Estado</span><select value={estadoFiltro} onChange={(e) => setEstadoFiltro(e.target.value)}><option value="">Todos los estados</option><option value="EN_ESPERA">En espera</option><option value="EN_PROCESO">En proceso</option><option value="FINALIZADO">Finalizada</option><option value="CANCELADO">Cancelada</option></select></label>
              <label className="ot-field"><span>Mecánico / responsable</span><select value={mecanicoFiltroId} onChange={(e) => setMecanicoFiltroId(Number(e.target.value))}><option value={0}>Todos los responsables</option>{mecanicos.map((m) => <option key={m.id_usuario} value={m.id_usuario}>{m.nombre_completo}</option>)}</select></label>
            </div>
            <div className="ot-filter-dates">
              <label className="ot-field"><span>Desde</span><input type="date" value={fechaInicio} onChange={(e) => setFechaInicio(e.target.value)} /></label>
              <label className="ot-field"><span>Hasta</span><input type="date" value={fechaFin} onChange={(e) => setFechaFin(e.target.value)} /></label>
              <button className="ot-button ot-filter-submit" onClick={() => cargarTablaPaginada(1)}><Filter size={15} />Aplicar filtros</button>
            </div>
          </div>
          {errorTabla && <div className="ot-feedback-error" role="alert"><span>{errorTabla}</span><button className="ot-button ot-small" onClick={() => void cargarTablaPaginada(paginaActual)}>Reintentar historial</button></div>}
          {cargandoTabla ? <div className="ot-empty" role="status">Cargando historial de órdenes...</div> : (
            <>
              <div className="ot-table-scroll" tabIndex={0} role="region" aria-label="Resultados de órdenes">
                <table className="ot-table ot-history-table">
                  <thead><tr><th scope="col">Código OT</th><th scope="col">Vehículo</th><th scope="col">Fecha de ingreso</th><th scope="col">Estado</th><th scope="col" className="ot-numeric">Total (S/)</th><th scope="col">Consultar</th></tr></thead>
                  <tbody>{resultadoPaginado?.items && resultadoPaginado.items.length > 0 ? resultadoPaginado.items.map((otItem: OrdenTrabajoHistorialDTO) => (
                    <tr key={otItem.id_ot}>
                      <td><span className="ot-order-code">{otItem.codigo_ot}</span></td>
                      <td><strong className="ot-table-plate">{otItem.placa}</strong></td>
                      <td><span className="ot-date">{formatearIngreso(otItem.fecha_ingreso)}</span></td>
                      <td><span className="ot-status" data-state={otItem.estado}>{etiquetaEstado(otItem.estado)}</span></td>
                      <td className="ot-numeric">S/ {(otItem.total_general_soles ?? 0).toFixed(2)}</td>
                      <td><button className="ot-button ot-small ot-open" onClick={() => setOtSeleccionada(otItem)}>Ver detalle<ArrowRight size={14} /></button></td>
                    </tr>
                  )) : <tr><td colSpan={6} className="ot-empty-cell">{errorTabla ? 'No se pudo completar la consulta.' : 'No se encontraron órdenes de trabajo.'}</td></tr>}</tbody>
                </table>
              </div>
              {resultadoPaginado && (resultadoPaginado.total_paginas ?? resultadoPaginado.totalPaginas ?? 1) > 0 && (
                <div className="ot-pagination">
                  <span>Página {resultadoPaginado.pagina_actual ?? resultadoPaginado.paginaActual ?? 1} de {resultadoPaginado.total_paginas ?? resultadoPaginado.totalPaginas ?? 1} · Total: {resultadoPaginado.total_items ?? resultadoPaginado.totalItems ?? resultadoPaginado.total_registros ?? 0}</span>
                  <div><button className="ot-button ot-small" disabled={paginaActual <= 1} onClick={() => cargarTablaPaginada(paginaActual - 1)}><ChevronLeft size={15} />Anterior</button><button className="ot-button ot-small" disabled={paginaActual >= (resultadoPaginado.total_paginas ?? resultadoPaginado.totalPaginas ?? 1)} onClick={() => cargarTablaPaginada(paginaActual + 1)}>Siguiente<ChevronRight size={15} /></button></div>
                </div>
              )}
            </>
          )}
        </section>
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
          onActualizarOT={async () => {
            await cargarTablero(true);
            if (vistaActiva === 'HISTORIAL_PAGINADO') await cargarTablaPaginada(paginaActual, true);
          }}
        />
      )}
    </div>
    </AvisosOTContext.Provider>
  );
};

const etiquetaEstado = (estado: string) => ({
  EN_ESPERA: 'En espera', EN_PROCESO: 'En proceso', FINALIZADO: 'Finalizada', CANCELADO: 'Cancelada',
}[estado] || estado);

const formatearIngreso = (valor: string) => {
  // SQL sin zona: conservar la hora local recibida. ISO con zona: presentar en Lima.
  const local = /^(\d{4})-(\d{2})-(\d{2})[T ](\d{2}:\d{2})(?::\d{2}(?:\.\d+)?)?$/.exec(valor);
  if (local) return `${local[3]}/${local[2]}/${local[1]} · ${local[4]}`;
  const fecha = new Date(valor);
  return Number.isNaN(fecha.getTime()) ? valor : new Intl.DateTimeFormat('es-PE', { timeZone: 'America/Lima', day: '2-digit', month: '2-digit', year: 'numeric', hour: '2-digit', minute: '2-digit', hour12: false }).format(fecha);
};
