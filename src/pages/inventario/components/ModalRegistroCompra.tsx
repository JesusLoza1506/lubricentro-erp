import React, { useState, useEffect } from 'react';
import { Plus, Trash2, UserPlus, Edit } from 'lucide-react';
import {
  Producto,
  ProveedorDTO,
  CrearProveedorDTO,
  ActualizarProveedorDTO,
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
  cantidadInput: number; // En UND o Litros
  costoUnitarioSoles: number; // En Soles por unidad o litro
}

function getFechaActualLima(): string {
  const options: Intl.DateTimeFormatOptions = {
    timeZone: 'America/Lima',
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
  };
  const parts = new Intl.DateTimeFormat('en-CA', options).formatToParts(new Date());
  const year = parts.find((p) => p.type === 'year')?.value;
  const month = parts.find((p) => p.type === 'month')?.value;
  const day = parts.find((p) => p.type === 'day')?.value;
  return `${year}-${month}-${day}`;
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
  const [fechaDocumento, setFechaDocumento] = useState<string>(getFechaActualLima());
  const [observacion, setObservacion] = useState<string>('');

  const [detalles, setDetalles] = useState<ItemDetalleForm[]>([]);
  const [modalProvAbierto, setModalProvAbierto] = useState<boolean>(false);
  const [proveedorAEditar, setProveedorAEditar] = useState<ProveedorDTO | null>(null);
  const [guardando, setGuardando] = useState<boolean>(false);

  const [prodSeleccionadoId, setProdSeleccionadoId] = useState<number | ''>('');
  const [cantInput, setCantInput] = useState<number>(1);
  const [costoInput, setCostoInput] = useState<number>(0);

  useEffect(() => {
    if (abierto) {
      setFechaDocumento(getFechaActualLima());
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

    if (detalles.some((d) => d.id_producto === prod.id_producto)) {
      onError('El producto ya está en la lista de la compra');
      return;
    }

    setDetalles([
      ...detalles,
      {
        id_producto: prod.id_producto,
        nombre: prod.nombre,
        unidad: prod.unidad_medida,
        cantidadInput: cantInput,
        costoUnitarioSoles: costoInput,
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

  const handleActualizarProveedor = async (payload: ActualizarProveedorDTO) => {
    try {
      await inventarioService.actualizarProveedor(payload);
      const listaActualizada = await inventarioService.listarProveedores();
      setProveedores(listaActualizada);
    } catch (err: unknown) {
      const msg =
        typeof err === 'string'
          ? err
          : (err as { message?: string })?.message || 'Error al actualizar proveedor';
      onError(msg);
    }
  };

  const handleEditarProveedorSeleccionado = () => {
    if (!idProveedor) return;
    const prov = proveedores.find((p) => p.id_proveedor === Number(idProveedor));
    if (prov) {
      setProveedorAEditar(prov);
      setModalProvAbierto(true);
    }
  };

  const totalCompraSoles = detalles.reduce(
    (acc, item) => acc + item.cantidadInput * item.costoUnitarioSoles,
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

      const detallesPayload: DetalleIngresoRequestDTO[] = detalles.map((d) => {
        const esML = d.unidad.toUpperCase() === 'ML';
        const cantidadBase = esML
          ? Math.round(d.cantidadInput * 1000)
          : Math.round(d.cantidadInput);

        const microRaw = Math.round(d.costoUnitarioSoles * 1_000_000);
        const costoMicroCalculado = Math.round(microRaw / 10_000) * 10_000;
        const costoEfectivoMicro = Math.max(costoMicroCalculado, 10_000);

        return {
          id_producto: d.id_producto,
          cantidad: cantidadBase,
          costo_efectivo_micro: costoEfectivoMicro,
        };
      });

      const payload: RegistrarIngresoDTO = {
        origen: 'COMPRA',
        id_proveedor: Number(idProveedor),
        tipo_documento: tipoDocumento,
        numero_documento: tipoDocumento === 'SIN_DOCUMENTO' ? null : numeroDocumento.trim(),
        fecha_documento: tipoDocumento === 'SIN_DOCUMENTO' ? null : fechaDocumento,
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
          width: '740px',
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
            <div style={{ display: 'flex', gap: '6px' }}>
              <button
                type="button"
                onClick={() => {
                  setProveedorAEditar(null);
                  setModalProvAbierto(true);
                }}
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
              {idProveedor && (
                <button
                  type="button"
                  onClick={handleEditarProveedorSeleccionado}
                  style={{
                    padding: '10px 14px',
                    backgroundColor: '#1E293B',
                    border: '1px solid #334155',
                    color: '#FBBF24',
                    borderRadius: '8px',
                    cursor: 'pointer',
                    display: 'flex',
                    alignItems: 'center',
                    gap: '6px',
                    fontWeight: '600',
                  }}
                >
                  <Edit size={16} />
                  Editar
                </button>
              )}
            </div>
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
                <option value="OTRO">OTRO COMPROBANTE</option>
                <option value="SIN_DOCUMENTO">SIN DOCUMENTO (Interno)</option>
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
                N° Comprobante {tipoDocumento === 'SIN_DOCUMENTO' ? '(Autogenerado)' : '*'}
              </label>
              <input
                required={tipoDocumento !== 'SIN_DOCUMENTO'}
                disabled={tipoDocumento === 'SIN_DOCUMENTO'}
                type="text"
                placeholder={tipoDocumento === 'SIN_DOCUMENTO' ? 'SD-Autogenerado' : 'F001-0001234'}
                value={tipoDocumento === 'SIN_DOCUMENTO' ? '' : numeroDocumento}
                onChange={(e) => setNumeroDocumento(e.target.value)}
                style={{
                  ...modalInputStyle,
                  opacity: tipoDocumento === 'SIN_DOCUMENTO' ? 0.5 : 1,
                }}
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
                required={tipoDocumento !== 'SIN_DOCUMENTO'}
                disabled={tipoDocumento === 'SIN_DOCUMENTO'}
                type="date"
                value={fechaDocumento}
                onChange={(e) => setFechaDocumento(e.target.value)}
                style={{
                  ...modalInputStyle,
                  opacity: tipoDocumento === 'SIN_DOCUMENTO' ? 0.5 : 1,
                }}
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
              placeholder="Ej. Recepción de lote con guía interna o factura crédito"
              value={observacion}
              onChange={(e) => setObservacion(e.target.value)}
              style={modalInputStyle}
            />
          </div>

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
                      {p.nombre} ({p.codigo}) - [{p.unidad_medida}]
                    </option>
                  ))}
                </select>
              </div>
              <div>
                <input
                  type="number"
                  step="0.001"
                  min="0.001"
                  placeholder="Cant / Litros"
                  value={cantInput}
                  onChange={(e) => setCantInput(parseFloat(e.target.value) || 0)}
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
                  <th style={{ padding: '8px 12px' }}>Cantidad</th>
                  <th style={{ padding: '8px 12px' }}>Costo U. (S/)</th>
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
                        {d.cantidadInput} {d.unidad === 'ML' ? 'Litros (L)' : 'UND'}
                      </td>
                      <td style={{ padding: '8px 12px' }}>S/ {d.costoUnitarioSoles.toFixed(2)}</td>
                      <td style={{ padding: '8px 12px', fontWeight: '700', color: '#34D399' }}>
                        S/ {(d.cantidadInput * d.costoUnitarioSoles).toFixed(2)}
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
                disabled={guardando}
                style={{
                  padding: '10px 16px',
                  backgroundColor: 'transparent',
                  border: '1px solid #334155',
                  color: '#CBD5E1',
                  borderRadius: '8px',
                  cursor: guardando ? 'not-allowed' : 'pointer',
                  opacity: guardando ? 0.6 : 1,
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
                  cursor: guardando ? 'not-allowed' : 'pointer',
                  opacity: guardando ? 0.6 : 1,
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
        proveedorAEditar={proveedorAEditar}
        onClose={() => {
          setModalProvAbierto(false);
          setProveedorAEditar(null);
        }}
        onGuardar={handleCrearProveedor}
        onEditar={handleActualizarProveedor}
      />
    </div>
  );
};
