import React, { useEffect, useState } from 'react';
import {
  Search,
  Plus,
  AlertTriangle,
  RefreshCw,
  Eye,
  Pencil,
  Trash2,
  CheckCircle2,
  XCircle,
} from 'lucide-react';
import {
  Producto,
  Categoria,
  CrearProductoDTO,
  EditarProductoDTO,
  ProductoForm,
} from '../types/inventario';
import { inventarioService } from '../services/inventarioService';
import { InventarioModal, ModoModal } from './inventario/InventarioModal';
import { getBadgeStyles } from './inventario/inventarioStyles';
import { ConfirmModal } from '../components/ConfirmModal';

const prodInicial: ProductoForm = {
  id_categoria: 1,
  nombre: '',
  codigo_barras: '',
  unidad_medida: 'UNIDAD',
  stock_actual: 0,
  stock_minimo: 5,
  precio_compra: 0,
  precio_venta: 0,
};

export const InventarioPage: React.FC = () => {
  const [productos, setProductos] = useState<Producto[]>([]);
  const [categorias, setCategorias] = useState<Categoria[]>([]);
  const [cargando, setCargando] = useState<boolean>(true);
  const [busqueda, setBusqueda] = useState<string>('');
  const [filtroCritico, setFiltroCritico] = useState<boolean>(false);

  // Modal de Producto
  const [modalAbierto, setModalAbierto] = useState<boolean>(false);
  const [modoModal, setModoModal] = useState<ModoModal>('CREAR');
  const [prodForm, setProdForm] = useState<ProductoForm>(prodInicial);

  // Modal de Confirmación de Eliminación
  const [confirmModal, setConfirmModal] = useState<{
    abierto: boolean;
    id: number | null;
    nombre: string;
  }>({
    abierto: false,
    id: null,
    nombre: '',
  });

  // Notificación Toast de Éxito
  const [notificacion, setNotificacion] = useState<string | null>(null);

  // Modal de Error Integrado
  const [errorModalMsg, setErrorModalMsg] = useState<string | null>(null);

  const mostrarToast = (msg: string) => {
    setNotificacion(msg);
    setTimeout(() => setNotificacion(null), 3500);
  };

  const cargarDatos = async () => {
    setCargando(true);
    try {
      const [prodsData, catsData] = await Promise.all([
        filtroCritico
          ? inventarioService.listarProductosCriticos()
          : inventarioService.listarProductos(),
        inventarioService.listarCategorias(),
      ]);
      setProductos(prodsData || []);
      setCategorias(catsData || []);
    } catch (err: unknown) {
      const msg =
        typeof err === 'string'
          ? err
          : (err as { message?: string })?.message || 'Error al cargar los datos del inventario';
      setErrorModalMsg(msg);
    } finally {
      setCargando(false);
    }
  };

  useEffect(() => {
    cargarDatos();
  }, [filtroCritico]);

  const abrirModalCrear = () => {
    setModoModal('CREAR');
    setProdForm({ ...prodInicial, id_categoria: categorias[0]?.id_categoria || 1 });
    setModalAbierto(true);
  };

  const abrirModalVer = (p: Producto) => {
    setModoModal('VER');
    setProdForm({
      ...p,
      idProducto: p.id_producto,
      idCategoria: p.id_categoria,
    });
    setModalAbierto(true);
  };

  const abrirModalEditar = (p: Producto) => {
    setModoModal('EDITAR');
    setProdForm({
      ...p,
      idProducto: p.id_producto,
      id_producto: p.id_producto,
      idCategoria: p.id_categoria,
      id_categoria: p.id_categoria,
    });
    setModalAbierto(true);
  };

  const solicitarEliminacion = (idProducto: number, nombre: string) => {
    setConfirmModal({ abierto: true, id: idProducto, nombre });
  };

  const ejecutarEliminacion = async () => {
    if (!confirmModal.id) return;
    try {
      const idUsuarioActual = 1;
      await inventarioService.eliminarProducto(confirmModal.id, idUsuarioActual);
      setConfirmModal({ abierto: false, id: null, nombre: '' });
      mostrarToast(`Producto "${confirmModal.nombre}" eliminado correctamente`);
      cargarDatos();
    } catch (err: unknown) {
      setConfirmModal({ abierto: false, id: null, nombre: '' });
      const msg =
        typeof err === 'string'
          ? err
          : (err as { message?: string })?.message || 'Error al eliminar el producto';
      setErrorModalMsg(msg);
    }
  };

  const handleGuardar = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      if (modoModal === 'CREAR') {
        const payloadCrear: CrearProductoDTO = {
          id_categoria: Number(prodForm.id_categoria || prodForm.idCategoria || 1),
          nombre: prodForm.nombre,
          codigo_barras: prodForm.codigo_barras || prodForm.codigoBarras || null,
          unidad_medida: prodForm.unidad_medida || prodForm.unidadMedida || null,
          stock_actual: Number(prodForm.stock_actual ?? prodForm.stockActual ?? 0),
          stock_minimo: Number(prodForm.stock_minimo ?? prodForm.stockMinimo ?? 0),
          precio_compra: Number(prodForm.precio_compra ?? prodForm.precioCompra ?? 0),
          precio_venta: Number(prodForm.precio_venta ?? prodForm.precioVenta ?? 0),
        };
        await inventarioService.registrarProducto(payloadCrear);
        mostrarToast('Producto registrado con éxito');
      } else if (modoModal === 'EDITAR') {
        const payloadEditar: EditarProductoDTO = {
          id_producto: Number(prodForm.id_producto || prodForm.idProducto),
          id_categoria: Number(prodForm.id_categoria || prodForm.idCategoria || 1),
          nombre: prodForm.nombre,
          codigo_barras: prodForm.codigo_barras || prodForm.codigoBarras || null,
          unidad_medida: prodForm.unidad_medida || prodForm.unidadMedida || null,
          stock_actual: Number(prodForm.stock_actual ?? prodForm.stockActual ?? 0),
          stock_minimo: Number(prodForm.stock_minimo ?? prodForm.stockMinimo ?? 0),
          precio_compra: Number(prodForm.precio_compra ?? prodForm.precioCompra ?? 0),
          precio_venta: Number(prodForm.precio_venta ?? prodForm.precioVenta ?? 0),
        };
        await inventarioService.actualizarProducto(payloadEditar);
        mostrarToast('Producto actualizado con éxito');
      }
      setModalAbierto(false);
      cargarDatos();
    } catch (err: unknown) {
      const msg =
        typeof err === 'string'
          ? err
          : (err as { message?: string })?.message || 'Error al guardar el producto';
      setErrorModalMsg(msg);
    }
  };

  const productosFiltrados = productos.filter(
    (p) =>
      p.nombre.toLowerCase().includes(busqueda.toLowerCase()) ||
      (p.codigo_barras && p.codigo_barras.includes(busqueda))
  );

  return (
    <div
      style={{
        flex: 1,
        padding: '32px',
        display: 'flex',
        flexDirection: 'column',
        gap: '24px',
        height: '100%',
        boxSizing: 'border-box',
        overflow: 'hidden',
        position: 'relative',
      }}
    >
      {/* Toast de Notificación de Éxito */}
      {notificacion && (
        <div
          style={{
            position: 'fixed',
            bottom: '24px',
            right: '24px',
            backgroundColor: '#10B981',
            color: '#FFFFFF',
            padding: '12px 20px',
            borderRadius: '10px',
            boxShadow: '0 10px 25px rgba(0,0,0,0.5)',
            display: 'flex',
            alignItems: 'center',
            gap: '10px',
            zIndex: 1200,
            fontWeight: '600',
          }}
        >
          <CheckCircle2 size={18} />
          {notificacion}
        </div>
      )}

      {/* Modal Personalizado de Error */}
      {errorModalMsg && (
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
            zIndex: 1300,
          }}
        >
          <div
            style={{
              width: '420px',
              backgroundColor: '#0F172A',
              border: '1px solid #EF4444',
              borderRadius: '16px',
              padding: '24px',
              boxShadow: '0 20px 40px rgba(239, 68, 68, 0.2)',
              textAlign: 'center',
            }}
          >
            <XCircle size={48} color="#EF4444" style={{ marginBottom: '12px' }} />
            <h3
              style={{ fontSize: '18px', fontWeight: '800', color: '#F8FAFC', margin: '0 0 8px 0' }}
            >
              Error en la Operación
            </h3>
            <p
              style={{
                color: '#CBD5E1',
                fontSize: '14px',
                lineHeight: '1.5',
                marginBottom: '20px',
              }}
            >
              {errorModalMsg}
            </p>
            <button
              onClick={() => setErrorModalMsg(null)}
              style={{
                padding: '10px 24px',
                backgroundColor: '#EF4444',
                color: '#FFFFFF',
                border: 'none',
                borderRadius: '8px',
                fontWeight: '700',
                cursor: 'pointer',
              }}
            >
              Entendido
            </button>
          </div>
        </div>
      )}

      {/* Encabezado */}
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <div>
          <h2 style={{ fontSize: '24px', fontWeight: '800', color: '#F8FAFC', margin: 0 }}>
            Inventario & Productos
          </h2>
          <p style={{ color: '#94A3B8', fontSize: '14px', margin: '4px 0 0 0' }}>
            Gestión de catálogo, precios y control de stock mínimo.
          </p>
        </div>
        <button
          onClick={abrirModalCrear}
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: '8px',
            padding: '12px 20px',
            backgroundColor: '#2563EB',
            color: '#FFFFFF',
            border: 'none',
            borderRadius: '10px',
            fontWeight: '700',
            cursor: 'pointer',
          }}
        >
          <Plus size={18} /> Nuevo Producto
        </button>
      </div>

      {/* Controles de Búsqueda y Filtros */}
      <div style={{ display: 'flex', gap: '16px', alignItems: 'center' }}>
        <div style={{ position: 'relative', flex: 1 }}>
          <Search
            size={18}
            color="#64748B"
            style={{
              position: 'absolute',
              left: '14px',
              top: '50%',
              transform: 'translateY(-50%)',
            }}
          />
          <input
            type="text"
            placeholder="Buscar por nombre o código de barras..."
            value={busqueda}
            onChange={(e) => setBusqueda(e.target.value)}
            style={{
              width: '100%',
              padding: '12px 16px 12px 42px',
              backgroundColor: 'rgba(15, 23, 42, 0.6)',
              border: '1px solid #334155',
              borderRadius: '10px',
              color: '#F8FAFC',
              fontSize: '14px',
              outline: 'none',
              boxSizing: 'border-box',
            }}
          />
        </div>

        <button
          onClick={() => setFiltroCritico(!filtroCritico)}
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: '8px',
            padding: '12px 16px',
            backgroundColor: filtroCritico ? 'rgba(239, 68, 68, 0.2)' : 'rgba(30, 41, 59, 0.6)',
            color: filtroCritico ? '#FCA5A5' : '#CBD5E1',
            border: filtroCritico ? '1px solid #EF4444' : '1px solid #334155',
            borderRadius: '10px',
            fontWeight: '600',
            cursor: 'pointer',
          }}
        >
          <AlertTriangle size={18} color={filtroCritico ? '#EF4444' : '#94A3B8'} />
          {filtroCritico ? 'Mostrando Stock Crítico' : 'Ver Stock Crítico'}
        </button>

        <button
          onClick={cargarDatos}
          style={{
            padding: '12px',
            backgroundColor: 'rgba(30, 41, 59, 0.6)',
            border: '1px solid #334155',
            borderRadius: '10px',
            color: '#CBD5E1',
            cursor: 'pointer',
          }}
        >
          <RefreshCw size={18} />
        </button>
      </div>

      {/* Tabla con Sticky Header */}
      <div
        style={{
          flex: 1,
          backgroundColor: 'rgba(15, 23, 42, 0.6)',
          border: '1px solid #334155',
          borderRadius: '16px',
          overflowY: 'auto',
          maxHeight: 'calc(100vh - 280px)',
        }}
      >
        <table
          style={{ width: '100%', borderCollapse: 'collapse', textAlign: 'left', fontSize: '14px' }}
        >
          <thead
            style={{
              position: 'sticky',
              top: 0,
              zIndex: 10,
              backgroundColor: '#1E293B',
              boxShadow: '0 2px 8px rgba(0,0,0,0.3)',
            }}
          >
            <tr style={{ borderBottom: '1px solid #334155', color: '#94A3B8' }}>
              <th style={{ padding: '16px' }}>CÓDIGO</th>
              <th style={{ padding: '16px' }}>PRODUCTO</th>
              <th style={{ padding: '16px' }}>UNIDAD</th>
              <th style={{ padding: '16px' }}>P. COMPRA</th>
              <th style={{ padding: '16px' }}>P. VENTA</th>
              <th style={{ padding: '16px' }}>STOCK</th>
              <th style={{ padding: '16px' }}>ESTADO</th>
              <th style={{ padding: '16px', textAlign: 'center' }}>ACCIONES</th>
            </tr>
          </thead>
          <tbody>
            {cargando ? (
              <tr>
                <td colSpan={8} style={{ padding: '32px', textAlign: 'center', color: '#94A3B8' }}>
                  Cargando productos...
                </td>
              </tr>
            ) : productosFiltrados.length === 0 ? (
              <tr>
                <td colSpan={8} style={{ padding: '32px', textAlign: 'center', color: '#94A3B8' }}>
                  No se encontraron productos en el inventario.
                </td>
              </tr>
            ) : (
              productosFiltrados.map((p) => {
                const esCritico = p.stock_actual <= p.stock_minimo;
                return (
                  <tr
                    key={p.id_producto}
                    style={{ borderBottom: '1px solid rgba(51, 65, 85, 0.4)' }}
                  >
                    <td style={{ padding: '16px', color: '#38BDF8', fontFamily: 'monospace' }}>
                      {p.codigo_barras || 'N/A'}
                    </td>
                    <td style={{ padding: '16px', fontWeight: '600', color: '#F8FAFC' }}>
                      {p.nombre}
                    </td>
                    <td style={{ padding: '16px', color: '#94A3B8' }}>
                      {p.unidad_medida || 'UNIDAD'}
                    </td>
                    <td style={{ padding: '16px', color: '#CBD5E1' }}>
                      S/ {p.precio_compra.toFixed(2)}
                    </td>
                    <td style={{ padding: '16px', fontWeight: '700', color: '#34D399' }}>
                      S/ {p.precio_venta.toFixed(2)}
                    </td>
                    <td
                      style={{
                        padding: '16px',
                        fontWeight: '700',
                        color: esCritico ? '#EF4444' : '#F8FAFC',
                      }}
                    >
                      {p.stock_actual}
                    </td>
                    <td style={{ padding: '16px' }}>
                      <span style={getBadgeStyles(esCritico)}>
                        {esCritico ? 'STOCK CRÍTICO' : 'NORMAL'}
                      </span>
                    </td>
                    <td style={{ padding: '16px', textAlign: 'center' }}>
                      <div style={{ display: 'flex', justifyContent: 'center', gap: '8px' }}>
                        <button
                          onClick={() => abrirModalVer(p)}
                          title="Ver Detalle"
                          style={btnAccionStyle}
                        >
                          <Eye size={16} color="#38BDF8" />
                        </button>
                        <button
                          onClick={() => abrirModalEditar(p)}
                          title="Editar Producto"
                          style={btnAccionStyle}
                        >
                          <Pencil size={16} color="#FBBF24" />
                        </button>
                        <button
                          onClick={() => solicitarEliminacion(p.id_producto, p.nombre)}
                          title="Eliminar Producto"
                          style={{
                            ...btnAccionStyle,
                            backgroundColor: 'rgba(239, 68, 68, 0.15)',
                            border: '1px solid rgba(239, 68, 68, 0.3)',
                          }}
                        >
                          <Trash2 size={16} color="#EF4444" />
                        </button>
                      </div>
                    </td>
                  </tr>
                );
              })
            )}
          </tbody>
        </table>
      </div>

      {/* Modales */}
      <InventarioModal
        modalAbierto={modalAbierto}
        setModalAbierto={setModalAbierto}
        modo={modoModal}
        prodForm={prodForm}
        setProdForm={setProdForm}
        categorias={categorias}
        handleGuardar={handleGuardar}
      />

      <ConfirmModal
        abierto={confirmModal.abierto}
        titulo="Confirmar Eliminación"
        mensaje={`¿Estás seguro de que deseas eliminar "${confirmModal.nombre}"? Esta acción removerá el producto del catálogo.`}
        onConfirmar={ejecutarEliminacion}
        onCancelar={() => setConfirmModal({ abierto: false, id: null, nombre: '' })}
      />
    </div>
  );
};

const btnAccionStyle: React.CSSProperties = {
  padding: '6px 10px',
  backgroundColor: 'rgba(30, 41, 59, 0.8)',
  border: '1px solid #334155',
  borderRadius: '8px',
  cursor: 'pointer',
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
};
