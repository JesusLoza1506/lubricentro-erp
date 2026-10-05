import React from 'react';
import { Categoria, ProductoForm } from '../../../types/inventario';
import { modalInputStyle } from '../inventarioStyles';

export type ModoModal = 'CREAR' | 'EDITAR' | 'VER';

interface InventarioModalProps {
  modalAbierto: boolean;
  setModalAbierto: (abierto: boolean) => void;
  modo: ModoModal;
  prodForm: ProductoForm;
  setProdForm: React.Dispatch<React.SetStateAction<ProductoForm>>;
  categorias: Categoria[];
  handleGuardar: (e: React.FormEvent) => void;
}

export const InventarioModal: React.FC<InventarioModalProps> = ({
  modalAbierto,
  setModalAbierto,
  modo,
  prodForm,
  setProdForm,
  categorias,
  handleGuardar,
}) => {
  if (!modalAbierto) return null;

  const esSoloLectura = modo === 'VER';

  const tituloModal =
    modo === 'CREAR'
      ? 'Nuevo Producto'
      : modo === 'EDITAR'
        ? 'Editar Producto'
        : 'Detalles del Producto';

  const categoriaActualId = prodForm.id_categoria ?? prodForm.idCategoria ?? 1;

  const categoriaNombreActual =
    prodForm.nombre_categoria ||
    prodForm.nombreCategoria ||
    categorias.find((c) => (c.id_categoria ?? c.idCategoria) === Number(categoriaActualId))
      ?.nombre_categoria ||
    categorias.find((c) => (c.id_categoria ?? c.idCategoria) === Number(categoriaActualId))
      ?.nombreCategoria ||
    'Sin Categoría';

  return (
    <div
      style={{
        position: 'fixed',
        top: 0,
        left: 0,
        width: '100vw',
        height: '100vh',
        backgroundColor: 'rgba(0,0,0,0.75)',
        backdropFilter: 'blur(4px)',
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        zIndex: 1000,
      }}
    >
      <div
        style={{
          width: '500px',
          backgroundColor: '#0F172A',
          border: '1px solid #334155',
          borderRadius: '16px',
          padding: '28px',
          boxShadow: '0 20px 40px rgba(0,0,0,0.5)',
        }}
      >
        <h3
          style={{
            fontSize: '20px',
            fontWeight: '800',
            color: '#F8FAFC',
            marginTop: 0,
            marginBottom: '20px',
          }}
        >
          {tituloModal}
        </h3>

        <form
          onSubmit={handleGuardar}
          style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}
        >
          <div>
            <label
              style={{ display: 'block', fontSize: '12px', color: '#94A3B8', marginBottom: '6px' }}
            >
              Nombre del Producto *
            </label>
            <input
              required
              disabled={esSoloLectura}
              type="text"
              value={prodForm.nombre || ''}
              onChange={(e) => setProdForm({ ...prodForm, nombre: e.target.value })}
              style={{ ...modalInputStyle, opacity: esSoloLectura ? 0.7 : 1 }}
              placeholder="Ej. Aceite Mobil Super 3000 5W-30 1L"
            />
          </div>

          <div>
            <label
              style={{ display: 'block', fontSize: '12px', color: '#94A3B8', marginBottom: '6px' }}
            >
              Categoría *
            </label>
            {esSoloLectura ? (
              <input
                disabled
                type="text"
                value={categoriaNombreActual}
                style={{ ...modalInputStyle, opacity: 0.7 }}
              />
            ) : (
              <select
                required
                value={categoriaActualId}
                onChange={(e) => setProdForm({ ...prodForm, id_categoria: Number(e.target.value) })}
                style={{
                  ...modalInputStyle,
                  backgroundColor: '#1E293B',
                  color: '#F8FAFC',
                  cursor: 'pointer',
                }}
              >
                {categorias.map((cat) => {
                  const id = cat.id_categoria ?? cat.idCategoria;
                  const nombre =
                    cat.nombre_categoria ??
                    cat.nombreCategoria ??
                    (cat as any).nombre ??
                    `Categoría ${id}`;
                  return (
                    <option
                      key={id}
                      value={id}
                      style={{
                        backgroundColor: '#0F172A',
                        color: '#F8FAFC',
                        padding: '8px',
                      }}
                    >
                      {nombre}
                    </option>
                  );
                })}
              </select>
            )}
          </div>

          <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '12px' }}>
            <div>
              <label
                style={{
                  display: 'block',
                  fontSize: '12px',
                  color: '#94A3B8',
                  marginBottom: '6px',
                }}
              >
                Código Barras
              </label>
              <input
                disabled={esSoloLectura}
                type="text"
                value={prodForm.codigo_barras || ''}
                onChange={(e) => setProdForm({ ...prodForm, codigo_barras: e.target.value })}
                style={{ ...modalInputStyle, opacity: esSoloLectura ? 0.7 : 1 }}
                placeholder="775..."
              />
            </div>
            <div>
              <label
                style={{
                  display: 'block',
                  fontSize: '12px',
                  color: '#94A3B8',
                  marginBottom: '6px',
                }}
              >
                Unidad Medida
              </label>
              <input
                disabled={esSoloLectura}
                type="text"
                value={prodForm.unidad_medida || ''}
                onChange={(e) => setProdForm({ ...prodForm, unidad_medida: e.target.value })}
                style={{ ...modalInputStyle, opacity: esSoloLectura ? 0.7 : 1 }}
                placeholder="UNIDAD / GALON / LITRO"
              />
            </div>
          </div>

          <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '12px' }}>
            <div>
              <label
                style={{
                  display: 'block',
                  fontSize: '12px',
                  color: '#94A3B8',
                  marginBottom: '6px',
                }}
              >
                Precio Compra (S/) *
              </label>
              <input
                required
                disabled={esSoloLectura}
                type="number"
                step="0.01"
                min="0"
                value={prodForm.precio_compra ?? 0}
                onChange={(e) =>
                  setProdForm({ ...prodForm, precio_compra: parseFloat(e.target.value) || 0 })
                }
                style={{ ...modalInputStyle, opacity: esSoloLectura ? 0.7 : 1 }}
              />
            </div>
            <div>
              <label
                style={{
                  display: 'block',
                  fontSize: '12px',
                  color: '#94A3B8',
                  marginBottom: '6px',
                }}
              >
                Precio Venta (S/) *
              </label>
              <input
                required
                disabled={esSoloLectura}
                type="number"
                step="0.01"
                min="0"
                value={prodForm.precio_venta ?? 0}
                onChange={(e) =>
                  setProdForm({ ...prodForm, precio_venta: parseFloat(e.target.value) || 0 })
                }
                style={{ ...modalInputStyle, opacity: esSoloLectura ? 0.7 : 1 }}
              />
            </div>
          </div>

          <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '12px' }}>
            <div>
              <label
                style={{
                  display: 'block',
                  fontSize: '12px',
                  color: '#94A3B8',
                  marginBottom: '6px',
                }}
              >
                Stock Actual *
              </label>
              <input
                required
                disabled={esSoloLectura}
                type="number"
                min="0"
                value={prodForm.stock_actual ?? 0}
                onChange={(e) =>
                  setProdForm({ ...prodForm, stock_actual: parseFloat(e.target.value) || 0 })
                }
                style={{ ...modalInputStyle, opacity: esSoloLectura ? 0.7 : 1 }}
              />
            </div>
            <div>
              <label
                style={{
                  display: 'block',
                  fontSize: '12px',
                  color: '#94A3B8',
                  marginBottom: '6px',
                }}
              >
                Stock Mínimo Alerta *
              </label>
              <input
                required
                disabled={esSoloLectura}
                type="number"
                min="0"
                value={prodForm.stock_minimo ?? 0}
                onChange={(e) =>
                  setProdForm({ ...prodForm, stock_minimo: parseFloat(e.target.value) || 0 })
                }
                style={{ ...modalInputStyle, opacity: esSoloLectura ? 0.7 : 1 }}
              />
            </div>
          </div>

          <div
            style={{ display: 'flex', justifyContent: 'flex-end', gap: '12px', marginTop: '12px' }}
          >
            <button
              type="button"
              onClick={() => setModalAbierto(false)}
              style={{
                padding: '10px 16px',
                backgroundColor: 'transparent',
                border: '1px solid #334155',
                color: '#CBD5E1',
                borderRadius: '8px',
                cursor: 'pointer',
              }}
            >
              {esSoloLectura ? 'Cerrar' : 'Cancelar'}
            </button>
            {!esSoloLectura && (
              <button
                type="submit"
                style={{
                  padding: '10px 20px',
                  backgroundColor: '#2563EB',
                  color: '#FFFFFF',
                  border: 'none',
                  borderRadius: '8px',
                  fontWeight: '700',
                  cursor: 'pointer',
                }}
              >
                Guardar
              </button>
            )}
          </div>
        </form>
      </div>
    </div>
  );
};
