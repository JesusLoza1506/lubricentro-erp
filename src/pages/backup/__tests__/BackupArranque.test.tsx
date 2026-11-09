import { cleanup, render, screen } from '@testing-library/react';
import { invoke } from '@tauri-apps/api/core';
import { afterEach, expect, it, vi } from 'vitest';
import { App } from '../../../App';

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));
afterEach(cleanup);

it('bloquea el login y el inicio comercial si el conjunto activo no puede abrirse', async () => {
  vi.mocked(invoke).mockResolvedValue({
    bloqueado: true, previa: true, ambiente: 'PRUEBA',
    motivo: 'Falta la base de la restauración activa.',
  });
  render(<App />);
  await screen.findByRole('heading', { name: 'No se pudo abrir la base activa' });
  expect(screen.queryByRole('heading', { name: /iniciar sesión|inicio/i })).toBeNull();
  expect(screen.getByRole('heading', { name: 'Recuperar desde la copia previa registrada' })).toBeTruthy();
  expect((screen.getByRole('button', { name: 'Verificar y recuperar' }) as HTMLButtonElement).disabled).toBe(true);
  expect(invoke).toHaveBeenCalledExactlyOnceWith('backup_arranque_cmd');
});
