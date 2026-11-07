import { afterEach, describe, expect, it, vi } from 'vitest';
import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import type { PagoPos, CredencialesPos } from '../../../types/pos';
import { CobroPos } from '../components/CobroPos';
afterEach(cleanup);
const totales = {
  bruto_cent: 8000,
  descuento_cent: 0,
  base_cent: 6780,
  igv_cent: 1220,
  total_cent: 8000,
  lineas: [],
};
describe('cobro guiado', () => {
  it('propone el total y bloquea la confirmación mientras guarda', async () => {
    let resolver: () => void = () => {};
    const confirmar = vi.fn(
      (_p: PagoPos[], _c?: CredencialesPos) =>
        new Promise<void>((r) => {
          resolver = r;
        })
    );
    render(
      <CobroPos
        totales={totales}
        requiereAdmin={false}
        onCerrar={() => {}}
        onConfirmar={confirmar}
      />
    );
    fireEvent.click(screen.getByRole('button', { name: 'Confirmar cobro' }));
    fireEvent.click(screen.getByRole('button', { name: 'Registrando cobro…' }));
    expect(confirmar).toHaveBeenCalledTimes(1);
    expect(confirmar.mock.calls[0][0]).toEqual([
      { medio: 'EFECTIVO', monto_cent: 8000, recibido_cent: 8000, referencia: null },
    ]);
    resolver();
    await waitFor(() =>
      expect(screen.getByRole('button', { name: 'Confirmar cobro' })).toBeTruthy()
    );
  });
  it('conserva captura y muestra el error al rechazar el guardado', async () => {
    render(
      <CobroPos
        totales={totales}
        requiereAdmin={false}
        onCerrar={() => {}}
        onConfirmar={async () => {
          throw 'VALIDATION_ERROR: Caja cerrada';
        }}
      />
    );
    fireEvent.change(screen.getByLabelText('Efectivo recibido (S/)'), { target: { value: '100' } });
    fireEvent.click(screen.getByRole('button', { name: 'Confirmar cobro' }));
    await screen.findByText('Caja cerrada');
    expect((screen.getByLabelText('Efectivo recibido (S/)') as HTMLInputElement).value).toBe('100');
  });
});
