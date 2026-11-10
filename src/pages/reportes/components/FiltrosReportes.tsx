import { Filter, RotateCcw, Search } from 'lucide-react';
import { REPORTES, type Filtros, type Reporte, type UsuarioFiltro } from '../reportesApi';

const MEDIOS = [
  ['EFECTIVO', 'Efectivo'],
  ['TARJETA', 'Tarjeta'],
  ['YAPE', 'Yape'],
  ['PLIN', 'Plin'],
  ['TRANSFERENCIA', 'Transferencia'],
];
const MODULOS = [
  ['LOGIN', 'Login'],
  ['DASHBOARD', 'Inicio'],
  ['FICHA_VEHICULAR', 'Ficha Vehicular'],
  ['OT', 'Taller'],
  ['INVENTARIO', 'Inventario'],
  ['POS', 'Punto de Venta'],
  ['CAJA', 'Caja Chica'],
  ['BACKUP', 'Backup y Copias'],
  ['ADMIN', 'Administración'],
  ['REPORTES', 'Reportes'],
];
const PERIODOS = [
  ['hoy', 'Hoy'],
  ['ayer', 'Ayer'],
  ['esta_semana', 'Esta semana'],
  ['este_mes', 'Este mes'],
  ['mes_anterior', 'Mes anterior'],
  ['personalizado', 'Personalizado'],
];
const ESTADOS = [
  ['PENDIENTE', 'Pendiente'],
  ['ENVIANDO', 'Enviando'],
  ['ERROR', 'Error'],
  ['ACEPTADO', 'Aceptado'],
  ['RECHAZADO', 'Rechazado'],
];

function Selector({
  nombre,
  value,
  opciones,
  cambiar,
}: {
  nombre: string;
  value: string;
  opciones: readonly (readonly string[])[];
  cambiar: (s: string) => void;
}) {
  return (
    <label>
      {nombre}
      <select value={value} onChange={(e) => cambiar(e.currentTarget.value)}>
        {opciones.map(([k, v]) => (
          <option key={k} value={k}>
            {v}
          </option>
        ))}
      </select>
    </label>
  );
}

export function FiltrosReportes({
  reporte,
  filtros: f,
  usuarios,
  ocupado,
  onReporte,
  onFiltros,
  onConsultar,
  onLimpiar,
}: {
  reporte: Reporte;
  filtros: Filtros;
  usuarios: UsuarioFiltro[];
  ocupado: boolean;
  onReporte: (r: Reporte) => void;
  onFiltros: (f: Filtros) => void;
  onConsultar: () => void;
  onLimpiar: () => void;
}) {
  const cambio = <K extends keyof Filtros>(k: K, v: Filtros[K]) => onFiltros({ ...f, [k]: v });
  const todos = (opciones: readonly (readonly string[])[]) => [['', 'Todos'], ...opciones];
  const check = (nombre: string, k: 'ver_todos' | 'solo_criticos' | 'con_diferencia') => (
    <label className="rep-check">
      <input
        type="checkbox"
        checked={!!f[k]}
        onChange={(e) => cambio(k, e.currentTarget.checked)}
      />
      {nombre}
    </label>
  );
  const busca = (label: string, ejemplo: string) => (
    <label className="rep-busqueda">
      {label}
      <input
        type="search"
        value={f.buscar ?? ''}
        maxLength={100}
        placeholder={ejemplo}
        onChange={(e) => cambio('buscar', e.currentTarget.value)}
      />
    </label>
  );
  return (
    <form
      className="rep-panel rep-filtros"
      onSubmit={(e) => {
        e.preventDefault();
        onConsultar();
      }}
    >
      <h2>
        <Filter size={17} aria-hidden="true" /> Consulta
      </h2>
      <fieldset disabled={ocupado} aria-label="Filtros de consulta">
        <div className="rep-campos">
          <Selector
            nombre="Reporte"
            value={reporte}
            opciones={Object.entries(REPORTES).map(([k, v]) => [k, `${k} · ${v}`])}
            cambiar={(s) => onReporte(s as Reporte)}
          />
          {reporte !== 'R04' && (
            <>
              <Selector
                nombre="Periodo"
                value={f.periodo}
                opciones={PERIODOS}
                cambiar={(v) =>
                  onFiltros({
                    ...f,
                    periodo: v as Filtros['periodo'],
                    desde: undefined,
                    hasta: undefined,
                  })
                }
              />
              {f.periodo === 'personalizado' && (
                <>
                  <label>
                    Desde
                    <input
                      type="date"
                      required
                      value={f.desde ?? ''}
                      onChange={(e) => cambio('desde', e.currentTarget.value || undefined)}
                    />
                  </label>
                  <label>
                    Hasta
                    <input
                      type="date"
                      required
                      value={f.hasta ?? ''}
                      onChange={(e) => cambio('hasta', e.currentTarget.value || undefined)}
                    />
                  </label>
                </>
              )}
            </>
          )}
          {reporte === 'R02' && (
            <Selector
              nombre="Medio de pago"
              value={f.medio ?? ''}
              opciones={todos(MEDIOS)}
              cambiar={(s) => cambio('medio', s || undefined)}
            />
          )}
          {reporte === 'R03' && (
            <>
              <Selector
                nombre="Tipo de ítem"
                value={f.tipo ?? ''}
                opciones={[
                  ['', 'Productos'],
                  ['SERVICIOS', 'Servicios'],
                ]}
                cambiar={(s) => cambio('tipo', s || undefined)}
              />
              {check('Ver todos', 'ver_todos')}
            </>
          )}
          {reporte === 'R04' && (
            <>
              {busca('Código o nombre', 'Ej. Filtro de aceite')}
              <Selector
                nombre="Categoría"
                value={f.categoria ?? ''}
                opciones={[
                  ['', 'Todas'],
                  ['ACEITE', 'Aceite'],
                  ['FILTRO', 'Filtro'],
                  ['ADITIVO', 'Aditivo'],
                  ['GRANEL', 'Granel'],
                  ['OTRO', 'Otro'],
                ]}
                cambiar={(s) => cambio('categoria', s || undefined)}
              />
              <Selector
                nombre="Estado"
                value={f.estado ?? ''}
                opciones={[
                  ['', 'Ambos'],
                  ['ACTIVOS', 'Activos'],
                  ['INACTIVOS', 'Inactivos con stock'],
                ]}
                cambiar={(s) => cambio('estado', s || undefined)}
              />
              {check('Solo críticos', 'solo_criticos')}
            </>
          )}
          {(reporte === 'R05' || reporte === 'R06') && (
            <>
              {reporte === 'R06' && (
                <Selector
                  nombre="Actividad"
                  value={f.actividad ?? ''}
                  opciones={[
                    ['', 'Finalizadas'],
                    ['RECIBIDAS', 'Recibidas'],
                  ]}
                  cambiar={(s) => cambio('actividad', s || undefined)}
                />
              )}
              <Selector
                nombre={reporte === 'R05' ? 'Titular' : 'Responsable'}
                value={String(f.usuario ?? '')}
                opciones={todos(
                  usuarios
                    .filter((u) => reporte !== 'R06' || u.responsable_ot)
                    .map((u) => [String(u.id), u.nombre])
                )}
                cambiar={(s) => cambio('usuario', s ? Number(s) : undefined)}
              />
              {reporte === 'R05' && check('Con diferencia', 'con_diferencia')}
            </>
          )}
          {reporte === 'R07' && (
            <>
              <Selector
                nombre="Estado del comprobante"
                value={f.estado ?? ''}
                opciones={todos(ESTADOS)}
                cambiar={(s) => cambio('estado', s || undefined)}
              />
              {busca('Serie o número', 'Ej. B001-00000042')}
            </>
          )}
          {reporte === 'R08' && (
            <>
              <Selector
                nombre="Actor"
                value={f.actor ?? ''}
                opciones={todos([
                  ['SISTEMA', 'Sistema'],
                  ['NO_IDENTIFICADO', 'No identificado'],
                  ...usuarios.map((u) => [String(u.id), u.nombre]),
                ])}
                cambiar={(s) => cambio('actor', s || undefined)}
              />
              <Selector
                nombre="Módulo"
                value={f.modulo ?? ''}
                opciones={todos(MODULOS)}
                cambiar={(s) => cambio('modulo', s || undefined)}
              />
            </>
          )}
        </div>
        <div className="rep-acciones">
          <button type="submit" className="gt-button gt-button--primary">
            <Search size={16} aria-hidden="true" /> Consultar
          </button>
          <button type="button" className="gt-button gt-button--quiet" onClick={onLimpiar}>
            <RotateCcw size={16} aria-hidden="true" /> Limpiar
          </button>
        </div>
      </fieldset>
    </form>
  );
}
