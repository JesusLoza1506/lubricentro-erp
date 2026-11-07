import { useEffect, useRef, useState } from 'react';
import type { OtPos, PaginaPos } from '../../../types/pos';
import { mensajePos, posApi } from '../../../services/posService';
import { ModalPos } from './ModalPos';
export function CargarOtPos({
  onCerrar,
  onSeleccionar,
}: {
  onCerrar: () => void;
  onSeleccionar: (ot: OtPos) => Promise<void>;
}) {
  const [buscar, Sbuscar] = useState(''),
    [pagina, Spagina] = useState(1),
    [datos, Sdatos] = useState<PaginaPos<OtPos>>({ filas: [], total: 0 }),
    [error, Serror] = useState(''),
    [ocupado, Socupado] = useState(false);
  const seq = useRef(0);
  useEffect(() => {
    const id = ++seq.current;
    const t = setTimeout(() => {
      posApi<PaginaPos<OtPos>>('ots', { buscar, pagina })
        .then((r) => {
          if (id === seq.current) Sdatos(r);
        })
        .catch((e) => {
          if (id === seq.current) Serror(mensajePos(e));
        });
    }, 200);
    return () => {
      clearTimeout(t);
      seq.current++;
    };
  }, [buscar, pagina]);
  return (
    <ModalPos titulo="Cobrar una orden finalizada" onCerrar={onCerrar} ocupado={ocupado} ancho>
      <p className="pos-hint">
        Sus líneas conservan precios históricos. Los insumos ya utilizados no se descontarán
        nuevamente.
      </p>
      <input
        aria-label="Buscar orden finalizada"
        placeholder="Código OT o placa"
        value={buscar}
        onChange={(e) => {
          Sbuscar(e.target.value);
          Spagina(1);
        }}
      />
      {error && (
        <p role="alert" className="pos-error">
          {error}
        </p>
      )}
      <div className="pos-table-wrap">
        <table>
          <thead>
            <tr>
              <th>Orden</th>
              <th>Vehículo</th>
              <th>Finalización</th>
              <th />
            </tr>
          </thead>
          <tbody>
            {datos.filas.map((ot) => (
              <tr key={ot.id}>
                <td>
                  <strong>{ot.codigo}</strong>
                </td>
                <td>{ot.placa}</td>
                <td>{ot.fecha}</td>
                <td>
                  <button
                    disabled={ocupado}
                    onClick={async () => {
                      Socupado(true);
                      try {
                        await onSeleccionar(ot);
                      } catch (e) {
                        Serror(mensajePos(e));
                      } finally {
                        Socupado(false);
                      }
                    }}
                  >
                    Seleccionar
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
        {!datos.filas.length && (
          <p className="pos-empty">No hay órdenes finalizadas pendientes para esta búsqueda.</p>
        )}
      </div>
      <footer className="pos-pagination">
        <button disabled={pagina === 1 || ocupado} onClick={() => Spagina((n) => n - 1)}>
          Anterior
        </button>
        <span>
          Página {pagina} · {datos.total} órdenes
        </span>
        <button
          disabled={pagina * 20 >= datos.total || ocupado}
          onClick={() => Spagina((n) => n + 1)}
        >
          Siguiente
        </button>
      </footer>
    </ModalPos>
  );
}
