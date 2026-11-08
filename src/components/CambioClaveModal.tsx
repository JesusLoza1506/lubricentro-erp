import React, { useState, useEffect, useRef } from 'react';
import { useAuth } from '../context/AuthContext';
import { apiCambiarClaveVoluntaria, mensajeErrorAuth } from '../services/authService';
import { Check, X, KeyRound, Minus } from 'lucide-react';
import { AccessField, AccessNotice, AccessSubmit } from '../pages/auth/components/AccessUi';
import '../styles/gloriaTaller.css';
import { ConfirmModal } from './ConfirmModal';

interface Props {
  onClose: () => void;
}

const ItemCheck: React.FC<{ ok: boolean; text: string }> = ({ ok, text }) => (
  <li className="gt-check" data-ok={ok}>
    {ok ? <Check size={14} aria-hidden="true" /> : <Minus size={14} aria-hidden="true" />}
    <span>
      <span className="gt-sr-only">{ok ? 'Cumplido: ' : 'Pendiente: '}</span>
      {text}
    </span>
  </li>
);

export const CambioClaveModal: React.FC<Props> = ({ onClose }) => {
  const { usuario, setHayCambiosSinGuardar } = useAuth();
  const modalRef = useRef<HTMLDivElement>(null);
  const previousFocusRef = useRef<HTMLElement | null>(null);

  const [passwordActual, setPasswordActual] = useState('');
  const [nuevaPassword, setNuevaPassword] = useState('');
  const [confirmarPassword, setConfirmarPassword] = useState('');

  const [errorMsg, setErrorMsg] = useState<string | null>(null);
  const [procesando, setProcesando] = useState(false);
  const [mostrarConfirmacionCierre, setMostrarConfirmacionCierre] = useState(false);

  useEffect(() => {
    previousFocusRef.current = document.activeElement as HTMLElement;

    const timer = setTimeout(() => {
      const inputActual = document.getElementById('modal-password-actual');
      if (inputActual) inputActual.focus();
    }, 50);

    return () => {
      clearTimeout(timer);
      setHayCambiosSinGuardar(false);
      if (previousFocusRef.current) {
        previousFocusRef.current.focus();
      }
    };
  }, [setHayCambiosSinGuardar]);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (mostrarConfirmacionCierre) return;

      if (e.key === 'Escape') {
        e.preventDefault();
        handleIntentarCerrar();
        return;
      }

      if (e.key === 'Tab' && modalRef.current) {
        const focusables = modalRef.current.querySelectorAll<HTMLElement>(
          'button:not([disabled]), input:not([disabled]), [tabindex]:not([tabindex="-1"])'
        );
        if (focusables.length === 0) return;

        const firstElement = focusables[0];
        const lastElement = focusables[focusables.length - 1];

        if (e.shiftKey && document.activeElement === firstElement) {
          e.preventDefault();
          lastElement.focus();
        } else if (!e.shiftKey && document.activeElement === lastElement) {
          e.preventDefault();
          firstElement.focus();
        }
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [mostrarConfirmacionCierre, passwordActual, nuevaPassword, confirmarPassword]);

  const handleInputChange = (
    setter: React.Dispatch<React.SetStateAction<string>>,
    valor: string
  ) => {
    setter(valor);
    setHayCambiosSinGuardar(true);
  };

  // Conteo exacto Unicode (Punto 6 R7)
  const len = [...nuevaPassword].length;
  const lenOk = len >= 8 && len <= 128;
  const tieneLetra = /\p{L}/u.test(nuevaPassword);
  const tieneNumero = /\d/.test(nuevaPassword);
  const noEsUsername = usuario?.username
    ? nuevaPassword.toLowerCase() !== usuario.username.toLowerCase()
    : true;
  const noEsIgualActual = passwordActual.length > 0 && nuevaPassword !== passwordActual;
  const coinciden = nuevaPassword.length > 0 && nuevaPassword === confirmarPassword;

  const todoValido =
    lenOk && tieneLetra && tieneNumero && noEsUsername && noEsIgualActual && coinciden;

  const handleIntentarCerrar = () => {
    if (passwordActual || nuevaPassword || confirmarPassword) {
      setMostrarConfirmacionCierre(true);
    } else {
      setHayCambiosSinGuardar(false);
      onClose();
    }
  };

  const handleConfirmarDescarte = () => {
    setHayCambiosSinGuardar(false);
    setMostrarConfirmacionCierre(false);
    onClose();
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setErrorMsg(null);

    if (!passwordActual || !nuevaPassword || !confirmarPassword) {
      setErrorMsg('Todos los campos son obligatorios.');
      return;
    }

    if (!todoValido) {
      setErrorMsg('Por favor, cumple con todos los requisitos de seguridad.');
      return;
    }

    setProcesando(true);
    try {
      await apiCambiarClaveVoluntaria({
        password_actual: passwordActual,
        nueva_password: nuevaPassword,
      });

      setHayCambiosSinGuardar(false);
      onClose();
    } catch (err: any) {
      const mensaje = mensajeErrorAuth(err, 'Error al cambiar la contraseña.');
      setErrorMsg(mensaje);
    } finally {
      setProcesando(false);
    }
  };

  return (
    <>
      <div
        className="gloria-theme gt-overlay"
        role="dialog"
        aria-modal={mostrarConfirmacionCierre ? undefined : true}
        aria-labelledby="modal-cambio-clave-title"
      >
        <div className="gt-dialog" ref={modalRef}>
          <header className="gt-dialog-header">
            <KeyRound size={21} aria-hidden="true" />
            <div>
              <h3 id="modal-cambio-clave-title" className="gt-dialog-title">
                Cambiar contraseña
              </h3>
              <p className="gt-dialog-description">Actualiza la contraseña de tu cuenta.</p>
            </div>
            <button
              type="button"
              className="gt-button gt-button--icon"
              onClick={handleIntentarCerrar}
              disabled={procesando}
              aria-label="Cerrar modal"
            >
              <X size={18} aria-hidden="true" />
            </button>
          </header>

          {errorMsg && <AccessNotice tone="error">{errorMsg}</AccessNotice>}

          <form className="gt-dialog-form" onSubmit={handleSubmit}>
            <AccessField
              id="modal-password-actual"
              label="Contraseña actual"
              type="password"
              value={passwordActual}
              onChange={(e) => handleInputChange(setPasswordActual, e.target.value)}
              disabled={procesando}
              autoComplete="current-password"
              placeholder="••••••••"
            />
            <AccessField
              id="modal-nueva-password"
              label="Nueva contraseña"
              type="password"
              value={nuevaPassword}
              onChange={(e) => handleInputChange(setNuevaPassword, e.target.value)}
              disabled={procesando}
              autoComplete="new-password"
              placeholder="••••••••"
              maxLength={256}
            />
            <AccessField
              id="modal-confirm-password"
              label="Confirmar contraseña"
              type="password"
              value={confirmarPassword}
              onChange={(e) => handleInputChange(setConfirmarPassword, e.target.value)}
              disabled={procesando}
              autoComplete="new-password"
              placeholder="••••••••"
              maxLength={256}
            />

            <div className="gt-requirements">
              <p>Requisitos de la nueva contraseña</p>
              <ul className="gt-checks">
                <ItemCheck ok={lenOk} text="8 a 128 caracteres" />
                <ItemCheck ok={tieneLetra} text="Al menos una letra" />
                <ItemCheck ok={tieneNumero} text="Al menos un número" />
                <ItemCheck
                  ok={nuevaPassword.length > 0 && noEsUsername}
                  text="Distinta del usuario"
                />
                <ItemCheck ok={noEsIgualActual} text="Distinta a la actual" />
                <ItemCheck ok={coinciden} text="Las contraseñas coinciden" />
              </ul>
            </div>

            <div className="gt-actions">
              <button
                type="button"
                className="gt-button"
                disabled={procesando}
                onClick={handleIntentarCerrar}
              >
                Cancelar
              </button>
              <AccessSubmit
                busy={procesando}
                busyLabel="Guardando…"
                disabled={!todoValido || procesando}
              >
                Actualizar contraseña
              </AccessSubmit>
            </div>
          </form>
        </div>
      </div>

      <ConfirmModal
        abierto={mostrarConfirmacionCierre}
        varianteVisual="gloria"
        titulo="¿Descartar el cambio de contraseña?"
        mensaje="Si cierras esta ventana, se perderá lo que escribiste."
        textoConfirmar="Descartar cambios"
        textoCancelar="Seguir editando"
        onConfirmar={handleConfirmarDescarte}
        onCancelar={() => setMostrarConfirmacionCierre(false)}
      />
    </>
  );
};
