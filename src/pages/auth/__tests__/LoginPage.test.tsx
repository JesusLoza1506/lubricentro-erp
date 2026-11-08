import '@testing-library/jest-dom/vitest';
import { act, cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { invoke } from '@tauri-apps/api/core';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { useAuth } from '../../../context/AuthContext';
import { authDePrueba } from '../../__tests__/authFixture';
import { LoginPage } from '../LoginPage';

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));
vi.mock('../../../context/AuthContext', () => ({ useAuth: vi.fn() }));

const llamada = vi.mocked(invoke);
let auth: ReturnType<typeof authDePrueba>;
const codigoPrueba = 'CODIGO-LOCAL-DE-PRUEBA';

function escribir(etiqueta: string, valor: string) {
  fireEvent.change(screen.getByLabelText(etiqueta, { exact: true }), {
    target: { value: valor },
  });
}

async function abrirLogin() {
  await act(async () => {
    render(<LoginPage />);
  });
}

beforeEach(() => {
  auth = authDePrueba({ usuario: null, estadoSesion: 'SIN_SESION' });
  vi.mocked(useAuth).mockReturnValue(auth);
  llamada.mockReset();
  llamada.mockImplementation(async (comando) => {
    if (comando === 'requiere_instalacion') return false;
    throw new Error(`Comando inesperado: ${comando}`);
  });
});

afterEach(cleanup);

describe('Pantallas de acceso', () => {
  it('permite revelar la clave, la limpia tras un rechazo y muestra el error de sesión', async () => {
    vi.mocked(auth.iniciarSesion).mockRejectedValueOnce(new Error('AUTH_ERROR: Clave incorrecta'));
    await abrirLogin();
    const usuario = screen.getByLabelText('Usuario', { exact: true });
    const clave = screen.getByLabelText('Contraseña', { exact: true });
    expect(usuario).toHaveFocus();
    escribir('Usuario', '  ana  ');
    escribir('Contraseña', 'ClaveLocal1');
    expect(clave).toHaveAttribute('type', 'password');
    fireEvent.click(screen.getByRole('button', { name: 'Mostrar contraseña' }));
    expect(clave).toHaveAttribute('type', 'text');
    fireEvent.click(screen.getByRole('button', { name: 'Ocultar contraseña' }));
    expect(clave).toHaveAttribute('type', 'password');

    fireEvent.click(screen.getByRole('button', { name: 'Ingresar' }));
    expect(await screen.findByRole('alert')).toHaveTextContent('Clave incorrecta');
    expect(auth.iniciarSesion).toHaveBeenCalledExactlyOnceWith('ana', 'ClaveLocal1');
    expect(clave).toHaveValue('');
    expect(usuario).toHaveValue('  ana  ');
  });

  it('mantiene bloqueada la salida de instalación hasta confirmar que se guardó el código', async () => {
    llamada.mockImplementation(async (comando) => {
      if (comando === 'requiere_instalacion') return true;
      if (comando === 'instalacion_inicial')
        return { username: 'admin', codigo_recuperacion: codigoPrueba };
      throw new Error(`Comando inesperado: ${comando}`);
    });
    await abrirLogin();
    expect(screen.getByRole('heading', { name: 'Crear administrador' })).toHaveFocus();
    escribir('Nombre completo', '  Ana Torres  ');
    escribir('Usuario', '  admin  ');
    escribir('Contraseña', 'ClaveLocal1');
    escribir('Confirmar contraseña', 'ClaveLocal1');
    fireEvent.click(screen.getByRole('button', { name: 'Crear administrador' }));

    expect(await screen.findByRole('heading', { name: 'Guarda tu código' })).toBeInTheDocument();
    expect(llamada).toHaveBeenCalledWith('instalacion_inicial', {
      req: { nombre_completo: 'Ana Torres', username: 'admin', password: 'ClaveLocal1' },
    });
    const continuar = screen.getByRole('button', { name: 'Ir a iniciar sesión' });
    expect(continuar).toBeDisabled();
    expect(screen.getByText(codigoPrueba)).toBeInTheDocument();
    fireEvent.click(screen.getByRole('checkbox', { name: /he guardado el código/i }));
    fireEvent.click(continuar);
    expect(screen.getByRole('heading', { name: 'Iniciar sesión' })).toBeInTheDocument();
    expect(screen.queryByText(codigoPrueba)).not.toBeInTheDocument();
    expect(screen.getByLabelText('Contraseña', { exact: true })).toHaveValue('');
  });

  it('valida la confirmación en recuperación y borra los campos al salir y regresar', async () => {
    await abrirLogin();
    const irARecuperacion = () =>
      fireEvent.click(screen.getByRole('button', { name: '¿No puedes ingresar?' }));
    irARecuperacion();
    expect(screen.getByRole('heading', { name: 'Recuperar acceso' })).toHaveFocus();
    escribir('Usuario administrador', 'admin');
    escribir('Código de recuperación', codigoPrueba);
    escribir('Nueva contraseña', 'ClaveLocal1');
    escribir('Confirmar nueva contraseña', 'ClaveLocal2');

    fireEvent.click(screen.getByRole('button', { name: 'Restablecer acceso' }));
    expect(screen.getByRole('alert')).toHaveTextContent(/no coincide/i);
    expect(llamada.mock.calls.map(([comando]) => comando)).toEqual(['requiere_instalacion']);

    fireEvent.click(screen.getByRole('button', { name: 'Volver al inicio de sesión' }));
    irARecuperacion();
    expect(screen.queryByRole('alert')).not.toBeInTheDocument();
    for (const etiqueta of [
      'Usuario administrador',
      'Código de recuperación',
      'Nueva contraseña',
      'Confirmar nueva contraseña',
    ]) {
      expect(screen.getByLabelText(etiqueta, { exact: true })).toHaveValue('');
    }
  });

  it('muestra el nuevo código de recuperación y lo retira al volver al acceso', async () => {
    const codigoNuevo = 'NUEVO-CODIGO-DE-PRUEBA';
    llamada.mockImplementation(async (comando) => {
      if (comando === 'requiere_instalacion') return false;
      if (comando === 'recuperar_acceso_admin')
        return { username: 'admin', codigo_recuperacion: codigoNuevo };
      throw new Error(`Comando inesperado: ${comando}`);
    });
    await abrirLogin();
    fireEvent.click(screen.getByRole('button', { name: '¿No puedes ingresar?' }));
    escribir('Usuario administrador', ' admin ');
    escribir('Código de recuperación', ` ${codigoPrueba} `);
    escribir('Nueva contraseña', 'ClaveLocal1');
    escribir('Confirmar nueva contraseña', 'ClaveLocal1');
    fireEvent.click(screen.getByRole('button', { name: 'Restablecer acceso' }));

    expect(await screen.findByRole('heading', { name: 'Acceso restablecido' })).toBeInTheDocument();
    expect(llamada).toHaveBeenCalledWith('recuperar_acceso_admin', {
      req: { username: 'admin', codigo_recuperacion: codigoPrueba, nueva_password: 'ClaveLocal1' },
    });
    expect(screen.getByText(codigoNuevo)).toBeInTheDocument();
    expect(screen.queryByLabelText('Nueva contraseña', { exact: true })).not.toBeInTheDocument();
    fireEvent.click(screen.getByRole('button', { name: 'Ir a iniciar sesión' }));
    expect(screen.queryByText(codigoNuevo)).not.toBeInTheDocument();
    expect(screen.getByLabelText('Contraseña', { exact: true })).toHaveValue('');
  });

  it('solo habilita la sesión tras confirmar el cambio obligatorio de clave', async () => {
    auth = authDePrueba({ estadoSesion: 'CAMBIO_OBLIGATORIO' });
    vi.mocked(useAuth).mockReturnValue(auth);
    let confirmarCambio!: () => void;
    llamada.mockImplementation(async (comando) => {
      if (comando === 'cambiar_clave_obligatoria')
        return new Promise<void>((resolve) => {
          confirmarCambio = resolve;
        });
      throw new Error(`Comando inesperado: ${comando}`);
    });
    await abrirLogin();
    expect(screen.queryByRole('button', { name: 'Ingresar' })).not.toBeInTheDocument();
    escribir('Nueva contraseña', 'ClaveLocal1');
    escribir('Confirmar nueva contraseña', 'ClaveLocal1');
    fireEvent.click(screen.getByRole('button', { name: 'Guardar contraseña y continuar' }));

    expect(llamada).toHaveBeenCalledExactlyOnceWith('cambiar_clave_obligatoria', {
      req: { nueva_password: 'ClaveLocal1' },
    });
    expect(screen.getByRole('button', { name: 'Guardando…' })).toBeDisabled();
    expect(auth.completarCambioObligatorio).not.toHaveBeenCalled();
    confirmarCambio();
    await waitFor(() => expect(auth.completarCambioObligatorio).toHaveBeenCalledTimes(1));
    expect(screen.getByLabelText('Nueva contraseña', { exact: true })).toHaveValue('');
  });
});
