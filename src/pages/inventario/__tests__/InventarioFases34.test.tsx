import { render, screen, fireEvent, waitFor, within, act, cleanup } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import '@testing-library/jest-dom';
import { InventarioPage } from '../InventarioPage';
import { ModalMotivoEstadoProducto } from '../components/ModalMotivoEstadoProducto';
import { ModalRegistroCompra } from '../components/ModalRegistroCompra';
import { ModalKardex } from '../components/ModalKardex';
import { ModalOrigenKardex, dineroMicro, cantidadKardex } from '../components/ModalOrigenKardex';
import { inventarioService } from '../../../services/inventarioService';
import { useAuth } from '../../../context/AuthContext';
vi.mock('../../../services/inventarioService');
vi.mock('../../../context/AuthContext');
const producto = {
  id_producto: 1,
  codigo: 'FIL-01',
  nombre: 'Filtro Uno',
  categoria: 'FILTRO' as const,
  unidad_medida: 'UND' as const,
  stock_actual: 0,
  stock_minimo: 5,
  precio_venta_cent: 1500,
  activo: true,
};
const pagina = { productos: [producto], total_registros: 1 };
function pendiente<T>() {
  let resolve!: (value: T) => void;
  let reject!: (error: unknown) => void;
  const promise = new Promise<T>((a, b) => {
    resolve = a;
    reject = b;
  });
  return { promise, resolve, reject };
}
beforeEach(() => {
  vi.resetAllMocks();
  vi.mocked(useAuth).mockReturnValue({
    usuario: { id_usuario: 1, rol: 'ADMINISTRADOR' },
  } as unknown as ReturnType<typeof useAuth>);
  vi.mocked(inventarioService.listarProductosAdminPaginado).mockResolvedValue(pagina as never);
  vi.mocked(inventarioService.listarProductosPublicosPaginado).mockResolvedValue(pagina);
  vi.mocked(inventarioService.listarCategorias).mockResolvedValue([]);
  vi.mocked(inventarioService.listarProveedores).mockResolvedValue([]);
  vi.mocked(inventarioService.listarServiciosAdmin).mockResolvedValue([]);
});
afterEach(cleanup);

describe('Fases 3 y 4 · Catálogo y motivo', () => {
  it.each(['CAJERO', 'MECANICO'])('usa paginación remota pública para %s', async (rol) => {
    vi.mocked(useAuth).mockReturnValue({ usuario: { id_usuario: 2, rol } } as unknown as ReturnType<
      typeof useAuth
    >);
    render(<InventarioPage />);
    expect(await screen.findByText('Filtro Uno')).toBeInTheDocument();
    expect(inventarioService.listarProductosPublicosPaginado).toHaveBeenCalledWith(
      undefined,
      'TODOS',
      20,
      0,
      false,
      'TODOS',
      'ACTIVO'
    );
    expect(inventarioService.listarProductosPublicos).not.toHaveBeenCalled();
    expect(inventarioService.listarProductosAdminPaginado).not.toHaveBeenCalled();
    fireEvent.click(screen.getByRole('button', { name: 'Ver Stock Crítico' }));
    await waitFor(() =>
      expect(inventarioService.listarProductosPublicosPaginado).toHaveBeenLastCalledWith(
        undefined,
        'TODOS',
        20,
        0,
        true,
        'TODOS',
        'ACTIVO'
      )
    );
  });
  it('reinicia la página al alternar críticos y conserva búsqueda y límite', async () => {
    vi.mocked(inventarioService.listarProductosAdminPaginado).mockResolvedValue({
      productos: [producto],
      total_registros: 45,
    } as never);
    render(<InventarioPage />);
    await screen.findByText('Filtro Uno');
    fireEvent.click(screen.getByRole('button', { name: 'Siguiente' }));
    await waitFor(() =>
      expect(inventarioService.listarProductosAdminPaginado).toHaveBeenLastCalledWith(
        undefined,
        'TODOS',
        false,
        20,
        20,
        false,
        'TODOS',
        'ACTIVO'
      )
    );
    fireEvent.change(screen.getByPlaceholderText('Buscar por nombre o código de barras...'), {
      target: { value: 'Filtro' },
    });
    fireEvent.click(screen.getByRole('button', { name: 'Ver Stock Crítico' }));
    await waitFor(() =>
      expect(inventarioService.listarProductosAdminPaginado).toHaveBeenLastCalledWith(
        'Filtro',
        'TODOS',
        false,
        20,
        0,
        true,
        'TODOS',
        'ACTIVO'
      )
    );
    fireEvent.click(screen.getByRole('button', { name: 'Siguiente' }));
    await waitFor(() =>
      expect(inventarioService.listarProductosAdminPaginado).toHaveBeenLastCalledWith(
        'Filtro',
        'TODOS',
        false,
        20,
        20,
        true,
        'TODOS',
        'ACTIVO'
      )
    );
    fireEvent.click(screen.getByRole('button', { name: 'Mostrando Stock Crítico' }));
    await waitFor(() =>
      expect(inventarioService.listarProductosAdminPaginado).toHaveBeenLastCalledWith(
        'Filtro',
        'TODOS',
        false,
        20,
        0,
        false,
        'TODOS',
        'ACTIVO'
      )
    );
    expect(inventarioService.listarProductosCriticos).not.toHaveBeenCalled();
  });
  it('una respuesta antigua no sustituye el filtro más reciente', async () => {
    const lenta = pendiente<typeof pagina>();
    vi.mocked(inventarioService.listarProductosAdminPaginado).mockImplementation(async (q) =>
      q === 'lento'
        ? (lenta.promise as never)
        : ({
            productos: [
              { ...producto, nombre: q === 'rapido' ? 'Resultado actual' : 'Filtro Uno' },
            ],
            total_registros: 1,
          } as never)
    );
    render(<InventarioPage />);
    await screen.findByText('Filtro Uno');
    const campo = screen.getByPlaceholderText('Buscar por nombre o código de barras...');
    fireEvent.change(campo, { target: { value: 'lento' } });
    fireEvent.change(campo, { target: { value: 'rapido' } });
    await screen.findByText('Resultado actual');
    await act(async () =>
      lenta.resolve({
        productos: [{ ...producto, nombre: 'Resultado antiguo' }],
        total_registros: 1,
      })
    );
    expect(screen.getByText('Resultado actual')).toBeInTheDocument();
    expect(screen.queryByText('Resultado antiguo')).not.toBeInTheDocument();
  });
  it('rechaza motivos vacíos, cortos o mayores a 250 caracteres', async () => {
    const confirmar = vi.fn();
    render(
      <ModalMotivoEstadoProducto
        abierto
        nombre="Filtro"
        accion="DESACTIVAR"
        onConfirmar={confirmar}
        onCancelar={vi.fn()}
      />
    );
    for (const value of ['', 'abcd', 'a'.repeat(251)]) {
      fireEvent.change(screen.getByLabelText('Motivo administrativo'), { target: { value } });
      fireEvent.submit(screen.getByRole('dialog'));
      expect(await screen.findByRole('alert')).toHaveTextContent('5 a 250');
    }
    expect(confirmar).not.toHaveBeenCalled();
  });
  it('bloquea doble envío y conserva el motivo tras un rechazo', async () => {
    const envio = pendiente<void>();
    const confirmar = vi.fn().mockReturnValue(envio.promise);
    render(
      <ModalMotivoEstadoProducto
        abierto
        nombre="Filtro"
        accion="REACTIVAR"
        onConfirmar={confirmar}
        onCancelar={vi.fn()}
      />
    );
    fireEvent.change(screen.getByLabelText('Motivo administrativo'), {
      target: { value: 'Disponibilidad confirmada' },
    });
    fireEvent.submit(screen.getByRole('dialog'));
    fireEvent.submit(screen.getByRole('dialog'));
    expect(confirmar).toHaveBeenCalledTimes(1);
    expect(screen.getByRole('button', { name: 'Guardando…' })).toBeDisabled();
    await act(async () => envio.reject('Operación denegada'));
    expect(await screen.findByRole('alert')).toHaveTextContent('Operación denegada');
    expect(screen.getByLabelText('Motivo administrativo')).toHaveValue('Disponibilidad confirmada');
  });
});

describe('Fase 4 · Proveedores', () => {
  it('el error de creación llega al formulario y conserva lo escrito', async () => {
    vi.mocked(inventarioService.crearProveedor).mockRejectedValue('Documento ya registrado');
    const cerrar = vi.fn();
    render(
      <ModalRegistroCompra
        abierto
        productos={[producto]}
        onClose={cerrar}
        onCompraExitosa={vi.fn()}
        onError={vi.fn()}
      />
    );
    fireEvent.click(screen.getByRole('button', { name: /Nuevo Proveedor|Nuevo/i }));
    const nombre = screen.getByPlaceholderText('Distribuidora de Lubricantes SAC');
    fireEvent.change(nombre, { target: { value: 'Distribuidora Chincha SAC' } });
    fireEvent.change(screen.getByPlaceholderText('20100070970'), {
      target: { value: '20100070970' },
    });
    fireEvent.click(screen.getByRole('button', { name: 'Crear Proveedor' }));
    expect(await screen.findByRole('alert')).toHaveTextContent('Documento ya registrado');
    expect(nombre).toHaveValue('Distribuidora Chincha SAC');
    expect(cerrar).not.toHaveBeenCalled();
    expect(screen.getByRole('heading', { name: 'Nuevo Proveedor' })).toBeInTheDocument();
  });
});

describe('Fase 4 · Kardex', () => {
  it('muestra importes históricos, precisión de granel y abre el origen', async () => {
    vi.mocked(inventarioService.listarKardexProducto).mockResolvedValue([
      {
        id_movimiento: 1,
        id_producto: 1,
        tipo_movimiento: 'STOCK_INICIAL',
        unidad_medida: 'UND',
        stock_anterior: 0,
        diferencia: 1,
        stock_resultante: 1,
        costo_unitario_micro: 1234567,
        valor_anterior_micro: 0,
        valor_delta_micro: 1234567,
        valor_resultante_micro: 1234567,
        fecha_movimiento: '2026-11-01 09:00:00',
        id_ingreso: 1,
        id_ot: null,
        id_venta: null,
        id_devolucion: null,
        codigo_origen: 'ING-001',
      },
    ]);
    vi.mocked(inventarioService.obtenerResumenSaldosKardex).mockResolvedValue({
      saldo_inicial: 0,
      saldo_final: 1,
      saldo_actual: 1,
    });
    vi.mocked(inventarioService.obtenerOrigenKardex).mockResolvedValue({
      tipo: 'INGRESO',
      codigo: 'ING-001',
      fecha: '2026-11-01 09:00:00',
      estado: 'REGISTRADO',
      documento: 'F001-1234',
      resumen: null,
      lineas: [],
      comprobantes: [],
    });
    render(<ModalKardex abierto producto={producto} onClose={vi.fn()} />);
    expect((await screen.findAllByText('S/ 1.234567')).length).toBe(3);
    fireEvent.click(screen.getByRole('button', { name: 'Ver ING-001' }));
    expect(
      await screen.findByRole('dialog', { name: 'Origen del movimiento' })
    ).toBeInTheDocument();
    await waitFor(() => expect(inventarioService.obtenerOrigenKardex).toHaveBeenCalledWith(1));
    expect(await screen.findByText(/F001-1234/)).toBeInTheDocument();
    expect(cantidadKardex(1, 'ML')).toBe('0.001 L');
    expect(dineroMicro(8_999_999_999_999_999)).toBe('S/ 8,999,999,999.999999');
  });
  it('permite reintentar la consulta del origen sin ejecutar escrituras', async () => {
    vi.mocked(inventarioService.obtenerOrigenKardex)
      .mockRejectedValueOnce('Origen no disponible')
      .mockResolvedValue({
        tipo: 'OT',
        codigo: 'OT-001',
        fecha: '2026-11-01 09:00:00',
        estado: 'FINALIZADO',
        documento: 'ABC123',
        resumen: 'Cambio de filtro',
        lineas: [],
        comprobantes: [],
      });
    render(<ModalOrigenKardex idMovimiento={15} onClose={vi.fn()} />);
    expect(await screen.findByRole('alert')).toHaveTextContent('Origen no disponible');
    fireEvent.click(within(screen.getByRole('alert')).getByRole('button', { name: 'Reintentar' }));
    expect(await screen.findByText('Cambio de filtro')).toBeInTheDocument();
    expect(inventarioService.registrarIngresoCompra).not.toHaveBeenCalled();
  });
});

describe('Proveedor existente', () => {
  it('mantiene abierta la edición cuando el backend la rechaza', async () => {
    vi.mocked(inventarioService.listarProveedores).mockResolvedValue([
      {
        id_proveedor: 8,
        tipo_documento: 'RUC',
        numero_documento: '20100070970',
        nombre_razon_social: 'Distribuidora Original SAC',
      },
    ]);
    vi.mocked(inventarioService.actualizarProveedor).mockRejectedValue(
      'No se pudo actualizar el proveedor'
    );
    render(
      <ModalRegistroCompra
        abierto
        productos={[producto]}
        onClose={vi.fn()}
        onCompraExitosa={vi.fn()}
        onError={vi.fn()}
      />
    );
    await screen.findByRole('option', { name: /Distribuidora Original SAC/ });
    fireEvent.change(screen.getAllByRole('combobox')[0], { target: { value: '8' } });
    fireEvent.click(screen.getByRole('button', { name: 'Editar proveedor seleccionado' }));
    const nombre = screen.getByPlaceholderText('Distribuidora de Lubricantes SAC');
    fireEvent.change(nombre, { target: { value: 'Distribuidora Corregida SAC' } });
    fireEvent.click(screen.getByRole('button', { name: 'Guardar Cambios' }));
    expect(await screen.findByRole('alert')).toHaveTextContent(
      'No se pudo actualizar el proveedor'
    );
    expect(nombre).toHaveValue('Distribuidora Corregida SAC');
    expect(screen.getByRole('heading', { name: 'Editar Proveedor' })).toBeInTheDocument();
    expect(inventarioService.crearProveedor).not.toHaveBeenCalled();
  });
});
