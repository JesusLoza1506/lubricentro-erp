import { useEffect, useState } from 'react';
import { ClipboardList, Clock, RefreshCw, Wallet, Wrench } from 'lucide-react';
import { useAuth } from '../../context/AuthContext';
import { esErrorNoAutenticado } from '../../services/authService';
import { obtenerDatosDashboard, type DashboardDTO } from '../../services/dashboardService';
import { fechaInicio, GraficoColumnas, GraficoGrupos, monedaInicio } from './GraficosInicio';
import './dashboardVisual.css';

type Tarjeta = {
  titulo: string;
  valor: string | number;
  detalle: string;
  icono: typeof Clock;
  destacado?: boolean;
};
const cantidad = (n: number, singular: string, plural: string) =>
  `${n} ${n === 1 ? singular : plural}`;
const trabajos = (n: number) => cantidad(n, 'trabajo', 'trabajos');

function duracion(seg: number | null) {
  if (seg === null) return 'No disponible';
  if (seg < 60) return `${seg} s`;
  const min = Math.floor(seg / 60);
  const h = Math.floor(min / 60);
  return h ? `${h} h${min % 60 ? ` ${min % 60} min` : ''}` : `${min} min`;
}

function tarjetas(d: DashboardDTO): Tarjeta[] {
  switch (d.rol) {
    case 'ADMINISTRADOR':
      return [
        {
          titulo: 'Ventas netas hoy',
          valor: monedaInicio(d.ventas_netas_hoy_cent),
          detalle: 'Después de descuentos y reintegros',
          icono: Wallet,
        },
        {
          titulo: 'Trabajos terminados hoy',
          valor: d.trabajos_terminados_hoy,
          detalle: 'Órdenes finalizadas en Taller',
          icono: Wrench,
        },
        {
          titulo: 'Tiempo habitual hasta el cobro',
          valor: duracion(d.mediana_cobro_seg),
          detalle: 'Mediana de las OT cobradas en 7 días',
          icono: Clock,
        },
      ];
    case 'MECANICO':
      return [
        {
          titulo: 'Trabajos que terminé hoy',
          valor: d.trabajos_terminados_hoy,
          detalle: 'Órdenes propias finalizadas',
          icono: Wrench,
        },
        {
          titulo: 'Mis trabajos abiertos',
          valor: d.trabajos_abiertos,
          detalle: 'En espera o en proceso',
          icono: ClipboardList,
        },
        {
          titulo: 'Tiempo habitual de mis trabajos',
          valor: duracion(d.mediana_trabajo_seg),
          detalle: 'Mediana de mis OT finalizadas en 7 días',
          icono: Clock,
        },
      ];
    case 'CAJERO': {
      const turno = d.turno;
      const propio = turno.estado === 'PROPIO';
      const detalleTurno = propio
        ? `Desde las ${turno.apertura.slice(11, 16)}${turno.apertura.slice(0, 10) === d.fecha ? '' : ` del ${turno.apertura.slice(8, 10)}/${turno.apertura.slice(5, 7)}`} · Lima`
        : turno.estado === 'AJENO'
          ? 'Caja a cargo de otro usuario'
          : 'No hay un turno abierto';
      return [
        {
          titulo: 'Mi turno',
          valor: propio ? 'Abierto' : turno.estado === 'AJENO' ? 'Caja ajena' : 'Sin caja',
          detalle: detalleTurno,
          icono: Wallet,
          destacado: propio,
        },
        {
          titulo: 'Cobros que hice en mi turno',
          valor: d.actividad_turno?.cobros ?? 'No aplica',
          detalle: 'Operaciones confirmadas en mi turno',
          icono: ClipboardList,
        },
        {
          titulo: 'Mayor espera por cobrar',
          valor:
            d.pendientes.total === 0 ? 'Sin pendientes' : duracion(d.pendientes.mayor_espera_seg),
          detalle: 'Desde que Taller finalizó la OT',
          icono: Clock,
        },
      ];
    }
  }
}

function Graficos({ d }: { d: DashboardDTO }) {
  if (d.rol === 'ADMINISTRADOR')
    return (
      <>
        <GraficoColumnas
          titulo="Ventas netas de los últimos 7 días"
          subtitulo="Soles por día"
          puntos={d.ventas_por_dia}
          hoy={d.fecha}
          formato={monedaInicio}
          dinero
        />
        <GraficoColumnas
          titulo="Trabajos terminados por hora"
          subtitulo="Órdenes finalizadas hoy · Lima"
          puntos={d.trabajos_por_hora}
          hoy={d.fecha}
          formato={trabajos}
          horas
        />
      </>
    );
  if (d.rol === 'MECANICO')
    return (
      <>
        <GraficoGrupos
          titulo="Mi jornada por estado"
          subtitulo="Solo órdenes propias"
          segmentado
          vacio="No tienes órdenes para mostrar."
          grupos={[
            { nombre: 'En espera', valor: d.por_estado.en_espera, color: 'ambar' },
            { nombre: 'En proceso', valor: d.por_estado.en_proceso, color: 'azul' },
            { nombre: 'Finalizadas hoy', valor: d.por_estado.finalizadas_hoy, color: 'verde' },
          ]}
        />
        <GraficoColumnas
          titulo="Mis trabajos terminados por día"
          subtitulo="Últimos 7 días · órdenes propias finalizadas"
          puntos={d.trabajos_por_dia}
          hoy={d.fecha}
          formato={trabajos}
        />
      </>
    );
  return (
    <>
      <GraficoGrupos
        titulo="Trabajos pendientes por tiempo de espera"
        subtitulo="Desde que Taller finalizó la orden"
        vacio={
          d.pendientes.total
            ? 'No hay fechas válidas para calcular la espera.'
            : 'No hay trabajos pendientes por cobrar.'
        }
        grupos={[
          { nombre: 'Menos de 30 min', valor: d.pendientes.menos_30 },
          { nombre: 'De 30 a 60 min', valor: d.pendientes.entre_30_y_60 },
          { nombre: 'Más de 1 hora', valor: d.pendientes.mas_60, color: 'ambar' },
        ]}
      />
      {d.actividad_turno ? (
        <GraficoColumnas
          titulo="Mis cobros por hora"
          subtitulo="Operaciones de mi turno · Lima"
          puntos={d.actividad_turno.cobros_por_hora}
          hoy={d.fecha}
          formato={(n) => cantidad(n, 'cobro', 'cobros')}
          horas
        />
      ) : (
        <section className="inicio-grafico" aria-label="Mis cobros por hora">
          <h4>Mis cobros por hora</h4>
          <p className="inicio-subtitulo">Operaciones de mi turno · Lima</p>
          <p className="inicio-grafico-vacio">No aplica sin un turno propio abierto.</p>
        </section>
      )}
    </>
  );
}

const roles: Record<string, string> = {
  ADMINISTRADOR: 'Administrador',
  MECANICO: 'Mecánico',
  CAJERO: 'Cajero',
};

function InicioSesion({
  sessionId,
  nombre,
  rol,
  ambiente,
}: {
  sessionId: string;
  nombre: string;
  rol: string;
  ambiente: string;
}) {
  const { verificarSesion } = useAuth();
  const [lectura, setLectura] = useState<DashboardDTO | null>(null);
  const [cargando, setCargando] = useState(true);
  const [error, setError] = useState(false);
  const [intento, setIntento] = useState(0);

  useEffect(() => {
    let vigente = true;
    // Evita enviar la primera consulta del montaje simulado de StrictMode.
    void Promise.resolve().then(async () => {
      if (!vigente) return;
      try {
        const datos = await obtenerDatosDashboard(sessionId);
        if (!vigente) return;
        if (datos.rol !== rol || datos.ambiente !== ambiente) {
          setLectura(null);
          setError(true);
          return;
        }
        setLectura(datos);
        setError(false);
      } catch (e) {
        if (!vigente) return;
        if (esErrorNoAutenticado(e)) {
          setLectura(null);
          void verificarSesion();
        }
        setError(true);
      } finally {
        if (vigente) setCargando(false);
      }
    });
    return () => {
      vigente = false;
    };
  }, [sessionId, ambiente, rol, intento, verificarSesion]);

  const actualizar = () => {
    if (cargando) return;
    setCargando(true);
    setIntento((n) => n + 1);
  };

  return (
    <div className="gloria-theme inicio-page">
      <div className="inicio-content">
        <header className="inicio-bienvenida">
          <div>
            <p className="inicio-kicker">Resumen de jornada</p>
            <h2>Hola, {nombre.trim().split(/\s+/)[0]}</h2>
            <p className="inicio-subtitulo">Consulta el ritmo de tu jornada.</p>
          </div>
          <div className="inicio-meta">
            <span className="inicio-rol">{roles[rol] ?? rol}</span>
            <span>
              {ambiente === 'PRUEBA'
                ? 'Prueba'
                : ambiente === 'PRODUCCION'
                  ? 'Producción'
                  : 'Ambiente no disponible'}
            </span>
            {lectura && <span>{fechaInicio(lectura.fecha)} · Lima</span>}
          </div>
        </header>
        <section aria-labelledby="inicio-resumen" aria-busy={cargando}>
          <div className="inicio-cabecera">
            <div>
              <h3 id="inicio-resumen">Resumen de hoy</h3>
              {lectura && (
                <p className="inicio-subtitulo">
                  Actualizado a las {lectura.actualizado_en.slice(11, 19)} · Lima
                </p>
              )}
            </div>
            <button
              className="gt-button gt-button--quiet"
              type="button"
              onClick={actualizar}
              disabled={cargando}
            >
              <RefreshCw size={16} aria-hidden="true" />{' '}
              {error && !lectura ? 'Reintentar' : 'Actualizar'}
            </button>
          </div>
          {cargando && (
            <p className="inicio-mensaje" role="status">
              {lectura ? 'Actualizando resumen…' : 'Cargando resumen…'}
            </p>
          )}
          {error && !cargando && (
            <p className="inicio-mensaje inicio-error" role="alert">
              {lectura
                ? 'Sin actualizar. Se conserva la última lectura.'
                : 'No se pudo cargar Inicio.'}
            </p>
          )}
          {lectura && (
            <dl className="inicio-tarjetas">
              {tarjetas(lectura).map(({ titulo, valor, detalle, icono: Icono, destacado }) => (
                <div className="inicio-tarjeta" key={titulo}>
                  <dt>
                    {titulo}
                    <Icono size={16} aria-hidden="true" />
                  </dt>
                  <dd className={`inicio-valor${destacado ? ' inicio-valor-abierto' : ''}`}>
                    {valor}
                  </dd>
                  <dd className="inicio-subtitulo">{detalle}</dd>
                </div>
              ))}
            </dl>
          )}
        </section>
        {lectura && (
          <section aria-labelledby="inicio-actividad">
            <h3 id="inicio-actividad">Actividad en gráficos</h3>
            <div className="inicio-graficos">
              <Graficos d={lectura} />
            </div>
          </section>
        )}
      </div>
    </div>
  );
}

export function DashboardPage() {
  const { usuario, estadoSesion, ambienteActual } = useAuth();
  if (!usuario?.session_id || estadoSesion !== 'NORMAL') return null;
  const ambiente = usuario.ambiente_actual ?? ambienteActual;
  const alcance = JSON.stringify([usuario.session_id, usuario.id_usuario, usuario.rol, ambiente]);
  return (
    <InicioSesion
      key={alcance}
      sessionId={usuario.session_id}
      nombre={usuario.nombre_completo}
      rol={usuario.rol}
      ambiente={ambiente}
    />
  );
}
