import React from 'react';
import { X, Save } from 'lucide-react';
import { Categoria, CategoriaEnum, ProductoForm } from '../../../types/inventario';
export type ModoModal = 'CREAR' | 'EDITAR' | 'VER';
interface InventarioModalProps {
  modalAbierto: boolean;
  setModalAbierto: (abierto: boolean) => void;
  modo: ModoModal;
  prodForm: ProductoForm;
  setProdForm: React.Dispatch<React.SetStateAction<ProductoForm>>;
  categorias: Categoria[];
  handleGuardar: (e: React.FormEvent) => void;
  guardando?: boolean;
  error?: string;
}
const CATEGORIAS_LISTA: CategoriaEnum[] = ['ACEITE', 'FILTRO', 'ADITIVO', 'GRANEL', 'OTRO'];
export function InventarioModal({
  modalAbierto,
  setModalAbierto,
  modo,
  prodForm,
  setProdForm,
  handleGuardar,
  guardando = false,
  error = '',
}: InventarioModalProps) {
  if (!modalAbierto) return null;
  const lectura = modo === 'VER';
  const granel = prodForm.categoria === 'GRANEL';
  const titulo =
    modo === 'CREAR'
      ? 'Nuevo Producto'
      : modo === 'EDITAR'
        ? 'Editar Producto'
        : 'Detalles del Producto';
  return (
    <div
      className="inv-overlay"
      style={{
        position: 'fixed',
        inset: 0,
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        zIndex: 1100,
      }}
    >
      <div
        className="inv-dialog"
        role="dialog"
        aria-modal="true"
        aria-labelledby="inv-product-title"
      >
        <header className="inv-dialog-header">
          <div>
            <h3 id="inv-product-title">{titulo}</h3>
            <p>
              {lectura
                ? 'Información del catálogo de productos.'
                : 'Completa la información del producto y sus existencias.'}
            </p>
          </div>
          <button
            className="inv-icon-button"
            aria-label="Cerrar producto"
            disabled={guardando}
            onClick={() => setModalAbierto(false)}
          >
            <X size={18} />
          </button>
        </header>
        <form className="inv-form" onSubmit={handleGuardar}>
          <fieldset disabled={guardando || lectura} className="inv-form-section">
            <legend className="inv-section-label">Información del producto</legend>
            <div className="inv-form-grid is-identification">
              <label className="inv-field">
                Código *
                <input
                  name="codigo"
                  required
                  disabled={modo === 'EDITAR'}
                  value={prodForm.codigo || ''}
                  onChange={(e) =>
                    setProdForm({ ...prodForm, codigo: e.target.value.toUpperCase() })
                  }
                  placeholder="MOBIL-5W30"
                />
              </label>
              <label className="inv-field">
                Nombre del producto *
                <input
                  name="nombre"
                  required
                  value={prodForm.nombre || ''}
                  onChange={(e) => setProdForm({ ...prodForm, nombre: e.target.value })}
                  placeholder="Ej. Aceite Mobil Super 3000 5W-30 1L"
                />
              </label>
            </div>
            <label className="inv-field">
              Descripción u observaciones
              <input
                value={prodForm.descripcion || ''}
                onChange={(e) => setProdForm({ ...prodForm, descripcion: e.target.value })}
                placeholder="Ej. Sintético 100% para motores a gasolina"
              />
            </label>
            <div className="inv-form-grid">
              <label className="inv-field">
                Categoría *
                <select
                  required
                  disabled={modo === 'EDITAR'}
                  value={prodForm.categoria}
                  onChange={(e) => {
                    const cat = e.target.value as CategoriaEnum;
                    setProdForm({
                      ...prodForm,
                      categoria: cat,
                      unidad_medida: cat === 'GRANEL' ? 'ML' : 'UND',
                    });
                  }}
                >
                  {CATEGORIAS_LISTA.map((c) => (
                    <option key={c} value={c}>
                      {c}
                    </option>
                  ))}
                </select>
              </label>
              <label className="inv-field">
                Unidad de medida
                <input disabled value={granel ? 'Litros (L)' : 'Unidades (UND)'} />
              </label>
            </div>
          </fieldset>
          <fieldset disabled={guardando || lectura} className="inv-form-section">
            <legend className="inv-section-label">Precio y control de stock</legend>
            <div className="inv-form-grid">
              <label className="inv-field">
                Precio de venta por {granel ? 'litro' : 'unidad'} (S/) *
                <input
                  aria-label="Precio de venta"
                  required
                  type="number"
                  min="0.01"
                  step="0.01"
                  value={prodForm.precio_venta_soles ?? 0}
                  onChange={(e) =>
                    setProdForm({
                      ...prodForm,
                      precio_venta_soles: parseFloat(e.target.value) || 0,
                    })
                  }
                />
              </label>
              <label className="inv-field">
                Stock mínimo de alerta ({granel ? 'L' : 'UND'}) *
                <input
                  required
                  type="number"
                  min="0"
                  step={granel ? '0.001' : '1'}
                  value={prodForm.stock_minimo ?? 0}
                  onChange={(e) =>
                    setProdForm({ ...prodForm, stock_minimo: parseFloat(e.target.value) || 0 })
                  }
                />
              </label>
            </div>
            <p className="inv-help">El mínimo indica cuándo el producto necesita reposición.</p>
          </fieldset>
          {modo === 'CREAR' && (
            <fieldset disabled={guardando} className="inv-initial-stock">
              <label className="inv-initial-toggle">
                <input
                  type="checkbox"
                  checked={prodForm.tiene_stock_inicial}
                  onChange={(e) =>
                    setProdForm({ ...prodForm, tiene_stock_inicial: e.target.checked })
                  }
                />
                Registrar existencias iniciales
              </label>
              <p className="inv-help">
                Úsalo si ya tienes unidades disponibles al crear este producto.
              </p>
              {prodForm.tiene_stock_inicial && (
                <>
                  <div className="inv-form-grid">
                    <label className="inv-field">
                      Cantidad inicial ({granel ? 'L' : 'UND'})
                      <input
                        required
                        type="number"
                        min="0.001"
                        step={granel ? '0.001' : '1'}
                        value={prodForm.cantidad_inicial || ''}
                        onChange={(e) =>
                          setProdForm({
                            ...prodForm,
                            cantidad_inicial: parseFloat(e.target.value) || 0,
                          })
                        }
                      />
                    </label>
                    <label className="inv-field">
                      Costo por {granel ? 'litro' : 'unidad'} (S/)
                      <input
                        required
                        type="number"
                        min="0.01"
                        step="0.01"
                        value={prodForm.costo_unitario_soles || 0}
                        onChange={(e) =>
                          setProdForm({
                            ...prodForm,
                            costo_unitario_soles: parseFloat(e.target.value) || 0,
                          })
                        }
                      />
                    </label>
                  </div>
                  <label className="inv-field">
                    Referencia / guía · opcional
                    <input
                      placeholder="Se generará automáticamente"
                      value={prodForm.referencia_inicial || ''}
                      onChange={(e) =>
                        setProdForm({ ...prodForm, referencia_inicial: e.target.value })
                      }
                    />
                  </label>
                </>
              )}
            </fieldset>
          )}
          {error && (
            <p role="alert" className="inv-form-error">
              {error}
            </p>
          )}
          <footer className="inv-form-footer">
            <button
              type="button"
              className="inv-button"
              disabled={guardando}
              onClick={() => setModalAbierto(false)}
            >
              {lectura ? 'Cerrar' : 'Cancelar'}
            </button>
            {!lectura && (
              <button type="submit" disabled={guardando} className="inv-button inv-primary">
                <Save size={15} />
                {guardando ? 'Guardando...' : 'Guardar'}
              </button>
            )}
          </footer>
        </form>
      </div>
    </div>
  );
}
