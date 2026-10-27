import { fireEvent, render, screen, waitFor, within, act } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import '@testing-library/jest-dom';
import { OrdenesPage } from '../OrdenesPage';
import { ordenesService } from '../../../services/ordenesService';
import { vehiculoService } from '../../../services/vehiculoService';
import { useAuth } from '../../../context/AuthContext';
vi.mock('../../../services/ordenesService');
vi.mock('../../../services/vehiculoService');
vi.mock('../../../context/AuthContext');
const usuario = { id_usuario: 1, nombre_completo: 'Administrador', rol: 'ADMINISTRADOR' };
const ot = { id_ot: 12, codigo_ot: 'OT-0012', placa: 'ABC123', estado: 'EN_ESPERA', kilometraje_ingreso: 60000, servicios: [], insumos: [] } as any;
const tablero = { zanjas: [{ id_zanja: 2, activa: 1, ocupada: false }], cola_espera: [ot] };
beforeEach(() => {
 vi.resetAllMocks();
 vi.mocked(useAuth).mockReturnValue({ usuario, tienePermiso: () => true } as any);
 vi.mocked(ordenesService.obtenerTableroZanjas).mockResolvedValue(tablero);
 vi.mocked(ordenesService.listarMecanicos).mockResolvedValue([]);
 vi.mocked(ordenesService.listarServicios).mockResolvedValue([{ id_servicio: 1, nombre: 'Cambio de aceite', precio_venta_soles: 20 }] as any);
 vi.mocked(ordenesService.listarProductosInsumo).mockResolvedValue([]);
 vi.mocked(vehiculoService.buscarVehiculosCandidatos).mockResolvedValue([{ id_vehiculo: 1, id_cliente: 2, placa: 'ABC123', marca: 'Toyota', modelo: 'Yaris', kilometraje_actual: 60000, activo: 1 }] as any);
});
const abrirRecepcion = async () => {
 fireEvent.click(screen.getByRole('button', { name: '+ Recepcionar vehículo' }));
 const dialogo = screen.getByRole('dialog', { name: 'Recepcionar vehículo' });
 const input = within(dialogo).getByPlaceholderText('Buscar por placa...');
 fireEvent.focus(input);
 fireEvent.change(input, { target: { value: 'ABC123' } });
 fireEvent.click(await within(dialogo).findByText(/Toyota Yaris/));
 await waitFor(() => expect(within(dialogo).getByRole('button', { name: 'Recepcionar OT' })).toBeEnabled());
 return dialogo;
};
describe('Avisos de recepción, inicio directo y consultas', () => {
 it('una recepción pendiente no anuncia éxito y lo muestra al confirmarse', async () => {
  let completar!: (valor: any) => void;
  vi.mocked(ordenesService.crearOrdenTrabajo).mockReturnValue(new Promise(resolve => { completar = resolve; }));
  render(<OrdenesPage />);
  await screen.findByText('ABC123');
  const dialogo = await abrirRecepcion();
  fireEvent.click(within(dialogo).getByRole('button', { name: 'Recepcionar OT' }));
  expect(screen.queryByText('¡Orden de trabajo creada correctamente!')).not.toBeInTheDocument();
  await act(async () => { completar(ot); });
  await screen.findByText('¡Orden de trabajo creada correctamente!');
  expect(ordenesService.crearOrdenTrabajo).toHaveBeenCalledTimes(1);
  expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
 });
 it('el rechazo conserva la recepción escrita y no anuncia éxito', async () => {
  vi.mocked(ordenesService.crearOrdenTrabajo).mockRejectedValue('El vehículo ya tiene una orden abierta.');
  render(<OrdenesPage />);
  await screen.findByText('ABC123');
  const dialogo = await abrirRecepcion();
  const pedido = within(dialogo).getByPlaceholderText('Detalle de fallas o pedido especial...');
  fireEvent.change(pedido, { target: { value: 'Revisar niveles y filtro' } });
  fireEvent.click(within(dialogo).getByRole('button', { name: 'Recepcionar OT' }));
  await waitFor(() => expect(within(dialogo).getByRole('alert')).toHaveTextContent('ya tiene una orden abierta'));
  expect(pedido).toHaveValue('Revisar niveles y filtro');
  expect(within(dialogo).getByPlaceholderText('Buscar por placa...')).toHaveValue('ABC123');
  expect(screen.queryByText('¡Orden de trabajo creada correctamente!')).not.toBeInTheDocument();
 });
 it('captura el rechazo del inicio directo y muestra su causa', async () => {
  vi.mocked(ordenesService.iniciarTrabajoOT).mockRejectedValue('La zanja seleccionada está ocupada.');
  render(<OrdenesPage />);
  fireEvent.change(await screen.findByRole('combobox'), { target: { value: '12' } });
  await screen.findByText('La zanja seleccionada está ocupada.');
  expect(screen.queryByText('¡Atención iniciada en la zanja seleccionada!')).not.toBeInTheDocument();
  expect(ordenesService.iniciarTrabajoOT).toHaveBeenCalledTimes(1);
 });
 it('notifica el inicio directo una vez después de guardarlo', async () => {
  vi.mocked(ordenesService.iniciarTrabajoOT).mockResolvedValue(ot);
  render(<OrdenesPage />);
  fireEvent.change(await screen.findByRole('combobox'), { target: { value: '12' } });
  await screen.findByText('¡Atención iniciada en la zanja seleccionada!');
  expect(screen.getAllByText('¡Atención iniciada en la zanja seleccionada!')).toHaveLength(1);
 });
 it('una consulta fallida ofrece reintento y no queda en carga indefinida', async () => {
  vi.mocked(ordenesService.obtenerTableroZanjas).mockRejectedValueOnce('Consulta interrumpida');
  render(<OrdenesPage />);
  const reintentar = await screen.findByRole('button', { name: 'Reintentar tablero' });
  expect(screen.queryByText('Cargando tablero en tiempo real...')).not.toBeInTheDocument();
  fireEvent.click(reintentar);
  await screen.findByText('ABC123');
  expect(screen.queryByRole('button', { name: 'Reintentar tablero' })).not.toBeInTheDocument();
 });
});
