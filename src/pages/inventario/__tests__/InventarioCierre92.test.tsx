import { render, screen, fireEvent, waitFor, cleanup } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import '@testing-library/jest-dom';
import { ModalAjusteStock } from '../components/ModalAjusteStock';
import { ModalRegistroCompra } from '../components/ModalRegistroCompra';
import { ModalHistorialEntradas } from '../components/ModalHistorialEntradas';
import { inventarioService } from '../../../services/inventarioService';
import { configurarContextoInventario } from '../../../services/operacionInventario';
vi.mock('../../../services/inventarioService');
const producto = {
  id_producto: 1,
  codigo: 'FIL-01',
  nombre: 'Filtro de aceite',
  categoria: 'FILTRO' as const,
  unidad_medida: 'UND' as const,
  stock_actual: 10,
  stock_minimo: 0,
  precio_venta_cent: 1500,
  activo: true,
};
beforeEach(() => {
  localStorage.clear();
  vi.resetAllMocks();
  configurarContextoInventario(1, 'PRUEBA');
  vi.mocked(inventarioService.listarProductosPublicos).mockResolvedValue([producto]);
  vi.mocked(inventarioService.listarProveedores).mockResolvedValue([]);
  vi.mocked(inventarioService.listarIngresosStock).mockResolvedValue([]);
});
afterEach(cleanup);
describe('Cierre de inventario · captura y recuperación', () => {
  it('reabre un conteo incierto con los datos originales y permite reintentar', async () => {
    const payload = {
      id_producto: 1,
      stock_fisico_contado: 9,
      stock_observado: 10,
      costo_unitario_micro: null,
      motivo: 'Conteo de almacén',
    };
    localStorage.setItem(
      'inventario.solicitud.v1.PRUEBA.1.procesar_conteo_fisico_cmd',
      JSON.stringify({ id: 'operacion-pendiente-01', payload })
    );
    const enviar = vi.fn().mockResolvedValue(undefined);
    const cerrar = vi.fn();
    render(
      <ModalAjusteStock
        abierto
        producto={{ ...producto, stock_actual: 9 }}
        onSubmit={enviar}
        onClose={cerrar}
      />
    );
    expect(screen.getAllByRole('spinbutton')[0]).toHaveValue(9);
    expect(screen.getAllByRole('spinbutton')[0]).toBeDisabled();
    fireEvent.click(screen.getByRole('button', { name: 'Reintentar conteo pendiente' }));
    await waitFor(() => expect(enviar).toHaveBeenCalledWith(payload));
    expect(cerrar).toHaveBeenCalledOnce();
  });
  it('bloquea doble envío del conteo sin diferencias y no exige motivo', async () => {
    let resolver!: (v: void) => void;
    const enviar = vi.fn(
      () =>
        new Promise<void>((r) => {
          resolver = r;
        })
    );
    render(<ModalAjusteStock abierto producto={producto} onSubmit={enviar} onClose={vi.fn()} />);
    const boton = screen.getByRole('button', { name: 'Aplicar Reconciliación' });
    fireEvent.click(boton);
    fireEvent.click(boton);
    expect(enviar).toHaveBeenCalledOnce();
    resolver(undefined);
    await waitFor(() => expect(boton).not.toBeDisabled());
  });
  it('compra carga su propio catálogo completo, independiente de la página visible', async () => {
    vi.mocked(inventarioService.listarProductosPublicos).mockResolvedValue([
      producto,
      { ...producto, id_producto: 50, codigo: 'FIL-50', nombre: 'Filtro fuera de página' },
    ]);
    render(
      <ModalRegistroCompra
        abierto
        productos={[producto]}
        onClose={vi.fn()}
        onCompraExitosa={vi.fn()}
        onError={vi.fn()}
      />
    );
    expect(
      await screen.findByRole('option', { name: /Filtro fuera de página/ })
    ).toBeInTheDocument();
  });
  it('conserva una compra pendiente al reabrir y congela sus campos', async () => {
    const payload = {
      origen: 'COMPRA',
      id_proveedor: 1,
      tipo_documento: 'SIN_DOCUMENTO',
      numero_documento: null,
      fecha_documento: null,
      observacion: 'Compra pendiente',
      detalles: [{ id_producto: 1, cantidad: 2, costo_efectivo_micro: 1230000 }],
    };
    localStorage.setItem(
      'inventario.solicitud.v1.PRUEBA.1.registrar_ingreso_compra_cmd',
      JSON.stringify({ id: 'compra-pendiente-01', payload })
    );
    vi.mocked(inventarioService.listarProveedores).mockResolvedValue([
      {
        id_proveedor: 1,
        tipo_documento: 'RUC',
        numero_documento: '20100070970',
        nombre_razon_social: 'Proveedor',
      },
    ]);
    vi.mocked(inventarioService.registrarIngresoCompra).mockResolvedValue({
      status: 'OK',
      id_ingreso: 1,
    });
    render(
      <ModalRegistroCompra
        abierto
        productos={[producto]}
        onClose={vi.fn()}
        onCompraExitosa={vi.fn()}
        onError={vi.fn()}
      />
    );
    const boton = await screen.findByRole('button', { name: 'Reintentar compra pendiente' });
    expect(screen.getByDisplayValue('Compra pendiente')).toBeDisabled();
    fireEvent.click(boton);
    await waitFor(() =>
      expect(inventarioService.registrarIngresoCompra).toHaveBeenCalledWith(payload)
    );
  });
  it('envía las fechas de entradas al backend desde la primera página', async () => {
    render(
      <ModalHistorialEntradas
        abierto
        integrado
        onClose={vi.fn()}
        onError={vi.fn()}
        onActualizado={vi.fn()}
      />
    );
    await waitFor(() => expect(inventarioService.listarIngresosStock).toHaveBeenCalled());
    fireEvent.change(screen.getByLabelText('Entradas desde'), { target: { value: '2026-10-01' } });
    fireEvent.change(screen.getByLabelText('Entradas hasta'), { target: { value: '2026-10-31' } });
    fireEvent.click(screen.getByRole('button', { name: 'Aplicar fechas' }));
    await waitFor(() =>
      expect(inventarioService.listarIngresosStock).toHaveBeenLastCalledWith(
        20,
        0,
        undefined,
        undefined,
        '2026-10-01',
        '2026-10-31'
      )
    );
  });
});
