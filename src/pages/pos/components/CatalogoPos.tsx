import { useEffect, useRef, useState } from 'react';
import { Search, Plus, RefreshCw } from 'lucide-react';
import type { PaginaPos, ProductoPos } from '../../../types/pos';
import { posApi, mensajePos } from '../../../services/posService';
import { cantidad, soles } from '../posUtils';
export function CatalogoPos({ onAgregar }: { onAgregar: (p: ProductoPos) => void }) {
  const [buscar, Sbuscar] = useState(''),
    [tipo, Stipo] = useState('TODOS'),
    [pagina, Spagina] = useState(1),
    [datos, Sdatos] = useState<PaginaPos<ProductoPos>>({ filas: [], total: 0 }),
    [error, Serror] = useState(''),
    [cargando, Scargando] = useState(false),
    [revision, Srevision] = useState(0);
  const sec = useRef(0);
  useEffect(() => {
    const id = ++sec.current;
    Scargando(true);
    const t = setTimeout(() => {
      posApi<PaginaPos<ProductoPos>>('catalogo', { buscar, tipo, pagina })
        .then((r) => {
          if (id === sec.current) {
            Sdatos(r);
            Serror('');
          }
        })
        .catch((e) => {
          if (id === sec.current) Serror(mensajePos(e));
        })
        .finally(() => {
          if (id === sec.current) Scargando(false);
        });
    }, 200);
    return () => {
      clearTimeout(t);
      sec.current++;
    };
  }, [buscar, tipo, pagina, revision]);
  return (
    <section className="pos-panel">
      <div className="pos-panel-heading">
        <div>
          <h2>Productos y servicios</h2>
          <p>Agrega solo los adicionales de venta directa.</p>
        </div>
        <button
          className="pos-icon"
          aria-label="Actualizar catálogo"
          onClick={() => Srevision((n) => n + 1)}
        >
          <RefreshCw size={17} />
        </button>
      </div>
      <div className="pos-toolbar">
        <div className="pos-search">
          <Search size={18} />
          <input
            aria-label="Buscar catálogo"
            placeholder="Código o nombre del producto…"
            value={buscar}
            onChange={(e) => {
              Sbuscar(e.target.value);
              Spagina(1);
            }}
          />
        </div>
        <select
          aria-label="Tipo de catálogo"
          value={tipo}
          onChange={(e) => {
            Stipo(e.target.value);
            Spagina(1);
          }}
        >
          <option value="TODOS">Todos</option>
          <option value="PRODUCTO">Productos</option>
          <option value="SERVICIO">Servicios</option>
        </select>
      </div>
      {error ? (
        <div role="alert" className="pos-error">
          {error}
          <button onClick={() => Srevision((n) => n + 1)}>Reintentar</button>
        </div>
      ) : (
        <>
          <div className="pos-table-wrap" aria-busy={cargando}>
            <table>
              <thead>
                <tr>
                  <th>Producto / servicio</th>
                  <th>Disponible</th>
                  <th>Precio</th>
                  <th />
                </tr>
              </thead>
              <tbody>
                {datos.filas.map((p) => (
                  <tr key={`${p.tipo}-${p.id}`}>
                    <td>
                      <strong>{p.nombre}</strong>
                      <small>
                        {p.codigo} · {p.tipo === 'SERVICIO' ? 'Servicio' : 'Producto'}
                      </small>
                    </td>
                    <td>
                      {p.stock === null ? (
                        '—'
                      ) : (
                        <span className={p.stock === 0 ? 'pos-danger' : ''}>
                          {cantidad(p.stock, p.unidad)}
                        </span>
                      )}
                    </td>
                    <td className="pos-money">
                      {soles(p.precio_cent)}
                      <small>por {p.unidad === 'ML' ? 'litro' : 'unidad'}</small>
                    </td>
                    <td>
                      <button
                        className="pos-icon"
                        aria-label={`Agregar ${p.nombre}`}
                        disabled={cargando || p.stock === 0}
                        onClick={() => onAgregar(p)}
                      >
                        <Plus size={17} />
                      </button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
            {!datos.filas.length && !cargando && (
              <p className="pos-empty">No hay coincidencias. Prueba otra búsqueda.</p>
            )}
          </div>
          <footer className="pos-pagination">
            <span>{cargando ? 'Consultando…' : `${datos.total} resultados`}</span>
            <div>
              <button disabled={pagina === 1 || cargando} onClick={() => Spagina((n) => n - 1)}>
                Anterior
              </button>
              <span>
                {pagina} / {Math.max(1, Math.ceil(datos.total / 20))}
              </span>
              <button
                disabled={pagina * 20 >= datos.total || cargando}
                onClick={() => Spagina((n) => n + 1)}
              >
                Siguiente
              </button>
            </div>
          </footer>
        </>
      )}
    </section>
  );
}
