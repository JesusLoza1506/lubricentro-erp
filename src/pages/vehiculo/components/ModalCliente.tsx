import React, { useState, useEffect } from 'react';
import { ClienteDTO, CrearClienteDTO } from '../../../types/cliente';

export type ModoModalCliente = 'CREAR' | 'EDITAR';

interface ModalClienteProps {
  modalAbierto: boolean;
  setModalAbierto: (abierto: boolean) => void;
  modo: ModoModalCliente;
  clienteEditar?: ClienteDTO | null;
  onClienteGuardado: (cliente: any) => void;
}

export const ModalCliente: React.FC<ModalClienteProps> = ({
  modalAbierto,
  setModalAbierto,
  modo,
  clienteEditar,
  onClienteGuardado,
}) => {
  const [nombre, setNombre] = useState('');
  const [tipoDoc, setTipoDoc] = useState<'DNI' | 'RUC' | 'CE'>('DNI');
  const [numDoc, setNumDoc] = useState('');
  const [telefono, setTelefono] = useState('');
  const [correo, setCorreo] = useState('');
  const [direccion, setDireccion] = useState('');
  const [motivo, setMotivo] = useState('');
  const [cargando, setCargando] = useState(false);
  const [errorModal, setErrorModal] = useState<string | null>(null);

  useEffect(() => {
    setErrorModal(null);
    setMotivo('');
    if (modo === 'EDITAR' && clienteEditar) {
      setNombre(clienteEditar.nombre_razon_social || '');
      setTipoDoc((clienteEditar.tipo_documento as 'DNI' | 'RUC' | 'CE') || 'DNI');
      setNumDoc(clienteEditar.numero_documento || '');
      setTelefono(clienteEditar.telefono || '');
      setCorreo(clienteEditar.correo || '');
      setDireccion(clienteEditar.direccion || '');
    } else {
      setNombre('');
      setTipoDoc('DNI');
      setNumDoc('');
      setTelefono('');
      setCorreo('');
      setDireccion('');
    }
  }, [modo, clienteEditar, modalAbierto]);

  if (!modalAbierto) return null;

  const handleNumDocChange = (val: string) => {
    if (tipoDoc === 'DNI') setNumDoc(val.replace(/\D/g, '').slice(0, 8));
    else if (tipoDoc === 'RUC') setNumDoc(val.replace(/\D/g, '').slice(0, 11));
    else setNumDoc(val.replace(/[^a-zA-Z0-9]/g, '').slice(0, 12));
  };

  const handleTelefonoChange = (val: string) => {
    let clean = val.trim().replace(/[\s-]/g, '');
    if (clean.startsWith('+51')) {
      clean = clean.slice(3);
    } else if (clean.startsWith('51') && clean.length > 9) {
      clean = clean.slice(2);
    }
    setTelefono(clean.replace(/\D/g, '').slice(0, 9));
  };

  const onSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setCargando(true);
    setErrorModal(null);

    try {
      if (modo === 'CREAR') {
        const payload: CrearClienteDTO = {
          tipo_documento: tipoDoc,
          numero_documento: numDoc.trim(),
          nombre_razon_social: nombre.trim(),
          telefono: telefono.trim() || undefined,
          correo: correo.trim() || undefined,
          direccion: direccion.trim() || undefined,
        };
        await onClienteGuardado(payload);
      } else {
        if (!clienteEditar) return;
        const payload = {
          id_cliente: clienteEditar.id_cliente,
          tipo_documento: tipoDoc,
          numero_documento: numDoc.trim(),
          nombre_razon_social: nombre.trim(),
          telefono: telefono.trim() || undefined,
          correo: correo.trim() || undefined,
          direccion: direccion.trim() || undefined,
          motivo: motivo.trim() || undefined,
        };
        await onClienteGuardado(payload);
      }
      setModalAbierto(false);
    } catch (err: unknown) {
      if (typeof err === 'string') setErrorModal(err);
      else if (err && typeof err === 'object' && 'message' in err) {
        setErrorModal(String((err as { message: unknown }).message));
      } else {
        setErrorModal('Error al procesar la solicitud.');
      }
    } finally {
      setCargando(false);
    }
  };

  return (
    <div style={overlayStyle}>
      <div style={modalBoxStyle}>
        <h3 style={{ fontSize: '18px', fontWeight: '800', color: '#F8FAFC', margin: '0 0 16px 0' }}>
          {modo === 'CREAR' ? 'Registrar Nuevo Cliente' : 'Editar Información del Cliente'}
        </h3>

        {errorModal && <div style={errorBoxStyle}>⚠️ {errorModal}</div>}

        <form onSubmit={onSubmit} style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
          <div>
            <label style={labelStyle}>Nombre / Razón Social *</label>
            <input
              required
              type="text"
              placeholder="Ej. Juan Pérez / Empresa S.A.C."
              value={nombre}
              onChange={(e) => setNombre(e.target.value)}
              style={inputStyle}
            />
          </div>

          <div style={{ display: 'grid', gridTemplateColumns: '1fr 1.5fr', gap: '10px' }}>
            <div>
              <label style={labelStyle}>Tipo Documento *</label>
              <select
                value={tipoDoc}
                onChange={(e) => {
                  setTipoDoc(e.target.value as 'DNI' | 'RUC' | 'CE');
                  setNumDoc('');
                }}
                style={{ ...inputStyle, backgroundColor: '#1E293B', cursor: 'pointer' }}
              >
                <option value="DNI">DNI</option>
                <option value="RUC">RUC</option>
                <option value="CE">CE</option>
              </select>
            </div>
            <div>
              <label style={labelStyle}>N° Documento *</label>
              <input
                required
                type="text"
                value={numDoc}
                onChange={(e) => handleNumDocChange(e.target.value)}
                style={inputStyle}
              />
            </div>
          </div>

          <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '10px' }}>
            <div>
              <label style={labelStyle}>Teléfono</label>
              <input
                type="text"
                placeholder="987654321"
                value={telefono}
                onChange={(e) => handleTelefonoChange(e.target.value)}
                style={inputStyle}
              />
            </div>
            <div>
              <label style={labelStyle}>Correo Electrónico</label>
              <input
                type="email"
                placeholder="cliente@correo.com"
                value={correo}
                onChange={(e) => setCorreo(e.target.value)}
                style={inputStyle}
              />
            </div>
          </div>

          <div>
            <label style={labelStyle}>Dirección</label>
            <input
              type="text"
              placeholder="Av. Principal 123"
              value={direccion}
              onChange={(e) => setDireccion(e.target.value)}
              style={inputStyle}
            />
          </div>

          <div
            style={{ display: 'flex', justifyContent: 'flex-end', gap: '10px', marginTop: '12px' }}
          >
            <button type="button" onClick={() => setModalAbierto(false)} style={btnSecondaryStyle}>
              Cancelar
            </button>
            <button type="submit" disabled={cargando} style={btnPrimaryStyle}>
              {cargando ? 'Guardando...' : 'Guardar Cliente'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

const overlayStyle: React.CSSProperties = {
  position: 'fixed',
  top: 0,
  left: 0,
  width: '100vw',
  height: '100vh',
  backgroundColor: 'rgba(0,0,0,0.75)',
  backdropFilter: 'blur(4px)',
  display: 'flex',
  justifyContent: 'center',
  alignItems: 'center',
  zIndex: 1100,
};

const modalBoxStyle: React.CSSProperties = {
  width: '480px',
  backgroundColor: '#0F172A',
  border: '1px solid #334155',
  borderRadius: '16px',
  padding: '24px',
  boxShadow: '0 20px 40px rgba(0,0,0,0.5)',
};

const errorBoxStyle: React.CSSProperties = {
  padding: '10px 14px',
  backgroundColor: 'rgba(239, 68, 68, 0.15)',
  border: '1px solid rgba(239, 68, 68, 0.4)',
  borderRadius: '8px',
  color: '#FCA5A5',
  fontSize: '12px',
};

const labelStyle: React.CSSProperties = {
  display: 'block',
  fontSize: '11px',
  color: '#94A3B8',
  marginBottom: '4px',
};

const inputStyle: React.CSSProperties = {
  width: '100%',
  padding: '8px 12px',
  backgroundColor: '#0F172A',
  border: '1px solid #334155',
  borderRadius: '8px',
  color: '#F8FAFC',
  fontSize: '13px',
  outline: 'none',
  boxSizing: 'border-box',
};

const btnPrimaryStyle: React.CSSProperties = {
  padding: '8px 16px',
  backgroundColor: '#2563EB',
  color: '#FFFFFF',
  border: 'none',
  borderRadius: '8px',
  fontWeight: '700',
  cursor: 'pointer',
};

const btnSecondaryStyle: React.CSSProperties = {
  padding: '8px 14px',
  backgroundColor: 'transparent',
  border: '1px solid #334155',
  color: '#CBD5E1',
  borderRadius: '8px',
  cursor: 'pointer',
};
