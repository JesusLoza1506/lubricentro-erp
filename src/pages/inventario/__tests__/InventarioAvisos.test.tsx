import { GestionProveedores } from '../components/GestionProveedores';
import { act, cleanup, fireEvent, render, screen, waitFor, within } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import '@testing-library/jest-dom';
import { InventarioPage } from '../InventarioPage';
import { inventarioService } from '../../../services/inventarioService';
import { useAuth } from '../../../context/AuthContext';
import {
  AvisosInventarioContext,
  AvisoInventarioVisible,
  useAvisosInventario,
} from '../AvisosInventario';
vi.mock('../../../services/inventarioService');
vi.mock('../../../context/AuthContext');
const producto = {
  id_producto: 1,
  codigo: 'FIL-001',
  nombre: 'Filtro de aceite',
  categoria: 'FILTRO' as const,
  unidad_medida: 'UND' as const,
  stock_actual: 10,
  stock_minimo: 2,
  precio_venta_cent: 1500,
  activo: true,
};
function diferido<T>() {
  let resolver!: (v: T) => void;
  let rechazar!: (v: unknown) => void;
  const promesa = new Promise<T>((a, b) => {
    resolver = a;
    rechazar = b;
  });
  return { promesa, resolver, rechazar };
}
beforeEach(() => {
  vi.resetAllMocks();
  localStorage.clear();
  vi.mocked(useAuth).mockReturnValue({
    usuario: { id_usuario: 1, rol: 'ADMINISTRADOR' },
    ambienteActual: 'PRUEBA',
  } as ReturnType<typeof useAuth>);
  vi.mocked(inventarioService.listarProductosAdminPaginado).mockResolvedValue({
    productos: [producto],
    total_registros: 1,
  } as never);
  vi.mocked(inventarioService.listarCategorias).mockResolvedValue([]);
  vi.mocked(inventarioService.listarServiciosAdmin).mockResolvedValue([]);
  vi.mocked(inventarioService.listarProveedores).mockResolvedValue([]);
  vi.mocked(inventarioService.listarProductosPublicos).mockResolvedValue([producto]);
  vi.mocked(inventarioService.proponerCodigo).mockResolvedValue('PROD-000002');
});
afterEach(() => {
  cleanup();
  vi.useRealTimers();
});
async function crearFormulario() {
  render(<InventarioPage />);
  fireEvent.click(await screen.findByRole('button', { name: 'Nuevo Producto' }));
  const modal = await screen.findByRole('dialog', { name: 'Nuevo Producto' });
  fireEvent.change(within(modal).getByLabelText('Nombre del producto *'), {
    target: { value: 'Filtro nuevo' },
  });
  fireEvent.change(within(modal).getByLabelText('Precio de venta'), { target: { value: '15.00' } });
  return modal;
}
describe('Avisos de Inventario', () => {
  it('creación pendiente bloquea envío y anuncia éxito solo al confirmarse', async () => {
    const espera = diferido<number>();
    vi.mocked(inventarioService.registrarProducto).mockReturnValue(espera.promesa);
    const modal = await crearFormulario();
    const guardar = within(modal).getByRole('button', { name: 'Guardar' });
    fireEvent.click(guardar);
    fireEvent.click(guardar);
    expect(inventarioService.registrarProducto).toHaveBeenCalledOnce();
    expect(within(modal).getByRole('button', { name: 'Guardando...' })).toBeDisabled();
    expect(screen.queryByText('Producto registrado correctamente')).not.toBeInTheDocument();
    await act(async () => espera.resolver(2));
    expect(await screen.findByRole('status')).toHaveTextContent(
      'Producto registrado correctamente'
    );
    expect(screen.queryByRole('dialog', { name: 'Nuevo Producto' })).not.toBeInTheDocument();
  });
  it('rechazo al crear conserva valores, formulario y error, sin éxito', async () => {
    vi.mocked(inventarioService.registrarProducto).mockRejectedValue('Código ya registrado');
    const modal = await crearFormulario();
    fireEvent.click(within(modal).getByRole('button', { name: 'Guardar' }));
    expect(await within(modal).findByRole('alert')).toHaveTextContent('Código ya registrado');
    expect(within(modal).getByLabelText('Nombre del producto *')).toHaveValue('Filtro nuevo');
    expect(screen.queryByText('Producto registrado correctamente')).not.toBeInTheDocument();
    expect(document.querySelectorAll('.inv-notification')).toHaveLength(1);
  });
  it('edición anuncia un único éxito después de actualizar', async () => {
    vi.mocked(inventarioService.actualizarProducto).mockResolvedValue();
    render(<InventarioPage />);
    await screen.findByText('Filtro de aceite');
    fireEvent.click(screen.getByTitle('Editar Producto'));
    const modal = screen.getByRole('dialog', { name: 'Editar Producto' });
    fireEvent.change(within(modal).getByLabelText('Nombre del producto *'), {
      target: { value: 'Filtro actualizado' },
    });
    fireEvent.click(within(modal).getByRole('button', { name: 'Guardar' }));
    expect(await screen.findByRole('status')).toHaveTextContent(
      'Producto actualizado correctamente'
    );
    expect(inventarioService.actualizarProducto).toHaveBeenCalledWith(
      expect.objectContaining({ id_producto: 1, nombre: 'Filtro actualizado' })
    );
    expect(document.querySelectorAll('.inv-notification')).toHaveLength(1);
  });
  it('cancelar un alta no genera aviso de éxito ni escritura', async () => {
    const modal = await crearFormulario();
    fireEvent.click(within(modal).getByRole('button', { name: 'Cancelar' }));
    expect(inventarioService.registrarProducto).not.toHaveBeenCalled();
    expect(screen.queryByRole('status')).not.toBeInTheDocument();
  });
  it('el menú mantiene accesibles conteo y Kardex y se cierra con Escape', async () => {
    render(<InventarioPage />);
    await screen.findByText('Filtro de aceite');
    const trigger = screen.getByRole('button', { name: 'Más acciones: Filtro de aceite' });
    fireEvent.click(trigger);
    const menu = screen.getByRole('menu');
    expect(within(menu).getByRole('menuitem', { name: 'Conteo físico' })).toBeVisible();
    fireEvent.keyDown(menu, { key: 'Escape' });
    await waitFor(() => expect(screen.queryByRole('menu')).not.toBeInTheDocument());
    expect(trigger).toHaveFocus();
  });
  it('el temporizador de un aviso anterior no borra el siguiente', async () => {
    vi.useFakeTimers();
    function Avisos() {
      const { aviso, notificar, cerrar } = useAvisosInventario();
      return (
        <AvisosInventarioContext.Provider value={notificar}>
          <button onClick={() => notificar('Primer guardado', 'EXITO')}>Uno</button>
          <button onClick={() => notificar('Segundo guardado', 'EXITO')}>Dos</button>
          <AvisoInventarioVisible aviso={aviso} onCerrar={cerrar} />
        </AvisosInventarioContext.Provider>
      );
    }
    render(<Avisos />);
    fireEvent.click(screen.getByText('Uno'));
    act(() => vi.advanceTimersByTime(3000));
    fireEvent.click(screen.getByText('Dos'));
    act(() => vi.advanceTimersByTime(600));
    expect(screen.getByRole('status')).toHaveTextContent('Segundo guardado');
    act(() => vi.advanceTimersByTime(2900));
    expect(screen.queryByRole('status')).not.toBeInTheDocument();
  });
  it('proveedor avisa una sola vez y únicamente después de confirmar el alta', async () => {
    const espera = diferido<number>();
    const notificar = vi.fn();
    vi.mocked(inventarioService.crearProveedor).mockReturnValue(espera.promesa);
    render(
      <AvisosInventarioContext.Provider value={notificar}>
        <GestionProveedores />
      </AvisosInventarioContext.Provider>
    );
    fireEvent.click(screen.getByText('Proveedores'));
    fireEvent.click(screen.getByRole('button', { name: 'Nuevo proveedor' }));
    const modal = screen.getByRole('dialog', { name: 'Proveedor' });
    fireEvent.change(within(modal).getByPlaceholderText('20100070970'), {
      target: { value: '20100070970' },
    });
    fireEvent.change(within(modal).getByPlaceholderText('Distribuidora de Lubricantes SAC'), {
      target: { value: 'Proveedor de lubricantes SAC' },
    });
    fireEvent.click(within(modal).getByRole('button', { name: 'Crear Proveedor' }));
    expect(notificar).not.toHaveBeenCalled();
    await act(async () => espera.resolver(1));
    expect(notificar).toHaveBeenCalledExactlyOnceWith(
      'Proveedor registrado correctamente',
      'EXITO'
    );
    expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
  });
});
