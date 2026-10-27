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

  const cargarTablaPaginada = async (pagina = 1) => {
    try {
      setCargandoTabla(true);
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
      console.error('Error al obtener lista paginada de OTs:', err);
    } finally {
      setCargandoTabla(false);
    }
  };

  useEffect(() => {
    cargarTablero();
    ordenesService.listarMecanicos().then(setMecanicos).catch(console.error);
  }, [usuario]);

  useEffect(() => {
    if (vistaActiva === 'HISTORIAL_PAGINADO') {
      cargarTablaPaginada(1);
    }
  }, [vistaActiva, estadoFiltro, subVistaHistorial]);

  const handleCrearOT = async (payload: CrearOrdenTrabajoPayload) => {
    await ordenesService.crearOrdenTrabajo(payload, idUsuario, rol);
    await cargarTablero();
    if (vistaActiva === 'HISTORIAL_PAGINADO') await cargarTablaPaginada(paginaActual);
  };

  const handleIniciarEnZanja = async (idOt: number, idZanja: number, idMecanico?: number) => {
    await ordenesService.iniciarTrabajoOT(
      { id_ot: idOt, id_zanja_ejecucion: idZanja, id_mecanico: idMecanico },
      idUsuario,
      rol
    );
    await cargarTablero();
    if (vistaActiva === 'HISTORIAL_PAGINADO') await cargarTablaPaginada(paginaActual);
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
    if (vistaActiva === 'HISTORIAL_PAGINADO') await cargarTablaPaginada(paginaActual);
  };

  const handleCancelarOT = async (idOt: number, motivo: string) => {
    await ordenesService.cancelarOT({ id_ot: idOt, motivo }, idUsuario, rol);
    await cargarTablero();
    if (vistaActiva === 'HISTORIAL_PAGINADO') await cargarTablaPaginada(paginaActual);
  };

  const handleReasignarMecanico = async (idOt: number, nuevoIdMecanico: number, motivo: string) => {
    await ordenesService.reasignarMecanicoOT(
      { id_ot: idOt, nuevo_id_mecanico: nuevoIdMecanico, motivo },
      idUsuario,
      rol
    );
    await cargarTablero();
    if (vistaActiva === 'HISTORIAL_PAGINADO') await cargarTablaPaginada(paginaActual);
  };

  return (
    <div style={styles.container}>
      <div style={styles.header}>
        <div>
          <h1 style={styles.title}>Órdenes de Trabajo / Taller v7.0</h1>
          <p style={styles.subtitle}>
            Gestión transaccional de zanjas 1 a 6, cola de espera e historial paginado
          </p>
        </div>
        <div style={{ display: 'flex', gap: '8px' }}>
          <button
            onClick={() => setVistaActiva('TABLERO')}
            style={{
              padding: '8px 14px',
              backgroundColor: vistaActiva === 'TABLERO' ? '#2563EB' : '#334155',
              color: '#FFF',
              border: 'none',
              borderRadius: '6px',
              fontWeight: 'bold',
              cursor: 'pointer',
            }}
          >
            📊 Tablero Zanjas
          </button>
          <button
            onClick={() => setVistaActiva('HISTORIAL_PAGINADO')}
            style={{
              padding: '8px 14px',
              backgroundColor: vistaActiva === 'HISTORIAL_PAGINADO' ? '#2563EB' : '#334155',
              color: '#FFF',
              border: 'none',
              borderRadius: '6px',
              fontWeight: 'bold',
              cursor: 'pointer',
            }}
          >
            📋 Historial Completo
          </button>

          {puedeCrearOT && (
            <button style={styles.btnNuevaOT} onClick={() => setModalCrearAbierto(true)}>
              + Recepcionar Vehículo
            </button>
          )}
        </div>
      </div>

      {vistaActiva === 'TABLERO' ? (
        loading || !tablero ? (
          <div style={{ padding: '32px', textAlign: 'center', color: '#94A3B8' }}>
            Cargando tablero en tiempo real...
          </div>
        ) : (
          <TableroZanjas
            tablero={tablero}
            onVerDetalle={(ot) => setOtSeleccionada(ot)}
            onIniciarEnZanja={handleIniciarEnZanja}
          />
        )
      ) : (
        <div style={{ backgroundColor: '#1E293B', borderRadius: '8px', padding: '16px' }}>
          {/* Pestañas de OTs Propias vs Libres vs Todas */}
          <div style={{ display: 'flex', gap: '8px', marginBottom: '16px' }}>
            <button
              onClick={() => setSubVistaHistorial('TODAS')}
              style={{
                padding: '6px 14px',
                backgroundColor: subVistaHistorial === 'TODAS' ? '#0284C7' : '#334155',
                color: '#FFF',
                border: 'none',
                borderRadius: '6px',
                fontWeight: '600',
                cursor: 'pointer',
                fontSize: '12px',
              }}
            >
              Todas las OTs
            </button>
            <button
              onClick={() => setSubVistaHistorial('PROPIAS')}
              style={{
                padding: '6px 14px',
                backgroundColor: subVistaHistorial === 'PROPIAS' ? '#0284C7' : '#334155',
                color: '#FFF',
                border: 'none',
                borderRadius: '6px',
                fontWeight: '600',
                cursor: 'pointer',
                fontSize: '12px',
              }}
            >
              👤 OTs Propias
            </button>
            <button
              onClick={() => setSubVistaHistorial('LIBRES')}
              style={{
                padding: '6px 14px',
                backgroundColor: subVistaHistorial === 'LIBRES' ? '#0284C7' : '#334155',
                color: '#FFF',
                border: 'none',
                borderRadius: '6px',
                fontWeight: '600',
                cursor: 'pointer',
                fontSize: '12px',
              }}
            >
              🔓 OTs Libres (En Espera)
            </button>
          </div>

          {/* Filtros de Búsqueda */}
          <div
            style={{
              display: 'flex',
              gap: '12px',
              marginBottom: '16px',
              flexWrap: 'wrap',
              alignItems: 'center',
            }}
          >
            <input
              type="text"
              placeholder="Buscar por placa..."
              value={busquedaPlaca}
              onChange={(e) => setBusquedaPlaca(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && cargarTablaPaginada(1)}
              style={{
                padding: '8px 12px',
                backgroundColor: '#0F172A',
                border: '1px solid #334155',
                borderRadius: '6px',
                color: '#FFF',
                width: '160px',
              }}
            />
            <input
              type="text"
              placeholder="Código OT..."
              value={busquedaCodigoOT}
              onChange={(e) => setBusquedaCodigoOT(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && cargarTablaPaginada(1)}
              style={{
                padding: '8px 12px',
                backgroundColor: '#0F172A',
                border: '1px solid #334155',
                borderRadius: '6px',
                color: '#FFF',
                width: '140px',
              }}
            />
            <select
              value={estadoFiltro}
              onChange={(e) => setEstadoFiltro(e.target.value)}
              style={{
                padding: '8px 12px',
                backgroundColor: '#0F172A',
                border: '1px solid #334155',
                borderRadius: '6px',
                color: '#FFF',
              }}
            >
              <option value="">-- Todos los Estados --</option>
              <option value="EN_ESPERA">EN_ESPERA</option>
              <option value="EN_PROCESO">EN_PROCESO</option>
              <option value="FINALIZADO">FINALIZADO</option>
              <option value="CANCELADO">CANCELADO</option>
            </select>

            <select
              value={mecanicoFiltroId}
              onChange={(e) => setMecanicoFiltroId(Number(e.target.value))}
              style={{
                padding: '8px 12px',
                backgroundColor: '#0F172A',
                border: '1px solid #334155',
                borderRadius: '6px',
                color: '#FFF',
              }}
            >
              <option value={0}>-- Todos los Mecánicos / Responsables --</option>
              {mecanicos.map((m) => (
                <option key={m.id_usuario} value={m.id_usuario}>
                  {m.nombre_completo}
                </option>
              ))}
            </select>

            <div
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: '6px',
                color: '#94A3B8',
                fontSize: '13px',
              }}
            >
              <span>Desde:</span>
              <input
                type="date"
                value={fechaInicio}
                onChange={(e) => setFechaInicio(e.target.value)}
                style={{
                  padding: '7px',
                  backgroundColor: '#0F172A',
                  border: '1px solid #334155',
                  borderRadius: '6px',
                  color: '#FFF',
                }}
              />
              <span>Hasta:</span>
              <input
                type="date"
                value={fechaFin}
                onChange={(e) => setFechaFin(e.target.value)}
                style={{
                  padding: '7px',
                  backgroundColor: '#0F172A',
                  border: '1px solid #334155',
                  borderRadius: '6px',
                  color: '#FFF',
                }}
              />
            </div>

            <button
              onClick={() => cargarTablaPaginada(1)}
              style={{
                padding: '8px 16px',
                backgroundColor: '#0284C7',
                color: '#FFF',
                border: 'none',
                borderRadius: '6px',
                cursor: 'pointer',
              }}
            >
              Filtrar
            </button>
          </div>

          {cargandoTabla ? (
            <div style={{ padding: '24px', textAlign: 'center', color: '#94A3B8' }}>
              Cargando historial de órdenes...
            </div>
          ) : (
            <>
              <table
                style={{
                  width: '100%',
                  borderCollapse: 'collapse',
                  fontSize: '13px',
                  color: '#F8FAFC',
                }}
              >
                <thead>
                  <tr
                    style={{
                      borderBottom: '1px solid #334155',
                      color: '#94A3B8',
                      textAlign: 'left',
                    }}
                  >
                    <th style={{ padding: '8px' }}>Código OT</th>
                    <th style={{ padding: '8px' }}>Placa</th>
                    <th style={{ padding: '8px' }}>Fecha Ingreso</th>
                    <th style={{ padding: '8px' }}>Estado</th>
                    <th style={{ padding: '8px' }}>Total (S/)</th>
                    <th style={{ padding: '8px', textAlign: 'center' }}>Acciones</th>
                  </tr>
                </thead>
                <tbody>
                  {resultadoPaginado?.items && resultadoPaginado.items.length > 0 ? (
                    resultadoPaginado.items.map((otItem: OrdenTrabajoHistorialDTO) => (
                      <tr key={otItem.id_ot} style={{ borderBottom: '1px solid #0F172A' }}>
                        <td style={{ padding: '8px', fontWeight: 'bold', color: '#38BDF8' }}>
                          {otItem.codigo_ot}
                        </td>
                        <td style={{ padding: '8px' }}>{otItem.placa}</td>
                        <td style={{ padding: '8px' }}>{otItem.fecha_ingreso}</td>
                        <td style={{ padding: '8px' }}>
                          <span
                            style={{
                              padding: '2px 8px',
                              borderRadius: '4px',
                              fontSize: '11px',
                              fontWeight: 'bold',
                              backgroundColor:
                                otItem.estado === 'FINALIZADO'
                                  ? '#16A34A'
                                  : otItem.estado === 'EN_PROCESO'
                                    ? '#2563EB'
                                    : otItem.estado === 'CANCELADO'
                                      ? '#DC2626'
                                      : '#D97706',
                            }}
                          >
                            {otItem.estado}
                          </span>
                        </td>
                        <td style={{ padding: '8px' }}>
                          S/ {(otItem.total_general_soles ?? 0).toFixed(2)}
                        </td>
                        <td style={{ padding: '8px', textAlign: 'center' }}>
                          <button
                            onClick={() => setOtSeleccionada(otItem)}
                            style={{
                              padding: '4px 10px',
                              backgroundColor: '#0284C7',
                              color: '#FFF',
                              border: 'none',
                              borderRadius: '4px',
                              cursor: 'pointer',
                              fontSize: '11px',
                            }}
                          >
                            Ver Detalle
                          </button>
                        </td>
                      </tr>
                    ))
                  ) : (
                    <tr>
                      <td
                        colSpan={6}
                        style={{ padding: '16px', textAlign: 'center', color: '#94A3B8' }}
                      >
                        No se encontraron órdenes de trabajo.
                      </td>
                    </tr>
                  )}
                </tbody>
              </table>

              {resultadoPaginado &&
                (resultadoPaginado.total_paginas ?? resultadoPaginado.totalPaginas ?? 1) > 0 && (
                  <div
                    style={{
                      display: 'flex',
                      justifyContent: 'space-between',
                      alignItems: 'center',
                      marginTop: '16px',
                      color: '#94A3B8',
                      fontSize: '13px',
                    }}
                  >
                    <span>
                      Página{' '}
                      {resultadoPaginado.pagina_actual ?? resultadoPaginado.paginaActual ?? 1} de{' '}
                      {resultadoPaginado.total_paginas ?? resultadoPaginado.totalPaginas ?? 1}{' '}
                      (Total:{' '}
                      {resultadoPaginado.total_items ??
                        resultadoPaginado.totalItems ??
                        resultadoPaginado.total_registros ??
                        0}
                      )
                    </span>
                    <div style={{ display: 'flex', gap: '8px' }}>
                      <button
                        disabled={paginaActual <= 1}
                        onClick={() => cargarTablaPaginada(paginaActual - 1)}
                        style={{
                          padding: '4px 10px',
                          backgroundColor: paginaActual <= 1 ? '#334155' : '#2563EB',
                          color: '#FFF',
                          border: 'none',
                          borderRadius: '4px',
                          cursor: paginaActual <= 1 ? 'not-allowed' : 'pointer',
                        }}
                      >
                        Anterior
                      </button>
                      <button
                        disabled={
                          paginaActual >=
                          (resultadoPaginado.total_paginas ?? resultadoPaginado.totalPaginas ?? 1)
                        }
                        onClick={() => cargarTablaPaginada(paginaActual + 1)}
                        style={{
                          padding: '4px 10px',
                          backgroundColor:
                            paginaActual >=
                            (resultadoPaginado.total_paginas ?? resultadoPaginado.totalPaginas ?? 1)
                              ? '#334155'
                              : '#2563EB',
                          color: '#FFF',
                          border: 'none',
                          borderRadius: '4px',
                          cursor:
                            paginaActual >=
                            (resultadoPaginado.total_paginas ?? resultadoPaginado.totalPaginas ?? 1)
                              ? 'not-allowed'
                              : 'pointer',
                        }}
                      >
                        Siguiente
                      </button>
                    </div>
                  </div>
                )}
            </>
          )}
        </div>
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
            await cargarTablero();
            if (vistaActiva === 'HISTORIAL_PAGINADO') await cargarTablaPaginada(paginaActual);
          }}
        />
      )}
    </div>
  );
};
