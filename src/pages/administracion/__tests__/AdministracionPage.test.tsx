import type { ReactNode } from 'react';
import { act, cleanup, fireEvent, render, screen, waitFor, within } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { invoke } from '@tauri-apps/api/core';
import { App } from '../../../App';
import { FILTROS_USUARIOS_INICIALES } from '../adminApi';
import { useAuth } from '../../../context/AuthContext';
import { authDePrueba, sesionDePrueba } from '../../__tests__/authFixture';
import { AdministracionPage, type PestanaAdmin } from '../AdministracionPage';

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));
vi.mock('../../../context/AuthContext', () => ({
  useAuth: vi.fn(),
  AuthProvider: ({ children }: { children: ReactNode }) => children,
}));
vi.mock('../../ordenes/OrdenesPage', () => ({ OrdenesPage: () => <h1>Taller abierto</h1> }));
vi.mock('../../caja/CajaPage', () => ({ CajaPage: () => <h1>Caja abierta</h1> }));
vi.mock('../../auth/LoginPage', () => ({ LoginPage: () => <h1>Iniciar sesión</h1> }));
const llamar = vi.mocked(invoke);
const auth = vi.mocked(useAuth);
function montar(pestana: PestanaAdmin) {
  return render(
    <AdministracionPage
      pestana={pestana}
      onPestana={vi.fn()}
      onNavegar={vi.fn()}
      filtrosUsuarios={FILTROS_USUARIOS_INICIALES}
      onFiltrosUsuarios={vi.fn()}
    />
  );
}
beforeEach(() => {
  auth.mockReturnValue(authDePrueba({ usuario: sesionDePrueba({ ambiente_actual: 'PRUEBA' }) }));
});
afterEach(() => {
  cleanup();
  llamar.mockReset();
  auth.mockReset();
  vi.restoreAllMocks();
});

it('no carga datos administrativos para un cajero', () => {
  auth.mockReturnValue(authDePrueba({ usuario: sesionDePrueba({ rol: 'CAJERO' }) }));
  montar('usuarios');
  expect(screen.queryByRole('heading', { name: 'Administración' })).toBeNull();
  expect(llamar).not.toHaveBeenCalled();
});

it('reutiliza la operación ante una respuesta incierta y oculta la temporal al cerrar la entrega', async () => {
  const lista = { usuarios: [], actor: 7, pagina: 1, paginas: 1, total: 0 };
  llamar
    .mockResolvedValueOnce(lista)
    .mockRejectedValueOnce('Respuesta interrumpida.')
    .mockResolvedValueOnce({ login: 'demo', temporal: 'Temporal0012', repetida: false })
    .mockResolvedValueOnce(lista);
  montar('usuarios');
  await screen.findByText('No se encontraron cuentas.');
  fireEvent.click(screen.getByRole('button', { name: 'Nueva cuenta' }));
  fireEvent.change(screen.getByLabelText('Login'), { target: { value: 'demo' } });
  fireEvent.change(screen.getByLabelText('Nombre completo'), { target: { value: 'Cajero Demo' } });
  fireEvent.click(screen.getByRole('button', { name: 'Guardar' }));
  await screen.findByText('Respuesta interrumpida.');
  fireEvent.click(screen.getByRole('button', { name: 'Guardar' }));
  await screen.findByText('Temporal0012');
  expect(llamar.mock.calls[1]).toEqual(llamar.mock.calls[2]);
  fireEvent.click(screen.getByRole('button', { name: 'Cerrar entrega' }));
  expect(screen.queryByText('Temporal0012')).toBeNull();
});

it('identifica la serie que se desactivará y espera confirmación antes del reemplazo', async () => {
  const datos = {
    ruta: null,
    version: 0,
    token_configurado: false,
    configuracion_completa: false,
    series: [
      {
        id: 1,
        tipo: 'BOLETA',
        codigo: 'B001',
        activa: true,
        pendiente: false,
        ultimo: 120,
        siguiente: 121,
      },
      {
        id: 2,
        tipo: 'BOLETA',
        codigo: 'B002',
        activa: false,
        pendiente: false,
        ultimo: 135,
        siguiente: 136,
      },
    ],
  };
  llamar.mockResolvedValueOnce(datos).mockResolvedValueOnce({}).mockResolvedValueOnce(datos);
  montar('facturacion');
  fireEvent.click(await screen.findByRole('button', { name: 'Activar' }));
  expect(screen.getByText(/La serie B001 quedará inactiva/)).toBeTruthy();
  expect(llamar).toHaveBeenCalledTimes(1);
  fireEvent.click(screen.getByRole('button', { name: 'Confirmar' }));
  await waitFor(() =>
    expect(llamar).toHaveBeenCalledWith('admin_operacion_cmd', {
      req: { accion: 'estado_serie', id: 2, activa: true, activa_esperada: 1 },
    })
  );
});

it('muestra la orden que ocupa una zanja antes de intentar reducir la capacidad', async () => {
  llamar.mockResolvedValueOnce({
    cantidad: 2,
    zanjas: [
      { id: 1, activa: true, ot: null },
      { id: 2, activa: true, ot: 'OT-003' },
    ],
  });
  montar('taller');
  const cantidad = await screen.findByLabelText('Cantidad de zanjas');
  fireEvent.change(cantidad, { target: { value: '1' } });
  expect(screen.getByRole('alert').textContent).toContain('OT-003');
  expect((screen.getByRole('button', { name: 'Guardar' }) as HTMLButtonElement).disabled).toBe(
    true
  );
  expect(llamar).toHaveBeenCalledTimes(1);
});

it('conserva el ambiente visible cuando el cambio confirmado es bloqueado por Caja', async () => {
  llamar
    .mockResolvedValueOnce({ usuarios: [], actor: 7, pagina: 1, paginas: 1, total: 0 })
    .mockRejectedValueOnce('Cierra el turno CAJA-01.');
  montar('usuarios');
  await screen.findByText('No se encontraron cuentas.');
  fireEvent.click(screen.getByRole('button', { name: 'Cambiar ambiente' }));
  expect(llamar).toHaveBeenCalledTimes(1);
  fireEvent.click(screen.getByRole('button', { name: 'Cambiar a Producción' }));
  await screen.findByText('Cierra el turno CAJA-01.');
  expect(screen.getByText('Ambiente: Prueba')).toBeTruthy();
});

const cuenta = {
  id: 12,
  login: 'cajero01',
  nombre: 'Cajero Demo',
  rol: 'CAJERO',
  activo: true,
  bloqueado: false,
  debe_cambiar_clave: false,
  version: 3,
};
const usuarios = { usuarios: [cuenta], actor: 7, pagina: 1, paginas: 1, total: 1 };

it('abre Editar inmediatamente y permite guardar el nombre con un turno abierto', async () => {
  llamar
    .mockResolvedValueOnce(usuarios)
    .mockResolvedValueOnce({ usuario: cuenta, pendientes: { caja: 'CAJA-01', ordenes: [] } })
    .mockResolvedValueOnce({})
    .mockResolvedValueOnce({
      ...usuarios,
      usuarios: [{ ...cuenta, nombre: 'Cajero Actualizado' }],
    });
  montar('usuarios');
  const editar = await screen.findByRole('button', { name: 'Editar' });
  editar.focus();
  fireEvent.click(editar);
  const dialogo = screen.getByRole('dialog', { name: 'Editar cuenta' });
  expect(within(dialogo).getByRole('status').textContent).toContain('Cargando');
  expect(document.activeElement).toBe(within(dialogo).getByRole('heading'));
  const nombre = await within(dialogo).findByLabelText('Nombre completo');
  expect(within(dialogo).getByRole('button', { name: 'Ver en Caja' })).toBeTruthy();
  const rol = within(dialogo).getByLabelText('Rol de la cuenta');
  const guardar = within(dialogo).getByRole('button', { name: 'Guardar' }) as HTMLButtonElement;
  fireEvent.change(rol, { target: { value: 'MECANICO' } });
  expect(guardar.disabled).toBe(true);
  fireEvent.change(rol, { target: { value: 'CAJERO' } });
  fireEvent.change(nombre, { target: { value: 'Cajero Actualizado' } });
  expect(guardar.disabled).toBe(false);
  fireEvent.click(guardar);
  await screen.findByText('Cuenta actualizada.');
  expect(screen.queryByRole('dialog')).toBeNull();
  expect(await screen.findByText('Cajero Actualizado')).toBeTruthy();
  expect(llamar).toHaveBeenCalledWith('admin_operacion_cmd', {
    req: {
      accion: 'editar_usuario',
      datos: { id: 12, version: 3, nombre: 'Cajero Actualizado', rol: 'CAJERO', activo: true, motivo: null },
    },
  });
});

it('muestra el fallo de detalle dentro del diálogo, permite reintentar y descarta una respuesta tardía al cancelar', async () => {
  let terminar!: (detalle: unknown) => void;
  llamar
    .mockResolvedValueOnce(usuarios)
    .mockRejectedValueOnce('No se pudo consultar la cuenta.')
    .mockImplementationOnce(() => new Promise((resolve) => { terminar = resolve; }));
  montar('usuarios');
  const editar = await screen.findByRole('button', { name: 'Editar' });
  editar.focus();
  fireEvent.click(editar);
  const dialogo = screen.getByRole('dialog', { name: 'Editar cuenta' });
  expect((await within(dialogo).findByRole('alert')).textContent).toContain('No se pudo consultar');
  fireEvent.click(within(dialogo).getByRole('button', { name: 'Reintentar' }));
  expect(within(dialogo).getByRole('status').textContent).toContain('Cargando');
  fireEvent.click(within(dialogo).getByRole('button', { name: 'Cancelar' }));
  await act(async () => terminar({ usuario: cuenta, pendientes: { caja: null, ordenes: [] } }));
  expect(screen.queryByRole('dialog')).toBeNull();
  expect(document.activeElement).toBe(editar);
  expect(llamar).toHaveBeenCalledTimes(3);
});

it('X, Escape y Cancelar protegen los cambios y cierran sin preguntar si se revirtieron', async () => {
  const confirmar = vi.spyOn(window, 'confirm').mockReturnValue(false);
  llamar
    .mockResolvedValueOnce(usuarios)
    .mockResolvedValueOnce({ usuario: cuenta, pendientes: { caja: null, ordenes: [] } });
  montar('usuarios');
  fireEvent.click(await screen.findByRole('button', { name: 'Editar' }));
  const dialogo = screen.getByRole('dialog', { name: 'Editar cuenta' });
  const nombre = (await within(dialogo).findByLabelText('Nombre completo')) as HTMLInputElement;
  fireEvent.change(nombre, { target: { value: 'Captura nueva' } });

  fireEvent.click(within(dialogo).getByRole('button', { name: 'Cerrar diálogo' }));
  expect(confirmar).toHaveBeenCalledTimes(1);
  expect(nombre.value).toBe('Captura nueva');
  fireEvent(dialogo, new Event('cancel', { cancelable: true }));
  expect(confirmar).toHaveBeenCalledTimes(2);
  expect(nombre.value).toBe('Captura nueva');
  fireEvent.click(within(dialogo).getByRole('button', { name: 'Cancelar' }));
  expect(confirmar).toHaveBeenCalledTimes(3);
  expect(nombre.value).toBe('Captura nueva');

  fireEvent.change(nombre, { target: { value: cuenta.nombre } });
  fireEvent.click(within(dialogo).getByRole('button', { name: 'Cerrar diálogo' }));
  expect(confirmar).toHaveBeenCalledTimes(3);
  expect(screen.queryByRole('dialog')).toBeNull();
  expect(llamar).toHaveBeenCalledTimes(2);
});

it('confirma el descarte en Facturación y cierra solo cuando el usuario acepta', async () => {
  const confirmar = vi.spyOn(window, 'confirm').mockReturnValueOnce(false).mockReturnValueOnce(true);
  llamar.mockResolvedValue({
    ruta: null, version: 0, token_configurado: false, configuracion_completa: false, series: [],
  });
  montar('facturacion');
  fireEvent.click(await screen.findByRole('button', { name: 'Nueva serie' }));
  const dialogo = screen.getByRole('dialog', { name: 'Nueva serie' });
  const codigo = within(dialogo).getByLabelText('Código de serie') as HTMLInputElement;
  fireEvent.change(codigo, { target: { value: 'B099' } });
  fireEvent.click(within(dialogo).getByRole('button', { name: 'Cancelar' }));
  expect(codigo.value).toBe('B099');
  fireEvent.click(within(dialogo).getByRole('button', { name: 'Cancelar' }));
  expect(screen.queryByRole('dialog')).toBeNull();
  expect(confirmar).toHaveBeenCalledTimes(2);
  expect(llamar).toHaveBeenCalledTimes(1);
});

it('recarga Usuarios con búsqueda, filtros y página al volver de Taller y Caja, y limpia el contexto al cambiar de sesión', async () => {
  const permisos = (modulo: string) => ['ADMIN', 'OT', 'CAJA'].includes(modulo);
  auth.mockReturnValue(authDePrueba({ tienePermiso: permisos }));
  let dependencia = 'OT-003';
  llamar.mockImplementation(async (cmd, args) => {
    if (cmd === 'backup_arranque_cmd') return { bloqueado: false, ambiente: 'PRUEBA', previa: false };
    if (cmd === 'backup_contexto_cmd') return { ambiente: 'PRUEBA', pendiente: false };
    if (cmd === 'obtener_dashboard_cmd') return {};
    const req = (args as { req?: { accion: string; pagina?: number } } | undefined)?.req;
    if (req?.accion === 'usuarios') {
      return { ...usuarios, pagina: req.pagina, paginas: 2, total: 21 };
    }
    if (req?.accion === 'detalle_usuario') {
      return { usuario: cuenta, pendientes: { caja: 'CAJA-01', ordenes: [dependencia] } };
    }
    throw new Error(`Consulta no prevista: ${cmd}`);
  });

  const vista = render(<App />);
  const entrar = () => fireEvent.click(within(screen.getByRole('navigation', { name: 'Módulos' }))
    .getByRole('button', { name: 'Administración' }));
  await screen.findByRole('navigation', { name: 'Módulos' });
  entrar();
  await screen.findByText(cuenta.nombre);
  fireEvent.change(screen.getByLabelText('Buscar por login o nombre'), { target: { value: 'cajero' } });
  fireEvent.click(screen.getByRole('button', { name: 'Buscar' }));
  fireEvent.change(screen.getByLabelText('Rol'), { target: { value: 'CAJERO' } });
  fireEvent.change(screen.getByLabelText('Estado'), { target: { value: 'ACTIVO' } });
  await waitFor(() => expect((screen.getByRole('button', { name: 'Siguiente' }) as HTMLButtonElement).disabled).toBe(false));
  fireEvent.click(screen.getByRole('button', { name: 'Siguiente' }));
  await screen.findByText(/Página 2 de 2/);
  const consulta = { req: { accion: 'usuarios', busqueda: 'cajero', rol: 'CAJERO', estado: 'ACTIVO', pagina: 2 } };
  fireEvent.click(screen.getByRole('button', { name: 'Editar' }));
  fireEvent.click(await screen.findByRole('button', { name: 'Ver en Taller' }));
  await screen.findByRole('heading', { name: 'Taller abierto' });

  const antes = llamar.mock.calls.length;
  dependencia = 'OT-008';
  entrar();
  await screen.findByText(/Página 2 de 2/);
  expect(llamar.mock.calls.slice(antes)).toContainEqual(['admin_operacion_cmd', consulta]);
  expect((screen.getByLabelText('Buscar por login o nombre') as HTMLInputElement).value).toBe('cajero');
  expect((screen.getByLabelText('Rol') as HTMLSelectElement).value).toBe('CAJERO');
  expect((screen.getByLabelText('Estado') as HTMLSelectElement).value).toBe('ACTIVO');
  fireEvent.click(screen.getByRole('button', { name: 'Editar' }));
  await screen.findByText('OT-008');
  fireEvent.click(screen.getByRole('button', { name: 'Ver en Caja' }));
  await screen.findByRole('heading', { name: 'Caja abierta' });

  const antesDeCaja = llamar.mock.calls.length;
  entrar();
  await screen.findByText(/Página 2 de 2/);
  expect(llamar.mock.calls.slice(antesDeCaja)).toContainEqual(['admin_operacion_cmd', consulta]);
  fireEvent.click(screen.getByRole('button', { name: 'Editar' }));
  await screen.findByText('CAJA-01');
  const nombre = screen.getByLabelText('Nombre completo');
  fireEvent.change(nombre, { target: { value: 'Edición al vencer sesión' } });

  const confirmar = vi.spyOn(window, 'confirm');
  auth.mockReturnValue(authDePrueba({ usuario: null, estadoSesion: 'SIN_SESION' }));
  vista.rerender(<App />);
  await screen.findByRole('heading', { name: 'Iniciar sesión' });
  expect(screen.queryByRole('dialog')).toBeNull();
  expect(confirmar).not.toHaveBeenCalled();
  auth.mockReturnValue(authDePrueba({ usuario: sesionDePrueba({ session_id: 'nueva-sesion' }), tienePermiso: permisos }));
  vista.rerender(<App />);
  await screen.findByRole('navigation', { name: 'Módulos' });
  entrar();
  await screen.findByText(/Página 1 de 2/);
  expect((screen.getByLabelText('Buscar por login o nombre') as HTMLInputElement).value).toBe('');
  expect((screen.getByLabelText('Rol') as HTMLSelectElement).value).toBe('');
  expect((screen.getByLabelText('Estado') as HTMLSelectElement).value).toBe('');
});

it('asocia el rechazo de ubigeo al campo sin perder la captura', async () => {
  llamar
    .mockResolvedValueOnce({
      ruc: '20600695771', razon_social: 'Empresa Demo', nombre_comercial: 'Demo Local',
      direccion: 'Avenida Principal 123', ubigeo: '110201', telefono: '', correo: '',
      version: 5, ruc_fijo: false,
    })
    .mockRejectedValueOnce('VALIDATION_ERROR: Ubigeo: el código no corresponde a un distrito del Perú.');
  montar('empresa');
  const campo = (await screen.findByLabelText('Ubigeo')) as HTMLInputElement;
  fireEvent.change(campo, { target: { value: '119999' } });
  fireEvent.click(screen.getByRole('button', { name: 'Guardar' }));
  const error = await screen.findByRole('alert');
  expect(error.textContent).toContain('no corresponde a un distrito');
  expect(campo.getAttribute('aria-describedby')).toBe(error.id);
  expect(campo.getAttribute('aria-invalid')).toBe('true');
  expect(campo.value).toBe('119999');
  fireEvent.change(campo, { target: { value: '010101' } });
  expect(screen.queryByRole('alert')).toBeNull();
});
