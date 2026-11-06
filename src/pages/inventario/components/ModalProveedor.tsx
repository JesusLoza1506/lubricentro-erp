import { useNotificarInventario } from '../AvisosInventario';
import React, { useState, useEffect, useRef } from 'react';
import { ProveedorDTO, CrearProveedorDTO, ActualizarProveedorDTO } from '../../../types/inventario';

interface ModalProveedorProps {
  abierto: boolean;
  proveedorAEditar?: ProveedorDTO | null;
  onClose: () => void;
  onGuardar: (proveedor: CrearProveedorDTO) => Promise<void>;
  onEditar?: (proveedor: ActualizarProveedorDTO) => Promise<void>;
}

export const ModalProveedor: React.FC<ModalProveedorProps> = ({
  abierto,
  proveedorAEditar,
  onClose,
  onGuardar,
  onEditar,
}) => {
  const notificar = useNotificarInventario();
  const [form, setForm] = useState<CrearProveedorDTO>({
    tipo_documento: 'RUC',
    numero_documento: '',
    nombre_razon_social: '',
    telefono: '',
    correo: '',
    direccion: '',
    contacto_asesor: '',
  });
  const envioEnCurso = useRef(false);
  const [guardando, setGuardando] = useState(false);
  const [errorModal, setErrorModal] = useState<string | null>(null);

  useEffect(() => {
    setErrorModal(null);
    if (proveedorAEditar) {
      setForm({
        tipo_documento: proveedorAEditar.tipo_documento,
        numero_documento: proveedorAEditar.numero_documento,
        nombre_razon_social: proveedorAEditar.nombre_razon_social,
        telefono: proveedorAEditar.telefono || '',
        correo: proveedorAEditar.correo || '',
        direccion: proveedorAEditar.direccion || '',
        contacto_asesor: proveedorAEditar.contacto_asesor || '',
      });
    } else {
      setForm({
        tipo_documento: 'RUC',
        numero_documento: '',
        nombre_razon_social: '',
        telefono: '',
        correo: '',
        direccion: '',
        contacto_asesor: '',
      });
    }
  }, [proveedorAEditar, abierto]);

  if (!abierto) return null;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (envioEnCurso.current) return;
    envioEnCurso.current = true;
    setErrorModal(null);
    try {
      setGuardando(true);
      if (proveedorAEditar && !onEditar)
        throw new Error('No se puede editar el proveedor en esta pantalla.');
      if (proveedorAEditar && onEditar) {
        await onEditar({
          ...form,
          id_proveedor: proveedorAEditar.id_proveedor,
          tipo_documento: form.tipo_documento.trim().toUpperCase(),
          numero_documento: form.numero_documento.trim(),
          nombre_razon_social: form.nombre_razon_social.trim(),
          telefono: form.telefono?.trim() || null,
          correo: form.correo?.trim() || null,
          direccion: form.direccion?.trim() || null,
          contacto_asesor: form.contacto_asesor?.trim() || null,
        } as ActualizarProveedorDTO);
      } else {
        await onGuardar({
          ...form,
          tipo_documento: form.tipo_documento.trim().toUpperCase(),
          numero_documento: form.numero_documento.trim(),
          nombre_razon_social: form.nombre_razon_social.trim(),
          telefono: form.telefono?.trim() || null,
          correo: form.correo?.trim() || null,
          direccion: form.direccion?.trim() || null,
          contacto_asesor: form.contacto_asesor?.trim() || null,
        });
      }
      onClose();
    } catch (err: unknown) {
      const msg =
        typeof err === 'string'
          ? err
          : (err as { message?: string })?.message || 'Error al guardar el proveedor.';
      setErrorModal(msg);
      notificar(msg, 'ERROR');
    } finally {
      envioEnCurso.current = false;
      setGuardando(false);
    }
  };

  return (
    <div
      className="inv-overlay"
      style={{
        position: 'fixed',
        inset: 0,
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        zIndex: 1400,
      }}
    >
      <div className="inv-dialog" role="dialog" aria-modal="true" aria-label="Proveedor">
        <header className="inv-dialog-header">
          <div>
            <h3>{proveedorAEditar ? 'Editar Proveedor' : 'Nuevo Proveedor'}</h3>
            <p>Datos de identificación y contacto para las compras.</p>
          </div>
        </header>
        <form className="inv-form" onSubmit={handleSubmit}>
          <fieldset disabled={guardando} className="inv-form-section">
            <legend className="inv-section-label">Identificación</legend>
            <div className="inv-form-grid is-identification">
              <label className="inv-field">
                Tipo documento
                <select
                  value={form.tipo_documento}
                  onChange={(e) => setForm({ ...form, tipo_documento: e.target.value })}
                >
                  <option value="RUC">RUC</option>
                  <option value="DNI">DNI</option>
                  <option value="CE">CE (Carné Extr.)</option>
                </select>
              </label>
              <label className="inv-field">
                Número documento *
                <input
                  required
                  placeholder="20100070970"
                  value={form.numero_documento}
                  onChange={(e) => setForm({ ...form, numero_documento: e.target.value })}
                />
              </label>
            </div>
            <label className="inv-field">
              Nombre o razón social *
              <input
                required
                placeholder="Distribuidora de Lubricantes SAC"
                value={form.nombre_razon_social}
                onChange={(e) => setForm({ ...form, nombre_razon_social: e.target.value })}
              />
            </label>
          </fieldset>
          <fieldset disabled={guardando} className="inv-form-section">
            <legend className="inv-section-label">Contacto · opcional</legend>
            <div className="inv-form-grid">
              <label className="inv-field">
                Teléfono
                <input
                  value={form.telefono || ''}
                  onChange={(e) => setForm({ ...form, telefono: e.target.value })}
                />
              </label>
              <label className="inv-field">
                Persona de contacto
                <input
                  value={form.contacto_asesor || ''}
                  onChange={(e) => setForm({ ...form, contacto_asesor: e.target.value })}
                />
              </label>
            </div>
            <label className="inv-field">
              Correo electrónico
              <input
                type="email"
                placeholder="contacto@proveedor.com"
                value={form.correo || ''}
                onChange={(e) => setForm({ ...form, correo: e.target.value })}
              />
            </label>
            <label className="inv-field">
              Dirección
              <input
                placeholder="Av. Industrial 123"
                value={form.direccion || ''}
                onChange={(e) => setForm({ ...form, direccion: e.target.value })}
              />
            </label>
          </fieldset>
          {errorModal && (
            <p role="alert" className="inv-form-error">
              {errorModal}
            </p>
          )}
          <footer className="inv-form-footer">
            <button type="button" className="inv-button" disabled={guardando} onClick={onClose}>
              Cancelar
            </button>
            <button type="submit" className="inv-button inv-primary" disabled={guardando}>
              {guardando
                ? 'Guardando...'
                : proveedorAEditar
                  ? 'Guardar Cambios'
                  : 'Crear Proveedor'}
            </button>
          </footer>
        </form>
      </div>
    </div>
  );
};
