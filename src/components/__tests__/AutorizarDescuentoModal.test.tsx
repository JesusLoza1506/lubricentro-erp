import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { AutorizarDescuentoModal } from '../AutorizarDescuentoModal';
import * as authService from '../../services/authService';

vi.mock('../../services/authService', () => ({
  apiAutorizarDescuento: vi.fn(),
}));

describe('AutorizarDescuentoModal Component (Vitest)', () => {
  const defaultProps = {
    referenciaVenta: 'VENTA-2026-001',
    descuentoBp: 500, // 5.00%
    montoDescuentoTexto: '5.00%',
    onAutorizado: vi.fn(),
    onClose: vi.fn(),
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders correctly with labels and details', () => {
    render(<AutorizarDescuentoModal {...defaultProps} />);

    expect(screen.getByRole('heading', { name: /Autorizar Descuento/i })).toBeDefined();
    expect(screen.getByText('VENTA-2026-001')).toBeDefined();
    expect(screen.getByText('5.00%')).toBeDefined();
    expect(screen.getByLabelText(/Usuario Administrador/i)).toBeDefined();
    expect(screen.getByLabelText(/Contraseña de Administrador/i)).toBeDefined();
  });

  it('validates empty inputs without submitting', async () => {
    render(<AutorizarDescuentoModal {...defaultProps} />);

    const btnAutorizar = screen.getByRole('button', { name: /Autorizar/i });
    fireEvent.click(btnAutorizar);

    const errorEl = await screen.findByText(/Ingresa el usuario y contraseña del Administrador/i);
    expect(errorEl).toBeDefined();
    expect(authService.apiAutorizarDescuento).not.toHaveBeenCalled();
  });

  it('submits data successfully and purges sensitive password state', async () => {
    const mockToken = 'desc_token_test_123';
    vi.spyOn(authService, 'apiAutorizarDescuento').mockResolvedValue(mockToken);

    render(<AutorizarDescuentoModal {...defaultProps} />);

    const inputUser = screen.getByLabelText(/Usuario Administrador/i);
    const inputPass = screen.getByLabelText(/Contraseña de Administrador/i) as HTMLInputElement;

    fireEvent.change(inputUser, { target: { value: 'admin' } });
    fireEvent.change(inputPass, { target: { value: 'AdminPassword123' } });

    const btnAutorizar = screen.getByRole('button', { name: /Autorizar/i });
    fireEvent.click(btnAutorizar);

    await waitFor(() => {
      expect(authService.apiAutorizarDescuento).toHaveBeenCalledWith({
        username_admin: 'admin',
        password_admin: 'AdminPassword123',
        referencia_venta: 'VENTA-2026-001',
        descuento_bp: 500,
      });
      expect(defaultProps.onAutorizado).toHaveBeenCalledWith(mockToken);
      expect(defaultProps.onClose).toHaveBeenCalled();
      expect(inputPass.value).toBe('');
    });
  });

  it('handles backend authentication errors and purges password field', async () => {
    vi.spyOn(authService, 'apiAutorizarDescuento').mockRejectedValue(
      new Error('Credenciales de autorizante inválidas.')
    );

    render(<AutorizarDescuentoModal {...defaultProps} />);

    const inputUser = screen.getByLabelText(/Usuario Administrador/i);
    const inputPass = screen.getByLabelText(/Contraseña de Administrador/i) as HTMLInputElement;

    fireEvent.change(inputUser, { target: { value: 'admin' } });
    fireEvent.change(inputPass, { target: { value: 'WrongPass' } });

    const btnAutorizar = screen.getByRole('button', { name: /Autorizar/i });
    fireEvent.click(btnAutorizar);

    const errorMsgEl = await screen.findByText(/Credenciales de autorizante inválidas/i);
    expect(errorMsgEl).toBeDefined();
    expect(inputPass.value).toBe('');
  });
});
