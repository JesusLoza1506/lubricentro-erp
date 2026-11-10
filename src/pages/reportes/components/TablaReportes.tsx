import { ArrowUpRight, ChevronLeft, ChevronRight, ClipboardList } from 'lucide-react';
import { type Detalle, type Origen, valor } from '../reportesApi';

export const DESTINOS: Record<Origen['destino'], string> = {
  pos: 'Punto de Venta',
  caja: 'Caja Chica',
  inventario: 'Inventario',
  taller: 'Taller',
  administracion: 'Administración',
  backup: 'Backup y Copias',
};

export function TablaReportes({
  datos,
  nombre,
  ocupado,
  ver,
  abrir,
}: {
  datos: Detalle;
  nombre: string;
  ocupado: boolean;
  ver?: (fila: number, control: HTMLButtonElement) => void;
  abrir: (fila: number, indice: number) => void;
}) {
  if (!datos.filas.length)
    return (
      <div className="rep-vacio">
        <ClipboardList size={25} aria-hidden="true" />
        <strong>Sin resultados</strong>
        <p>No hay registros que coincidan con los filtros aplicados.</p>
      </div>
    );
  return (
    <div className="rep-tabla-scroll" role="region" aria-label={nombre} tabIndex={0}>
      <table className="rep-tabla" aria-label={nombre}>
        <thead>
          <tr>
            {datos.columnas.map((k) => (
              <th key={k} scope="col">
                {k}
              </th>
            ))}
            <th scope="col">Acciones</th>
          </tr>
        </thead>
        <tbody>
          {datos.filas.map((f, i) => (
            <tr key={i}>
              {f.celdas.map((v, j) => (
                <td key={j} data-numero={v.tipo === 'numero' || undefined}>
                  {valor(v)}
                </td>
              ))}
              <td>
                <div className="rep-acciones-fila">
                  {ver && (
                    <button
                      type="button"
                      className="gt-button gt-button--quiet"
                      disabled={ocupado}
                      onClick={(e) => ver(i, e.currentTarget)}
                    >
                      Ver detalle
                    </button>
                  )}
                  {f.origenes.map((o, k) => (
                    <button
                      type="button"
                      className="gt-button gt-button--quiet"
                      key={k}
                      disabled={ocupado}
                      onClick={() => abrir(i, k)}
                    >
                      <ArrowUpRight size={14} aria-hidden="true" /> {DESTINOS[o.destino]}
                    </button>
                  ))}
                  {!ver && !f.origenes.length && '—'}
                </div>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export function Paginacion({
  pagina,
  paginas,
  total,
  ocupado,
  ir,
}: {
  pagina: number;
  paginas: number;
  total: number;
  ocupado: boolean;
  ir: (p: number) => void;
}) {
  return (
    <div className="rep-pie">
      <span>
        {total} {total === 1 ? 'fila' : 'filas'}
      </span>
      <nav aria-label="Paginación del resultado">
        <button
          type="button"
          className="gt-button gt-button--quiet"
          disabled={ocupado || pagina <= 1}
          onClick={() => ir(pagina - 1)}
        >
          <ChevronLeft size={15} aria-hidden="true" /> Anterior
        </button>
        <span>
          Página {pagina} de {paginas}
        </span>
        <button
          type="button"
          className="gt-button gt-button--quiet"
          disabled={ocupado || pagina >= paginas}
          onClick={() => ir(pagina + 1)}
        >
          Siguiente <ChevronRight size={15} aria-hidden="true" />
        </button>
      </nav>
    </div>
  );
}
