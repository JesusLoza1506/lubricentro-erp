import React, { useState, useEffect } from 'react';
import { Plus, Trash2, UserPlus } from 'lucide-react';
import {
  Producto,
  ProveedorDTO,
  CrearProveedorDTO,
  RegistrarIngresoDTO,
  DetalleIngresoRequestDTO,
} from '../../../types/inventario';
import { inventarioService } from '../../../services/inventarioService';
import { ModalProveedor } from './ModalProveedor';
import { modalInputStyle } from '../inventarioStyles';

interface ModalRegistroCompraProps {
  abierto: boolean;
  productos: Producto[];
  onClose: () => void;
  onCompraExitosa: () => void;
  onError: (msg: string) => void;
}

interface ItemDetalleForm {
  id_producto: number;
  nombre: string;
  unidad: string;
  cantidad: number;
  costo_soles: number;
}

export const ModalRegistroCompra: React.FC<ModalRegistroCompraProps> = ({
  abierto,
  productos,
  onClose,
  onCompraExitosa,
  onError,
}) => {
  const [proveedores, setProveedores] = useState<ProveedorDTO[]>([]);
  const [idProveedor, setIdProveedor] = useState<number | ''>('');
  const [tipoDocumento, setTipoDocumento] = useState<string>('FACTURA');
  const [numeroDocumento, setNumeroDocumento] = useState<string>('');
  const [fechaDocumento, setFechaDocumento] = useState<string>(
    new Date().toISOString().split('T')[0]
  );
  const [observacion, setObservacion] = useState<string>('');

  const [detalles, setDetalles] = useState<ItemDetalleForm[]>([]);
  const [modalProvAbierto, setModalProvAbierto] = useState<boolean>(false);
  const [guardando, setGuardando] = useState<boolean>(false);

  // Selector para agregar item
  const [prodSeleccionadoId, setProdSeleccionadoId] = useState<number | ''>('');
  const [cantInput, setCantInput] = useState<number>(1);
  const [costoInput, setCostoInput] = useState<number>(0);

  useEffect(() => {
    if (abierto) {
      inventarioService
        .listarProveedores()
        .then(setProveedores)
        .catch(() => {});
    }
  }, [abierto]);

  if (!abierto) return null;

  const handleAgregarItem = () => {
    if (!prodSeleccionadoId) return;
    const prod = productos.find((p) => p.id_producto === Number(prodSeleccionadoId));
    if (!prod) return;

    const yaExiste = detalles.some((d) => d.id_producto === prod.id_producto);
    if (yaExiste) {
      onError('El producto ya está en la lista de la compra');
      return;
    }

    setDetalles([
      ...detalles,
      {
        id_producto: prod.id_producto,
        nombre: prod.nombre,
        unidad: prod.unidad_medida,
        cantidad: cantInput,
        costo_soles: costoInput,
      },
    ]);

    setProdSeleccionadoId('');
    setCantInput(1);
    setCostoInput(0);
  };

  const handleRemoverItem = (idProd: number) => {
    setDetalles(detalles.filter((d) => d.id_producto !== idProd));
  };

  const handleCrearProveedor = async (payload: CrearProveedorDTO) => {
    try {
      const newId = await inventarioService.crearProveedor(payload);
      const listaActualizada = await inventarioService.listarProveedores();
      setProveedores(listaActualizada);
      setIdProveedor(newId);
    } catch (err: unknown) {
      const msg =
        typeof err === 'string'
          ? err
          : (err as { message?: string })?.message || 'Error al crear proveedor';
      onError(msg);
    }
  };

  const totalCompraSoles = detalles.reduce(
    (acc, item) => acc + item.cantidad * item.costo_soles,
    0
  );

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!idProveedor) {
      onError('Debe seleccionar un proveedor');
      return;
    }
    if (detalles.length === 0) {
      onError('Debe agregar al menos un producto a la compra');
      return;
    }

    try {
      setGuardando(true);
      const detallesPayload: DetalleIngresoRequestDTO[] = detalles.map((d) => ({
        id_producto: d.id_producto,
        cantidad: d.cantidad,
        costo_efectivo_micro: Math.round(d.costo_soles * 1_000_000),
      }));

      const payload: RegistrarIngresoDTO = {
        origen: 'COMPRA',
        id_proveedor: Number(idProveedor),
        tipo_documento: tipoDocumento,
        numero_documento: numeroDocumento.trim(),
        fecha_documento: fechaDocumento,
        observacion: observacion.trim() || null,
        detalles: detallesPayload,
      };

      await inventarioService.registrarIngresoCompra(payload);
      onCompraExitosa();
      onClose();
    } catch (err: unknown) {
      const msg =
        typeof err === 'string'
          ? err
          : (err as { message?: string })?.message || 'Error al registrar la compra';
      onError(msg);
    } finally {
      setGuardando(false);
    }
  };

  return (
    <div
      style={{
        position: 'fixed',
        top: 0,
        left: 0,
        width: '100vw',
        height: '100vh',
        backgroundColor: 'rgba(0,0,0,0.8)',
        backdropFilter: 'blur(4px)',
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        zIndex: 1100,
      }}
    >
      <div
        style={{
          width: '720px',
          backgroundColor: '#0F172A',
          border: '1px solid #334155',
          borderRadius: '16px',
          padding: '28px',
          boxShadow: '0 20px 40px rgba(0,0,0,0.5)',
          maxHeight: '90vh',
          overflowY: 'auto',
        }}
      >
        <h3 style={{ fontSize: '20px', fontWeight: '800', color: '#F8FAFC', margin: '0 0 20px 0' }}>
          Ingreso por Compra a Proveedor
        </h3>

        <form
          onSubmit={handleSubmit}
          style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}
        >
          {/* Cabecera del Documento */}
          <div
            style={{
              display: 'grid',
              gridTemplateColumns: '2fr auto',
              gap: '10px',
              alignItems: 'end',
            }}
          >
            <div>
              <label
                style={{
                  display: 'block',
                  fontSize: '11px',
                  color: '#94A3B8',
                  marginBottom: '4px',
                }}
              >
                Proveedor *
              </label>
              <select
                required
                value={idProveedor}
                onChange={(e) => setIdProveedor(Number(e.target.value) || '')}
                style={{ ...modalInputStyle, backgroundColor: '#1E293B', color: '#F8FAFC' }}
              >
                <option value="">Seleccione Proveedor...</option>
                {proveedores.map((p) => (
                  <option key={p.id_proveedor} value={p.id_proveedor}>
                    {p.nombre_razon_social} ({p.tipo_documento}: {p.numero_documento})
                  </option>
                ))}
              </select>
            </div>
            <button
              type="button"
              onClick={() => setModalProvAbierto(true)}
              style={{
                padding: '10px 14px',
                backgroundColor: '#1E293B',
                border: '1px solid #334155',
                color: '#38BDF8',
                borderRadius: '8px',
                cursor: 'pointer',
                display: 'flex',
                alignItems: 'center',
                gap: '6px',
                fontWeight: '600',
              }}
            >
              <UserPlus size={16} />
              Nuevo
            </button>
          </div>

          <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr 1fr', gap: '10px' }}>
            <div>
              <label
                style={{
                  display: 'block',
                  fontSize: '11px',
                  color: '#94A3B8',
                  marginBottom: '4px',
                }}
              >
                Tipo Comprobante *
              </label>
              <select
                value={tipoDocumento}
                onChange={(e) => setTipoDocumento(e.target.value)}
                style={{ ...modalInputStyle, backgroundColor: '#1E293B', color: '#F8FAFC' }}
              >
                <option value="FACTURA">FACTURA</option>
                <option value="BOLETA">BOLETA</option>
                <option value="GUIA">GUÍA DE REMISIÓN</option>
              </select>
            </div>
            <div>
              <label
                style={{
                  display: 'block',
                  fontSize: '11px',
                  color: '#94A3B8',
                  marginBottom: '4px',
                }}
              >
                N° Comprobante *
              </label>
              <input
                required
                type="text"
                placeholder="F001-0001234"
                value={numeroDocumento}
                onChange={(e) => setNumeroDocumento(e.target.value)}
                style={modalInputStyle}
              />
            </div>
            <div>
              <label
                style={{
                  display: 'block',
                  fontSize: '11px',
                  color: '#94A3B8',
                  marginBottom: '4px',
                }}
              >
                Fecha Documento *
              </label>
              <input
                required
                type="date"
                value={fechaDocumento}
                onChange={(e) => setFechaDocumento(e.target.value)}
                style={modalInputStyle}
              />
            </div>
          </div>

          <div>
            <label
              style={{ display: 'block', fontSize: '11px', color: '#94A3B8', marginBottom: '4px' }}
            >
              Observación / Nota opcional
            </label>
            <input
              type="text"
              placeholder="Ej. Lote entregado en almacén principal con crédito a 30 días"
              value={observacion}
              onChange={(e) => setObservacion(e.target.value)}
              style={modalInputStyle}
            />
          </div>

          {/* Selector de Items */}
          <div
            style={{
              backgroundColor: '#1E293B',
              padding: '14px',
              borderRadius: '12px',
              border: '1px solid #334155',
            }}
          >
            <span
              style={{
                fontSize: '12px',
                fontWeight: '700',
                color: '#F8FAFC',
                display: 'block',
                marginBottom: '10px',
              }}
            >
              Agregar Productos al Lote
            </span>
            <div
              style={{
                display: 'grid',
                gridTemplateColumns: '3fr 1fr 1fr auto',
                gap: '8px',
                alignItems: 'end',
              }}
            >
              <div>
                <select
                  value={prodSeleccionadoId}
                  onChange={(e) => setProdSeleccionadoId(Number(e.target.value) || '')}
                  style={{ ...modalInputStyle, backgroundColor: '#0F172A', color: '#F8FAFC' }}
                >
                  <option value="">Buscar Producto...</option>
                  {productos.map((p) => (
                    <option key={p.id_producto} value={p.id_producto}>
                      {p.nombre} ({p.codigo})
                    </option>
                  ))}
                </select>
              </div>
              <div>
                <input
                  type="number"
                  min="1"
                  placeholder="Cant"
                  value={cantInput}
                  onChange={(e) => setCantInput(parseInt(e.target.value, 10) || 1)}
                  style={modalInputStyle}
                />
              </div>
              <div>
                <input
                  type="number"
                  step="0.01"
                  min="0.01"
                  placeholder="Costo S/"
                  value={costoInput || ''}
                  onChange={(e) => setCostoInput(parseFloat(e.target.value) || 0)}
                  style={modalInputStyle}
                />
              </div>
              <button
                type="button"
                onClick={handleAgregarItem}
                style={{
                  padding: '10px 14px',
                  backgroundColor: '#10B981',
                  color: '#FFFFFF',
                  border: 'none',
                  borderRadius: '8px',
                  cursor: 'pointer',
                  fontWeight: '700',
                  display: 'flex',
                  alignItems: 'center',
                }}
              >
                <Plus size={18} />
              </button>
            </div>
          </div>

          {/* Tabla de Detalle */}
          <div
            style={{
              maxHeight: '180px',
              overflowY: 'auto',
              border: '1px solid #334155',
              borderRadius: '8px',
            }}
          >
            <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: '13px' }}>
              <thead>
                <tr style={{ backgroundColor: '#1E293B', color: '#94A3B8', textAlign: 'left' }}>
                  <th style={{ padding: '8px 12px' }}>Producto</th>
                  <th style={{ padding: '8px 12px' }}>Cant</th>
                  <th style={{ padding: '8px 12px' }}>Costo U.</th>
                  <th style={{ padding: '8px 12px' }}>Subtotal</th>
                  <th style={{ padding: '8px 12px', textAlign: 'center' }}>Acción</th>
                </tr>
              </thead>
              <tbody>
                {detalles.length === 0 ? (
                  <tr>
                    <td
                      colSpan={5}
                      style={{ padding: '20px', textAlign: 'center', color: '#94A3B8' }}
                    >
                      Sin productos agregados al lote
                    </td>
                  </tr>
                ) : (
                  detalles.map((d) => (
                    <tr
                      key={d.id_producto}
                      style={{ borderBottom: '1px solid #1E293B', color: '#F8FAFC' }}
                    >
                      <td style={{ padding: '8px 12px' }}>{d.nombre}</td>
                      <td style={{ padding: '8px 12px' }}>
                        {d.cantidad} {d.unidad}
                      </td>
                      <td style={{ padding: '8px 12px' }}>S/ {d.costo_soles.toFixed(2)}</td>
                      <td style={{ padding: '8px 12px', fontWeight: '700', color: '#34D399' }}>
                        S/ {(d.cantidad * d.costo_soles).toFixed(2)}
                      </td>
                      <td style={{ padding: '8px 12px', textAlign: 'center' }}>
                        <button
                          type="button"
                          onClick={() => handleRemoverItem(d.id_producto)}
                          style={{
                            backgroundColor: 'transparent',
                            border: 'none',
                            cursor: 'pointer',
                          }}
                        >
                          <Trash2 size={16} color="#EF4444" />
                        </button>
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>

          {/* Pie de Totales */}
          <div
            style={{
              display: 'flex',
              justifyContent: 'space-between',
              alignItems: 'center',
              marginTop: '8px',
            }}
          >
            <span style={{ fontSize: '16px', fontWeight: '800', color: '#F8FAFC' }}>
              Total Compra:{' '}
              <span style={{ color: '#34D399' }}>S/ {totalCompraSoles.toFixed(2)}</span>
            </span>
            <div style={{ display: 'flex', gap: '10px' }}>
              <button
                type="button"
                onClick={onClose}
                style={{
                  padding: '10px 16px',
                  backgroundColor: 'transparent',
                  border: '1px solid #334155',
                  color: '#CBD5E1',
                  borderRadius: '8px',
                  cursor: 'pointer',
                }}
              >
                Cancelar
              </button>
              <button
                type="submit"
                disabled={guardando}
                style={{
                  padding: '10px 20px',
                  backgroundColor: '#10B981',
                  color: '#FFFFFF',
                  border: 'none',
                  borderRadius: '8px',
                  fontWeight: '700',
                  cursor: 'pointer',
                }}
              >
                {guardando ? 'Guardando...' : 'Registrar Compra'}
              </button>
            </div>
          </div>
        </form>
      </div>

      <ModalProveedor
        abierto={modalProvAbierto}
        onClose={() => setModalProvAbierto(false)}
        onGuardar={handleCrearProveedor}
      />
    </div>
  );
};
