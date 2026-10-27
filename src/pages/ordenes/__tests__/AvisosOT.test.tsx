import React from 'react';
import { act, render, renderHook, screen, fireEvent, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import '@testing-library/jest-dom';
import { AvisosOTContext, NotificarOT, AvisoOTVisible, useAvisosOT } from '../AvisosOT';
import { useModalDetalleOT } from '../components/modalDetalle/useModalDetalleOT';
import { ModalDetalleHeader } from '../components/modalDetalle/subcomponents/ModalDetalleHeader';
import { ordenesService } from '../../../services/ordenesService';
import { useAuth } from '../../../context/AuthContext';
vi.mock('../../../services/ordenesService');
vi.mock('../../../context/AuthContext');
const orden = {
 id_ot: 10, codigo_ot: 'OT-0010', placa: 'ABC123', estado: 'EN_PROCESO', id_mecanico: 1,
 kilometraje_ingreso: 60000, nota_interna: 'Nota original', servicios: [],
 insumos: [{ id_detalle: 8, unidad_medida: 'UND' }], total_general_soles: 20,
} as any;
const notificar = vi.fn<NotificarOT>();
const wrapper = ({ children }: { children: React.ReactNode }) => <AvisosOTContext.Provider value={notificar}>{children}</AvisosOTContext.Provider>;
beforeEach(() => {
 vi.resetAllMocks();
 vi.mocked(useAuth).mockReturnValue({ usuario: { id_usuario: 1, rol: 'ADMINISTRADOR' } } as any);
 vi.mocked(ordenesService.listarMecanicos).mockResolvedValue([]);
 vi.mocked(ordenesService.listarServicios).mockResolvedValue([]);
 vi.mocked(ordenesService.listarProductosInsumo).mockResolvedValue([]);
 vi.mocked(ordenesService.obtenerTableroZanjas).mockResolvedValue({ zanjas: [], cola_espera: [] });
});
describe('Avisos de operaciones de Órdenes de Trabajo', () => {
 it('no anuncia éxito mientras el guardado sigue pendiente', async () => {
  let completar!: (v: any) => void;
  vi.mocked(ordenesService.agregarServicioOT).mockReturnValue(new Promise(resolve => { completar = resolve; }));
  const { result } = renderHook(() => useModalDetalleOT({ ot: orden }), { wrapper });
  act(() => result.current.setIdServicioSel(2));
  let pendiente!: Promise<void>;
  act(() => { pendiente = result.current.handleAgregarServicioSubmit(); });
  expect(notificar.mock.calls.filter(([,tipo]) => tipo === 'EXITO')).toHaveLength(0);
  await act(async () => { completar(orden); await pendiente; });
  expect(notificar).toHaveBeenCalledWith('¡Servicio agregado correctamente!', 'EXITO');
 });
 it.each([
  ['agregarServicioOT', 'handleAgregarServicioSubmit', '¡Servicio agregado correctamente!'],
  ['modificarServicioOT', 'handleModificarServicioSubmit', '¡Servicio actualizado correctamente!'],
  ['eliminarServicioOT', 'handleEliminarServicio', '¡Servicio retirado correctamente!'],
  ['registrarInsumoOT', 'handleAgregarInsumoSubmit', '¡Despacho de insumo registrado correctamente!'],
  ['devolverInsumoOT', 'handleDevolverInsumoSubmit', '¡Devolución de insumo registrada correctamente!'],
 ])('%s genera un único aviso al completar la operación', async (servicio, handler, mensaje) => {
  vi.spyOn(window, 'confirm').mockReturnValue(true);
  (ordenesService as any)[servicio].mockResolvedValue(orden);
  const { result } = renderHook(() => useModalDetalleOT({ ot: orden, onActualizarOT: vi.fn().mockResolvedValue(undefined) }), { wrapper });
  act(() => { result.current.setIdServicioSel(2); result.current.setIdProductoSel(3); result.current.setIdDetalleDevolver(8); result.current.setMotivoAccion('Envase no utilizado'); });
  await act(async () => { await (result.current as any)[handler](8); });
  expect((ordenesService as any)[servicio]).toHaveBeenCalledTimes(1);
  expect(notificar.mock.calls.filter(([,tipo]) => tipo === 'EXITO')).toEqual([[mensaje, 'EXITO']]);
 });
 it('muestra el rechazo del backend sin éxito y conserva el despacho escrito', async () => {
  vi.mocked(ordenesService.registrarInsumoOT).mockRejectedValue('Stock insuficiente para realizar el despacho.');
  const { result } = renderHook(() => useModalDetalleOT({ ot: orden }), { wrapper });
  act(() => { result.current.setIdProductoSel(3); result.current.setCantProductoSel(7); result.current.setModoAccion('AGREGAR_INSUMO'); });
  await act(async () => { await result.current.handleAgregarInsumoSubmit(); });
  expect(result.current.errorMsg).toBe('Stock insuficiente para realizar el despacho.');
  expect(result.current.idProductoSel).toBe(3);
  expect(result.current.cantProductoSel).toBe(7);
  expect(result.current.modoAccion).toBe('AGREGAR_INSUMO');
  expect(notificar.mock.calls).toEqual([['Stock insuficiente para realizar el despacho.', 'ERROR']]);
 });
 it('distingue un fallo de recarga de un fallo de guardado', async () => {
  vi.mocked(ordenesService.modificarServicioOT).mockResolvedValue(orden);
  const { result } = renderHook(() => useModalDetalleOT({ ot: orden, onActualizarOT: vi.fn().mockRejectedValue(new Error('Consulta no disponible')) }), { wrapper });
  act(() => result.current.setModoAccion('EDITAR_SERVICIO'));
  await act(async () => { await result.current.handleModificarServicioSubmit(); });
  expect(ordenesService.modificarServicioOT).toHaveBeenCalledTimes(1);
  expect(notificar).toHaveBeenCalledWith('¡Servicio actualizado correctamente!', 'EXITO');
  expect(result.current.errorMsg).toContain('La operación se guardó, pero');
  expect(result.current.errorMsg).toContain('Consulta no disponible');
  expect(result.current.modoAccion).toBe('NINGUNO');
 });
 it('una validación fallida no ejecuta la cancelación ni muestra éxito', async () => {
  const cancelar = vi.fn();
  const { result } = renderHook(() => useModalDetalleOT({ ot: orden, onCancelarOT: cancelar }), { wrapper });
  act(() => result.current.setMotivoAccion('abc'));
  await act(async () => { await result.current.handleCancelarSubmit(); });
  expect(cancelar).not.toHaveBeenCalled();
  expect(notificar.mock.calls).toEqual([['El motivo debe tener al menos 5 caracteres.', 'ERROR']]);
 });
 it('cancelar la confirmación de retirada no muestra mensajes ni retira el servicio', async () => {
  vi.spyOn(window, 'confirm').mockReturnValue(false);
  const { result } = renderHook(() => useModalDetalleOT({ ot: orden }), { wrapper });
  await act(async () => { await result.current.handleEliminarServicio(8); });
  expect(ordenesService.eliminarServicioOT).not.toHaveBeenCalled();
  expect(notificar).not.toHaveBeenCalled();
 });
 it('la nota rechazada permanece editable con su contenido y muestra la causa', async () => {
  vi.mocked(ordenesService.modificarNotaInternaOT).mockRejectedValue('No tiene permisos para editar esta orden.');
  render(<ModalDetalleHeader ordenActual={orden} rol="ADMINISTRADOR" idUsuario={1} />, { wrapper });
  fireEvent.click(screen.getByRole('button', { name: 'Editar' }));
  fireEvent.change(screen.getByRole('textbox'), { target: { value: 'Revisar filtro de aire' } });
  fireEvent.click(screen.getByRole('button', { name: 'Guardar' }));
  await waitFor(() => expect(screen.getByRole('alert')).toHaveTextContent('No tiene permisos'));
  expect(screen.getByRole('textbox')).toHaveValue('Revisar filtro de aire');
  expect(notificar.mock.calls).toEqual([['No tiene permisos para editar esta orden.', 'ERROR']]);
 });
 it('informa el guardado de nota una vez y mantiene la distinción ante fallo de recarga', async () => {
  vi.mocked(ordenesService.modificarNotaInternaOT).mockResolvedValue(orden);
  render(<ModalDetalleHeader ordenActual={orden} rol="ADMINISTRADOR" idUsuario={1} onActualizarOT={vi.fn().mockRejectedValue('Consulta interrumpida')} />, { wrapper });
  fireEvent.click(screen.getByRole('button', { name: 'Editar' }));
  fireEvent.click(screen.getByRole('button', { name: 'Guardar' }));
  await waitFor(() => expect(screen.getByRole('alert')).toHaveTextContent('La nota se guardó, pero'));
  expect(notificar.mock.calls.filter(([,tipo]) => tipo === 'EXITO')).toEqual([['¡Nota interna actualizada correctamente!', 'EXITO']]);
 });
 it('reutiliza el aviso verde y rojo de Ficha y permite cerrarlo', () => {
  const Componente = () => {
   const { aviso, notificar: emitir, cerrar } = useAvisosOT();
   return <><button onClick={() => emitir('Guardado confirmado', 'EXITO')}>Éxito</button><button onClick={() => emitir('Operación denegada', 'ERROR')}>Error</button><AvisoOTVisible aviso={aviso} onCerrar={cerrar} /></>;
  };
  render(<Componente />);
  fireEvent.click(screen.getByRole('button', { name: 'Éxito' }));
  const texto = screen.getByText('Guardado confirmado');
  expect(texto.parentElement).toHaveStyle({backgroundColor:'#064E3B'});
  fireEvent.click(texto.parentElement!.querySelector('button')!);
  expect(screen.queryByText('Guardado confirmado')).not.toBeInTheDocument();
  fireEvent.click(screen.getByRole('button', { name: 'Error' }));
  expect(screen.getByText('Operación denegada').parentElement).toHaveStyle({backgroundColor:'#7F1D1D'});
 });
 it('el aviso de éxito desaparece automáticamente a los 3,5 segundos', () => {
  vi.useFakeTimers();
  try {
   const Componente = () => {
    const { aviso, notificar: emitir, cerrar } = useAvisosOT();
    return <><button onClick={() => emitir('Guardado temporal', 'EXITO')}>Guardar</button><AvisoOTVisible aviso={aviso} onCerrar={cerrar} /></>;
   };
   render(<Componente />);
   fireEvent.click(screen.getByRole('button', { name: 'Guardar' }));
   act(() => vi.advanceTimersByTime(3499));
   expect(screen.getByText('Guardado temporal')).toBeInTheDocument();
   act(() => vi.advanceTimersByTime(1));
   expect(screen.queryByText('Guardado temporal')).not.toBeInTheDocument();
  } finally { vi.useRealTimers(); }
 });

});
