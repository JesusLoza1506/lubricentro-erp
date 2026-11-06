import React, { useEffect, useState } from 'react';
import { CheckCircle2, XCircle, Wrench, Package, History } from 'lucide-react';
import {
  Producto,
  Categoria,
  CrearProductoDTO,
  EditarProductoDTO,
  ProductoForm,
  ConteoFisicoDTO,
  ProductoAdminDTO,
  ServicioDTO,
  CrearServicioDTO,
  ActualizarServicioDTO,
} from '../../types/inventario';
import { inventarioService } from '../../services/inventarioService';
import { InventarioHeader } from './components/InventarioHeader';
import { InventarioFiltros } from './components/InventarioFiltros';
import { InventarioTabla } from './components/InventarioTabla';
import { InventarioModal, ModoModal } from './components/InventarioModal';
import { ModalDespacho } from './components/ModalDespacho';
import { ModalAjusteStock } from './components/ModalAjusteStock';
import { ModalRegistroCompra } from './components/ModalRegistroCompra';
import { ModalKardex } from './components/ModalKardex';
import { ModalServicio } from './components/ModalServicio';
import { ModalHistorialEntradas } from './components/ModalHistorialEntradas';
import { ModalMotivoEstadoServicio } from './components/ModalMotivoEstadoServicio';
import { ServiciosTabla } from './components/ServiciosTabla';
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

  const [tabActiva, setTabActiva] = useState<'PRODUCTOS' | 'SERVICIOS'>('PRODUCTOS');

  // Estado de Productos (Paginados)
  const [productos, setProductos] = useState<Producto[]>([]);
  const [totalRegistros, setTotalRegistros] = useState<number>(0);
  const [paginaActual, setPaginaActual] = useState<number>(1);
  const limitePagina = 20;

  // Estado de Servicios
  const [servicios, setServicios] = useState<ServicioDTO[]>([]);

  const [categorias, setCategorias] = useState<Categoria[]>([]);
  const [cargando, setCargando] = useState<boolean>(true);
  const [despachando, setDespachando] = useState<boolean>(false);
  const [busqueda, setBusqueda] = useState<string>('');
  const [filtroCritico, setFiltroCritico] = useState<boolean>(false);
  const [mostrarInactivos, setMostrarInactivos] = useState<boolean>(false);

  // Modales
  const [modalAbierto, setModalAbierto] = useState<boolean>(false);
  const [modoModal, setModoModal] = useState<ModoModal>('CREAR');
  const [prodForm, setProdForm] = useState<ProductoForm>(prodInicial);
  const [modalCompraAbierto, setModalCompraAbierto] = useState<boolean>(false);
  const [modalHistorialEntradas, setModalHistorialEntradas] = useState<boolean>(false);

  const [modalServicioAbierto, setModalServicioAbierto] = useState(false);
  const [servicioAEditar, setServicioAEditar] = useState<ServicioDTO | null>(null);

  const [modalMotivoServicio, setModalMotivoServicio] = useState<{
    abierto: boolean;
    servicio: ServicioDTO | null;
    motivo: string;
  }>({ abierto: false, servicio: null, motivo: '' });

  const [modalKardex, setModalKardex] = useState<{ abierto: boolean; producto: Producto | null }>({
    abierto: false,
    producto: null,
  });

  const [modalDespacho, setModalDespacho] = useState<{
    abierto: boolean;
    producto: Producto | null;
    cantidad: number;
    motivo: string;
  }>({ abierto: false, producto: null, cantidad: 1, motivo: '' });

  const [modalAjuste, setModalAjuste] = useState<{
    abierto: boolean;
    producto: Producto | null;
  }>({ abierto: false, producto: null });

  const [confirmModal, setConfirmModal] = useState<{
    abierto: boolean;
    id: number | null;
    nombre: string;
    accion: 'DESACTIVAR' | 'REACTIVAR';
  }>({ abierto: false, id: null, nombre: '', accion: 'DESACTIVAR' });

  const [notificacion, setNotificacion] = useState<string | null>(null);
  const [errorModalMsg, setErrorModalMsg] = useState<string | null>(null);

  const mostrarToast = (msg: string) => {
    setNotificacion(msg);
    setTimeout(() => setNotificacion(null), 3500);
  };

  const cargarProductosPaginados = async () => {
    setCargando(true);
    try {
      if (filtroCritico) {
        const prodsCriticos = await inventarioService.listarProductosCriticos();
        setProductos((prodsCriticos as Producto[]) || []);
        setTotalRegistros(prodsCriticos ? prodsCriticos.length : 0);
      } else if (esAdmin) {
        const offset = (paginaActual - 1) * limitePagina;
        const resp = await inventarioService.listarProductosAdminPaginado(
          busqueda.trim() || undefined,
          undefined,
          mostrarInactivos,
          limitePagina,
          offset
        );
        setProductos(resp.productos || []);
        setTotalRegistros(resp.total_registros || 0);
      } else {
        const prodsPublicos = await inventarioService.listarProductosPublicos();
        let filtrados = prodsPublicos || [];
        if (busqueda.trim()) {
          const q = busqueda.trim().toLowerCase();
          filtrados = filtrados.filter(
            (p) => p.nombre.toLowerCase().includes(q) || p.codigo.toLowerCase().includes(q)
          );
        }
        setTotalRegistros(filtrados.length);
        const offset = (paginaActual - 1) * limitePagina;
        const paginados = filtrados.slice(offset, offset + limitePagina);
        setProductos(paginados as Producto[]);
      }
    } catch (err: unknown) {
      const msg =
        typeof err === 'string'
          ? err
          : (err as { message?: string })?.message || 'Error al cargar productos';
      setErrorModalMsg(msg);
    } finally {
      setCargando(false);
    }
  };

  const cargarServicios = async () => {
    setCargando(true);
    try {
      const data = esAdmin
        ? await inventarioService.listarServiciosAdmin()
        : await inventarioService.listarServiciosPublicos();
      setServicios(data || []);
    } catch (err: unknown) {
      const msg =
        typeof err === 'string'
          ? err
          : (err as { message?: string })?.message || 'Error al cargar servicios';
      setErrorModalMsg(msg);
    } finally {
      setCargando(false);
    }
  };

  const cargarCategorias = async () => {
    try {
      const cats = await inventarioService.listarCategorias();
      setCategorias(cats || []);
    } catch {
      // Fallback ignorado
    }
  };

  useEffect(() => {
    cargarCategorias();
  }, []);

  useEffect(() => {
    if (tabActiva === 'PRODUCTOS') {
      cargarProductosPaginados();
    } else {
      cargarServicios();
    }
  }, [tabActiva, paginaActual, filtroCritico, mostrarInactivos, busqueda, esAdmin]);

  const totalPaginas = Math.max(1, Math.ceil(totalRegistros / limitePagina));

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
      cargarProductosPaginados();
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
      cargarProductosPaginados();
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

  const ejecutarAjusteConteo = async (payload: ConteoFisicoDTO) => {
    try {
      await inventarioService.procesarConteoFisico(payload);
      mostrarToast('Conteo físico reconciliado correctamente');
      cargarProductosPaginados();
    } catch (err: unknown) {
      const msg =
        typeof err === 'string'
          ? err
          : (err as { message?: string })?.message || 'Error al aplicar el conteo físico';
      setErrorModalMsg(msg);
      throw err;
    }
  };

  const ejecutarCambioEstado = async () => {
    if (!confirmModal.id) return;
    try {
      if (confirmModal.accion === 'DESACTIVAR') {
        await inventarioService.eliminarProducto(confirmModal.id);
        mostrarToast(`Producto "${confirmModal.nombre}" dado de baja correctamente`);
      } else {
        await inventarioService.reactivarProducto(confirmModal.id);
        mostrarToast(`Producto "${confirmModal.nombre}" reactivado correctamente`);
      }
      setConfirmModal({ abierto: false, id: null, nombre: '', accion: 'DESACTIVAR' });
      cargarProductosPaginados();
    } catch (err: unknown) {
      setConfirmModal({ abierto: false, id: null, nombre: '', accion: 'DESACTIVAR' });
      const msg =
        typeof err === 'string'
          ? err
          : (err as { message?: string })?.message || 'Error al cambiar estado del producto';
      setErrorModalMsg(msg);
    }
  };

  const handleGuardarServicio = async (payload: CrearServicioDTO | ActualizarServicioDTO) => {
    try {
      if ('id_servicio' in payload) {
        await inventarioService.actualizarServicio(payload);
        mostrarToast('Servicio actualizado correctamente');
      } else {
        await inventarioService.crearServicio(payload);
        mostrarToast('Servicio creado correctamente');
      }
      cargarServicios();
    } catch (err: unknown) {
      const msg =
        typeof err === 'string'
          ? err
          : (err as { message?: string })?.message || 'Error al guardar servicio';
      setErrorModalMsg(msg);
      throw err;
    }
  };

  const handleIniciarToggleEstadoServicio = (s: ServicioDTO) => {
    setModalMotivoServicio({
      abierto: true,
      servicio: s,
      motivo: s.activo
        ? 'Desactivación temporal por actualización de tarifas'
        : 'Reactivación de servicio para atención al público',
    });
  };

  const handleConfirmarToggleEstadoServicio = async (e: React.FormEvent) => {
    e.preventDefault();
    const { servicio, motivo } = modalMotivoServicio;
    if (!servicio) return;

    if (motivo.trim().length < 5) {
      setErrorModalMsg('El motivo administrativo debe tener al menos 5 caracteres.');
      return;
    }

    try {
      await inventarioService.cambiarEstadoServicio(
        servicio.id_servicio,
        !servicio.activo,
        motivo.trim()
      );
      mostrarToast(
        `Servicio "${servicio.nombre}" ${!servicio.activo ? 'activado' : 'desactivado'}`
      );
      setModalMotivoServicio({ abierto: false, servicio: null, motivo: '' });
      cargarServicios();
    } catch (err: unknown) {
      const msg =
        typeof err === 'string'
          ? err
          : (err as { message?: string })?.message || 'Error al cambiar estado del servicio';
      setErrorModalMsg(msg);
    }
  };

  return (
    <div
      style={{
        flex: 1,
        padding: '32px',
        display: 'flex',
        flexDirection: 'column',
        gap: '20px',
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

      {/* Selector de Pestañas */}
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <div
          style={{
            display: 'flex',
            gap: '8px',
            backgroundColor: '#1E293B',
            padding: '4px',
            borderRadius: '10px',
          }}
        >
          <button
            onClick={() => setTabActiva('PRODUCTOS')}
            style={{
              padding: '8px 16px',
              backgroundColor: tabActiva === 'PRODUCTOS' ? '#2563EB' : 'transparent',
              color: tabActiva === 'PRODUCTOS' ? '#FFFFFF' : '#94A3B8',
              border: 'none',
              borderRadius: '8px',
              fontWeight: '700',
              cursor: 'pointer',
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              fontSize: '13px',
            }}
          >
            <Package size={16} /> Productos
          </button>
          <button
            onClick={() => setTabActiva('SERVICIOS')}
            style={{
              padding: '8px 16px',
              backgroundColor: tabActiva === 'SERVICIOS' ? '#2563EB' : 'transparent',
              color: tabActiva === 'SERVICIOS' ? '#FFFFFF' : '#94A3B8',
              border: 'none',
              borderRadius: '8px',
              fontWeight: '700',
              cursor: 'pointer',
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              fontSize: '13px',
            }}
          >
            <Wrench size={16} /> Servicios de Taller
          </button>
        </div>

        {esAdmin && (
          <button
            onClick={() => setModalHistorialEntradas(true)}
            style={{
              padding: '8px 14px',
              backgroundColor: 'rgba(56, 189, 248, 0.15)',
              border: '1px solid rgba(56, 189, 248, 0.3)',
              color: '#38BDF8',
              borderRadius: '8px',
              fontWeight: '600',
              cursor: 'pointer',
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              fontSize: '13px',
            }}
          >
            <History size={16} /> Ver Historial Entradas / Compras
          </button>
        )}
      </div>

      {tabActiva === 'PRODUCTOS' ? (
        <>
          {esAdmin && (
            <div style={{ display: 'flex', gap: '12px', alignItems: 'center' }}>
              <label
                style={{
                  fontSize: '13px',
                  color: '#CBD5E1',
                  cursor: 'pointer',
                  display: 'flex',
                  alignItems: 'center',
                  gap: '8px',
                }}
              >
                <input
                  type="checkbox"
                  checked={mostrarInactivos}
                  onChange={(e) => {
                    setMostrarInactivos(e.target.checked);
                    setPaginaActual(1);
                  }}
                />
                Mostrar Productos Inactivos / Dados de Baja
              </label>
            </div>
          )}

          <InventarioFiltros
            busqueda={busqueda}
            setBusqueda={(b) => {
              setBusqueda(b);
              setPaginaActual(1);
            }}
            filtroCritico={filtroCritico}
            setFiltroCritico={(fc) => {
              setFiltroCritico(fc);
              setPaginaActual(1);
            }}
            onRefresh={cargarProductosPaginados}
          />

          <InventarioTabla
            cargando={cargando}
            productos={productos}
            esAdmin={esAdmin}
            paginaActual={paginaActual}
            totalPaginas={totalPaginas}
            totalRegistros={totalRegistros}
            onCambiarPagina={(n) => setPaginaActual(n)}
            onVer={abrirModalVer}
            onDespachar={(p: Producto) =>
              setModalDespacho({ abierto: true, producto: p, cantidad: 1, motivo: '' })
            }
            onAjustar={(p: Producto) => setModalAjuste({ abierto: true, producto: p })}
            onVerKardex={(p: Producto) => setModalKardex({ abierto: true, producto: p })}
            onEditar={abrirModalEditar}
            onEliminar={(id: number, nombre: string, activo: boolean) =>
              setConfirmModal({
                abierto: true,
                id,
                nombre,
                accion: activo ? 'DESACTIVAR' : 'REACTIVAR',
              })
            }
          />
        </>
      ) : (
        <ServiciosTabla
          cargando={cargando}
          servicios={servicios}
          esAdmin={esAdmin}
          onEditar={(s: ServicioDTO) => {
            setServicioAEditar(s);
            setModalServicioAbierto(true);
          }}
          onToggleEstado={handleIniciarToggleEstadoServicio}
          onNuevoServicio={() => {
            setServicioAEditar(null);
            setModalServicioAbierto(true);
          }}
        />
      )}

      {/* Modales */}
      <ModalMotivoEstadoServicio
        abierto={modalMotivoServicio.abierto}
        servicio={modalMotivoServicio.servicio}
        motivo={modalMotivoServicio.motivo}
        setMotivo={(mot) => setModalMotivoServicio({ ...modalMotivoServicio, motivo: mot })}
        onClose={() => setModalMotivoServicio({ abierto: false, servicio: null, motivo: '' })}
        onSubmit={handleConfirmarToggleEstadoServicio}
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
        onClose={() => setModalAjuste({ abierto: false, producto: null })}
        onSubmit={ejecutarAjusteConteo}
      />

      <ModalKardex
        abierto={modalKardex.abierto}
        producto={modalKardex.producto}
        onClose={() => setModalKardex({ abierto: false, producto: null })}
      />

      <ModalRegistroCompra
        abierto={modalCompraAbierto}
        productos={productos}
        onClose={() => setModalCompraAbierto(false)}
        onCompraExitosa={() => {
          mostrarToast('Compra e ingreso de stock registrados con éxito');
          cargarProductosPaginados();
        }}
        onError={(msg) => setErrorModalMsg(msg)}
      />

      <ModalServicio
        abierto={modalServicioAbierto}
        servicioAEditar={servicioAEditar}
        onClose={() => {
          setModalServicioAbierto(false);
          setServicioAEditar(null);
        }}
        onGuardar={handleGuardarServicio}
      />

      <ModalHistorialEntradas
        abierto={modalHistorialEntradas}
        onClose={() => setModalHistorialEntradas(false)}
        onError={(msg) => setErrorModalMsg(msg)}
        onActualizado={() => {
          mostrarToast('Entrada anulada correctamente');
          cargarProductosPaginados();
        }}
      />

      <ConfirmModal
        abierto={confirmModal.abierto}
        titulo={
          confirmModal.accion === 'DESACTIVAR' ? 'Dar de Baja Producto' : 'Reactivar Producto'
        }
        mensaje={
          confirmModal.accion === 'DESACTIVAR'
            ? `¿Estás seguro de que deseas desactivar "${confirmModal.nombre}"? Se conservará su historial de Kardex.`
            : `¿Deseas volver a activar "${confirmModal.nombre}" en el catálogo principal?`
        }
        onConfirmar={ejecutarCambioEstado}
        onCancelar={() =>
          setConfirmModal({ abierto: false, id: null, nombre: '', accion: 'DESACTIVAR' })
        }
      />
    </div>
  );
};
