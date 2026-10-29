import React, { useEffect, useState } from 'react';
import { CheckCircle2, XCircle } from 'lucide-react';
import {
  Producto,
  Categoria,
  CrearProductoDTO,
  EditarProductoDTO,
  ProductoForm,
  AjustarStockDTO,
  ProductoAdminDTO,
} from '../../types/inventario';
import { inventarioService } from '../../services/inventarioService';
import { InventarioHeader } from './components/InventarioHeader';
import { InventarioFiltros } from './components/InventarioFiltros';
import { InventarioTabla } from './components/InventarioTabla';
import { InventarioModal, ModoModal } from './components/InventarioModal';
import { ModalDespacho } from './components/ModalDespacho';
import { ModalAjusteStock } from './components/ModalAjusteStock';
import { ModalRegistroCompra } from './components/ModalRegistroCompra';
import { ConfirmModal } from '../../components/ConfirmModal';
import { useAuth } from '../../context/AuthContext';

const prodInicial: ProductoForm = {
  codigo: '',
  nombre: '',
  descripcion: '',
  categoria: 'ACEITE',
  unidad_medida: 'UND',
  stock_minimo: 5,
  precio_venta_soles: 0,
  tiene_stock_inicial: false,
  cantidad_inicial: 0,
  costo_unitario_soles: 0,
  referencia_inicial: '',
};

export const InventarioPage: React.FC = () => {
  const { usuario } = useAuth();
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
  const [modalCompraAbierto, setModalCompraAbierto] = useState<boolean>(false);

  const [modalDespacho, setModalDespacho] = useState<{
    abierto: boolean;
    producto: Producto | null;
    cantidad: number;
    motivo: string;
  }>({ abierto: false, producto: null, cantidad: 1, motivo: '' });

  const [modalAjuste, setModalAjuste] = useState<{
    abierto: boolean;
    producto: Producto | null;
    tipo_ajuste: 'ENTRADA' | 'SALIDA';
    cantidad: number;
    costo_unitario_soles: number;
    motivo: string;
  }>({
    abierto: false,
    producto: null,
    tipo_ajuste: 'SALIDA',
    cantidad: 1,
    costo_unitario_soles: 0,
    motivo: '',
  });

  const [confirmModal, setConfirmModal] = useState<{
    abierto: boolean;
    id: number | null;
    nombre: string;
  }>({ abierto: false, id: null, nombre: '' });

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
          : (err as { message?: string })?.message || 'Error al cargar inventario';
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
    setProdForm(prodInicial);
    setModalAbierto(true);
  };

  const abrirModalVer = (p: Producto) => {
    setModoModal('VER');
    const pAdmin = p as ProductoAdminDTO;
    const costoSol = pAdmin.ultimo_costo_micro ? pAdmin.ultimo_costo_micro / 1_000_000 : 0;
    setProdForm({
      id_producto: p.id_producto,
      codigo: p.codigo,
      nombre: p.nombre,
      descripcion: p.descripcion || '',
      categoria: p.categoria,
      unidad_medida: p.unidad_medida,
      stock_minimo: p.stock_minimo,
      precio_venta_soles: p.precio_venta_cent / 100,
      tiene_stock_inicial: false,
      cantidad_inicial: p.stock_actual,
      costo_unitario_soles: costoSol,
      referencia_inicial: '',
    });
    setModalAbierto(true);
  };

  const abrirModalEditar = (p: Producto) => {
    setModoModal('EDITAR');
    setProdForm({
      id_producto: p.id_producto,
      codigo: p.codigo,
      nombre: p.nombre,
      descripcion: p.descripcion || '',
      categoria: p.categoria,
      unidad_medida: p.unidad_medida,
      stock_minimo: p.stock_minimo,
      precio_venta_soles: p.precio_venta_cent / 100,
      tiene_stock_inicial: false,
      cantidad_inicial: 0,
      costo_unitario_soles: 0,
      referencia_inicial: '',
    });
    setModalAbierto(true);
  };

  const handleGuardarProducto = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const precioVentaCentimos = Math.round(Number(prodForm.precio_venta_soles || 0) * 100);

      if (modoModal === 'CREAR') {
        const payloadCrear: CrearProductoDTO = {
          codigo: prodForm.codigo.trim().toUpperCase(),
          nombre: prodForm.nombre.trim(),
          descripcion: prodForm.descripcion?.trim() || null,
          categoria: prodForm.categoria,
          unidad_medida: prodForm.categoria === 'GRANEL' ? 'ML' : 'UND',
          stock_minimo: Number(prodForm.stock_minimo || 0),
          precio_venta_cent: precioVentaCentimos,
          cantidad_inicial: prodForm.tiene_stock_inicial
            ? Number(prodForm.cantidad_inicial || 0)
            : null,
          costo_unitario_inicial_micro: prodForm.tiene_stock_inicial
            ? Math.round(Number(prodForm.costo_unitario_soles || 0) * 1_000_000)
            : null,
          referencia_inicial: prodForm.tiene_stock_inicial
            ? prodForm.referencia_inicial.trim()
            : null,
        };
        await inventarioService.registrarProducto(payloadCrear);
        mostrarToast('Producto registrado correctamente');
      } else if (modoModal === 'EDITAR') {
        const payloadEditar: EditarProductoDTO = {
          id_producto: Number(prodForm.id_producto),
          nombre: prodForm.nombre.trim(),
          descripcion: prodForm.descripcion?.trim() || null,
          stock_minimo: Number(prodForm.stock_minimo || 0),
          precio_venta_cent: precioVentaCentimos,
        };
        await inventarioService.actualizarProducto(payloadEditar);
        mostrarToast('Producto actualizado correctamente');
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

  const ejecutarDespacho = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!modalDespacho.producto || despachando) return;
    try {
      setDespachando(true);
      await inventarioService.despacharProductoGranel(
        modalDespacho.producto.id_producto,
        Number(modalDespacho.cantidad),
        modalDespacho.motivo.trim() || undefined
      );
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

  const ejecutarAjuste = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!modalAjuste.producto) return;
    try {
      const payload: AjustarStockDTO = {
        id_producto: modalAjuste.producto.id_producto,
        tipo_ajuste: modalAjuste.tipo_ajuste,
        cantidad: Number(modalAjuste.cantidad),
        costo_unitario_micro:
          modalAjuste.tipo_ajuste === 'ENTRADA'
            ? Math.round(Number(modalAjuste.costo_unitario_soles) * 1_000_000)
            : null,
        motivo: modalAjuste.motivo.trim(),
      };
      await inventarioService.ajustarStock(payload);
      mostrarToast(`Ajuste registrado para "${modalAjuste.producto.nombre}"`);
      setModalAjuste({
        abierto: false,
        producto: null,
        tipo_ajuste: 'SALIDA',
        cantidad: 1,
        costo_unitario_soles: 0,
        motivo: '',
      });
      cargarDatos();
    } catch (err: unknown) {
      const msg =
        typeof err === 'string'
          ? err
          : (err as { message?: string })?.message || 'Error al guardar el ajuste';
      setErrorModalMsg(msg);
    }
  };

  const ejecutarEliminacion = async () => {
    if (!confirmModal.id) return;
    try {
      await inventarioService.eliminarProducto(confirmModal.id);
      setConfirmModal({ abierto: false, id: null, nombre: '' });
      mostrarToast(`Producto "${confirmModal.nombre}" dado de baja correctamente`);
      cargarDatos();
    } catch (err: unknown) {
      setConfirmModal({ abierto: false, id: null, nombre: '' });
      const msg =
        typeof err === 'string'
          ? err
          : (err as { message?: string })?.message || 'Error al dar de baja el producto';
      setErrorModalMsg(msg);
    }
  };

  const productosFiltrados = productos.filter(
    (p) =>
      p.nombre.toLowerCase().includes(busqueda.toLowerCase()) ||
      p.codigo.toLowerCase().includes(busqueda.toLowerCase())
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

      <InventarioHeader
        esAdmin={esAdmin}
        onAbrirModalCrear={abrirModalCrear}
        onAbrirModalCompra={() => setModalCompraAbierto(true)}
      />

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
            tipo_ajuste: 'SALIDA',
            cantidad: 1,
            costo_unitario_soles: 0,
            motivo: '',
          })
        }
        onEditar={abrirModalEditar}
        onEliminar={(id, nombre) => setConfirmModal({ abierto: true, id, nombre })}
      />

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
        tipoAjuste={modalAjuste.tipo_ajuste}
        setTipoAjuste={(tipo) => setModalAjuste({ ...modalAjuste, tipo_ajuste: tipo })}
        cantidad={modalAjuste.cantidad}
        setCantidad={(cant) => setModalAjuste({ ...modalAjuste, cantidad: cant })}
        costoUnitarioSoles={modalAjuste.costo_unitario_soles}
        setCostoUnitarioSoles={(costo) =>
          setModalAjuste({ ...modalAjuste, costo_unitario_soles: costo })
        }
        motivo={modalAjuste.motivo}
        setMotivo={(mot) => setModalAjuste({ ...modalAjuste, motivo: mot })}
        onClose={() =>
          setModalAjuste({
            abierto: false,
            producto: null,
            tipo_ajuste: 'SALIDA',
            cantidad: 1,
            costo_unitario_soles: 0,
            motivo: '',
          })
        }
        onSubmit={ejecutarAjuste}
      />

      <ModalRegistroCompra
        abierto={modalCompraAbierto}
        productos={productos}
        onClose={() => setModalCompraAbierto(false)}
        onCompraExitosa={() => {
          mostrarToast('Compra e ingreso de stock registrados con éxito');
          cargarDatos();
        }}
        onError={(msg) => setErrorModalMsg(msg)}
      />

      <ConfirmModal
        abierto={confirmModal.abierto}
        titulo="Dar de Baja Producto"
        mensaje={`¿Estás seguro de que deseas dar de baja "${confirmModal.nombre}"? Esta acción desactivará el producto.`}
        onConfirmar={ejecutarEliminacion}
        onCancelar={() => setConfirmModal({ abierto: false, id: null, nombre: '' })}
      />
    </div>
  );
};
