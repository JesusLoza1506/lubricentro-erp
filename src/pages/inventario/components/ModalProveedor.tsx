import React, { useState } from 'react';
import { CrearProveedorDTO } from '../../../types/inventario';
import { modalInputStyle } from '../inventarioStyles';

interface ModalProveedorProps {
  abierto: boolean;
  onClose: () => void;
  onGuardar: (proveedor: CrearProveedorDTO) => Promise<void>;
}

export const ModalProveedor: React.FC<ModalProveedorProps> = ({ abierto, onClose, onGuardar }) => {
  const [form, setForm] = useState<CrearProveedorDTO>({
    tipo_documento: 'RUC',
    numero_documento: '',
    nombre_razon_social: '',
    telefono: '',
    correo: '',
    direccion: '',
    contacto_asesor: '',
  });
  const [guardando, setGuardando] = useState(false);

  if (!abierto) return null;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setGuardando(true);
      await onGuardar({
        ...form,
        numero_documento: form.numero_documento.trim(),
        nombre_razon_social: form.nombre_razon_social.trim(),
      });
      onClose();
    } finally {
      setGuardando(false);
    }
  };

  return (
    <div
      style={{
        position: 'fixed',
        top: 0,
        left: 0,
        width: '100vw',
        height: '100vh',
        backgroundColor: 'rgba(0,0,0,0.8)',
        backdropFilter: 'blur(4px)',
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        zIndex: 1200,
      }}
    >
      <div
        style={{
          width: '460px',
          backgroundColor: '#0F172A',
          border: '1px solid #334155',
          borderRadius: '16px',
          padding: '24px',
          boxShadow: '0 20px 40px rgba(0,0,0,0.5)',
        }}
      >
        <h3 style={{ fontSize: '18px', fontWeight: '800', color: '#F8FAFC', margin: '0 0 16px 0' }}>
          Nuevo Proveedor
        </h3>

        <form
          onSubmit={handleSubmit}
          style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}
        >
          <div style={{ display: 'grid', gridTemplateColumns: '1fr 2fr', gap: '10px' }}>
            <div>
              <label
                style={{
                  display: 'block',
                  fontSize: '11px',
                  color: '#94A3B8',
                  marginBottom: '4px',
                }}
              >
                Tipo Doc.
              </label>
              <select
                value={form.tipo_documento}
                onChange={(e) => setForm({ ...form, tipo_documento: e.target.value })}
                style={{ ...modalInputStyle, backgroundColor: '#1E293B', color: '#F8FAFC' }}
              >
                <option value="RUC">RUC</option>
                <option value="DNI">DNI</option>
              </select>
            </div>
            <div>
              <label
                style={{
                  display: 'block',
                  fontSize: '11px',
                  color: '#94A3B8',
                  marginBottom: '4px',
                }}
              >
                N° Documento *
              </label>
              <input
                required
                type="text"
                placeholder="20100070970"
                value={form.numero_documento}
                onChange={(e) => setForm({ ...form, numero_documento: e.target.value })}
                style={modalInputStyle}
              />
            </div>
          </div>

          <div>
            <label
              style={{ display: 'block', fontSize: '11px', color: '#94A3B8', marginBottom: '4px' }}
            >
              Razón Social / Nombre *
            </label>
            <input
              required
              type="text"
              placeholder="Distribuidora de Lubricantes SAC"
              value={form.nombre_razon_social}
              onChange={(e) => setForm({ ...form, nombre_razon_social: e.target.value })}
              style={modalInputStyle}
            />
          </div>

          <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '10px' }}>
            <div>
              <label
                style={{
                  display: 'block',
                  fontSize: '11px',
                  color: '#94A3B8',
                  marginBottom: '4px',
                }}
              >
                Teléfono
              </label>
              <input
                type="text"
                value={form.telefono || ''}
                onChange={(e) => setForm({ ...form, telefono: e.target.value })}
                style={modalInputStyle}
              />
            </div>
            <div>
              <label
                style={{
                  display: 'block',
                  fontSize: '11px',
                  color: '#94A3B8',
                  marginBottom: '4px',
                }}
              >
                Asesor de Venta
              </label>
              <input
                type="text"
                value={form.contacto_asesor || ''}
                onChange={(e) => setForm({ ...form, contacto_asesor: e.target.value })}
                style={modalInputStyle}
              />
            </div>
          </div>

          <div
            style={{ display: 'flex', justifyContent: 'flex-end', gap: '10px', marginTop: '12px' }}
          >
            <button
              type="button"
              onClick={onClose}
              style={{
                padding: '8px 14px',
                backgroundColor: 'transparent',
                border: '1px solid #334155',
                color: '#CBD5E1',
                borderRadius: '8px',
                cursor: 'pointer',
              }}
            >
              Cancelar
            </button>
            <button
              type="submit"
              disabled={guardando}
              style={{
                padding: '8px 18px',
                backgroundColor: '#2563EB',
                color: '#FFFFFF',
                border: 'none',
                borderRadius: '8px',
                fontWeight: '700',
                cursor: 'pointer',
              }}
            >
              {guardando ? 'Guardando...' : 'Crear Proveedor'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};
