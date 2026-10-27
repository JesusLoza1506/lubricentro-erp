import React, { useEffect, useState } from 'react';
import { CheckCircle2, XCircle } from 'lucide-react';
import {
  Producto,
  Categoria,
  CrearProductoDTO,
  EditarProductoDTO,
  ProductoForm,
  DespacharProductoDTO,
  AjustarStockDTO,
} from '../../types/inventario';
import { inventarioService } from '../../services/inventarioService';
import { InventarioHeader } from './components/InventarioHeader';
import { InventarioFiltros } from './components/InventarioFiltros';
import { InventarioTabla } from './components/InventarioTabla';
import { InventarioModal, ModoModal } from './components/InventarioModal';
import { ModalDespacho } from './components/ModalDespacho';
import { ModalAjusteStock } from './components/ModalAjusteStock';
import { ConfirmModal } from '../../components/ConfirmModal';
import { useAuth } from '../../context/AuthContext';

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
  const { usuario } = useAuth();
  const idUsuarioActual = usuario?.id_usuario ?? 1;
  const esAdmin = usuario?.rol?.toUpperCase() === 'ADMINISTRADOR';

  const [productos, setProductos] = useState<Producto[]>([]);
  const [categorias, setCategorias] = useState<Categoria[]>([]);
  const [cargando, setCargando] = useState<boolean>(true);
  const [despachando, setDespachando] = useState<boolean>(false);
  const [busqueda, setBusqueda] = useState<string>('');
  const [filtroCritico, setFiltroCritico] = useState<boolean>(false);

  // Modales
  const [modalAbierto, setModalAbierto] = useState<boolean>(false);
  const [modoModal, setModoModal] = useState<ModoModal>('CREAR');
  const [prodForm, setProdForm] = useState<ProductoForm>(prodInicial);

  const [modalDespacho, setModalDespacho] = useState<{
    abierto: boolean;
    producto: Producto | null;
    cantidad: number;
    motivo: string;
  }>({ abierto: false, producto: null, cantidad: 1, motivo: '' });

  const [modalAjuste, setModalAjuste] = useState<{
    abierto: boolean;
    producto: Producto | null;
    tipo_movimiento: 'ENTRADA' | 'AJUSTE';
    cantidad_nueva: number;
    motivo: string;
  }>({ abierto: false, producto: null, tipo_movimiento: 'AJUSTE', cantidad_nueva: 0, motivo: '' });

  const [confirmModal, setConfirmModal] = useState<{
    abierto: boolean;
    id: number | null;
    nombre: string;
  }>({ abierto: false, id: null, nombre: '' });

  // Notificaciones & Errores
  const [notificacion, setNotificacion] = useState<string | null>(null);
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
          ? inventarioService.listarProductosCriticos(idUsuarioActual)
          : inventarioService.listarProductos(idUsuarioActual),
        inventarioService.listarCategorias(idUsuarioActual),
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
  }, [filtroCritico, idUsuarioActual]);

  // Handlers para Productos
  const abrirModalCrear = () => {
    setModoModal('CREAR');
    setProdForm({ ...prodInicial, id_categoria: categorias[0]?.id_categoria || 1 });
    setModalAbierto(true);
  };

  const abrirModalVer = (p: Producto) => {
    setModoModal('VER');
    const precioCompraSol = 'precio_compra' in p ? p.precio_compra / 100 : 0;
    setProdForm({
      ...p,
      id_producto: p.id_producto,
      id_categoria: p.id_categoria,
      precio_compra: precioCompraSol,
      precio_venta: p.precio_venta / 100,
    });
    setModalAbierto(true);
  };

  const abrirModalEditar = (p: Producto) => {
    setModoModal('EDITAR');
    const precioCompraSol = 'precio_compra' in p ? p.precio_compra / 100 : 0;
    setProdForm({
      ...p,
      id_producto: p.id_producto,
      id_categoria: p.id_categoria,
      precio_compra: precioCompraSol,
      precio_venta: p.precio_venta / 100,
    });
    setModalAbierto(true);
  };

  const handleGuardarProducto = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const precioCompraCentimos = Math.round(Number(prodForm.precio_compra || 0) * 100);
      const precioVentaCentimos = Math.round(Number(prodForm.precio_venta || 0) * 100);

      if (modoModal === 'CREAR') {
        const payloadCrear: CrearProductoDTO = {
          id_categoria: Number(prodForm.id_categoria || 1),
          nombre: prodForm.nombre.trim(),
          codigo_barras: prodForm.codigo_barras?.trim() || null,
          unidad_medida: prodForm.unidad_medida?.trim() || 'UNIDAD',
          stock_actual: Number(prodForm.stock_actual || 0),
          stock_minimo: Number(prodForm.stock_minimo || 0),
          precio_compra: precioCompraCentimos,
          precio_venta: precioVentaCentimos,
        };
        await inventarioService.registrarProducto(payloadCrear, idUsuarioActual);
        mostrarToast('Producto registrado con éxito');
      } else if (modoModal === 'EDITAR') {
        const payloadEditar: EditarProductoDTO = {
          id_producto: Number(prodForm.id_producto),
          id_categoria: Number(prodForm.id_categoria || 1),
          nombre: prodForm.nombre.trim(),
          codigo_barras: prodForm.codigo_barras?.trim() || null,
          unidad_medida: prodForm.unidad_medida?.trim() || 'UNIDAD',
          stock_actual: Number(prodForm.stock_actual || 0),
          stock_minimo: Number(prodForm.stock_minimo || 0),
          precio_compra: precioCompraCentimos,
          precio_venta: precioVentaCentimos,
        };
        await inventarioService.actualizarProducto(payloadEditar, idUsuarioActual);
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

  // Handlers para Despacho
  const ejecutarDespacho = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!modalDespacho.producto || despachando) return;
    try {
      setDespachando(true);
      const payload: DespacharProductoDTO = {
        id_producto: modalDespacho.producto.id_producto,
        cantidad: Number(modalDespacho.cantidad),
        motivo: modalDespacho.motivo.trim() || undefined,
      };
      await inventarioService.despacharProductoGranel(payload, idUsuarioActual);
      mostrarToast(`Despacho registrado para "${modalDespacho.producto.nombre}"`);
      setModalDespacho({ abierto: false, producto: null, cantidad: 1, motivo: '' });
      cargarDatos();
    } catch (err: unknown) {
      const msg =
        typeof err === 'string'
          ? err
          : (err as { message?: string })?.message || 'Error al procesar el despacho';
      setErrorModalMsg(msg);
    } finally {
      setDespachando(false);
    }
  };

  // Handlers para Ajuste de Stock
  const ejecutarAjuste = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!modalAjuste.producto) return;
    try {
      const payload: AjustarStockDTO = {
        id_producto: modalAjuste.producto.id_producto,
        tipo_movimiento: modalAjuste.tipo_movimiento,
        cantidad_nueva: Number(modalAjuste.cantidad_nueva),
        motivo: modalAjuste.motivo.trim(),
      };
      await inventarioService.ajustarStock(payload, idUsuarioActual);
      mostrarToast(`Ajuste de stock guardado para "${modalAjuste.producto.nombre}"`);
      setModalAjuste({
        abierto: false,
        producto: null,
        tipo_movimiento: 'AJUSTE',
        cantidad_nueva: 0,
        motivo: '',
      });
      cargarDatos();
    } catch (err: unknown) {
      const msg =
        typeof err === 'string'
          ? err
          : (err as { message?: string })?.message || 'Error al guardar el ajuste de stock';
      setErrorModalMsg(msg);
    }
  };

  // Handlers para Eliminación
  const ejecutarEliminacion = async () => {
    if (!confirmModal.id) return;
    try {
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
      {/* Toast Notificación */}
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

      {/* Modal Error */}
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

      {/* Componentes Principales */}
      <InventarioHeader esAdmin={esAdmin} onAbrirModalCrear={abrirModalCrear} />

      <InventarioFiltros
        busqueda={busqueda}
        setBusqueda={setBusqueda}
        filtroCritico={filtroCritico}
        setFiltroCritico={setFiltroCritico}
        onRefresh={cargarDatos}
      />

      <InventarioTabla
        cargando={cargando}
        productos={productosFiltrados}
        esAdmin={esAdmin}
        onVer={abrirModalVer}
        onDespachar={(p) =>
          setModalDespacho({ abierto: true, producto: p, cantidad: 1, motivo: '' })
        }
        onAjustar={(p) =>
          setModalAjuste({
            abierto: true,
            producto: p,
            tipo_movimiento: 'AJUSTE',
            cantidad_nueva: p.stock_actual,
            motivo: '',
          })
        }
        onEditar={abrirModalEditar}
        onEliminar={(id, nombre) => setConfirmModal({ abierto: true, id, nombre })}
      />

      {/* Modales Desacoplados */}
      <InventarioModal
        modalAbierto={modalAbierto}
        setModalAbierto={setModalAbierto}
        modo={modoModal}
        prodForm={prodForm}
        setProdForm={setProdForm}
        categorias={categorias}
        handleGuardar={handleGuardarProducto}
      />

      <ModalDespacho
        abierto={modalDespacho.abierto}
        producto={modalDespacho.producto}
        cantidad={modalDespacho.cantidad}
        setCantidad={(cant) => setModalDespacho({ ...modalDespacho, cantidad: cant })}
        motivo={modalDespacho.motivo}
        setMotivo={(mot) => setModalDespacho({ ...modalDespacho, motivo: mot })}
        onClose={() =>
          setModalDespacho({ abierto: false, producto: null, cantidad: 1, motivo: '' })
        }
        onSubmit={ejecutarDespacho}
        loading={despachando}
      />

      <ModalAjusteStock
        abierto={modalAjuste.abierto}
        producto={modalAjuste.producto}
        tipoMovimiento={modalAjuste.tipo_movimiento}
        setTipoMovimiento={(tipo) => setModalAjuste({ ...modalAjuste, tipo_movimiento: tipo })}
        cantidadNueva={modalAjuste.cantidad_nueva}
        setCantidadNueva={(cant) => setModalAjuste({ ...modalAjuste, cantidad_nueva: cant })}
        motivo={modalAjuste.motivo}
        setMotivo={(mot) => setModalAjuste({ ...modalAjuste, motivo: mot })}
        onClose={() =>
          setModalAjuste({
            abierto: false,
            producto: null,
            tipo_movimiento: 'AJUSTE',
            cantidad_nueva: 0,
            motivo: '',
          })
        }
        onSubmit={ejecutarAjuste}
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
