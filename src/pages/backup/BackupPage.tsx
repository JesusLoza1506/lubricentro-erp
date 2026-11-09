import { useCallback, useEffect, useRef, useState } from 'react';
import {
  ArrowLeft,
  CalendarClock,
  ChevronLeft,
  ChevronRight,
  CircleCheck,
  DatabaseBackup,
  FolderOpen,
  History,
  Layers,
  RefreshCw,
  Shield,
  ShieldCheck,
  TriangleAlert,
  type LucideIcon,
} from 'lucide-react';
import { ConfirmModal } from '../../components/ConfirmModal';
import {
  backup,
  elegirBackup,
  errorBackup,
  fechaBackup,
  type ContextoDesdeCaja,
  type EstadoBackup,
  type Recuperacion,
  type SeleccionBackup,
  type SerieRecuperada,
} from './backupService';
import '../../styles/gloriaTaller.css';
import './backupVisual.css';
import { useProgresoBackup } from './useProgresoBackup';

const tipoNombre: Record<string, string> = {
  MANUAL: 'Manual',
  AUTOMATICO: 'Automática',
  PREVIA: 'Previa',
  IMPORTADO: 'Importada',
};
const estadoNombre: Record<string, string> = {
  AUSENTE: 'Archivo ausente',
  VALIDO: 'Válida',
  FALLIDO: 'Fallida',
  DANADO: 'No se pudo abrir',
  INCOMPATIBLE: 'Incompatible',
};
function ResumenBackup({
  icono: Icono,
  titulo,
  valor,
  detalle,
  tono,
}: {
  icono: LucideIcon;
  titulo: string;
  valor: string;
  detalle: string;
  tono?: 'aviso' | 'exito';
}) {
  return (
    <div className="bkp-indicador" data-tono={tono}>
      <dt>
        {titulo}
        <Icono size={17} aria-hidden="true" />
      </dt>
      <dd className="bkp-indicador-valor">{valor}</dd>
      <dd className="bkp-indicador-detalle">{detalle}</dd>
    </div>
  );
}

const bytes = (n: number | null) =>
  n == null ? '—' : `${(n / 1024 / 1024).toLocaleString('es-PE', { maximumFractionDigits: 2 })} MB`;

type Origen = { id_backup: number } | { ruta: string };
type Confirmacion =
  | { tipo: 'restaurar' }
  | { tipo: 'eliminar_previa'; id_backup: number; nombre: string; fecha: string };

export function BackupPage({
  inicial = false,
  onVolver,
  onCaja,
  onFinalizar,
  contextoCaja,
}: {
  inicial?: boolean;
  onVolver?: () => void;
  onCaja?: (datos: ContextoDesdeCaja) => void;
  onFinalizar?: () => void;
  contextoCaja?: ContextoDesdeCaja;
}) {
  const [estado, setEstado] = useState<EstadoBackup | null>(null);
  const [tipo, setTipo] = useState(contextoCaja?.tipo ?? '');
  const [pagina, setPagina] = useState(contextoCaja?.pagina ?? 1);
  const [origen, setOrigen] = useState<Origen | null>(null);
  const [sel, setSel] = useState<SeleccionBackup | null>(null);
  const regreso = useRef(contextoCaja?.seleccion);
  const [frase, setFrase] = useState('');
  const [repetir, setRepetir] = useState('');
  const [config, setConfig] = useState(false);
  const [pedirFrase, setPedirFrase] = useState(false);
  const [confirmar, setConfirmar] = useState<Confirmacion | null>(null);
  const [ocupado, setOcupado] = useState('');
  const [error, setError] = useState('');
  const [errorEstado, setErrorEstado] = useState('');
  const [aviso, setAviso] = useState('');
  const enCurso = useRef(false);
  const revision = useRef(0);
  const actualizar = useCallback(async () => {
    if (inicial) return;
    const n = ++revision.current;
    setErrorEstado('');
    try {
      const data = await backup<EstadoBackup>('estado', { tipo, pagina });
      if (revision.current === n) setEstado(data);
    } catch (e) {
      if (revision.current === n) setErrorEstado(errorBackup(e));
    }
  }, [inicial, tipo, pagina]);

  useEffect(() => {
    void actualizar();
    return () => {
      revision.current += 1;
    };
  }, [actualizar]);

  useEffect(() => {
    const anterior = regreso.current;
    if (!anterior) return;
    let vigente = true;
    void backup<{ vigente: boolean; caja_abierta: boolean; compatible: boolean }>('revalidar', {
      seleccion: anterior.seleccion,
    })
      .then((r) => {
        if (!vigente) return;
        if (r.vigente)
          setSel({
            ...anterior,
            caja_abierta: r.caja_abierta,
            compatible: anterior.compatible && r.compatible,
          });
        else setAviso('La selección ya no está disponible. Comprueba la copia de nuevo.');
      })
      .catch((e) => {
        if (vigente) setError(errorBackup(e));
      });
    return () => {
      vigente = false;
    };
  }, []);

  const irCaja = onCaja ? () => onCaja({ tipo, pagina, seleccion: sel }) : undefined;
  const etapa = useProgresoBackup(!!ocupado);

  const tarea = async (progreso: string, f: () => Promise<void>) => {
    if (enCurso.current) return;
    enCurso.current = true;
    setOcupado(progreso);
    setError('');
    setAviso('');
    try {
      await f();
    } catch (e) {
      setError(errorBackup(e));
    } finally {
      setOcupado('');
      enCurso.current = false;
    }
  };
  const comprobar = async (fuente: Origen, clave?: string) => {
    setOrigen(fuente);
    setSel(null);
    setPedirFrase(false);
    try {
      const info = await backup<SeleccionBackup>('comprobar', {
        ...fuente,
        inicial,
        frase: clave || undefined,
      });
      setSel(info);
    } catch (e) {
      if (/frase|autenticar/i.test(errorBackup(e))) setPedirFrase(true);
      throw e;
    } finally {
      setFrase('');
    }
  };
  const abrir = () =>
    tarea('Eligiendo copia…', async () => {
      const ruta = await elegirBackup(false, inicial);
      if (!ruta) return;
      setSel(null);
      setOrigen({ ruta });
      setFrase('');
      setPedirFrase(true);
    });
  const salir = () =>
    tarea('Cerrando selección…', async () => {
      await backup('cancelar', { inicial });
      onVolver?.();
    });
  const confirmarOperacion = () => {
    const datos = confirmar;
    if (!datos) return;
    setConfirmar(null);
    void tarea(
      datos.tipo === 'restaurar' ? 'Preparando la restauración…' : 'Eliminando copia previa…',
      async () => {
        if (datos.tipo === 'restaurar' && sel) {
          await backup('restaurar', { seleccion: sel.seleccion, inicial, confirmado: true });
          // También descarta el estado en memoria del frontend.
          window.location.reload();
        } else if (datos.tipo === 'eliminar_previa') {
          await backup('eliminar_previa', { id_backup: datos.id_backup, confirmado: true });
          setAviso('Copia previa eliminada.');
          await actualizar();
        }
      }
    );
  };
  const recovery = estado?.recuperacion;
  return (
    <section className="gloria-theme bkp-page">
      <div className="bkp-stage gt-stage">
        <header className="bkp-header">
          <div>
            <p className="bkp-kicker">Continuidad del negocio</p>
            <h1>{inicial ? 'Restaurar una copia' : 'Backup y Copias'}</h1>
            <p>
              {inicial
                ? 'Abre un respaldo de la empresa antes de crear la primera cuenta.'
                : 'Protege tus datos, verifica respaldos y recupera una instalación.'}
            </p>
          </div>
          <span className="bkp-icono bkp-icono--hero">
            <DatabaseBackup size={24} aria-hidden="true" />
          </span>
        </header>

        {errorEstado && (
          <div className="gt-notice gt-notice--error" role="alert">
            <TriangleAlert size={17} aria-hidden="true" />
            {errorEstado}
          </div>
        )}
        {error && error !== errorEstado && (
          <div className="gt-notice gt-notice--error" role="alert">
            <TriangleAlert size={17} aria-hidden="true" />
            {error}
          </div>
        )}
        {aviso && (
          <div className="gt-notice gt-notice--success" role="status">
            <CircleCheck size={17} aria-hidden="true" />
            {aviso}
          </div>
        )}
        {ocupado && (
          <div className="bkp-progreso" role="status">
            <span className="gt-spinner" aria-hidden="true" />
            {etapa || ocupado}
          </div>
        )}

        {!inicial && (
          <section
            className="bkp-resumen-seccion"
            aria-labelledby="bkp-resumen-titulo"
            aria-busy={!estado && !errorEstado}
          >
            <h2 id="bkp-resumen-titulo">Estado del respaldo</h2>
            <dl className="bkp-resumen">
              <ResumenBackup
                icono={ShieldCheck}
                titulo="Última copia válida"
                valor={
                  estado?.ultima ? fechaBackup(estado.ultima) : estado ? 'Sin copia válida' : '—'
                }
                detalle={
                  estado
                    ? estado.vencida
                      ? 'Conviene generar una copia reciente.'
                      : 'Hora de Lima'
                    : errorEstado
                      ? 'Dato no disponible'
                      : 'Cargando copias…'
                }
                tono={estado ? (estado.vencida ? 'aviso' : 'exito') : undefined}
              />
              <ResumenBackup
                icono={CalendarClock}
                titulo="Próxima copia diaria"
                valor={
                  estado?.configurada ? fechaBackup(estado.proxima) : estado ? 'Sin programar' : '—'
                }
                detalle={
                  estado
                    ? estado.configurada
                      ? (estado.diaria ?? 'Programada a las 23:00 (Lima)')
                      : 'Configura la protección para habilitarla.'
                    : errorEstado
                      ? 'Dato no disponible'
                      : 'Consultando programación…'
                }
                tono={estado && !estado.configurada ? 'aviso' : undefined}
              />
              <ResumenBackup
                icono={Layers}
                titulo="Conservación"
                valor="14 copias rutinarias"
                detalle="La limpieza automática conserva las previas."
              />
            </dl>
            {estado?.ultima_restauracion && (
              <p className="bkp-restauracion">
                <History size={16} aria-hidden="true" />
                <span>
                  <strong>Última restauración:</strong> {estado.ultima_restauracion.nombre} ·{' '}
                  {fechaBackup(estado.ultima_restauracion.fecha)} ·{' '}
                  {estado.ultima_restauracion.estado === 'COMPLETA'
                    ? 'Completa'
                    : 'Controles pendientes'}
                  {estado.ultima_restauracion.previa &&
                    ` · Previa: ${estado.ultima_restauracion.previa}`}
                </span>
              </p>
            )}
          </section>
        )}

        {recovery?.pendiente && (
          <Completar
            data={recovery}
            ocupado={!!ocupado}
            onCaja={irCaja}
            onAccion={(accion, datos, mensaje) =>
              tarea('Guardando controles…', async () => {
                await backup(accion, datos);
                await actualizar();
                setAviso(mensaje);
                if (accion === 'finalizar') onFinalizar?.();
              })
            }
          />
        )}

        <section className="bkp-panel bkp-toolbar" aria-labelledby="bkp-acciones-titulo">
          <div>
            <h2 id="bkp-acciones-titulo">
              {inicial ? 'Selecciona el archivo de respaldo' : 'Gestionar copias'}
            </h2>
            <p>
              {inicial
                ? 'Usa un archivo .lgbackup y su frase original.'
                : estado && !estado.configurada
                  ? 'Configura la frase de protección para generar nuevos respaldos.'
                  : 'Crea un respaldo local o abre una copia existente para verificarla.'}
            </p>
          </div>
          <div className="bkp-acciones">
            {!inicial && (
              <>
                <button
                  className={
                    estado && !estado.configurada ? 'gt-button gt-button--primary' : 'gt-button'
                  }
                  disabled={!!ocupado}
                  aria-expanded={config}
                  onClick={() => {
                    setConfig(!config);
                    setPedirFrase(false);
                    setFrase('');
                    setRepetir('');
                  }}
                >
                  <Shield size={16} aria-hidden="true" />
                  Configurar protección
                </button>
                <button
                  className="gt-button"
                  disabled={!!ocupado}
                  onClick={() => void actualizar()}
                >
                  <RefreshCw size={16} aria-hidden="true" />
                  Actualizar
                </button>
              </>
            )}
            {inicial && (
              <button className="gt-button" disabled={!!ocupado} onClick={() => void salir()}>
                <ArrowLeft size={16} aria-hidden="true" />
                Volver
              </button>
            )}
            <button
              className={inicial ? 'gt-button gt-button--primary' : 'gt-button'}
              disabled={!!ocupado}
              onClick={() => void abrir()}
            >
              <FolderOpen size={17} aria-hidden="true" />
              Abrir copia
            </button>
            {!inicial && (
              <button
                className="gt-button gt-button--primary"
                disabled={!!ocupado || !estado?.configurada}
                onClick={() =>
                  void tarea('Iniciando respaldo…', async () => {
                    await backup('generar');
                    await actualizar();
                    setAviso('Copia creada y verificada.');
                  })
                }
              >
                <DatabaseBackup size={17} aria-hidden="true" />
                Generar ahora
              </button>
            )}
          </div>
        </section>
        {config && (
          <form
            className="bkp-panel bkp-form"
            onSubmit={(e) => {
              e.preventDefault();
              if (frase !== repetir) {
                setError('Las frases no coinciden.');
                return;
              }
              void tarea('Guardando protección local…', async () => {
                try {
                  await backup('configurar', { frase });
                  setConfig(false);
                  setAviso('Protección configurada en Windows.');
                  await actualizar();
                } finally {
                  setFrase('');
                  setRepetir('');
                }
              });
            }}
          >
            <h2>Protección local</h2>
            <p>
              Use una frase de 16 a 128 caracteres. Las copias anteriores conservan su frase
              original.
            </p>
            <label>
              Frase nueva
              <input
                className="gt-input"
                required
                type="password"
                autoComplete="new-password"
                value={frase}
                onChange={(e) => setFrase(e.target.value)}
              />
            </label>
            <label>
              Repetir frase
              <input
                className="gt-input"
                required
                type="password"
                autoComplete="new-password"
                value={repetir}
                onChange={(e) => setRepetir(e.target.value)}
              />
            </label>
            <div className="bkp-acciones">
              <button
                type="button"
                className="gt-button"
                disabled={!!ocupado}
                onClick={() => {
                  setConfig(false);
                  setFrase('');
                  setRepetir('');
                  setError('');
                }}
              >
                Cancelar
              </button>
              <button className="gt-button gt-button--primary" disabled={!!ocupado}>
                Guardar protección
              </button>
            </div>
          </form>
        )}
        {pedirFrase && origen && (
          <form
            className="bkp-panel bkp-form"
            onSubmit={(e) => {
              e.preventDefault();
              setConfig(false);
              void tarea('Comprobando autenticidad e integridad…', () => comprobar(origen, frase));
            }}
          >
            <h2>Verificar respaldo</h2>
            <label>
              Frase original de la copia
              <input
                className="gt-input"
                type="password"
                required
                autoComplete="off"
                value={frase}
                onChange={(e) => setFrase(e.target.value)}
              />
            </label>
            <button className="gt-button gt-button--primary" disabled={!!ocupado}>
              Comprobar copia
            </button>
          </form>
        )}
        {sel && (
          <div className="bkp-panel">
            <h2>Copia verificada</h2>
            <p>
              <strong>{sel.nombre}</strong>
            </p>
            <p>
              Fecha recuperable: {fechaBackup(sel.manifiesto.fecha)} (Lima) ·{' '}
              {sel.manifiesto.identidad.ambiente} · RUC{' '}
              {sel.manifiesto.identidad.ruc ?? 'sin configurar'}
            </p>
            <p>
              {bytes(sel.tamano)} · {sel.manifiesto.archivos.length - 1} archivos oficiales locales
              · Formato {sel.manifiesto.formato} · Esquema {sel.manifiesto.esquema} · App{' '}
              {sel.manifiesto.app}
            </p>
            <p>
              Autor: {sel.manifiesto.autor} · Generada: {fechaBackup(sel.manifiesto.generado)}
            </p>
            {!sel.compatible && (
              <p className="gt-notice" role="alert">
                No se puede restaurar aquí: comprueba la empresa, el ambiente y que la copia tenga
                un administrador activo.
              </p>
            )}
            {sel.caja_abierta && (
              <p>
                Hay un turno activo. Ciérrelo antes de restaurar.{' '}
                {irCaja && (
                  <button className="gt-button" onClick={irCaja} disabled={!!ocupado}>
                    Ir a Caja
                  </button>
                )}
              </p>
            )}
            <div className="bkp-acciones">
              {!inicial && (
                <button
                  className="gt-button"
                  disabled={!!ocupado}
                  onClick={() =>
                    void tarea('Copiando y verificando el destino…', async () => {
                      const ruta = await elegirBackup(true, false);
                      if (!ruta) return;
                      const data = await backup<{ nombre: string }>('exportar', {
                        seleccion: sel.seleccion,
                        ruta,
                      });
                      setAviso(`Copia externa verificada: ${data.nombre}`);
                      await actualizar();
                    })
                  }
                >
                  Guardar fuera
                </button>
              )}
              <button
                className="gt-button gt-button--danger"
                disabled={!!ocupado || !sel.compatible || sel.caja_abierta}
                onClick={() => setConfirmar({ tipo: 'restaurar' })}
              >
                Restaurar copia
              </button>
              <button
                className="gt-button"
                disabled={!!ocupado}
                onClick={() =>
                  void tarea('Cerrando selección…', async () => {
                    await backup('cancelar', { inicial });
                    setSel(null);
                  })
                }
              >
                Quitar selección
              </button>
            </div>
          </div>
        )}
        {!inicial && estado && (
          <section className="bkp-panel bkp-historial" aria-labelledby="bkp-historial-titulo">
            <div className="bkp-encabezado-lista">
              <div>
                <h2 id="bkp-historial-titulo">Copias locales</h2>
                <p>Selecciona un respaldo para comprobarlo antes de exportar o restaurar.</p>
              </div>
              <label className="bkp-filtro">
                Tipo de copia
                <select
                  className="gt-input"
                  value={tipo}
                  disabled={!!ocupado}
                  onChange={(e) => {
                    setTipo(e.target.value);
                    setPagina(1);
                  }}
                >
                  <option value="">Todas</option>
                  {Object.entries(tipoNombre).map(([v, n]) => (
                    <option key={v} value={v}>
                      {n}
                    </option>
                  ))}
                </select>
              </label>
            </div>
            {estado.copias.length > 0 ? (
              <>
                <div
                  className="bkp-tabla"
                  tabIndex={0}
                  role="region"
                  aria-label="Tabla de copias locales"
                >
                  <table>
                    <thead>
                      <tr>
                        <th scope="col">Fecha recuperable (Lima)</th>
                        <th scope="col">Tipo</th>
                        <th scope="col">Tamaño</th>
                        <th scope="col">Estado</th>
                        <th scope="col">Acciones</th>
                      </tr>
                    </thead>
                    <tbody>
                      {estado.copias.map((c) => (
                        <tr key={c.id}>
                          <td>
                            <strong>{fechaBackup(c.fecha ?? c.creado)}</strong>
                          </td>
                          <td>{tipoNombre[c.tipo] ?? c.tipo}</td>
                          <td>{bytes(c.tamano)}</td>
                          <td>
                            <span
                              className="bkp-sello"
                              data-tono={
                                c.estado === 'VALIDO'
                                  ? 'exito'
                                  : ['FALLIDO', 'AUSENTE'].includes(c.estado)
                                    ? 'error'
                                    : 'aviso'
                              }
                            >
                              {estadoNombre[c.estado] ?? c.estado}
                            </span>
                            {c.externo === 'COPIADO' && <small>Guardada fuera</small>}
                            {c.externo === 'FALLIDO' && <small>Falló el guardado externo</small>}
                          </td>
                          <td>
                            <div className="bkp-acciones bkp-acciones--fila">
                              {['VALIDO', 'DANADO', 'INCOMPATIBLE'].includes(c.estado) && (
                                <button
                                  className="gt-button"
                                  disabled={!!ocupado}
                                  onClick={() => {
                                    setConfig(false);
                                    void tarea('Comprobando copia…', () =>
                                      comprobar({ id_backup: c.id })
                                    );
                                  }}
                                >
                                  Seleccionar
                                </button>
                              )}
                              {c.tipo === 'PREVIA' && c.estado === 'VALIDO' && (
                                <button
                                  className="gt-button bkp-secundario-peligro"
                                  disabled={!!ocupado || !!recovery?.pendiente}
                                  onClick={() =>
                                    setConfirmar({
                                      tipo: 'eliminar_previa',
                                      id_backup: c.id,
                                      nombre: c.nombre,
                                      fecha: c.fecha ?? c.creado,
                                    })
                                  }
                                >
                                  Eliminar previa
                                </button>
                              )}
                            </div>
                          </td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
                <div className="bkp-paginas">
                  <span>
                    Página {pagina} de {Math.max(1, Math.ceil(estado.total / 20))} · {estado.total}{' '}
                    {estado.total === 1 ? 'copia' : 'copias'}
                  </span>
                  <div className="bkp-acciones">
                    <button
                      className="gt-button"
                      disabled={pagina <= 1 || !!ocupado}
                      onClick={() => setPagina((p) => p - 1)}
                    >
                      <ChevronLeft size={16} aria-hidden="true" />
                      Anterior
                    </button>
                    <button
                      className="gt-button"
                      disabled={pagina * 20 >= estado.total || !!ocupado}
                      onClick={() => setPagina((p) => p + 1)}
                    >
                      Siguiente
                      <ChevronRight size={16} aria-hidden="true" />
                    </button>
                  </div>
                </div>
              </>
            ) : (
              <div className="bkp-vacio">
                <span className="bkp-icono">
                  <DatabaseBackup size={23} aria-hidden="true" />
                </span>
                <p>
                  <strong>
                    {tipo ? 'No hay copias para este filtro' : 'Aún no hay copias locales'}
                  </strong>
                </p>
                <p>
                  {tipo
                    ? 'Selecciona otro tipo para seguir buscando.'
                    : 'Genera tu primer respaldo o abre una copia guardada.'}
                </p>
              </div>
            )}
          </section>
        )}
      </div>
      <ConfirmModal
        abierto={!!confirmar}
        varianteVisual="gloria"
        cargando={!!ocupado}
        titulo={confirmar?.tipo === 'restaurar' ? 'Restaurar esta copia' : 'Eliminar copia previa'}
        mensaje={
          confirmar?.tipo === 'restaurar'
            ? `Se sustituirán los datos por la copia del ${sel?.manifiesto.fecha ?? ''} (Lima). Los cambios posteriores no aparecerán en la base restaurada. ${inicial ? '' : 'Se generará una copia previa. '}Deberá iniciar sesión con un administrador de la copia.`
            : `${confirmar?.nombre ?? ''} · ${confirmar?.fecha ?? ''}. Se eliminará el archivo local seleccionado.`
        }
        textoConfirmar={confirmar?.tipo === 'restaurar' ? 'Restaurar' : 'Eliminar previa'}
        onConfirmar={confirmarOperacion}
        onCancelar={() => setConfirmar(null)}
      />
    </section>
  );
}

function Completar({
  data,
  ocupado,
  onCaja,
  onAccion,
}: {
  data: Recuperacion;
  ocupado: boolean;
  onCaja?: () => void;
  onAccion: (accion: string, datos: Record<string, unknown>, mensaje: string) => Promise<void>;
}) {
  const [token, setToken] = useState('');
  const [ruta, setRuta] = useState('');
  const [serie, setSerie] = useState<SerieRecuperada | null>(null);
  const [numero, setNumero] = useState('');
  const [referencia, setReferencia] = useState('');
  return (
    <div className="bkp-panel bkp-recuperacion">
      <h2>Completar recuperación</h2>
      <p>
        POS permanece bloqueado hasta cerrar la caja recuperada, configurar la credencial cuando
        corresponda y conciliar las series.
      </p>
      {data.caja && (
        <button className="gt-button" type="button" disabled={ocupado} onClick={onCaja}>
          Ir a Caja Chica
        </button>
      )}
      {data.token && (
        <form
          className="bkp-form"
          onSubmit={(e) => {
            e.preventDefault();
            const frase = token;
            setToken('');
            void onAccion('token', { frase, ruta: ruta || undefined }, 'Credencial guardada.');
          }}
        >
          <label>
            Token de NubeFact
            <input
              className="gt-input"
              type="password"
              required
              autoComplete="off"
              value={token}
              onChange={(e) => setToken(e.target.value)}
            />
          </label>
          <label>
            URL del API (solo si no está configurada)
            <input
              className="gt-input"
              type="url"
              value={ruta}
              onChange={(e) => setRuta(e.target.value)}
            />
          </label>
          <button className="gt-button gt-button--primary" disabled={ocupado}>
            Guardar credencial
          </button>
        </form>
      )}
      {data.series.map((s) => (
        <p key={s.id}>
          <strong>
            {s.tipo} {s.serie}
          </strong>{' '}
          · Local: {s.local} · Previo: {s.previo} · Real registrado: {s.remoto}
          {s.pendiente ? (
            <button
              className="gt-button"
              disabled={ocupado}
              onClick={() => {
                setSerie(s);
                setNumero('');
                setReferencia('');
              }}
            >
              Conciliar
            </button>
          ) : (
            ` · Siguiente: ${s.siguiente ?? 'Agotada'}`
          )}
        </p>
      ))}
      {serie && data.series.some((s) => s.id === serie.id && !!s.pendiente) && (
        <form
          className="bkp-form"
          onSubmit={(e) => {
            e.preventDefault();
            void onAccion(
              'conciliar',
              { id_serie: serie.id, ultimo_real: Number(numero), referencia },
              'Serie conciliada.'
            );
          }}
        >
          <label>
            Último número real de {serie.serie}
            <input
              className="gt-input"
              required
              type="number"
              min="0"
              max="99999999"
              step="1"
              value={numero}
              onChange={(e) => setNumero(e.target.value)}
            />
          </label>
          <label>
            Referencia de verificación
            <input
              className="gt-input"
              required
              minLength={5}
              maxLength={120}
              value={referencia}
              onChange={(e) => setReferencia(e.target.value)}
            />
          </label>
          <button className="gt-button gt-button--primary" disabled={ocupado}>
            Confirmar serie
          </button>
        </form>
      )}
      <button
        className="gt-button gt-button--primary"
        disabled={ocupado || data.caja || data.token || data.series.some((s) => !!s.pendiente)}
        onClick={() =>
          void onAccion('finalizar', {}, 'Recuperación completa. POS puede volver a operar.')
        }
      >
        Finalizar recuperación
      </button>
    </div>
  );
}
