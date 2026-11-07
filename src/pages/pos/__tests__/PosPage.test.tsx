import '@testing-library/jest-dom/vitest';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { cleanup, fireEvent, render, screen, waitFor, within } from '@testing-library/react';
import { invoke } from '@tauri-apps/api/core';
import { PosPage } from '../PosPage';
import { DetalleVentaPos } from '../components/DetalleVentaPos';
vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));
vi.mock('@tauri-apps/plugin-opener', () => ({ openUrl: vi.fn() }));
const dirty = vi.fn();
let rol: 'CAJERO' | 'ADMINISTRADOR' = 'CAJERO';
vi.mock('../../../context/AuthContext', () => ({
  useAuth: () => ({
    usuario: { id_usuario: 2, nombre_completo: 'Cajero de prueba', rol },
    ambienteActual: 'PRUEBA',
    setHayCambiosSinGuardar: dirty,
  }),
}));
const producto = {
  tipo: 'PRODUCTO',
  id: 1,
  codigo: 'FIL-01',
  nombre: 'Filtro de aceite',
  unidad: 'UND',
  precio_cent: 2000,
  stock: 10,
};
const detalle = {
  id: 1,
  id_ot: null,
  id_emisor: 2,
  emisor: 'Cajero de prueba',
  caja: 'CAJA-1',
  fecha: '2026-11-06 12:00:00',
  tipo: 'BOLETA',
  receptor: { nombre: 'Cliente varios' },
  bruto_cent: 2000,
  descuento_cent: 0,
  base_cent: 1695,
  igv_cent: 305,
  total_cent: 2000,
  devuelto_cent: 0,
  lineas: [],
  pagos: [],
  documentos: [],
  devoluciones: [],
};
let incierto = false;
beforeEach(() => {
  localStorage.clear();
  rol = 'CAJERO';
  vi.clearAllMocks();
  incierto = false;
  vi.mocked(invoke).mockImplementation(async (comando, args) => {
    const a = args as { accion?: string; payload?: any };
    if (comando === 'pos_documento_cmd') throw 'VALIDATION_ERROR: Sin conexión de prueba';
    if (a.accion === 'inicio')
      return {
        ambiente: 'PRUEBA',
        empresa: 'Lubricentro',
        configurada: true,
        caja_propia: 1,
        caja: { id: 1, codigo: 'CAJA-1', titular: 'Cajero', propia: 1, efectivo_cent: 10000 },
        hoy: '2026-11-06',
        series: [],
      };
    if (a.accion === 'catalogo') return { filas: [producto], total: 1 };
    if (a.accion === 'cotizar')
      return {
        ...detalle,
        lineas: [
          {
            ...producto,
            id_producto: 1,
            id_servicio: null,
            cantidad: 1,
            total_cent: 2000,
            base_cent: 1695,
          },
        ],
      };
    if (a.accion === 'confirmar') {
      if (incierto) {
        incierto = false;
        throw 'DATABASE_ERROR: Respuesta incierta';
      }
      return { id_venta: 1, id_documento: 1, total_cent: 2000 };
    }
    if (a.accion === 'detalle') return detalle;
    if (a.accion === 'listado') return { filas: [], total: 0 };
    throw new Error(`Comando inesperado: ${comando} ${a.accion}`);
  });
});
afterEach(cleanup);
async function prepararCarrito() {
  render(<PosPage />);
  fireEvent.click(await screen.findByRole('button', { name: 'Agregar Filtro de aceite' }));
  await waitFor(() =>
    expect(
      (screen.getByRole('button', { name: 'Cobrar venta' }) as HTMLButtonElement).disabled
    ).toBe(false)
  );
  const venta = within(screen.getByRole('complementary', { name: /Venta actual/ }));
  const resumen = within(venta.getByText('Total a cobrar').parentElement!);
  expect(resumen.getByText('S/ 20.00')).toBeInTheDocument();
  fireEvent.click(venta.getByRole('button', { name: 'Cobrar venta' }));
}
it('integra catálogo, confirmación y aviso de venta guardada aunque falle NubeFact', async () => {
  await prepararCarrito();
  fireEvent.click(screen.getByRole('button', { name: 'Confirmar cobro' }));
  await screen.findByRole('dialog', { name: 'Venta V-1' });
  expect(screen.getByText(/Venta guardada; documento pendiente/)).toBeTruthy();
  const ventas = vi.mocked(invoke).mock.calls.filter((c) => (c[1] as any)?.accion === 'confirmar');
  expect(ventas).toHaveLength(1);
  expect(ventas[0][1]).toMatchObject({
    payload: {
      id_cliente: 1,
      tipo_comprobante: 'BOLETA',
      lineas: [{ tipo: 'PRODUCTO', id: 1, cantidad: 1, precio_esperado_cent: 2000 }],
      pagos: [{ medio: 'EFECTIVO', monto_cent: 2000, recibido_cent: 2000 }],
    },
  });
});
it('reintenta desde el cobro con el mismo ID después de una respuesta incierta', async () => {
  await prepararCarrito();
  incierto = true;
  fireEvent.click(screen.getByRole('button', { name: 'Confirmar cobro' }));
  await screen.findByText('Respuesta incierta');
  fireEvent.click(screen.getByRole('button', { name: 'Confirmar cobro' }));
  await screen.findByRole('dialog', { name: 'Venta V-1' });
  const llamadas = vi
    .mocked(invoke)
    .mock.calls.filter((c) => (c[1] as any)?.accion === 'confirmar');
  expect(llamadas).toHaveLength(2);
  expect((llamadas[0][1] as any).payload.id_operacion).toBe(
    (llamadas[1][1] as any).payload.id_operacion
  );
});

it('permite retirar la captura cuando falla la cotización inicial', async () => {
  const original = vi.mocked(invoke).getMockImplementation()!;
  vi.mocked(invoke).mockImplementation(async (c, a) => {
    if ((a as any)?.accion === 'cotizar') throw 'VALIDATION_ERROR: Stock insuficiente';
    return original(c, a);
  });
  render(<PosPage />);
  fireEvent.click(await screen.findByRole('button', { name: 'Agregar Filtro de aceite' }));
  await screen.findByText('Stock insuficiente');
  fireEvent.click(screen.getByRole('button', { name: 'Quitar Filtro de aceite' }));
  await waitFor(() =>
    expect(screen.queryByRole('button', { name: 'Quitar Filtro de aceite' })).toBeNull()
  );
});

it.each([
  { configurada: false, cajaPropia: true, motivo: 'Cobro pendiente de configuración.' },
  {
    configurada: true,
    cajaPropia: false,
    motivo: 'Necesitas un turno propio abierto para cobrar.',
  },
])(
  'permite cotizar sin habilitar el cobro: $motivo',
  async ({ configurada, cajaPropia, motivo }) => {
    const original = vi.mocked(invoke).getMockImplementation()!;
    vi.mocked(invoke).mockImplementation(async (comando, args) => {
      if ((args as { accion?: string })?.accion === 'inicio') {
        const datos = (await original(comando, args)) as Record<string, unknown>;
        return {
          ...datos,
          configurada,
          caja_propia: Number(cajaPropia),
          caja: cajaPropia ? datos.caja : null,
        };
      }
      return original(comando, args);
    });
    render(<PosPage />);
    fireEvent.click(await screen.findByRole('button', { name: 'Agregar Filtro de aceite' }));
    const venta = within(screen.getByRole('complementary', { name: /Venta actual/ }));
    expect(await venta.findByText(motivo)).toBeInTheDocument();
    const resumen = within(venta.getByText('Total a cobrar').parentElement!);
    expect(await resumen.findByText('S/ 20.00')).toBeInTheDocument();
    expect(venta.getByRole('button', { name: 'Cobrar venta' })).toBeDisabled();
    expect(venta.getByRole('button', { name: 'Cobrar venta' })).toHaveAccessibleDescription(motivo);
    const acciones = vi
      .mocked(invoke)
      .mock.calls.map(([, args]) => (args as { accion?: string })?.accion);
    expect(acciones).toContain('cotizar');
    expect(acciones).not.toContain('confirmar');
  }
);

it('muestra una observación junto al comprobante aceptado y reimprime sin otra venta', async () => {
  const original = vi.mocked(invoke).getMockImplementation()!;
  vi.mocked(invoke).mockImplementation(async (cmd, args) => {
    if ((args as { accion?: string })?.accion === 'detalle') {
      return {
        ...detalle,
        documentos: [
          {
            id: 1,
            tipo: 'BOLETA',
            serie: 'B001',
            numero: 8,
            estado: 'ACEPTADO',
            relacion: 'ORIGINAL',
            mensaje: null,
            advertencias: ['Observación del emisor'],
          },
        ],
      };
    }
    return original(cmd, args);
  });
  const print = vi.spyOn(window, 'print').mockImplementation(() => {});
  try {
    await prepararCarrito();
    fireEvent.click(screen.getByRole('button', { name: 'Confirmar cobro' }));
    const dialogo = within(await screen.findByRole('dialog', { name: 'Venta V-1' }));
    const notas = await dialogo.findByRole('region', { name: 'Observaciones del comprobante' });
    expect(notas).toHaveTextContent('Observación del emisor');
    expect(dialogo.getByText('ACEPTADO')).toBeInTheDocument();
    expect(dialogo.queryByText(/pendiente de envío/i)).not.toBeInTheDocument();
    const antes = vi.mocked(invoke).mock.calls.length;
    fireEvent.click(dialogo.getByRole('button', { name: 'Reimprimir ticket' }));
    expect(print).toHaveBeenCalledOnce();
    expect(vi.mocked(invoke).mock.calls).toHaveLength(antes);
  } finally {
    print.mockRestore();
  }
});

const orden = {
  id: 42,
  codigo: 'OT-0042',
  placa: 'ABC123',
  cliente: 'Cliente de taller',
  fecha: '2026-11-06 11:30:00',
  total_cent: 5500,
};

it.each(['selector', 'taller'] as const)(
  'conserva la identidad de la OT al cargar desde %s',
  async (entrada) => {
    const original = vi.mocked(invoke).getMockImplementation()!;
    vi.mocked(invoke).mockImplementation(async (cmd, args) => {
      const a = args as { accion?: string; payload?: { id_ot?: number } };
      if (a.accion === 'ots') return { filas: [orden], total: 1 };
      if (a.accion === 'cotizar' && a.payload?.id_ot === orden.id) {
        return { ...detalle, ot: orden, total_cent: orden.total_cent, lineas: [] };
      }
      return original(cmd, args);
    });
    render(<PosPage otInicial={entrada === 'taller' ? orden.id : null} />);
    if (entrada === 'selector') {
      fireEvent.click(await screen.findByRole('button', { name: 'Cargar OT finalizada' }));
      const selector = within(screen.getByRole('dialog', { name: 'Cobrar una orden finalizada' }));
      const fila = within(await selector.findByRole('row', { name: /OT-0042/ }));
      expect(fila.getByText(orden.placa)).toBeInTheDocument();
      expect(fila.getByText(orden.cliente)).toBeInTheDocument();
      expect(fila.getByText(orden.fecha)).toBeInTheDocument();
      expect(fila.getByText('S/ 55.00')).toBeInTheDocument();
      fireEvent.click(fila.getByRole('button', { name: 'Seleccionar' }));
    }
    const carrito = within(screen.getByRole('complementary', { name: /Venta actual/ }));
    expect(await carrito.findByText('OT-0042 · ABC123')).toBeInTheDocument();
  }
);

it('conserva la cantidad rechazada de un servicio, bloquea el cobro y permite corregirla', async () => {
  const original = vi.mocked(invoke).getMockImplementation()!;
  const servicio = {
    ...producto,
    tipo: 'SERVICIO',
    id: 9,
    codigo: 'SERV-9',
    nombre: 'Revisión de luces',
    stock: null,
  };
  vi.mocked(invoke).mockImplementation(async (cmd, args) => {
    const a = args as { accion?: string; payload?: { lineas?: { cantidad: number }[] } };
    if (a.accion === 'catalogo') return { filas: [servicio], total: 1 };
    if (a.accion === 'cotizar') {
      const cantidad = a.payload?.lineas?.[0]?.cantidad ?? 0;
      if (cantidad > 99_999)
        throw 'VALIDATION_ERROR: Ingresa de 1 a 99 999 unidades por línea directa.';
      return {
        ...detalle,
        lineas: [{ ...servicio, id_servicio: 9, cantidad, total_cent: cantidad * 2000 }],
      };
    }
    return original(cmd, args);
  });
  render(<PosPage />);
  fireEvent.click(await screen.findByRole('button', { name: 'Agregar Revisión de luces' }));
  const cobrar = await screen.findByRole('button', { name: 'Cobrar venta' });
  expect(cobrar).toBeEnabled();
  const cambiar = (value: string) => {
    const input = screen.getByRole('textbox', { name: 'Cantidad Revisión de luces' });
    fireEvent.change(input, { target: { value } });
    fireEvent.blur(input);
  };
  cambiar('100000');
  await screen.findByText('Ingresa de 1 a 99 999 unidades por línea directa.');
  expect(screen.getByRole('textbox', { name: 'Cantidad Revisión de luces' })).toHaveValue('100000');
  expect(cobrar).toBeDisabled();
  cambiar('99999');
  await waitFor(() => expect(cobrar).toBeEnabled());
  expect(
    vi.mocked(invoke).mock.calls.some(([, a]) => (a as { accion?: string })?.accion === 'confirmar')
  ).toBe(false);
});

const baseDoc = {
  id: 1,
  tipo: 'BOLETA',
  serie: 'B001',
  numero: 8,
  relacion: 'ORIGINAL',
  estado: 'ACEPTADO',
  mensaje: null,
};

function abrirDetalle(admin: boolean) {
  render(
    <DetalleVentaPos
      id={1}
      usuario={2}
      ambiente="PRUEBA"
      admin={admin}
      cajaPropia
      empresa="Lubricentro"
      onCerrar={vi.fn()}
      onOperacion={vi.fn()}
      onAviso={vi.fn()}
    />
  );
  return within(screen.getByRole('dialog', { name: 'Venta V-1' }));
}

it.each([false, true])(
  'devuelve cantidades pendientes con autorización y recuperación física (todo: %s)',
  async (todo) => {
    const original = vi.mocked(invoke).getMockImplementation()!;
    const lineas = [
      {
        id_detalle: 11,
        id_producto: 2,
        codigo: 'FIL-2',
        nombre: 'Filtro',
        unidad: 'UND',
        cantidad: 3,
        devuelta: 1,
        total_cent: 1800,
      },
      {
        id_detalle: 12,
        id_servicio: 1,
        codigo: 'SER-1',
        nombre: 'Revisión',
        unidad: 'UND',
        cantidad: 1,
        devuelta: 0,
        total_cent: 200,
      },
    ];
    let actualizada = false;
    vi.mocked(invoke).mockImplementation(async (cmd, args) => {
      const a = args as { accion?: string };
      if (a.accion === 'detalle')
        return {
          ...detalle,
          lineas,
          documentos: [baseDoc],
          devuelto_cent: actualizada ? (todo ? 2000 : 1200) : 600,
        };
      if (a.accion === 'cotizar_devolucion') return { total_cent: todo ? 1400 : 600 };
      if (a.accion === 'devolver') {
        actualizada = true;
        return { id_venta: 1, id_documento: 2 };
      }
      return original(cmd, args);
    });
    const venta = abrirDetalle(false);
    fireEvent.click(await venta.findByRole('button', { name: 'Devolver' }));
    const modal = within(screen.getByRole('dialog', { name: 'Devolver · venta V-1' }));
    if (todo) {
      fireEvent.click(modal.getByRole('button', { name: 'Seleccionar todo lo pendiente' }));
      expect(modal.getByLabelText('Devolver Filtro')).toHaveValue('2');
      expect(modal.getByLabelText('Devolver Revisión')).toHaveValue('1');
    } else {
      fireEvent.change(modal.getByLabelText('Devolver Filtro'), { target: { value: '1' } });
    }
    expect(modal.getByLabelText('Recuperar Revisión')).toBeDisabled();
    fireEvent.change(modal.getByLabelText('Recuperar Filtro'), { target: { value: '1' } });
    fireEvent.change(modal.getByLabelText('Motivo'), {
      target: { value: 'Solicitud de devolución' },
    });
    fireEvent.change(modal.getByLabelText('Usuario administrador'), {
      target: { value: 'admin-test' },
    });
    fireEvent.change(modal.getByLabelText('Contraseña'), { target: { value: 'clave-ficticia' } });
    const confirmar = modal.getByRole('button', { name: 'Confirmar devolución' });
    await waitFor(() => expect(confirmar).toBeEnabled());
    fireEvent.click(confirmar);
    await waitFor(() => expect(screen.queryByRole('dialog', { name: /Devolver ·/ })).toBeNull());
    const llamadas = vi
      .mocked(invoke)
      .mock.calls.filter(([, a]) => (a as { accion?: string })?.accion === 'devolver');
    expect(llamadas).toHaveLength(1);
    expect(llamadas[0][1]).toMatchObject({
      payload: {
        id_venta: 1,
        lineas: [
          { id_detalle: 11, cantidad: todo ? 2 : 1, recuperada: 1 },
          ...(todo ? [{ id_detalle: 12, cantidad: 1, recuperada: 0 }] : []),
        ],
      },
      credenciales: { username: 'admin-test', password: 'clave-ficticia' },
    });
  }
);

it.each(['BOLETA', 'NC_BOLETA'])(
  'reemite %s rechazado y conserva el anterior vinculado',
  async (tipo) => {
    const original = vi.mocked(invoke).getMockImplementation()!;
    const esNota = tipo === 'NC_BOLETA';
    const doc = {
      ...baseDoc,
      tipo,
      serie: esNota ? 'BC01' : 'B001',
      estado: 'RECHAZADO',
      relacion: esNota ? 'NOTA_CREDITO' : 'ORIGINAL',
    };
    let reemitida = false;
    vi.mocked(invoke).mockImplementation(async (cmd, args) => {
      const a = args as { accion?: string };
      if (a.accion === 'detalle')
        return {
          ...detalle,
          documentos: reemitida
            ? [
                { ...doc, reemplazado: 1 },
                {
                  ...doc,
                  id: 2,
                  numero: 9,
                  estado: 'ACEPTADO',
                  relacion: esNota ? 'REEMISION_NC' : 'REEMISION',
                },
              ]
            : [doc],
        };
      if (a.accion === 'reemitir') {
        reemitida = true;
        return { id_venta: 1, id_documento: 2 };
      }
      if (cmd === 'pos_documento_cmd') return { estado: 'ACEPTADO' };
      return original(cmd, args);
    });
    const venta = abrirDetalle(true);
    fireEvent.click(await venta.findByRole('button', { name: 'Corregir y reemitir' }));
    const modal = within(screen.getByRole('dialog', { name: 'Corregir receptor y reemitir' }));
    const receptor = modal.getByLabelText('Nombre o razón social');
    if (esNota) expect(receptor).toHaveAttribute('readonly');
    else fireEvent.change(receptor, { target: { value: 'Cliente corregido' } });
    fireEvent.change(modal.getByLabelText('Motivo'), {
      target: { value: 'Corrección de datos del documento' },
    });
    fireEvent.click(modal.getByRole('button', { name: 'Confirmar reemisión' }));
    expect(await venta.findByText(/Reemplazado/)).toBeInTheDocument();
    expect(venta.getByText('ACEPTADO')).toBeInTheDocument();
    const llamadas = vi
      .mocked(invoke)
      .mock.calls.map(([, a]) => a as { accion?: string; payload?: unknown });
    expect(llamadas.filter((a) => a.accion === 'reemitir')).toMatchObject([
      {
        payload: {
          id_documento: 1,
          receptor: { nombre: esNota ? 'Cliente varios' : 'Cliente corregido' },
        },
      },
    ]);
    expect(llamadas.some((a) => a.accion === 'confirmar' || a.accion === 'devolver')).toBe(false);
  }
);

it('la revisión repite solo la solicitud habilitada por ausencia y mantiene el mismo documento', async () => {
  rol = 'ADMINISTRADOR';
  const original = vi.mocked(invoke).getMockImplementation()!;
  const secuencia: number[] = [];
  vi.mocked(invoke).mockImplementation(async (cmd, args) => {
    const a = args as { accion?: string; idDocumento?: number };
    if (a.accion === 'pendientes') return [{ id: 8 }, { id: 9 }];
    if (cmd === 'pos_documento_cmd') {
      const id = a.idDocumento!;
      const repetido = secuencia.includes(id);
      secuencia.push(id);
      return { estado: id === 9 ? 'ERROR' : repetido ? 'ACEPTADO' : 'PENDIENTE' };
    }
    return original(cmd, args);
  });
  render(<PosPage />);
  fireEvent.click(await screen.findByRole('button', { name: 'Ventas y comprobantes' }));
  const revisar = await screen.findByRole('button', { name: /Revisar pendientes/ });
  await waitFor(() => expect(revisar).toBeEnabled());
  fireEvent.click(revisar);
  await screen.findByText(/2 documentos atendidos; 1 con respuesta definitiva/);
  expect(secuencia).toEqual([8, 8, 9]);
});
