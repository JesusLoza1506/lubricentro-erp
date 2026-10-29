import React from 'react';
import { Categoria, CategoriaEnum, ProductoForm } from '../../../types/inventario';
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

const CATEGORIAS_LISTA: CategoriaEnum[] = ['ACEITE', 'FILTRO', 'ADITIVO', 'GRANEL', 'OTRO'];

export const InventarioModal: React.FC<InventarioModalProps> = ({
  modalAbierto,
  setModalAbierto,
  modo,
  prodForm,
  setProdForm,
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
          width: '560px',
          backgroundColor: '#0F172A',
          border: '1px solid #334155',
          borderRadius: '16px',
          padding: '28px',
          boxShadow: '0 20px 40px rgba(0,0,0,0.5)',
          maxHeight: '90vh',
          overflowY: 'auto',
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
          <div style={{ display: 'grid', gridTemplateColumns: '1fr 2fr', gap: '12px' }}>
            <div>
              <label
                style={{
                  display: 'block',
                  fontSize: '12px',
                  color: '#94A3B8',
                  marginBottom: '6px',
                }}
              >
                Código *
              </label>
              <input
                required
                disabled={esSoloLectura || modo === 'EDITAR'}
                type="text"
                value={prodForm.codigo || ''}
                onChange={(e) => setProdForm({ ...prodForm, codigo: e.target.value.toUpperCase() })}
                style={{
                  ...modalInputStyle,
                  opacity: esSoloLectura || modo === 'EDITAR' ? 0.7 : 1,
                }}
                placeholder="MOBIL-5W30"
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
          </div>

          <div>
            <label
              style={{ display: 'block', fontSize: '12px', color: '#94A3B8', marginBottom: '6px' }}
            >
              Descripción u Observaciones
            </label>
            <input
              disabled={esSoloLectura}
              type="text"
              value={prodForm.descripcion || ''}
              onChange={(e) => setProdForm({ ...prodForm, descripcion: e.target.value })}
              style={{ ...modalInputStyle, opacity: esSoloLectura ? 0.7 : 1 }}
              placeholder="Ej. Sintético 100% para motores a gasolina"
            />
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
                Categoría *
              </label>
              <select
                required
                disabled={esSoloLectura || modo === 'EDITAR'}
                value={prodForm.categoria}
                onChange={(e) => {
                  const cat = e.target.value as CategoriaEnum;
                  setProdForm({
                    ...prodForm,
                    categoria: cat,
                    unidad_medida: cat === 'GRANEL' ? 'ML' : 'UND',
                  });
                }}
                style={{
                  ...modalInputStyle,
                  backgroundColor: '#1E293B',
                  color: '#F8FAFC',
                  opacity: esSoloLectura || modo === 'EDITAR' ? 0.7 : 1,
                }}
              >
                {CATEGORIAS_LISTA.map((cat) => (
                  <option key={cat} value={cat}>
                    {cat}
                  </option>
                ))}
              </select>
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
                Unidad de Medida
              </label>
              <input
                disabled
                type="text"
                value={prodForm.categoria === 'GRANEL' ? 'ML' : 'UND'}
                style={{ ...modalInputStyle, opacity: 0.7, fontWeight: '700', color: '#38BDF8' }}
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
                Precio Venta (S/) *
              </label>
              <input
                required
                disabled={esSoloLectura}
                type="number"
                step="0.01"
                min="0"
                value={prodForm.precio_venta_soles ?? 0}
                onChange={(e) =>
                  setProdForm({ ...prodForm, precio_venta_soles: parseFloat(e.target.value) || 0 })
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
                step="1"
                min="0"
                value={prodForm.stock_minimo ?? 0}
                onChange={(e) =>
                  setProdForm({ ...prodForm, stock_minimo: parseInt(e.target.value, 10) || 0 })
                }
                style={{ ...modalInputStyle, opacity: esSoloLectura ? 0.7 : 1 }}
              />
            </div>
          </div>

          {modo === 'CREAR' && (
            <div
              style={{
                backgroundColor: '#1E293B',
                border: '1px dashed #334155',
                borderRadius: '12px',
                padding: '16px',
                marginTop: '8px',
              }}
            >
              <label
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '8px',
                  color: '#F8FAFC',
                  fontWeight: '600',
                  fontSize: '13px',
                  cursor: 'pointer',
                }}
              >
                <input
                  type="checkbox"
                  checked={prodForm.tiene_stock_inicial}
                  onChange={(e) =>
                    setProdForm({ ...prodForm, tiene_stock_inicial: e.target.checked })
                  }
                />
                ¿Dar de alta con Stock Inicial?
              </label>

              {prodForm.tiene_stock_inicial && (
                <div
                  style={{
                    display: 'grid',
                    gridTemplateColumns: '1fr 1fr 1fr',
                    gap: '10px',
                    marginTop: '12px',
                  }}
                >
                  <div>
                    <label style={{ display: 'block', fontSize: '11px', color: '#94A3B8' }}>
                      Cantidad Inicial
                    </label>
                    <input
                      required={prodForm.tiene_stock_inicial}
                      type="number"
                      min="1"
                      value={prodForm.cantidad_inicial || 0}
                      onChange={(e) =>
                        setProdForm({
                          ...prodForm,
                          cantidad_inicial: parseInt(e.target.value, 10) || 0,
                        })
                      }
                      style={modalInputStyle}
                    />
                  </div>
                  <div>
                    <label style={{ display: 'block', fontSize: '11px', color: '#94A3B8' }}>
                      Costo Unitario (S/)
                    </label>
                    <input
                      required={prodForm.tiene_stock_inicial}
                      type="number"
                      step="0.01"
                      min="0.01"
                      value={prodForm.costo_unitario_soles || 0}
                      onChange={(e) =>
                        setProdForm({
                          ...prodForm,
                          costo_unitario_soles: parseFloat(e.target.value) || 0,
                        })
                      }
                      style={modalInputStyle}
                    />
                  </div>
                  <div>
                    <label style={{ display: 'block', fontSize: '11px', color: '#94A3B8' }}>
                      Referencia / Guía
                    </label>
                    <input
                      required={prodForm.tiene_stock_inicial}
                      type="text"
                      placeholder="INV-INIT-01"
                      value={prodForm.referencia_inicial || ''}
                      onChange={(e) =>
                        setProdForm({ ...prodForm, referencia_inicial: e.target.value })
                      }
                      style={modalInputStyle}
                    />
                  </div>
                </div>
              )}
            </div>
          )}

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
