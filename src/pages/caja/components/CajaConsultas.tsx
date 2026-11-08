import { useCallback, useEffect, useState, type ReactNode } from 'react';
import { cajaApi, mensajeCaja } from '../../../services/cajaService';
import type {
  CajaDetalle,
  CajaInicio,
  CajaMovimiento,
  CajaPagina,
  CajaTurno,
} from '../../../types/caja';
import { soles } from '../../pos/posUtils';

const tipos: Record<CajaMovimiento['tipo'], string> = {
  COBRO: 'Cobro',
  REINTEGRO: 'Reintegro',
  INGRESO_MANUAL: 'Ingreso manual',
  EGRESO_MANUAL: 'Egreso manual',
  REVERSO_MANUAL: 'Reversión',
};
const categorias: Record<string, string> = {
  APORTE_FONDO: 'Aporte de fondo',
  GASTO_MENOR: 'Gasto menor',
  RETIRO_PARCIAL: 'Retiro parcial',
  OTRO: 'Otro',
};
export const fechaCaja = (s: string) =>
  `${s.slice(0, 10).split('-').reverse().join('/')} · ${s.slice(11, 16)}`;
const cent = (n: number | null) => (n === null ? '—' : soles(n));

function usarPagina<T>(accion: string, payload: object, revision: number) {
  const [datos, setDatos] = useState<CajaPagina<T> | null>(null);
  const [error, setError] = useState('');
  const [recarga, recargar] = useState(0);
  useEffect(() => {
    let actual = true;
    setError('');
    setDatos(null);
    cajaApi<CajaPagina<T>>(accion, payload)
      .then((p) => {
        if (actual) setDatos(p);
      })
      .catch((e) => {
        if (actual) setError(mensajeCaja(e));
      });
    return () => {
      actual = false;
    };
  }, [accion, payload, revision, recarga]);
  return { datos, error, reintentar: () => recargar((v) => v + 1) };
}

function Resultados<T>({
  datos,
  error,
  reintentar,
  onPagina,
  children,
}: {
  datos: CajaPagina<T> | null;
  error: string;
  reintentar: () => void;
  onPagina: (p: number) => void;
  children: (d: T[]) => ReactNode;
}) {
  if (error)
    return (
      <div className="caja-alerta" role="alert">
        {error} <button onClick={reintentar}>Reintentar</button>
      </div>
    );
  if (!datos) return <p role="status">Cargando resultados…</p>;
  const ultima = Math.max(1, Math.ceil(datos.total / datos.por_pagina));
  return (
    <>
      {datos.datos.length ? (
        children(datos.datos)
      ) : (
        <p className="caja-vacio">No hay resultados para estos filtros.</p>
      )}
      <footer className="caja-paginas">
        <span>
          {datos.total} resultados · Página {datos.pagina} de {ultima}
        </span>
        <div className="caja-acciones">
          <button disabled={datos.pagina <= 1} onClick={() => onPagina(datos.pagina - 1)}>
            Anterior
          </button>
          <button disabled={datos.pagina >= ultima} onClick={() => onPagina(datos.pagina + 1)}>
            Siguiente
          </button>
        </div>
      </footer>
    </>
  );
}

export function CajaMovimientos({
  detalle,
  revision,
  onRevertir,
  onVenta,
}: {
  detalle: CajaDetalle;
  revision: number;
  onRevertir: (m: CajaMovimiento) => void;
  onVenta: (id: number) => void;
}) {
  const base = { origen: '', direccion: '', buscar: '' };
  const [filtros, setFiltros] = useState(base);
  const [aplicados, setAplicados] = useState({
    ...base,
    id_caja: detalle.turno.id_caja,
    pagina: 1,
  });
  function irMovimiento(id: number) {
    const nuevos = { ...base, buscar: `#${id}` };
    setFiltros(nuevos);
    setAplicados({ ...nuevos, id_caja: detalle.turno.id_caja, pagina: 1 });
  }
  const resultado = usarPagina<CajaMovimiento>('movimientos', aplicados, revision);
  return (
    <section className="caja-panel">
      <h2>Movimientos del turno</h2>
      <form
        className="caja-filtros"
        onSubmit={(e) => {
          e.preventDefault();
          setAplicados({ ...filtros, id_caja: detalle.turno.id_caja, pagina: 1 });
        }}
      >
        <label>
          Concepto o referencia
          <input
            type="search"
            value={filtros.buscar}
            onChange={(e) => setFiltros({ ...filtros, buscar: e.target.value })}
          />
        </label>
        <label>
          Origen
          <select
            value={filtros.origen}
            onChange={(e) => setFiltros({ ...filtros, origen: e.target.value })}
          >
            <option value="">Todos</option>
            <option value="POS">Punto de Venta</option>
            <option value="MANUAL">Manual</option>
          </select>
        </label>
        <label>
          Dirección
          <select
            value={filtros.direccion}
            onChange={(e) => setFiltros({ ...filtros, direccion: e.target.value })}
          >
            <option value="">Todas</option>
            <option value="INGRESO">Ingresos</option>
            <option value="EGRESO">Egresos</option>
          </select>
        </label>
        <button type="submit">Aplicar filtros</button>
      </form>
      <Resultados {...resultado} onPagina={(pagina) => setAplicados({ ...aplicados, pagina })}>
        {(lista) => (
          <div className="caja-tabla" tabIndex={0} role="region" aria-label="Movimientos de caja">
            <table>
              <thead>
                <tr>
                  <th>Fecha y hora</th>
                  <th>Movimiento</th>
                  <th>Medio</th>
                  <th>Importe</th>
                  <th>Referencia</th>
                  <th>Responsables</th>
                  <th>Acción</th>
                </tr>
              </thead>
              <tbody>
                {lista.map((m) => (
                  <tr key={m.id_movimiento}>
                    <td>{fechaCaja(m.fecha)}</td>
                    <td>
                      <strong>{tipos[m.tipo]}</strong>
                      <small>
                        {m.id_venta === null ? 'Manual' : 'POS'} · {m.concepto}
                      </small>
                      {m.categoria && <small>{categorias[m.categoria] ?? m.categoria}</small>}
                      {m.motivo && <small>Motivo: {m.motivo}</small>}
                      {m.id_movimiento_original !== null && (
                        <small>
                          <button
                            className="caja-enlace"
                            onClick={() => irMovimiento(m.id_movimiento_original!)}
                          >
                            Original #{m.id_movimiento_original}
                          </button>
                        </small>
                      )}
                      {m.id_reverso !== null && (
                        <small>
                          <button
                            className="caja-enlace"
                            onClick={() => irMovimiento(m.id_reverso!)}
                          >
                            Revertido por #{m.id_reverso}
                          </button>
                        </small>
                      )}
                    </td>
                    <td>{m.medio_pago}</td>
                    <td className="caja-numero">
                      {m.signo > 0 ? '+' : '−'}
                      {soles(m.monto_cent)}
                    </td>
                    <td>
                      {m.id_venta === null ? (
                        (m.referencia ?? '—')
                      ) : (
                        <button className="caja-enlace" onClick={() => onVenta(m.id_venta!)}>
                          {m.id_devolucion === null
                            ? `Venta #${m.id_venta}`
                            : `Devolución #${m.id_devolucion}`}
                        </button>
                      )}
                      {m.id_venta !== null && m.referencia && <small>{m.referencia}</small>}
                    </td>
                    <td>
                      {m.ejecutor}
                      {m.autorizante && <small>Autorizó: {m.autorizante}</small>}
                    </td>
                    <td>
                      {detalle.puede_mover &&
                        (m.tipo === 'INGRESO_MANUAL' || m.tipo === 'EGRESO_MANUAL') &&
                        m.id_reverso === null && (
                          <button onClick={() => onRevertir(m)}>Revertir</button>
                        )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </Resultados>
    </section>
  );
}

export function CajaHistorial({
  inicio,
  admin,
  revision,
  onAbrir,
}: {
  inicio: CajaInicio;
  admin: boolean;
  revision: number;
  onAbrir: (id: number) => void;
}) {
  const [filtros, setFiltros] = useState({
    desde: inicio.hoy,
    hasta: inicio.hoy,
    codigo: '',
    con_diferencia: false,
    id_usuario: '',
    estado: 'CERRADA',
  });
  const aPayload = useCallback(
    (f: typeof filtros) => ({
      ...f,
      id_usuario: f.id_usuario ? Number(f.id_usuario) : null,
      pagina: 1,
    }),
    []
  );
  const [aplicados, setAplicados] = useState(() => aPayload(filtros));
  const resultado = usarPagina<CajaTurno>('historial', aplicados, revision);
  return (
    <section className="caja-panel">
      <h2>Historial de turnos</h2>
      <form
        className="caja-filtros"
        onSubmit={(e) => {
          e.preventDefault();
          setAplicados(aPayload(filtros));
        }}
      >
        <label>
          Desde
          <input
            required
            type="date"
            value={filtros.desde}
            onChange={(e) => setFiltros({ ...filtros, desde: e.target.value })}
          />
        </label>
        <label>
          Hasta
          <input
            required
            type="date"
            value={filtros.hasta}
            onChange={(e) => setFiltros({ ...filtros, hasta: e.target.value })}
          />
        </label>
        <label>
          Código
          <input
            type="search"
            value={filtros.codigo}
            onChange={(e) => setFiltros({ ...filtros, codigo: e.target.value })}
          />
        </label>
        {admin && (
          <>
            <label>
              Responsable
              <select
                value={filtros.id_usuario}
                onChange={(e) => setFiltros({ ...filtros, id_usuario: e.target.value })}
              >
                <option value="">Todos</option>
                {inicio.responsables.map((u) => (
                  <option key={u.id} value={u.id}>
                    {u.nombre}
                  </option>
                ))}
              </select>
            </label>
            <label>
              Estado
              <select
                value={filtros.estado}
                onChange={(e) => setFiltros({ ...filtros, estado: e.target.value })}
              >
                <option value="CERRADA">Cerrada</option>
                <option value="ABIERTA">Abierta</option>
                <option value="TODAS">Todos</option>
              </select>
            </label>
          </>
        )}
        <label className="caja-check">
          <input
            type="checkbox"
            checked={filtros.con_diferencia}
            onChange={(e) => setFiltros({ ...filtros, con_diferencia: e.target.checked })}
          />{' '}
          Solo con diferencia
        </label>
        <button type="submit">Aplicar filtros</button>
      </form>
      <Resultados {...resultado} onPagina={(pagina) => setAplicados({ ...aplicados, pagina })}>
        {(lista) => (
          <div className="caja-tabla" tabIndex={0} role="region" aria-label="Historial de caja">
            <table>
              <thead>
                <tr>
                  <th>Turno</th>
                  <th>Responsable</th>
                  <th>Apertura</th>
                  <th>Cierre</th>
                  <th>Cerró</th>
                  <th>Diferencia</th>
                  <th>Estado</th>
                  <th>Acción</th>
                </tr>
              </thead>
              <tbody>
                {lista.map((c) => (
                  <tr key={c.id_caja}>
                    <td>
                      <strong>{c.codigo_caja}</strong>
                      <small>{c.ambiente}</small>
                    </td>
                    <td>{c.titular}</td>
                    <td>{fechaCaja(c.fecha_apertura)}</td>
                    <td>{c.fecha_cierre ? fechaCaja(c.fecha_cierre) : '—'}</td>
                    <td>{c.quien_cerro ?? '—'}</td>
                    <td>{cent(c.diferencia_efectivo_cent)}</td>
                    <td>
                      {c.estado === 'ABIERTA'
                        ? c.por_conciliar
                          ? 'Por conciliar'
                          : 'Abierta'
                        : 'Cerrada'}
                    </td>
                    <td>
                      <button onClick={() => onAbrir(c.id_caja)}>Ver turno</button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </Resultados>
    </section>
  );
}
