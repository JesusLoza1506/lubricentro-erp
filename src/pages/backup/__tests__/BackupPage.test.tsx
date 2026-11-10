import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, expect, it, vi } from 'vitest';
import { invoke } from '@tauri-apps/api/core';
import { BackupPage } from '../BackupPage';
import { sincronizarCapturasBackup } from '../backupService';

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));
const llamar = vi.mocked(invoke);
function operaciones(accion: string) {
  return llamar.mock.calls.filter(([cmd, args]) =>
    cmd === 'backup_operacion_cmd' &&
    (args as { req?: { accion?: string } } | undefined)?.req?.accion === accion);
}
afterEach(() => {
  cleanup();
  llamar.mockReset();
  localStorage.clear();
});

it('permite corregir la frase y no restaura al comprobar o cancelar la confirmación', async () => {
  const vista = {
    nombre: 'prueba.lgbackup', tamano: 1024, seleccion: 'sel-1', compatible: true,
    caja_abierta: false, manifiesto: {
      identidad: { ambiente: 'PRUEBA', ruc: null }, fecha: '2026-11-08 12:00:00',
      generado: '2026-11-08 12:00:00', formato: 1, esquema: 700, app: '0.1.0',
      tipo: 'MANUAL', autor: 'Prueba', archivos: [{ nombre: 'base.sqlite' }],
    },
  };
  llamar.mockImplementation(async (cmd, args) => {
    if (cmd === 'backup_elegir_cmd') return 'C:/Respaldos/prueba.lgbackup';
    if (cmd === 'backup_progreso_cmd') return { fase: 0, no_cerrar: false };
    const req = (args as { req?: { accion: string; frase?: string } } | undefined)?.req;
    if (cmd === 'backup_operacion_cmd' && req?.accion === 'comprobar') {
      if (req.frase === 'incorrecta') throw 'VALIDATION: La frase es incorrecta.';
      return vista;
    }
    throw new Error(`Comando inesperado: ${cmd}/${req?.accion}`);
  });
  render(<BackupPage inicial />);
  fireEvent.click(screen.getByRole('button', { name: /abrir.*copia/i }));
  const frase = await screen.findByLabelText('Frase original de la copia');
  fireEvent.change(frase, { target: { value: 'incorrecta' } });
  fireEvent.click(screen.getByRole('button', { name: 'Comprobar copia' }));
  await screen.findByText('La frase es incorrecta.');
  fireEvent.change(screen.getByLabelText('Frase original de la copia'), { target: { value: 'otra frase de prueba' } });
  fireEvent.click(screen.getByRole('button', { name: 'Comprobar copia' }));
  await screen.findByRole('heading', { name: 'Copia verificada' });
  expect(operaciones('comprobar')).toHaveLength(2);
  expect(operaciones('restaurar')).toHaveLength(0);
  fireEvent.click(screen.getByRole('button', { name: 'Restaurar copia' }));
  expect(await screen.findByRole('dialog')).toBeTruthy();
  fireEvent.click(screen.getByRole('button', { name: 'Cancelar' }));
  await waitFor(() => expect(screen.queryByRole('dialog')).toBeNull());
  expect(operaciones('comprobar')).toHaveLength(2);
  expect(operaciones('restaurar')).toHaveLength(0);
});

it('elimina solo capturas del ambiente restaurado y conserva las de la nueva generación', () => {
  const pos = 'pos.operacion.v1.PRUEBA.1';
  const caja = 'gloria:caja:pendiente:PRUEBA:1';
  const inventario = 'inventario.solicitud.v1.PRUEBA.1.ajustar';
  const produccion = 'pos.operacion.v1.PRODUCCION.1';
  for (const k of [pos, caja, inventario, produccion, 'preferencia']) localStorage.setItem(k, 'previa');
  sincronizarCapturasBackup({ ambiente: 'PRUEBA', generacion: 'nueva' });
  for (const k of [pos, caja, inventario]) expect(localStorage.getItem(k)).toBeNull();
  expect(localStorage.getItem(produccion)).toBe('previa');
  expect(localStorage.getItem('preferencia')).toBe('previa');
  localStorage.setItem(pos, 'nueva captura');
  sincronizarCapturasBackup({ ambiente: 'PRUEBA', generacion: 'nueva' });
  expect(localStorage.getItem(pos)).toBe('nueva captura');
});

it('recupera el panel y elimina el aviso anterior al actualizar con éxito', async () => {
  llamar.mockRejectedValueOnce('Hay una tarea de Backup en curso. Espere a que termine.')
    .mockResolvedValueOnce({
      configurada: true, ultima: null, vencida: true, proxima: '2026-11-08 23:00',
      diaria: null, copias: [], total: 0,
      recuperacion: { pendiente: false, caja: false, token: false, series: [] },
    });
  render(<BackupPage />);
  expect((await screen.findByRole('alert')).textContent).toContain('Hay una tarea de Backup en curso.');
  expect((screen.getByRole('button', { name: 'Generar ahora' }) as HTMLButtonElement).disabled).toBe(true);
  fireEvent.click(screen.getByRole('button', { name: 'Actualizar' }));
  await screen.findByText('Sin copia válida');
  expect(screen.queryByRole('alert')).toBeNull();
  expect((screen.getByRole('button', { name: 'Generar ahora' }) as HTMLButtonElement).disabled).toBe(false);
});

const estadoVacio = {
  configurada: true, ultima: null, vencida: true, proxima: '2026-11-08 23:00',
  diaria: null, copias: [], total: 0,
  recuperacion: { pendiente: false, caja: false, token: false, series: [] },
};

it('conserva filtro y página al regresar y revalida la selección antes de restaurar', async () => {
  llamar.mockImplementation(async (_cmd, args) => {
    const req = (args as { req: { accion: string } }).req;
    return req.accion === 'revalidar' ? { vigente: true, caja_abierta: false, compatible: true } : estadoVacio;
  });
  render(<BackupPage contextoCaja={{ tipo: 'PREVIA', pagina: 2, seleccion: {
    seleccion: 'token-caja', nombre: 'previa.lgbackup', tamano: 1024, caja_abierta: true, compatible: true,
    manifiesto: { identidad: { ambiente: 'PRUEBA', ruc: null }, fecha: '2026-11-08 12:00:00',
      generado: '2026-11-08 12:00:00', formato: 1, esquema: 700, app: '0.1.0',
      tipo: 'PREVIA', autor: 'Prueba', archivos: [{ nombre: 'base.sqlite' }],
    },
  } }} />);
  await screen.findByRole('heading', { name: 'Copia verificada' });
  expect((screen.getByLabelText('Tipo de copia') as HTMLSelectElement).value).toBe('PREVIA');
  expect(llamar).toHaveBeenCalledWith('backup_operacion_cmd', { req: { accion: 'estado', tipo: 'PREVIA', pagina: 2 } });
  expect(llamar).toHaveBeenCalledWith('backup_operacion_cmd', { req: { accion: 'revalidar', seleccion: 'token-caja' } });
  expect((screen.getByRole('button', { name: 'Restaurar copia' }) as HTMLButtonElement).disabled).toBe(false);
});

it('cancelar protección borra la frase sin invocar el guardado', async () => {
  llamar.mockResolvedValue(estadoVacio);
  render(<BackupPage />);
  await screen.findByText('Sin copia válida');
  fireEvent.click(screen.getByRole('button', { name: 'Configurar protección' }));
  fireEvent.change(screen.getByLabelText('Frase nueva'), { target: { value: 'secreto temporal' } });
  fireEvent.change(screen.getByLabelText('Repetir frase'), { target: { value: 'secreto temporal' } });
  fireEvent.click(screen.getByRole('button', { name: 'Cancelar' }));
  expect(operaciones('configurar')).toHaveLength(0);
  fireEvent.click(screen.getByRole('button', { name: 'Configurar protección' }));
  expect((screen.getByLabelText('Frase nueva') as HTMLInputElement).value).toBe('');
  expect((screen.getByLabelText('Repetir frase') as HTMLInputElement).value).toBe('');
});
