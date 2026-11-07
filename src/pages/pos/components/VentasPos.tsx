import { useEffect, useRef, useState } from 'react';
import type { FilaVentaPos, PaginaPos } from '../../../types/pos';
import { mensajePos, posApi } from '../../../services/posService';
import { soles } from '../posUtils';
export function VentasPos({
  hoy,
  admin,
  revision,
  onAbrir,
  onPendientes,
}: {
  hoy: string;
  admin: boolean;
  revision: number;
  onAbrir: (id: number) => void;
  onPendientes: () => Promise<void>;
}) {
  const [desde, Sdesde] = useState(hoy),
    [hasta, Shasta] = useState(hoy),
    [buscar, Sbuscar] = useState(''),
    [estado, Sestado] = useState(''),
    [emisor, Semisor] = useState(''),
    [problemas, Sproblemas] = useState(false),
    [pagina, Spagina] = useState(1),
    [aplicados, Saplicados] = useState({
      desde: hoy,
      hasta: hoy,
      buscar: '',
      estado: '',
      emisor: 0,
      problemas: false,
      pagina: 1,
    }),
    [datos, Sdatos] = useState<PaginaPos<FilaVentaPos>>({ filas: [], total: 0 }),
    [error, Serror] = useState(''),
    [cargando, Scargando] = useState(false);
  const seq = useRef(0);
  useEffect(() => {
    const id = ++seq.current;
    Scargando(true);
    posApi<PaginaPos<FilaVentaPos>>('listado', { ...aplicados, pagina })
      .then((r) => {
        if (id === seq.current) {
          Sdatos(r);
          Serror('');
        }
      })
      .catch((e) => {
        if (id === seq.current) Serror(mensajePos(e));
      })
      .finally(() => {
        if (id === seq.current) Scargando(false);
      });
    return () => {
      seq.current++;
    };
  }, [aplicados, pagina, revision]);
  return (
    <section className="pos-panel">
      <div className="pos-panel-heading">
        <div>
          <h2>Ventas y comprobantes</h2>
          <p>
            {admin
              ? 'Consulta general y atención de documentos.'
              : 'Tus ventas, pagos y documentos vinculados.'}
          </p>
        </div>
        {admin && (
          <button onClick={() => void onPendientes()} disabled={cargando}>
            Revisar pendientes · hasta 20
          </button>
        )}
      </div>
      <form
        className="pos-filters"
        onSubmit={(e) => {
          e.preventDefault();
          Spagina(1);
          Saplicados({
            desde,
            hasta,
            buscar,
            estado,
            emisor: Number(emisor) || 0,
            problemas,
            pagina: 1,
          });
        }}
      >
        <label>
          Desde
          <input required type="date" value={desde} onChange={(e) => Sdesde(e.target.value)} />
        </label>
        <label>
          Hasta
          <input required type="date" value={hasta} onChange={(e) => Shasta(e.target.value)} />
        </label>
        <label>
          Venta, documento o cliente
          <input
            value={buscar}
            onChange={(e) => Sbuscar(e.target.value)}
            placeholder="Serie-número o nombre"
          />
        </label>
        <label>
          Estado electrónico
          <select value={estado} onChange={(e) => Sestado(e.target.value)}>
            <option value="">Todos</option>
            {['PENDIENTE', 'ENVIANDO', 'ERROR', 'ACEPTADO', 'RECHAZADO'].map((s) => (
              <option key={s}>{s}</option>
            ))}
          </select>
        </label>
        {admin && (
          <>
            <label>
              ID del emisor · opcional
              <input
                type="number"
                min="1"
                value={emisor}
                onChange={(e) => Semisor(e.target.value)}
              />
            </label>
            <label className="pos-check">
              <input
                type="checkbox"
                checked={problemas}
                onChange={(e) => Sproblemas(e.target.checked)}
              />
              Con problemas
            </label>
          </>
        )}
        <button type="submit" className="pos-primary">
          Consultar
        </button>
      </form>
      {error ? (
        <div role="alert" className="pos-error">
          {error}
          <button onClick={() => Saplicados((p) => ({ ...p }))}>Reintentar</button>
        </div>
      ) : (
        <div className="pos-table-wrap">
          <table aria-busy={cargando}>
            <thead>
              <tr>
                <th>Venta / fecha</th>
                <th>Cliente</th>
                <th>Origen</th>
                <th>Total</th>
                <th>Devuelto</th>
                <th>Documento</th>
                <th />
              </tr>
            </thead>
            <tbody>
              {datos.filas.map((v) => (
                <tr key={v.id}>
                  <td>
                    <strong>V-{v.id}</strong>
                    <small>{v.fecha}</small>
                  </td>
                  <td>
                    {v.cliente}
                    <small>{v.emisor}</small>
                  </td>
                  <td>{v.id_ot ? `OT #${v.id_ot}` : 'Directa'}</td>
                  <td>{soles(v.total_cent)}</td>
                  <td>{soles(v.devuelto_cent)}</td>
                  <td>
                    <span className={`pos-badge ${v.estado === 'ACEPTADO' ? 'pos-ok' : ''}`}>
                      {v.estado}
                    </span>
                  </td>
                  <td>
                    <button disabled={cargando} onClick={() => onAbrir(v.id)}>
                      Ver venta
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
          {!datos.filas.length && !cargando && (
            <p className="pos-empty">No hay ventas para los filtros seleccionados.</p>
          )}
        </div>
      )}
      <footer className="pos-pagination">
        <span>
          {datos.total} ventas · página {pagina}
        </span>
        <div>
          <button disabled={pagina === 1 || cargando} onClick={() => Spagina((n) => n - 1)}>
            Anterior
          </button>
          <button
            disabled={pagina * 20 >= datos.total || cargando}
            onClick={() => Spagina((n) => n + 1)}
          >
            Siguiente
          </button>
        </div>
      </footer>
    </section>
  );
}
