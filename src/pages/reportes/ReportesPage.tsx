import {
  ArrowLeft,
  ChartNoAxesCombined,
  FileSpreadsheet,
  FileText,
  LoaderCircle,
} from 'lucide-react';
import { useEffect, useRef, useState } from 'react';
import { useAuth } from '../../context/AuthContext';
import { FiltrosReportes } from './components/FiltrosReportes';
import { Paginacion, TablaReportes } from './components/TablaReportes';
import {
  type Detalle,
  type Filtros,
  type Formato,
  type Origen,
  type Pagina,
  type Reporte,
  errorReporte,
  filtrosIguales,
  inicial,
  POR_PAGINA,
  reportes,
  REPORTES,
  valor,
} from './reportesApi';
import '../../styles/gloriaTaller.css';
import './reportes.css';

type VistaDetalle = { fila: number; datos: Detalle; pagina: number };
export function ReportesPage({ onOrigen }: { onOrigen: (origen: Origen) => void }) {
  const { usuario, ambienteActual, verificarSesion } = useAuth();
  const [reporte, setReporte] = useState<Reporte>('R01');
  const [filtros, setFiltros] = useState<Filtros>(() => inicial());
  const [aplicado, setAplicado] = useState<Pagina | null>(null);
  const [detalle, setDetalle] = useState<VistaDetalle | null>(null);
  const [tarea, setTarea] = useState('');
  const [error, setError] = useState('');
  const [aviso, setAviso] = useState('');
  const [reintento, setReintento] = useState<(() => void) | null>(null);
  const ocupado = useRef(false);
  const activa = useRef(false);
  const corte = useRef<string | null>(null);
  const regreso = useRef<HTMLButtonElement | null>(null);
  const detalleTitulo = useRef<HTMLHeadingElement>(null);
  const pendiente =
    !aplicado || reporte !== aplicado.reporte || !filtrosIguales(filtros, aplicado.filtros);

  const descartar = (id: string) => void reportes({ accion: 'descartar', id }).catch(() => {});
  async function ejecutar(nombre: string, trabajo: () => Promise<void>) {
    if (ocupado.current || !activa.current) return;
    ocupado.current = true;
    setTarea(nombre);
    setError('');
    setAviso('');
    setReintento(null);
    try {
      await trabajo();
    } catch (e) {
      if (activa.current) {
        if (typeof e === 'string' && /^(NO_AUTENTICADO|PERMISO_DENEGADO):/.test(e)) {
          setAplicado(null);
          setDetalle(null);
          corte.current = null;
          if (e.startsWith('NO_AUTENTICADO:')) void verificarSesion();
        }
        setError(errorReporte(e));
        setReintento(() => () => void ejecutar(nombre, trabajo));
      }
    } finally {
      ocupado.current = false;
      if (activa.current) setTarea('');
    }
  }

  function consultar(r: Reporte, f: Filtros) {
    void ejecutar('Consultando…', async () => {
      const resultado = await reportes<Pagina>({ accion: 'consultar', reporte: r, filtros: f });
      if (!activa.current) {
        descartar(resultado.id);
        return;
      }
      corte.current = resultado.id;
      setAplicado(resultado);
      setDetalle(null);
    });
  }
  useEffect(() => {
    if (usuario?.rol !== 'ADMINISTRADOR') return;
    activa.current = true;
    let iniciar = true;
    queueMicrotask(() => {
      if (iniciar) consultar('R01', inicial());
    });
    return () => {
      iniciar = false;
      activa.current = false;
      if (corte.current) {
        descartar(corte.current);
        corte.current = null;
      }
    };
  }, [usuario?.rol]);

  useEffect(() => {
    if (detalle) detalleTitulo.current?.focus();
  }, [detalle?.fila]);
  const cerrarDetalle = () => {
    setDetalle(null);
    requestAnimationFrame(() => regreso.current?.focus());
  };

  function elegir(r: Reporte) {
    const f = inicial(filtros);
    setReporte(r);
    setFiltros(f);
    consultar(r, f);
  }
  function limpiar() {
    const f = inicial();
    setFiltros(f);
    consultar(reporte, f);
  }
  function paginar(pagina: number) {
    if (!aplicado) return;
    void ejecutar('Cargando página…', async () => {
      const resultado = await reportes<Pagina>({ accion: 'pagina', id: aplicado.id, pagina });
      if (activa.current) setAplicado(resultado);
    });
  }
  function ver(fila: number, control: HTMLButtonElement) {
    if (!aplicado) return;
    regreso.current = control;
    const indice = (aplicado.pagina - 1) * POR_PAGINA + fila;
    void ejecutar('Cargando detalle…', async () => {
      const datos = await reportes<Detalle>({ accion: 'detalle', id: aplicado.id, fila: indice });
      if (activa.current) setDetalle({ fila: indice, datos, pagina: 1 });
    });
  }
  function abrir(fila: number, indice: number) {
    if (!aplicado) return;
    const req = detalle
      ? { fila: detalle.fila, detalle: (detalle.pagina - 1) * POR_PAGINA + fila }
      : { fila: (aplicado.pagina - 1) * POR_PAGINA + fila };
    void ejecutar('Comprobando acceso…', async () => {
      const origen = await reportes<Origen>({ accion: 'origen', id: aplicado.id, indice, ...req });
      if (activa.current) onOrigen(origen);
    });
  }
  function exportar(formato: Formato) {
    if (!aplicado || pendiente) return;
    void ejecutar('Exportando…', async () => {
      const res = await reportes<{ guardado: boolean }>({
        accion: 'exportar',
        id: aplicado.id,
        reporte,
        filtros,
        formato,
      });
      if (activa.current && res.guardado)
        setAviso(formato === 'pdf' ? 'Reporte PDF guardado.' : 'Reporte Excel guardado.');
    });
  }

  if (usuario?.rol !== 'ADMINISTRADOR') return null;
  const ambiente = aplicado?.ambiente ?? ambienteActual;
  const usuarios = aplicado?.reporte === reporte ? aplicado.usuarios : [];
  const filasDetalle = detalle?.datos.filas.slice(
    (detalle.pagina - 1) * POR_PAGINA,
    detalle.pagina * POR_PAGINA
  );
  return (
    <div className="gloria-theme rep-pantalla">
      <header className="rep-cabecera">
        <div>
          <p className="rep-eyebrow">CONSULTA DEL NEGOCIO</p>
          <h1>Reportes</h1>
          <p>Consulta la operación y exporta las cifras del corte que estás viendo.</p>
        </div>
        <span className="rep-ambiente">
          <ChartNoAxesCombined size={20} aria-hidden="true" />
          {ambiente === 'PRUEBA'
            ? 'Prueba'
            : ambiente === 'PRODUCCION'
              ? 'Producción'
              : 'Consultando ambiente…'}
        </span>
      </header>
      <FiltrosReportes
        reporte={reporte}
        filtros={filtros}
        usuarios={usuarios}
        ocupado={!!tarea}
        onReporte={elegir}
        onFiltros={(f) => {
          setFiltros(f);
          setReintento(null);
        }}
        onConsultar={() => consultar(reporte, filtros)}
        onLimpiar={limpiar}
      />
      {tarea && (
        <p role="status" className="rep-estado">
          <LoaderCircle size={16} className="rep-giro" aria-hidden="true" />
          {tarea}
        </p>
      )}
      {error && (
        <div className="gt-notice gt-notice--error rep-estado" role="alert">
          {error}
          {reintento && (
            <button
              className="gt-button gt-button--quiet"
              type="button"
              disabled={!!tarea}
              onClick={reintento}
            >
              Reintentar
            </button>
          )}
        </div>
      )}
      {aviso && (
        <p role="status" className="gt-notice rep-estado">
          {aviso}
        </p>
      )}
      {aplicado && (
        <>
          {pendiente && (
            <p className="gt-notice rep-estado" role="status">
              Pulsa Consultar para aplicar los filtros. Se muestra el resultado anterior.
            </p>
          )}
          <section
            className="rep-panel rep-resultados"
            aria-label="Resultado del reporte"
            aria-busy={!!tarea}
          >
            <div className="rep-resultado-cab">
              <div>
                <p className="rep-codigo">
                  {aplicado.reporte} · {aplicado.ambiente}
                </p>
                <h2>{REPORTES[aplicado.reporte]}</h2>
                <p>
                  {aplicado.rango ? `${aplicado.rango[0]} a ${aplicado.rango[1]}` : 'Corte actual'}{' '}
                  · {aplicado.criterio}
                </p>
                <p>
                  {aplicado.filtros_texto} · Corte: <strong>{aplicado.hora} (Lima)</strong>
                </p>
              </div>
              <div className="rep-acciones">
                <button
                  type="button"
                  className="gt-button gt-button--quiet"
                  disabled={pendiente || !!tarea}
                  onClick={() => exportar('pdf')}
                >
                  <FileText size={16} aria-hidden="true" /> Exportar PDF
                </button>
                <button
                  type="button"
                  className="gt-button gt-button--quiet"
                  disabled={pendiente || !!tarea}
                  onClick={() => exportar('xlsx')}
                >
                  <FileSpreadsheet size={16} aria-hidden="true" /> Exportar Excel
                </button>
              </div>
            </div>
            {aplicado.totales.length > 0 && (
              <dl className="rep-totales">
                {aplicado.totales.map(([k, v]) => (
                  <div key={k}>
                    <dt>{k}</dt>
                    <dd>{valor(v)}</dd>
                  </div>
                ))}
              </dl>
            )}
            {aplicado.disponibles != null && aplicado.disponibles > aplicado.total_filas && (
              <p className="rep-nota">
                Mostrando {aplicado.total_filas} de {aplicado.disponibles} ítems.
              </p>
            )}
            {aplicado.reporte === 'R03' && (
              <p className="rep-nota">
                El margen de productos no representa la utilidad total del negocio.
              </p>
            )}
            <div hidden={!!detalle}>
              <TablaReportes
                datos={aplicado}
                nombre="Filas del reporte"
                ocupado={!!tarea}
                ver={ver}
                abrir={abrir}
              />
              <Paginacion
                pagina={aplicado.pagina}
                paginas={Math.max(1, Math.ceil(aplicado.total_filas / POR_PAGINA))}
                total={aplicado.total_filas}
                ocupado={!!tarea}
                ir={paginar}
              />
            </div>
            {detalle && (
              <section
                className="rep-detalle"
                onKeyDown={(e) => {
                  if (e.key === 'Escape') cerrarDetalle();
                }}
              >
                <div className="rep-detalle-cab">
                  <h3 ref={detalleTitulo} tabIndex={-1}>
                    Detalle al corte
                  </h3>
                  <button
                    type="button"
                    className="gt-button gt-button--quiet"
                    onClick={cerrarDetalle}
                  >
                    <ArrowLeft size={16} aria-hidden="true" /> Volver al resultado
                  </button>
                </div>
                <TablaReportes
                  datos={{ ...detalle.datos, filas: filasDetalle ?? [] }}
                  nombre="Filas del detalle"
                  ocupado={!!tarea}
                  abrir={abrir}
                />
                {detalle.datos.filas.length > POR_PAGINA && (
                  <Paginacion
                    pagina={detalle.pagina}
                    paginas={Math.ceil(detalle.datos.filas.length / POR_PAGINA)}
                    total={detalle.datos.filas.length}
                    ocupado={!!tarea}
                    ir={(p) => setDetalle({ ...detalle, pagina: p })}
                  />
                )}
              </section>
            )}
          </section>
        </>
      )}
    </div>
  );
}
