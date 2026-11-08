import { Pencil } from 'lucide-react';
import { ServicioDTO } from '../../../types/inventario';
import { MenuAccionesInventario } from './MenuAccionesInventario';
interface ServiciosTablaProps {
  cargando: boolean;
  servicios: ServicioDTO[];
  esAdmin: boolean;
  onEditar: (servicio: ServicioDTO) => void;
  onToggleEstado: (servicio: ServicioDTO) => void;
  onNuevoServicio: () => void;
}

export function ServiciosTabla({
  cargando,
  servicios,
  esAdmin,
  onEditar,
  onToggleEstado,
}: ServiciosTablaProps) {
  return (
    <section className="inv-table-section gt-stage" aria-label="Catálogo de servicios">
      <header className="inv-list-heading">
        <h3>Catálogo de servicios</h3>
        <p>Consulta precios y disponibilidad de los servicios del taller.</p>
      </header>
      <div
        className="inv-table-scroll"
        tabIndex={0}
        role="region"
        aria-label="Resultados del catálogo de servicios"
        aria-busy={cargando}
      >
        <table className="inv-table inv-services-table">
          <thead>
            <tr>
              <th>Código</th>
              <th>Servicio</th>
              <th>Descripción</th>
              <th className="inv-number">Precio venta</th>
              <th>Tipo</th>
              <th>Estado</th>
              {esAdmin && <th style={{ textAlign: 'right' }}>Acciones</th>}
            </tr>
          </thead>
          <tbody>
            {cargando ? (
              <tr>
                <td className="inv-empty" colSpan={esAdmin ? 7 : 6}>
                  Cargando servicios...
                </td>
              </tr>
            ) : !servicios.length ? (
              <tr>
                <td className="inv-empty" colSpan={esAdmin ? 7 : 6}>
                  No hay servicios que coincidan con la consulta.
                </td>
              </tr>
            ) : (
              servicios.map((s) => (
                <tr key={s.id_servicio} style={{ opacity: s.activo ? 1 : 0.65 }}>
                  <td className="inv-code">{s.codigo}</td>
                  <td className="inv-name">{s.nombre}</td>
                  <td>
                    <details className="inv-description">
                      <summary>{s.descripcion || 'Sin descripción'}</summary>
                    </details>
                  </td>
                  <td className="inv-number">S/ {(s.precio_venta_cent / 100).toFixed(2)}</td>
                  <td>
                    {s.es_cambio_aceite ? (
                      <span className="inv-badge is-warning">Cambio de aceite</span>
                    ) : (
                      <span className="inv-meta">General</span>
                    )}
                  </td>
                  <td>
                    <span className={`inv-badge ${s.activo ? '' : 'is-muted'}`}>
                      {s.activo ? 'Activo' : 'Inactivo'}
                    </span>
                  </td>
                  {esAdmin && (
                    <td>
                      <div className="inv-row-actions">
                        <button className="inv-button" onClick={() => onEditar(s)}>
                          <Pencil size={14} />
                          Editar
                        </button>
                        <MenuAccionesInventario
                          nombre={s.nombre}
                          acciones={[
                            {
                              texto: s.activo ? 'Desactivar servicio' : 'Reactivar servicio',
                              peligro: s.activo,
                              ejecutar: () => onToggleEstado(s),
                            },
                          ]}
                        />
                      </div>
                    </td>
                  )}
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
      <footer className="inv-table-footer">
        <span>
          Servicios mostrados: <strong>{servicios.length}</strong>
        </span>
        <span>Catálogo de atención del taller</span>
      </footer>
    </section>
  );
}
