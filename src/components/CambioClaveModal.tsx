import React, { useState, useEffect } from 'react';
import { useAuth } from '../context/AuthContext';
import { apiCambiarClaveVoluntaria } from '../services/authService';
import { ShieldAlert, Check, X, KeyRound } from 'lucide-react';
import { ConfirmModal } from './ConfirmModal';

interface Props {
  onClose: () => void;
}

const ItemCheck: React.FC<{ ok: boolean; text: string }> = ({ ok, text }) => (
  <div
    style={{
      display: 'flex',
      alignItems: 'center',
      gap: '6px',
      color: ok ? '#34D399' : '#64748B',
    }}
  >
    {ok ? <Check size={14} /> : <X size={14} />}
    <span>{text}</span>
  </div>
);

export const CambioClaveModal: React.FC<Props> = ({ onClose }) => {
  const { usuario, setHayCambiosSinGuardar } = useAuth();

  const [passwordActual, setPasswordActual] = useState('');
  const [nuevaPassword, setNuevaPassword] = useState('');
  const [confirmarPassword, setConfirmarPassword] = useState('');

  const [errorMsg, setErrorMsg] = useState<string | null>(null);
  const [procesando, setProcesando] = useState(false);
  const [mostrarConfirmacionCierre, setMostrarConfirmacionCierre] = useState(false);

  useEffect(() => {
    return () => {
      setHayCambiosSinGuardar(false);
    };
  }, [setHayCambiosSinGuardar]);

  const handleInputChange = (
    setter: React.Dispatch<React.SetStateAction<string>>,
    valor: string
  ) => {
    setter(valor);
    setHayCambiosSinGuardar(true);
  };

  const len = [...nuevaPassword].length;
  const lenOk = len >= 7 && len <= 128;
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
      const mensaje =
        typeof err === 'string' ? err : err?.message || 'Error al cambiar la contraseña.';
      setErrorMsg(mensaje);
    } finally {
      setProcesando(false);
    }
  };

  return (
    <>
      <div
        style={{
          position: 'fixed',
          top: 0,
          left: 0,
          width: '100vw',
          height: '100vh',
          backgroundColor: 'rgba(15, 23, 42, 0.85)',
          backdropFilter: 'blur(8px)',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          zIndex: 9999,
        }}
        role="dialog"
        aria-modal="true"
        aria-labelledby="modal-cambio-clave-title"
      >
        <div
          style={{
            width: '100%',
            maxWidth: '450px',
            backgroundColor: '#1E293B',
            border: '1px solid #334155',
            borderRadius: '20px',
            padding: '32px',
            boxShadow: '0 25px 50px -12px rgba(0, 0, 0, 0.7)',
            color: '#F8FAFC',
            fontFamily: 'Inter, system-ui, sans-serif',
          }}
        >
          <div
            style={{
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'space-between',
              marginBottom: '20px',
            }}
          >
            <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
              <div
                style={{
                  backgroundColor: 'rgba(37, 99, 235, 0.2)',
                  padding: '10px',
                  borderRadius: '12px',
                  color: '#38BDF8',
                }}
              >
                <KeyRound size={24} />
              </div>
              <div>
                <h3
                  id="modal-cambio-clave-title"
                  style={{ margin: 0, fontSize: '18px', fontWeight: '700' }}
                >
                  Cambiar Contraseña
                </h3>
                <p style={{ margin: 0, fontSize: '12px', color: '#94A3B8' }}>
                  Actualiza tu contraseña de acceso voluntariamente.
                </p>
              </div>
            </div>

            <button
              onClick={handleIntentarCerrar}
              disabled={procesando}
              aria-label="Cerrar modal"
              style={{
                background: 'none',
                border: 'none',
                color: '#64748B',
                cursor: 'pointer',
                borderRadius: '6px',
                outline: '2px solid transparent',
                outlineOffset: '2px',
              }}
            >
              <X size={20} />
            </button>
          </div>

          {errorMsg && (
            <div
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: '10px',
                padding: '12px',
                backgroundColor: 'rgba(239, 68, 68, 0.15)',
                border: '1px solid rgba(239, 68, 68, 0.4)',
                color: '#FCA5A5',
                borderRadius: '10px',
                fontSize: '13px',
                marginBottom: '16px',
              }}
            >
              <ShieldAlert size={18} style={{ flexShrink: 0 }} />
              <span>{errorMsg}</span>
            </div>
          )}

          <form onSubmit={handleSubmit}>
            <div style={{ marginBottom: '16px' }}>
              <label
                htmlFor="modal-password-actual"
                style={{
                  display: 'block',
                  fontSize: '12px',
                  fontWeight: '600',
                  color: '#94A3B8',
                  marginBottom: '6px',
                  cursor: 'pointer',
                }}
              >
                Contraseña Actual
              </label>
              <input
                id="modal-password-actual"
                type="password"
                disabled={procesando}
                value={passwordActual}
                onChange={(e) => handleInputChange(setPasswordActual, e.target.value)}
                placeholder="••••••••"
                style={{
                  width: '100%',
                  padding: '12px 14px',
                  backgroundColor: '#0F172A',
                  border: '1px solid #334155',
                  borderRadius: '10px',
                  color: '#F8FAFC',
                  fontSize: '14px',
                  outline: '2px solid transparent',
                  outlineOffset: '2px',
                  boxSizing: 'border-box',
                }}
                autoComplete="current-password"
              />
            </div>

            <div style={{ marginBottom: '16px' }}>
              <label
                htmlFor="modal-nueva-password"
                style={{
                  display: 'block',
                  fontSize: '12px',
                  fontWeight: '600',
                  color: '#94A3B8',
                  marginBottom: '6px',
                  cursor: 'pointer',
                }}
              >
                Nueva Contraseña
              </label>
              <input
                id="modal-nueva-password"
                type="password"
                disabled={procesando}
                value={nuevaPassword}
                onChange={(e) => handleInputChange(setNuevaPassword, e.target.value)}
                placeholder="••••••••"
                maxLength={128}
                style={{
                  width: '100%',
                  padding: '12px 14px',
                  backgroundColor: '#0F172A',
                  border: '1px solid #334155',
                  borderRadius: '10px',
                  color: '#F8FAFC',
                  fontSize: '14px',
                  outline: '2px solid transparent',
                  outlineOffset: '2px',
                  boxSizing: 'border-box',
                }}
                autoComplete="new-password"
              />
            </div>

            <div style={{ marginBottom: '20px' }}>
              <label
                htmlFor="modal-confirm-password"
                style={{
                  display: 'block',
                  fontSize: '12px',
                  fontWeight: '600',
                  color: '#94A3B8',
                  marginBottom: '6px',
                  cursor: 'pointer',
                }}
              >
                Confirmar Nueva Contraseña
              </label>
              <input
                id="modal-confirm-password"
                type="password"
                disabled={procesando}
                value={confirmarPassword}
                onChange={(e) => handleInputChange(setConfirmarPassword, e.target.value)}
                placeholder="••••••••"
                maxLength={128}
                style={{
                  width: '100%',
                  padding: '12px 14px',
                  backgroundColor: '#0F172A',
                  border: '1px solid #334155',
                  borderRadius: '10px',
                  color: '#F8FAFC',
                  fontSize: '14px',
                  outline: '2px solid transparent',
                  outlineOffset: '2px',
                  boxSizing: 'border-box',
                }}
                autoComplete="new-password"
              />
            </div>

            <div
              style={{
                backgroundColor: '#0F172A',
                padding: '14px',
                borderRadius: '10px',
                marginBottom: '20px',
                fontSize: '12px',
              }}
            >
              <div
                style={{
                  color: '#94A3B8',
                  fontWeight: '600',
                  marginBottom: '8px',
                }}
              >
                Requisitos de Seguridad V7:
              </div>
              <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '8px' }}>
                <ItemCheck ok={lenOk} text="7-128 caracteres" />
                <ItemCheck ok={tieneLetra} text="Al menos 1 letra" />
                <ItemCheck ok={tieneNumero} text="Al menos 1 número" />
                <ItemCheck ok={noEsUsername} text="Distinta de usuario" />
                <ItemCheck ok={noEsIgualActual} text="Distinta a la actual" />
                <ItemCheck ok={coinciden} text="Las claves coinciden" />
              </div>
            </div>

            <button
              type="submit"
              disabled={!todoValido || procesando}
              style={{
                width: '100%',
                padding: '14px',
                backgroundColor: todoValido ? '#2563EB' : '#334155',
                color: todoValido ? '#FFFFFF' : '#64748B',
                border: 'none',
                borderRadius: '10px',
                fontSize: '14px',
                fontWeight: '600',
                cursor: todoValido && !procesando ? 'pointer' : 'not-allowed',
                outline: '2px solid transparent',
                outlineOffset: '2px',
              }}
            >
              {procesando ? 'Guardando...' : 'Actualizar Contraseña'}
            </button>

            <button
              type="button"
              disabled={procesando}
              onClick={handleIntentarCerrar}
              style={{
                width: '100%',
                padding: '12px',
                backgroundColor: 'transparent',
                color: '#94A3B8',
                border: '1px solid #334155',
                borderRadius: '10px',
                fontSize: '13px',
                fontWeight: '500',
                cursor: 'pointer',
                marginTop: '10px',
                outline: '2px solid transparent',
                outlineOffset: '2px',
              }}
            >
              Cancelar
            </button>
          </form>
        </div>
      </div>

      <ConfirmModal
        abierto={mostrarConfirmacionCierre}
        titulo="Descartar Cambios"
        mensaje="Tiene datos ingresados en el formulario de cambio de contraseña. Si cierra la ventana, los perderá."
        textoConfirmar="Sí, Descartar"
        onConfirmar={handleConfirmarDescarte}
        onCancelar={() => setMostrarConfirmacionCierre(false)}
      />
    </>
  );
};
