import { inventarioService } from '../../../services/inventarioService';
import { solesAMicro } from '../../../services/operacionInventario';
import React, { useState, useEffect, useRef } from 'react';
import { ServicioDTO, CrearServicioDTO, ActualizarServicioDTO } from '../../../types/inventario';

interface ModalServicioProps {
  abierto: boolean;
  servicioAEditar: ServicioDTO | null;
  onClose: () => void;
  onGuardar: (servicio: CrearServicioDTO | ActualizarServicioDTO) => Promise<void>;
}

export const ModalServicio: React.FC<ModalServicioProps> = ({
  abierto,
  servicioAEditar,
  onClose,
  onGuardar,
}) => {
  const enviando = useRef(false);
  const [codigo, setCodigo] = useState('');
  const [nombre, setNombre] = useState('');
  const [descripcion, setDescripcion] = useState('');
  const [precioSoles, setPrecioSoles] = useState<number>(0);
  const [esCambioAceite, setEsCambioAceite] = useState(false);
  const [guardando, setGuardando] = useState(false);
  const [errorModal, setErrorModal] = useState<string | null>(null);

  useEffect(() => {
    setErrorModal(null);
    if (servicioAEditar) {
      setCodigo(servicioAEditar.codigo);
      setNombre(servicioAEditar.nombre);
      setDescripcion(servicioAEditar.descripcion || '');
      setPrecioSoles(servicioAEditar.precio_venta_cent / 100);
      setEsCambioAceite(servicioAEditar.es_cambio_aceite);
    } else {
      setCodigo('');
      if (abierto)
        inventarioService
          .proponerCodigo('SERVICIO')
          .then((c) => setCodigo((prev) => prev || c))
          .catch((e) => setErrorModal(String(e)));
      setNombre('');
      setDescripcion('');
      setPrecioSoles(0);
      setEsCambioAceite(false);
    }
  }, [servicioAEditar, abierto]);

  if (!abierto) return null;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (enviando.current) return;
    enviando.current = true;
    setErrorModal(null);
    try {
      setGuardando(true);
      const precioCentimos = solesAMicro(precioSoles) / 10000;

      if (servicioAEditar) {
        const payload: ActualizarServicioDTO = {
          id_servicio: servicioAEditar.id_servicio,
          nombre: nombre.trim(),
          descripcion: descripcion.trim() || null,
          precio_venta_cent: precioCentimos,
          es_cambio_aceite: esCambioAceite,
        };
        await onGuardar(payload);
      } else {
        const payload: CrearServicioDTO = {
          codigo: codigo.trim().toUpperCase(),
          nombre: nombre.trim(),
          descripcion: descripcion.trim() || null,
          precio_venta_cent: precioCentimos,
          es_cambio_aceite: esCambioAceite,
        };
        await onGuardar(payload);
      }
      onClose();
    } catch (err: unknown) {
      const msg =
        typeof err === 'string'
          ? err
          : (err as { message?: string })?.message || 'Error al guardar el servicio.';
      setErrorModal(msg);
    } finally {
      enviando.current = false;
      setGuardando(false);
    }
  };

  return (
    <div
      className="gloria-theme inv-overlay"
      style={{
        position: 'fixed',
        inset: 0,
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        zIndex: 1100,
      }}
    >
      <div className="inv-dialog" role="dialog" aria-modal="true" aria-label="Servicio">
        <header className="inv-dialog-header">
          <div>
            <span className="inv-dialog-kicker">Catálogo de servicios</span>
            <h3>{servicioAEditar ? 'Editar Servicio' : 'Nuevo Servicio'}</h3>
            <p>Define el servicio que ofrecerás en el taller.</p>
          </div>
        </header>
        <form className="inv-form" onSubmit={handleSubmit}>
          <fieldset disabled={guardando} className="inv-form-section">
            <legend className="inv-section-label">Información del servicio</legend>
            <div className="inv-form-grid is-identification">
              <label className="inv-field">
                Código del servicio *
                <input
                  required
                  disabled={Boolean(servicioAEditar)}
                  placeholder="SERV-01"
                  value={codigo}
                  onChange={(e) => setCodigo(e.target.value)}
                />
              </label>
              <label className="inv-field">
                Nombre del servicio *
                <input
                  required
                  placeholder="Cambio de Aceite y Filtro Motor"
                  value={nombre}
                  onChange={(e) => setNombre(e.target.value)}
                />
              </label>
            </div>
            <label className="inv-field">
              Descripción u observación
              <input
                placeholder="Incluye purgado de cárter y revisión de niveles"
                value={descripcion}
                onChange={(e) => setDescripcion(e.target.value)}
              />
            </label>
            <label className="inv-field">
              Precio de venta (S/) *
              <input
                required
                type="number"
                step="0.50"
                min="0.50"
                value={precioSoles || ''}
                onChange={(e) => setPrecioSoles(parseFloat(e.target.value) || 0)}
              />
            </label>
            <div className="inv-initial-stock">
              <label className="inv-initial-toggle">
                <input
                  type="checkbox"
                  checked={esCambioAceite}
                  onChange={(e) => setEsCambioAceite(e.target.checked)}
                />
                Servicio de cambio de aceite
              </label>
              <p className="inv-help">
                Se considera para las próximas atenciones de Ficha Vehicular.
              </p>
            </div>
          </fieldset>
          {errorModal && (
            <p role="alert" className="inv-form-error">
              {errorModal}
            </p>
          )}
          <footer className="inv-form-footer">
            <button type="button" disabled={guardando} className="inv-button" onClick={onClose}>
              Cancelar
            </button>
            <button type="submit" disabled={guardando} className="inv-button inv-primary">
              {guardando ? 'Guardando...' : servicioAEditar ? 'Guardar Cambios' : 'Crear Servicio'}
            </button>
          </footer>
        </form>
      </div>
    </div>
  );
};
