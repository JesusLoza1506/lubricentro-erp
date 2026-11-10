import './inventarioVisual.css';
import {
  AvisosInventarioContext,
  AvisoInventarioVisible,
  useAvisosInventario,
} from './AvisosInventario';
import { configurarContextoInventario, solesAMicro } from '../../services/operacionInventario';
import React, { useEffect, useState, useRef } from 'react';
import { Wrench, Package, History, Search, AlertTriangle, SlidersHorizontal } from 'lucide-react';
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
import { ModalAjusteStock } from './components/ModalAjusteStock';
import { ModalRegistroCompra } from './components/ModalRegistroCompra';
import { ModalKardex } from './components/ModalKardex';
import { ModalServicio } from './components/ModalServicio';
import { ModalHistorialEntradas } from './components/ModalHistorialEntradas';
import { ModalMotivoEstadoServicio } from './components/ModalMotivoEstadoServicio';
import { ServiciosTabla } from './components/ServiciosTabla';
import { ModalMotivoEstadoProducto } from './components/ModalMotivoEstadoProducto';
import { useAuth } from '../../context/AuthContext';

function cantidadMinimaBase(valor: number, granel: boolean): number {
  const base = Number(valor || 0) * (granel ? 1000 : 1);
  if (
    !Number.isFinite(base) ||
    base < 0 ||
    Math.abs(base - Math.round(base)) > 0.000001 ||
    base > (granel ? 9999999999 : 9999999)
  )
    throw new Error(
      'Stock mínimo inválido: usa unidades enteras o hasta tres decimales en litros.'
    );
  return Math.round(base);
}

const prodInicial: ProductoForm = {
  codigo: '',
  nombre: '',
  descripcion: '',
  categoria: 'ACEITE',
  unidad_medida: 'UND',
  stock_minimo: 0,
  precio_venta_soles: 0,
  tiene_stock_inicial: false,
  cantidad_inicial: 0,
  costo_unitario_soles: 0,
  referencia_inicial: '',
};

type ProductoKardex = Pick<Producto, 'id_producto' | 'codigo' | 'nombre' | 'unidad_medida'>;
export const InventarioPage: React.FC<{ origenReporte?: ProductoKardex }> = ({ origenReporte }) => {
  const { usuario, ambienteActual } = useAuth();
  if (usuario)
    configurarContextoInventario(
      usuario.id_usuario,
      ambienteActual || usuario.ambiente_actual || 'DESCONOCIDO'
    );
  const esAdmin = usuario?.rol?.toUpperCase() === 'ADMINISTRADOR';

  const [tabActiva, setTabActiva] = useState<'PRODUCTOS' | 'SERVICIOS' | 'ENTRADAS'>('PRODUCTOS');

  // Estado de Productos (Paginados)
  const [productos, setProductos] = useState<Producto[]>([]);
  const [totalRegistros, setTotalRegistros] = useState<number>(0);
  const [paginaActual, setPaginaActual] = useState<number>(1);
  const limitePagina = 20;

  // Estado de Servicios
  const [servicios, setServicios] = useState<ServicioDTO[]>([]);

  const [categorias, setCategorias] = useState<Categoria[]>([]);
  const [datosDesactualizados, setDatosDesactualizados] = useState(false);
  const [cargando, setCargando] = useState<boolean>(true);
  const [busqueda, setBusqueda] = useState<string>('');
  const [filtroCritico, setFiltroCritico] = useState<boolean>(false);
  const [mostrarInactivos, setMostrarInactivos] = useState<boolean>(false);
  const [categoriaFiltro, setCategoriaFiltro] = useState('TODOS');
  const [stockFiltro, setStockFiltro] = useState('TODOS');
  const [estadoFiltro, setEstadoFiltro] = useState('ACTIVO');
  const [busquedaServicios, setBusquedaServicios] = useState('');
  const guardandoProducto = useRef(false);

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

  const [modalKardex, setModalKardex] = useState<{ abierto: boolean; producto: ProductoKardex | null }>({
    abierto: !!origenReporte,
    producto: origenReporte ?? null,
  });

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

  const consultaProductos = useRef(0);

  const { aviso, notificar, cerrar } = useAvisosInventario();
  const [errorProducto, setErrorProducto] = useState('');
  const [productoGuardando, setProductoGuardando] = useState(false);
  const [errorEstadoServicio, setErrorEstadoServicio] = useState('');
  const mostrarToast = (msg: string) => notificar(msg, 'EXITO');
  const setErrorModalMsg = (msg: string) => notificar(msg, 'ERROR');

  const cargarProductosPaginados = async () => {
    const consulta = ++consultaProductos.current;
    setCargando(true);
    try {
      const offset = (paginaActual - 1) * limitePagina;
      const resp = esAdmin
        ? await inventarioService.listarProductosAdminPaginado(
            busqueda.trim() || undefined,
            categoriaFiltro,
            mostrarInactivos,
            limitePagina,
            offset,
            filtroCritico,
            stockFiltro,
            estadoFiltro
          )
        : await inventarioService.listarProductosPublicosPaginado(
            busqueda.trim() || undefined,
            categoriaFiltro,
            limitePagina,
            offset,
            filtroCritico,
            stockFiltro,
            'ACTIVO'
          );
      if (consulta !== consultaProductos.current) return;
      const ultimaPagina = Math.max(1, Math.ceil(resp.total_registros / limitePagina));
      setTotalRegistros(resp.total_registros);
      if (paginaActual > ultimaPagina) {
        setPaginaActual(ultimaPagina);
        return;
      }
      setProductos(resp.productos);
      setDatosDesactualizados(false);
    } catch (err: unknown) {
      if (consulta !== consultaProductos.current) return;
      const msg =
        typeof err === 'string'
          ? err
          : (err as { message?: string })?.message || 'Error al cargar productos';
      setDatosDesactualizados(true);
      setErrorModalMsg(msg);
    } finally {
      if (consulta === consultaProductos.current) setCargando(false);
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
    } else if (tabActiva === 'SERVICIOS') {
      cargarServicios();
    }
    return () => {
      consultaProductos.current += 1;
    };
  }, [
    tabActiva,
    paginaActual,
    filtroCritico,
    mostrarInactivos,
    busqueda,
    esAdmin,
    categoriaFiltro,
    stockFiltro,
    estadoFiltro,
  ]);

  useEffect(() => {
    const refrescar = () => {
      if (document.visibilityState === 'hidden') return;
      if (tabActiva === 'PRODUCTOS') void cargarProductosPaginados();
      if (tabActiva === 'SERVICIOS') void cargarServicios();
    };
    window.addEventListener('focus', refrescar);
    document.addEventListener('visibilitychange', refrescar);
    return () => {
      window.removeEventListener('focus', refrescar);
      document.removeEventListener('visibilitychange', refrescar);
    };
  }, [
    tabActiva,
    paginaActual,
    busqueda,
    filtroCritico,
    categoriaFiltro,
    stockFiltro,
    estadoFiltro,
    esAdmin,
  ]);

  const totalPaginas = Math.max(1, Math.ceil(totalRegistros / limitePagina));

  const abrirModalCrear = async () => {
    setErrorProducto('');
    setModoModal('CREAR');
    try {
      const codigo = await inventarioService.proponerCodigo('PRODUCTO');
      setProdForm({ ...prodInicial, codigo });
      setModalAbierto(true);
    } catch (error) {
      setErrorModalMsg(String(error));
    }
  };

  const abrirModalVer = (p: Producto) => {
    setErrorProducto('');
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
      stock_minimo: p.unidad_medida === 'ML' ? p.stock_minimo / 1000 : p.stock_minimo,
      precio_venta_soles: p.precio_venta_cent / 100,
      tiene_stock_inicial: false,
      cantidad_inicial: p.stock_actual,
      costo_unitario_soles: costoSol,
      referencia_inicial: '',
    });
    setModalAbierto(true);
  };

  const abrirModalEditar = (p: Producto) => {
    setErrorProducto('');
    setModoModal('EDITAR');
    setProdForm({
      id_producto: p.id_producto,
      codigo: p.codigo,
      nombre: p.nombre,
      descripcion: p.descripcion || '',
      categoria: p.categoria,
      unidad_medida: p.unidad_medida,
      stock_minimo: p.unidad_medida === 'ML' ? p.stock_minimo / 1000 : p.stock_minimo,
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
    if (guardandoProducto.current) return;
    guardandoProducto.current = true;
    setProductoGuardando(true);
    setErrorProducto('');
    try {
      const precioVentaCentimos = solesAMicro(Number(prodForm.precio_venta_soles || 0)) / 10000;

      if (modoModal === 'CREAR') {
        const payloadCrear: CrearProductoDTO = {
          codigo: prodForm.codigo.trim().toUpperCase(),
          nombre: prodForm.nombre.trim(),
          descripcion: prodForm.descripcion?.trim() || null,
          categoria: prodForm.categoria,
          unidad_medida: prodForm.categoria === 'GRANEL' ? 'ML' : 'UND',
          stock_minimo: cantidadMinimaBase(prodForm.stock_minimo, prodForm.categoria === 'GRANEL'),
          precio_venta_cent: precioVentaCentimos,
          cantidad_inicial: prodForm.tiene_stock_inicial
            ? Number(prodForm.cantidad_inicial || 0)
            : null,
          costo_unitario_inicial_micro: prodForm.tiene_stock_inicial
            ? solesAMicro(Number(prodForm.costo_unitario_soles || 0))
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
          stock_minimo: cantidadMinimaBase(prodForm.stock_minimo, prodForm.categoria === 'GRANEL'),
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
      setErrorProducto(msg);
      setErrorModalMsg(msg);
    } finally {
      guardandoProducto.current = false;
      setProductoGuardando(false);
    }
  };

  const ejecutarAjusteConteo = async (payload: ConteoFisicoDTO) => {
    let resultado;
    try {
      resultado = await inventarioService.procesarConteoFisico(payload);
    } catch (error) {
      notificar(
        typeof error === 'string'
          ? error
          : error instanceof Error
            ? error.message
            : 'No se pudo guardar el conteo.',
        'ERROR'
      );
      throw error;
    }
    mostrarToast(
      resultado?.sin_diferencias
        ? 'Conteo verificado: sin diferencias de stock'
        : 'Conteo físico reconciliado correctamente'
    );
    void cargarProductosPaginados();
  };

  const ejecutarCambioEstado = async (motivo: string) => {
    if (!confirmModal.id || !esAdmin) throw new Error('No tienes permisos para cambiar el estado.');
    if (confirmModal.accion === 'DESACTIVAR') {
      await inventarioService.eliminarProducto(confirmModal.id, motivo);
      mostrarToast(`Producto "${confirmModal.nombre}" dado de baja correctamente`);
    } else {
      await inventarioService.reactivarProducto(confirmModal.id, motivo);
      mostrarToast(`Producto "${confirmModal.nombre}" reactivado correctamente`);
    }
    setConfirmModal({ abierto: false, id: null, nombre: '', accion: 'DESACTIVAR' });
    void cargarProductosPaginados();
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
    setErrorEstadoServicio('');
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
      setErrorEstadoServicio(msg);
      setErrorModalMsg(msg);
    }
  };

  return (
    <AvisosInventarioContext.Provider value={notificar}>
      <div className="gloria-theme inventario-page">
        <AvisoInventarioVisible aviso={aviso} onCerrar={cerrar} />
        <InventarioHeader
          esAdmin={esAdmin}
          seccion={tabActiva}
          onNuevoServicio={() => {
            setServicioAEditar(null);
            setModalServicioAbierto(true);
          }}
          onAbrirModalCrear={abrirModalCrear}
          onAbrirModalCompra={() => setModalCompraAbierto(true)}
        />

        <nav className="inv-tabs" aria-label="Secciones de inventario">
          <button
            className={tabActiva === 'PRODUCTOS' ? 'is-active' : ''}
            aria-current={tabActiva === 'PRODUCTOS' ? 'page' : undefined}
            onClick={() => setTabActiva('PRODUCTOS')}
          >
            <Package size={17} />
            Productos
          </button>
          <button
            className={tabActiva === 'SERVICIOS' ? 'is-active' : ''}
            aria-current={tabActiva === 'SERVICIOS' ? 'page' : undefined}
            onClick={() => setTabActiva('SERVICIOS')}
          >
            <Wrench size={17} />
            Servicios de Taller
          </button>
          {esAdmin && (
            <button
              className={tabActiva === 'ENTRADAS' ? 'is-active' : ''}
              aria-current={tabActiva === 'ENTRADAS' ? 'page' : undefined}
              onClick={() => setTabActiva('ENTRADAS')}
            >
              <History size={17} />
              Entradas / Compras
            </button>
          )}
        </nav>

        {tabActiva === 'PRODUCTOS' ? (
          <>
            {datosDesactualizados && (
              <div role="status" className="gt-notice inv-stale-notice">
                <AlertTriangle size={17} aria-hidden="true" />
                <span>No se pudieron actualizar los datos. Se conserva la última consulta.</span>
                <button
                  type="button"
                  className="inv-button"
                  onClick={() => void cargarProductosPaginados()}
                >
                  Reintentar consulta
                </button>
              </div>
            )}
            <section className="inv-filter-panel gt-stage" aria-label="Filtros de productos">
              <InventarioFiltros
                busqueda={busqueda}
                setBusqueda={(b) => {
                  setBusqueda(b);
                  setPaginaActual(1);
                }}
                filtroCritico={filtroCritico || stockFiltro === 'CRITICO'}
                setFiltroCritico={(fc) => {
                  setFiltroCritico(fc);
                  setStockFiltro('TODOS');
                  setPaginaActual(1);
                }}
                onRefresh={cargarProductosPaginados}
              />
              <div className="inv-filter-fields">
                <span className="inv-filter-label">
                  <SlidersHorizontal size={14} aria-hidden="true" /> Filtros
                </span>
                <label>
                  Categoría{' '}
                  <select
                    aria-label="Categoría"
                    value={categoriaFiltro}
                    onChange={(e) => {
                      setCategoriaFiltro(e.target.value);
                      setPaginaActual(1);
                    }}
                  >
                    {['TODOS', 'ACEITE', 'FILTRO', 'ADITIVO', 'GRANEL', 'OTRO'].map((c) => (
                      <option key={c} value={c}>
                        {c === 'TODOS' ? 'Todas' : c}
                      </option>
                    ))}
                  </select>
                </label>
                <label>
                  Stock{' '}
                  <select
                    aria-label="Stock"
                    value={stockFiltro}
                    onChange={(e) => {
                      setStockFiltro(e.target.value);
                      setFiltroCritico(false);
                      setPaginaActual(1);
                    }}
                  >
                    <option value="TODOS">Todos</option>
                    <option value="SIN_STOCK">Sin stock</option>
                    <option value="CRITICO">Crítico</option>
                    <option value="DISPONIBLE">Disponible</option>
                  </select>
                </label>
                {esAdmin && (
                  <label>
                    Estado{' '}
                    <select
                      aria-label="Estado del producto"
                      value={estadoFiltro}
                      onChange={(e) => {
                        setEstadoFiltro(e.target.value);
                        setMostrarInactivos(e.target.value !== 'ACTIVO');
                        setPaginaActual(1);
                      }}
                    >
                      <option value="ACTIVO">Activos</option>
                      <option value="INACTIVO">Inactivos</option>
                      <option value="TODOS">Todos</option>
                    </select>
                  </label>
                )}
              </div>
            </section>

            <InventarioTabla
              cargando={cargando}
              productos={productos}
              esAdmin={esAdmin}
              paginaActual={paginaActual}
              totalPaginas={totalPaginas}
              totalRegistros={totalRegistros}
              busqueda={busqueda}
              esVistaCriticos={filtroCritico || stockFiltro === 'CRITICO'}
              onCambiarPagina={(n) => setPaginaActual(n)}
              onVer={abrirModalVer}
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
        ) : tabActiva === 'SERVICIOS' ? (
          <>
            <section className="inv-filter-panel gt-stage" aria-label="Filtro de servicios">
              <div className="inv-search inv-service-search">
                <Search size={17} aria-hidden="true" />
                <input
                  aria-label="Buscar servicios"
                  placeholder="Buscar servicio por nombre o código"
                  value={busquedaServicios}
                  onChange={(e) => setBusquedaServicios(e.target.value)}
                />
              </div>
            </section>
            <ServiciosTabla
              cargando={cargando}
              servicios={servicios.filter((s) =>
                `${s.codigo} ${s.nombre}`
                  .toLocaleLowerCase()
                  .includes(busquedaServicios.trim().toLocaleLowerCase())
              )}
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
          </>
        ) : esAdmin ? (
          <ModalHistorialEntradas
            abierto
            integrado
            onClose={() => setTabActiva('PRODUCTOS')}
            onError={setErrorModalMsg}
            onActualizado={cargarProductosPaginados}
          />
        ) : null}

        {/* Modales */}
        <ModalMotivoEstadoServicio
          abierto={modalMotivoServicio.abierto}
          servicio={modalMotivoServicio.servicio}
          motivo={modalMotivoServicio.motivo}
          error={errorEstadoServicio}
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
          guardando={productoGuardando}
          error={errorProducto}
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

        <ModalMotivoEstadoProducto
          abierto={confirmModal.abierto}
          nombre={confirmModal.nombre}
          accion={confirmModal.accion}
          onConfirmar={ejecutarCambioEstado}
          onCancelar={() =>
            setConfirmModal({ abierto: false, id: null, nombre: '', accion: 'DESACTIVAR' })
          }
        />
      </div>
    </AvisosInventarioContext.Provider>
  );
};
