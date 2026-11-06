import { Eye, Pencil, ChevronLeft, ChevronRight } from 'lucide-react';
import { Producto, ProductoAdminDTO } from '../../../types/inventario';
import { MenuAccionesInventario } from './MenuAccionesInventario';

export interface InventarioTablaProps {
  cargando: boolean;
  productos: Producto[];
  esAdmin: boolean;
  paginaActual: number;
  totalPaginas: number;
  totalRegistros: number;
  busqueda?: string;
  esVistaCriticos?: boolean;
  onCambiarPagina: (nuevaPagina: number) => void;
  onVer: (p: Producto) => void;
  onAjustar: (p: Producto) => void;
  onVerKardex: (p: Producto) => void;
  onEditar: (p: Producto) => void;
  onEliminar: (id: number, nombre: string, activo: boolean) => void;
}

export function InventarioTabla({
  cargando,
  productos,
  esAdmin,
  paginaActual,
  totalPaginas,
  totalRegistros,
  busqueda,
  esVistaCriticos,
  onCambiarPagina,
  onVer,
  onAjustar,
  onVerKardex,
  onEditar,
  onEliminar,
}: InventarioTablaProps) {
  const columnas = (esAdmin ? 9 : 8) + (esVistaCriticos ? 2 : 0);
  return (
    <section className="inv-table-section" aria-label="Catálogo de productos">
      <div className="inv-table-scroll">
        <table className="inv-table">
          <thead>
            <tr>
              <th>Código</th>
              <th>Producto</th>
              <th>Categoría</th>
              <th>Unidad</th>
              {esAdmin && (
                <th
                  className="inv-number"
                  title="Promedio actual por UND o litro; sin existencias se muestra el último costo conocido"
                >
                  Costo / UND o L
                </th>
              )}
              <th className="inv-number">Precio venta</th>
              <th className="inv-number">Stock</th>
              {esVistaCriticos && (
                <>
                  <th className="inv-number">Mínimo</th>
                  <th className="inv-number">Faltante</th>
                </>
              )}
              <th>Estado</th>
              <th style={{ textAlign: 'right' }}>Acciones</th>
            </tr>
          </thead>
          <tbody>
            {cargando ? (
              <tr>
                <td className="inv-empty" colSpan={columnas}>
                  Cargando productos...
                </td>
              </tr>
            ) : productos.length === 0 ? (
              <tr>
                <td className="inv-empty" colSpan={columnas}>
                  {esVistaCriticos
                    ? 'No se encontraron productos en estado crítico de stock.'
                    : busqueda?.trim()
                      ? `No se encontraron productos que coincidan con "${busqueda}".`
                      : 'No se encontraron productos en el catálogo.'}
                </td>
              </tr>
            ) : (
              productos.map((p) => {
                const esCritico = p.es_critico ?? p.stock_actual <= p.stock_minimo;
                const admin = p as ProductoAdminDTO;
                const costoBase =
                  p.stock_actual > 0 && typeof admin.valor_stock_micro === 'number'
                    ? admin.valor_stock_micro / p.stock_actual
                    : admin.ultimo_costo_micro;
                const costo =
                  esAdmin && costoBase
                    ? ((costoBase * (p.unidad_medida === 'ML' ? 1000 : 1)) / 1000000).toFixed(4)
                    : null;
                const cantidad = (n: number) =>
                  p.unidad_medida === 'ML' ? `${(n / 1000).toFixed(3)} L` : `${n} UND`;
                const activo = Boolean(p.activo);
                return (
                  <tr key={p.id_producto} style={{ opacity: activo ? 1 : 0.65 }}>
                    <td className="inv-code">{p.codigo}</td>
                    <td className="inv-name">
                      {p.nombre}
                      {!activo && <span className="inv-muted-line">Producto inactivo</span>}
                    </td>
                    <td className="inv-meta">{p.categoria.toLowerCase()}</td>
                    <td className="inv-meta">{p.unidad_medida === 'ML' ? 'L' : 'UND'}</td>
                    {esAdmin && (
                      <td className="inv-number">
                        {costo ? `S/ ${costo}` : <span className="inv-meta">Sin costo</span>}
                      </td>
                    )}
                    <td className="inv-number">S/ {(p.precio_venta_cent / 100).toFixed(2)}</td>
                    <td className="inv-number">
                      <strong>{cantidad(p.stock_actual)}</strong>
                    </td>
                    {esVistaCriticos && (
                      <>
                        <td className="inv-number">{cantidad(p.stock_minimo)}</td>
                        <td className="inv-number">
                          {cantidad(Math.max(p.stock_minimo - p.stock_actual, 0))}
                        </td>
                      </>
                    )}
                    <td>
                      <span
                        className={`inv-badge ${!activo ? 'is-muted' : p.stock_actual === 0 ? 'is-danger' : esCritico ? 'is-warning' : ''}`}
                      >
                        {!activo
                          ? 'Inactivo'
                          : p.stock_actual === 0
                            ? 'Sin stock'
                            : esCritico
                              ? 'Crítico'
                              : 'Disponible'}
                      </span>
                    </td>
                    <td>
                      <div className="inv-row-actions">
                        <button
                          className="inv-icon-button"
                          title="Ver Detalle"
                          aria-label={`Ver detalle de ${p.nombre}`}
                          onClick={() => onVer(p)}
                        >
                          <Eye size={16} />
                        </button>
                        {esAdmin && (
                          <>
                            <button
                              className="inv-icon-button"
                              title="Editar Producto"
                              aria-label={`Editar ${p.nombre}`}
                              onClick={() => onEditar(p)}
                            >
                              <Pencil size={15} />
                            </button>
                            <MenuAccionesInventario
                              nombre={p.nombre}
                              acciones={[
                                {
                                  texto: 'Ver Kardex',
                                  titulo: 'Ver Historial Kárdex',
                                  ejecutar: () => onVerKardex(p),
                                },
                                {
                                  texto: 'Conteo físico',
                                  titulo: 'Conteo Físico Reconciliado',
                                  ejecutar: () => onAjustar(p),
                                },
                                {
                                  texto: activo ? 'Desactivar producto' : 'Reactivar producto',
                                  titulo: activo ? 'Dar de Baja Producto' : 'Reactivar Producto',
                                  peligro: activo,
                                  ejecutar: () => onEliminar(p.id_producto, p.nombre, activo),
                                },
                              ]}
                            />
                          </>
                        )}
                      </div>
                    </td>
                  </tr>
                );
              })
            )}
          </tbody>
        </table>
      </div>
      <footer className="inv-table-footer">
        <span>
          Total de productos: <strong>{totalRegistros}</strong>
        </span>
        <div className="inv-pagination">
          <span>
            Página <strong>{paginaActual}</strong> de <strong>{totalPaginas || 1}</strong>
          </span>
          <button
            className="inv-button"
            disabled={paginaActual <= 1}
            onClick={() => onCambiarPagina(paginaActual - 1)}
          >
            <ChevronLeft size={14} />
            Anterior
          </button>
          <button
            className="inv-button"
            disabled={paginaActual >= totalPaginas}
            onClick={() => onCambiarPagina(paginaActual + 1)}
          >
            Siguiente
            <ChevronRight size={14} />
          </button>
        </div>
      </footer>
    </section>
  );
}
