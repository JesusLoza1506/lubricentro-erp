import { render, screen, fireEvent, waitFor, within } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import '@testing-library/jest-dom';
import { InventarioPage } from '../InventarioPage';
import { inventarioService } from '../../../services/inventarioService';
import { useAuth } from '../../../context/AuthContext';
import { invoke } from '@tauri-apps/api/core';

// Mocks
vi.mock('../../../services/inventarioService');
vi.mock('../../../context/AuthContext');
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

const mockProductos = [
  {
    id_producto: 1,
    codigo: 'ACE-5W30',
    nombre: 'Aceite Sintético 5W30 1L',
    categoria: 'ACEITE',
    unidad_medida: 'UND',
    stock_actual: 15,
    stock_minimo: 5,
    precio_venta_cent: 4500,
    activo: 1,
    es_critico: false,
    ultimo_costo_micro: 25000000,
  },
  {
    id_producto: 2,
    codigo: 'GRANEL-10W40',
    nombre: 'Aceite Granel 10W40',
    categoria: 'GRANEL',
    unidad_medida: 'ML',
    stock_actual: 3000,
    stock_minimo: 5000,
    precio_venta_cent: 15,
    activo: 1,
    es_critico: true,
    ultimo_costo_micro: 8000,
  },
];

const mockCategorias = [
  { id_categoria: 1, nombre: 'ACEITE', activo: 1 },
  { id_categoria: 2, nombre: 'FILTRO', activo: 1 },
  { id_categoria: 3, nombre: 'GRANEL', activo: 1 },
];

const mockProveedores = [
  {
    id_proveedor: 1,
    tipo_documento: 'RUC',
    numero_documento: '20100070970',
    nombre_razon_social: 'Distribuidora Central SAC',
  },
];

describe('InventarioPage - Pruebas de Integración Frontend FASE 7', () => {
  beforeEach(() => {
    vi.clearAllMocks();

    (useAuth as ReturnType<typeof vi.fn>).mockReturnValue({
      usuario: { id_usuario: 1, nombre: 'Admin System', rol: 'ADMINISTRADOR' },
    });

    (inventarioService.listarProductosAdminPaginado as ReturnType<typeof vi.fn>).mockImplementation(
      async (busqueda, _categoria, _inactivos, _limite, _offset, criticos) => {
        let resultado = [...mockProductos];
        if (criticos) resultado = resultado.filter((p) => p.es_critico);
        if (busqueda) {
          resultado = resultado.filter(
            (p) =>
              p.nombre.toLowerCase().includes(busqueda.toLowerCase()) ||
              p.codigo.toLowerCase().includes(busqueda.toLowerCase())
          );
        }
        return {
          productos: resultado,
          total_registros: resultado.length,
        };
      }
    );

    (inventarioService.listarProductos as ReturnType<typeof vi.fn>).mockResolvedValue(
      mockProductos
    );
    (inventarioService.listarProductosCriticos as ReturnType<typeof vi.fn>).mockResolvedValue(
      mockProductos.filter((p) => p.es_critico)
    );
    (inventarioService.listarCategorias as ReturnType<typeof vi.fn>).mockResolvedValue(
      mockCategorias
    );
    (inventarioService.listarProveedores as ReturnType<typeof vi.fn>).mockResolvedValue(
      mockProveedores
    );
    (inventarioService.listarServiciosAdmin as ReturnType<typeof vi.fn>).mockResolvedValue([]);

    (invoke as ReturnType<typeof vi.fn>).mockImplementation((cmd) => {
      if (cmd === 'listar_kardex_producto_cmd') {
        return Promise.resolve([
          {
            id_movimiento: 101,
            id_producto: 1,
            tipo_movimiento: 'STOCK_INICIAL',
            unidad_medida: 'UND',
            stock_anterior: 0,
            diferencia: 15,
            stock_resultante: 15,
            fecha_movimiento: '2026-10-29 10:00:00',
            motivo: 'Carga inicial',
            usuario_nombre: 'Admin System',
          },
        ]);
      }
      if (cmd === 'obtener_resumen_saldos_kardex_cmd') {
        return Promise.resolve({ saldo_inicial: 0, saldo_final: 15 });
      }
      return Promise.resolve(null);
    });
  });

  it('1. Renderiza correctamente la tabla de productos e indicadores de stock', async () => {
    render(<InventarioPage />);

    expect(await screen.findByText(/^Inventario y Stock$/i)).toBeInTheDocument();
    expect(await screen.findByText('Aceite Sintético 5W30 1L')).toBeInTheDocument();
    expect(screen.getByText('Aceite Granel 10W40')).toBeInTheDocument();
    expect(
      within(screen.getByRole('region', { name: 'Catálogo de productos' })).getByText('Crítico')
    ).toBeInTheDocument();
  });

  it('2. Permite filtrar productos por búsqueda de texto', async () => {
    render(<InventarioPage />);

    const searchInput = await screen.findByPlaceholderText(
      /Buscar por nombre o código de barras.../i
    );
    fireEvent.change(searchInput, { target: { value: 'Granel' } });

    expect(await screen.findByText('Aceite Granel 10W40')).toBeInTheDocument();
    expect(screen.queryByText('Aceite Sintético 5W30 1L')).not.toBeInTheDocument();
  });

  it('3. Abre el modal de creación de producto y registra exitosamente', async () => {
    (inventarioService.registrarProducto as ReturnType<typeof vi.fn>).mockResolvedValue(3);

    render(<InventarioPage />);

    const btnNuevo = await screen.findByRole('button', { name: /Nuevo Producto/i });
    fireEvent.click(btnNuevo);

    const modalTitle = await screen.findByRole('heading', { name: /Nuevo Producto/i });
    expect(modalTitle).toBeInTheDocument();

    const modal = screen.getByRole('dialog', { name: 'Nuevo Producto' });
    fireEvent.change(within(modal).getByRole('textbox', { name: 'Código *' }), {
      target: { value: 'FILT-001' },
    });
    fireEvent.change(within(modal).getByRole('textbox', { name: 'Nombre del producto *' }), {
      target: { value: 'Filtro Aceite PH6607' },
    });

    fireEvent.change(screen.getByRole('spinbutton', { name: 'Precio de venta' }), {
      target: { value: '25.00' },
    });
    const formElement = modalTitle.closest('form');
    if (formElement) {
      fireEvent.submit(formElement);
    } else {
      const btnGuardar = screen.getByRole('button', { name: /^Guardar$/i });
      fireEvent.click(btnGuardar);
    }

    await waitFor(() => {
      expect(inventarioService.registrarProducto).toHaveBeenCalledWith(
        expect.objectContaining({
          codigo: 'FILT-001',
          nombre: 'Filtro Aceite PH6607',
        })
      );
    });
  });

  it('4. Maneja el error de concurrencia optimista en el conteo físico', async () => {
    (inventarioService.procesarConteoFisico as ReturnType<typeof vi.fn>).mockRejectedValue(
      'El stock del producto fue modificado por otro usuario mientras realizaba el conteo.'
    );

    render(<InventarioPage />);

    await screen.findByText('Aceite Sintético 5W30 1L');
    fireEvent.click(screen.getByRole('button', { name: 'Más acciones: Aceite Sintético 5W30 1L' }));
    const btnAjusteList = await screen.findAllByTitle('Conteo Físico Reconciliado');
    fireEvent.click(btnAjusteList[0]);

    expect(
      await screen.findByText(/Conteo físico de stock/i)
    ).toBeInTheDocument();

    const btnAplicar = screen.getByRole('button', { name: /Aplicar Reconciliación/i });
    fireEvent.click(btnAplicar);

    await waitFor(() => {
      expect(
        within(screen.getByRole('dialog', { name: 'Conteo físico' })).getByText(
          /El stock del producto fue modificado por otro usuario/i
        )
      ).toBeInTheDocument();
    });
  });

  it('5. Permite dar de baja (desactivar) un producto con confirmación', async () => {
    (inventarioService.eliminarProducto as ReturnType<typeof vi.fn>).mockResolvedValue(undefined);

    render(<InventarioPage />);

    await screen.findByText('Aceite Sintético 5W30 1L');
    fireEvent.click(screen.getByRole('button', { name: 'Más acciones: Aceite Sintético 5W30 1L' }));
    const btnBajaList = await screen.findAllByTitle('Dar de Baja Producto');
    fireEvent.click(btnBajaList[0]);

    expect(await screen.findByText(/Dar de Baja Producto/i)).toBeInTheDocument();

    fireEvent.change(screen.getByLabelText('Motivo administrativo'), {
      target: { value: 'Producto discontinuado' },
    });
    const btnConfirmar = screen.getByRole('button', { name: /Confirmar/i });
    fireEvent.click(btnConfirmar);

    await waitFor(() => {
      expect(inventarioService.eliminarProducto).toHaveBeenCalledWith(1, 'Producto discontinuado');
    });
  });
});
