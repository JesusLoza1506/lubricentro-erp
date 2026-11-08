import { useNotificarInventario } from '../AvisosInventario';
import { solicitudPendiente, solesAMicro } from '../../../services/operacionInventario';
import React, { useState, useEffect, useRef } from 'react';
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
  productos: productosIniciales,
  onClose,
  onCompraExitosa,
  onError,
}) => {
  const notificar = useNotificarInventario();
  const [productos, setProductos] = useState<Producto[]>(productosIniciales);
  const enviando = useRef(false);
  const [incierta, setIncierta] = useState(false);
  const [errorLocal, setErrorLocal] = useState('');
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
      setErrorLocal('');
      void (async () => {
        try {
          const catalogo = await inventarioService.listarProductosPublicos();
          const lista = catalogo ?? productosIniciales;
          setProductos(lista);
          const pendiente = solicitudPendiente('registrar_ingreso_compra_cmd')?.payload as
            | RegistrarIngresoDTO
            | undefined;
          setIncierta(!!pendiente);
          if (pendiente) {
            setIdProveedor(pendiente.id_proveedor ?? '');
            setTipoDocumento(pendiente.tipo_documento ?? 'SIN_DOCUMENTO');
            setNumeroDocumento(pendiente.numero_documento ?? '');
            setFechaDocumento(pendiente.fecha_documento ?? getFechaActualLima());
            setObservacion(pendiente.observacion ?? '');
            setDetalles(
              pendiente.detalles.map((d) => {
                const p = lista.find((p) => p.id_producto === d.id_producto);
                return {
                  id_producto: d.id_producto,
                  nombre: p?.nombre ?? `Producto ${d.id_producto}`,
                  unidad: p?.unidad_medida ?? 'UND',
                  cantidadInput: p?.unidad_medida === 'ML' ? d.cantidad / 1000 : d.cantidad,
                  costoUnitarioSoles: d.costo_efectivo_micro / 1000000,
                };
              })
            );
          }
        } catch (error) {
          setErrorLocal(String(error));
        }
      })();
      inventarioService
        .listarProveedores()
        .then(setProveedores)
        .catch((err: unknown) =>
          onError(
            typeof err === 'string'
              ? err
              : err instanceof Error
                ? err.message
                : 'No se pudieron cargar los proveedores. Vuelve a abrir la recepción de compra para reintentar.'
          )
        );
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
    const newId = await inventarioService.crearProveedor(payload);
    const guardado: ProveedorDTO = { ...payload, id_proveedor: newId };
    setProveedores((prev) => [...prev.filter((p) => p.id_proveedor !== newId), guardado]);
    setIdProveedor(newId);
    notificar('Proveedor registrado correctamente', 'EXITO');
  };

  const handleActualizarProveedor = async (payload: ActualizarProveedorDTO) => {
    await inventarioService.actualizarProveedor(payload);
    setProveedores((prev) =>
      prev.map((p) => (p.id_proveedor === payload.id_proveedor ? { ...p, ...payload } : p))
    );
    notificar('Proveedor actualizado correctamente', 'EXITO');
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
    if (enviando.current) return;
    if (!idProveedor) {
      onError('Debe seleccionar un proveedor');
      return;
    }
    if (detalles.length === 0) {
      onError('Debe agregar al menos un producto a la compra');
      return;
    }

    try {
      enviando.current = true;
      setGuardando(true);
      setErrorLocal('');

      const detallesPayload: DetalleIngresoRequestDTO[] = detalles.map((d) => {
        const esML = d.unidad.toUpperCase() === 'ML';
        const cantidadBase = esML
          ? Math.round(d.cantidadInput * 1000)
          : Math.round(d.cantidadInput);

        if (
          !Number.isFinite(d.cantidadInput) ||
          d.cantidadInput <= 0 ||
          Math.abs((esML ? d.cantidadInput * 1000 : d.cantidadInput) - cantidadBase) > 0.000001
        ) {
          throw new Error('Usa cantidades enteras en UND o hasta tres decimales en litros.');
        }
        const costoEfectivoMicro = solesAMicro(d.costoUnitarioSoles);

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

      const pendiente = solicitudPendiente('registrar_ingreso_compra_cmd');
      await inventarioService.registrarIngresoCompra(
        (pendiente?.payload as RegistrarIngresoDTO) ?? payload
      );
      setDetalles([]);
      setNumeroDocumento('');
      setObservacion('');
      setIncierta(false);
      onCompraExitosa();
      onClose();
    } catch (err: unknown) {
      const msg =
        typeof err === 'string'
          ? err
          : (err as { message?: string })?.message || 'Error al registrar la compra';
      setErrorLocal(msg);
      setIncierta(!!solicitudPendiente('registrar_ingreso_compra_cmd'));
      onError(msg);
    } finally {
      enviando.current = false;
      setGuardando(false);
    }
  };

  return (
    <div
      className="gloria-theme inv-overlay"
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
        className="inv-dialog inv-dialog-wide"
        role="dialog"
        aria-modal="true"
        aria-labelledby="inv-purchase-title"
      >
        <header className="inv-dialog-header">
          <div>
            <span className="inv-dialog-kicker">Entrada al almacén</span>
            <h3 id="inv-purchase-title">Ingreso por Compra a Proveedor</h3>
            <p>Registra el documento y los productos que ingresan al almacén.</p>
          </div>
        </header>
        <form onSubmit={handleSubmit} className="inv-form">
          <fieldset disabled={guardando || incierta} className="inv-form-section">
            <section className="inv-form-section">
              <h4 className="inv-section-label">Proveedor y documento</h4>
              <div className="inv-supplier-field">
                <label className="inv-field" style={{ flex: 1 }}>
                  Proveedor *
                  <select
                    required
                    value={idProveedor}
                    onChange={(e) => setIdProveedor(Number(e.target.value) || '')}
                  >
                    <option value="">Seleccione Proveedor...</option>
                    {proveedores.map((p) => (
                      <option key={p.id_proveedor} value={p.id_proveedor}>
                        {p.nombre_razon_social} ({p.tipo_documento}: {p.numero_documento})
                      </option>
                    ))}
                  </select>
                </label>
                <button
                  type="button"
                  className="inv-button"
                  onClick={() => {
                    setProveedorAEditar(null);
                    setModalProvAbierto(true);
                  }}
                >
                  <UserPlus size={16} />
                  Nuevo
                </button>
                {idProveedor && (
                  <button
                    type="button"
                    className="inv-icon-button"
                    title="Editar proveedor seleccionado"
                    aria-label="Editar proveedor seleccionado"
                    onClick={handleEditarProveedorSeleccionado}
                  >
                    <Edit size={16} />
                  </button>
                )}
              </div>
              <div className="inv-form-grid inv-document-grid">
                <label className="inv-field">
                  Tipo comprobante *
                  <select value={tipoDocumento} onChange={(e) => setTipoDocumento(e.target.value)}>
                    <option value="FACTURA">FACTURA</option>
                    <option value="BOLETA">BOLETA</option>
                    <option value="OTRO">OTRO</option>
                    <option value="SIN_DOCUMENTO">SIN DOCUMENTO</option>
                  </select>
                </label>
                <label className="inv-field">
                  N.º comprobante {tipoDocumento !== 'SIN_DOCUMENTO' ? '*' : ''}
                  <input
                    required={tipoDocumento !== 'SIN_DOCUMENTO'}
                    disabled={tipoDocumento === 'SIN_DOCUMENTO'}
                    placeholder="F001-0001234"
                    value={tipoDocumento === 'SIN_DOCUMENTO' ? '' : numeroDocumento}
                    onChange={(e) => setNumeroDocumento(e.target.value)}
                  />
                </label>
                <label className="inv-field">
                  Fecha documento {tipoDocumento !== 'SIN_DOCUMENTO' ? '*' : ''}
                  <input
                    required={tipoDocumento !== 'SIN_DOCUMENTO'}
                    disabled={tipoDocumento === 'SIN_DOCUMENTO'}
                    type="date"
                    value={fechaDocumento}
                    onChange={(e) => setFechaDocumento(e.target.value)}
                  />
                </label>
              </div>
              <label className="inv-field">
                Observación · opcional
                <input
                  placeholder="Ej. Recepción de lote con guía interna o factura crédito"
                  value={observacion}
                  onChange={(e) => setObservacion(e.target.value)}
                />
              </label>
            </section>
            <section className="inv-purchase-block">
              <h4 className="inv-purchase-heading">Productos de la compra</h4>
              <div className="inv-purchase-add">
                <label className="inv-field">
                  Producto
                  <select
                    value={prodSeleccionadoId}
                    onChange={(e) => setProdSeleccionadoId(Number(e.target.value) || '')}
                  >
                    <option value="">Buscar Producto...</option>
                    {productos.map((p) => (
                      <option key={p.id_producto} value={p.id_producto}>
                        {p.nombre} ({p.codigo}) - [{p.unidad_medida === 'ML' ? 'L' : 'UND'}]
                      </option>
                    ))}
                  </select>
                </label>
                <label className="inv-field">
                  Cantidad (
                  {productos.find((p) => p.id_producto === prodSeleccionadoId)?.unidad_medida ===
                  'ML'
                    ? 'L'
                    : 'UND'}
                  )
                  <input
                    aria-label="Cantidad del producto"
                    type="number"
                    step="0.001"
                    min="0.001"
                    placeholder="Cant / Litros"
                    value={cantInput}
                    onChange={(e) => setCantInput(parseFloat(e.target.value) || 0)}
                  />
                </label>
                <label className="inv-field">
                  Costo por UND / L
                  <input
                    aria-label="Costo por unidad o litro"
                    type="number"
                    step="0.01"
                    min="0.01"
                    placeholder="Costo S/"
                    value={costoInput || ''}
                    onChange={(e) => setCostoInput(parseFloat(e.target.value) || 0)}
                  />
                </label>
                <button type="button" className="inv-button" onClick={handleAgregarItem}>
                  <Plus size={16} />
                  Agregar producto
                </button>
              </div>
            </section>
            <div
              className="inv-purchase-lines inv-table-scroll"
              tabIndex={0}
              role="region"
              aria-label="Productos de la compra"
            >
              <table className="inv-table">
                <thead>
                  <tr>
                    <th>Producto</th>
                    <th className="inv-number">Cantidad</th>
                    <th className="inv-number">Costo U. (S/)</th>
                    <th className="inv-number">Subtotal</th>
                    <th>Acción</th>
                  </tr>
                </thead>
                <tbody>
                  {!detalles.length ? (
                    <tr>
                      <td colSpan={5} className="inv-empty">
                        Sin productos agregados al lote
                      </td>
                    </tr>
                  ) : (
                    detalles.map((d) => (
                      <tr key={d.id_producto}>
                        <td>{d.nombre}</td>
                        <td className="inv-number">
                          {d.cantidadInput} {d.unidad === 'ML' ? 'L' : 'UND'}
                        </td>
                        <td className="inv-number">S/ {d.costoUnitarioSoles.toFixed(2)}</td>
                        <td className="inv-number">
                          S/ {(d.cantidadInput * d.costoUnitarioSoles).toFixed(2)}
                        </td>
                        <td>
                          <button
                            type="button"
                            className="inv-icon-button"
                            aria-label={`Retirar ${d.nombre}`}
                            onClick={() => handleRemoverItem(d.id_producto)}
                          >
                            <Trash2 size={15} />
                          </button>
                        </td>
                      </tr>
                    ))
                  )}
                </tbody>
              </table>
            </div>
          </fieldset>
          {errorLocal && (
            <p role="alert" className="inv-form-error">
              {errorLocal}
            </p>
          )}
          {incierta && (
            <p className="inv-help">
              Se conservan los datos de la solicitud pendiente. Reintenta para confirmar el
              resultado.
            </p>
          )}
          <footer className="inv-form-footer inv-purchase-footer">
            <div>
              <span className="inv-help">Total de la compra</span>
              <strong>
                S/{' '}
                {totalCompraSoles.toLocaleString('es-PE', {
                  minimumFractionDigits: 2,
                  maximumFractionDigits: 2,
                })}
              </strong>
            </div>
            <div className="inv-row-actions">
              <button type="button" className="inv-button" disabled={guardando} onClick={onClose}>
                {incierta ? 'Conservar y cerrar' : 'Cancelar'}
              </button>
              <button type="submit" className="inv-button inv-primary" disabled={guardando}>
                {guardando
                  ? 'Guardando...'
                  : incierta
                    ? 'Reintentar compra pendiente'
                    : 'Registrar Compra'}
              </button>
            </div>
          </footer>
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
