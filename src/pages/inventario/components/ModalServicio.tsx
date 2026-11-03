import React, { useState, useEffect } from 'react';
import { AlertCircle } from 'lucide-react';
import { ServicioDTO, CrearServicioDTO, ActualizarServicioDTO } from '../../../types/inventario';
import { modalInputStyle } from '../inventarioStyles';

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
      setNombre('');
      setDescripcion('');
      setPrecioSoles(0);
      setEsCambioAceite(false);
    }
  }, [servicioAEditar, abierto]);

  if (!abierto) return null;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setErrorModal(null);
    try {
      setGuardando(true);
      const precioCentimos = Math.round(precioSoles * 100);

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
          {servicioAEditar ? 'Editar Servicio de Mantenimiento' : 'Nuevo Servicio de Mantenimiento'}
        </h3>

        {errorModal && (
          <div
            style={{
              backgroundColor: 'rgba(239, 68, 68, 0.15)',
              border: '1px solid #EF4444',
              color: '#F87171',
              padding: '10px 14px',
              borderRadius: '8px',
              marginBottom: '14px',
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              fontSize: '12px',
            }}
          >
            <AlertCircle size={16} />
            <span>{errorModal}</span>
          </div>
        )}

        <form
          onSubmit={handleSubmit}
          style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}
        >
          <div>
            <label
              style={{ display: 'block', fontSize: '11px', color: '#94A3B8', marginBottom: '4px' }}
            >
              Código del Servicio *
            </label>
            <input
              required
              disabled={Boolean(servicioAEditar)}
              type="text"
              placeholder="SERV-01"
              value={codigo}
              onChange={(e) => setCodigo(e.target.value)}
              style={{ ...modalInputStyle, opacity: servicioAEditar ? 0.6 : 1 }}
            />
          </div>

          <div>
            <label
              style={{ display: 'block', fontSize: '11px', color: '#94A3B8', marginBottom: '4px' }}
            >
              Nombre del Servicio *
            </label>
            <input
              required
              type="text"
              placeholder="Cambio de Aceite y Filtro Motor"
              value={nombre}
              onChange={(e) => setNombre(e.target.value)}
              style={modalInputStyle}
            />
          </div>

          <div>
            <label
              style={{ display: 'block', fontSize: '11px', color: '#94A3B8', marginBottom: '4px' }}
            >
              Descripción u Observación
            </label>
            <input
              type="text"
              placeholder="Incluye purgado de cárter y revisión de niveles"
              value={descripcion}
              onChange={(e) => setDescripcion(e.target.value)}
              style={modalInputStyle}
            />
          </div>

          <div>
            <label
              style={{ display: 'block', fontSize: '11px', color: '#94A3B8', marginBottom: '4px' }}
            >
              Precio de Venta (S/) *
            </label>
            <input
              required
              type="number"
              step="0.50"
              min="0.50"
              value={precioSoles || ''}
              onChange={(e) => setPrecioSoles(parseFloat(e.target.value) || 0)}
              style={modalInputStyle}
            />
          </div>

          <label
            style={{
              fontSize: '13px',
              color: '#CBD5E1',
              cursor: 'pointer',
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              marginTop: '4px',
            }}
          >
            <input
              type="checkbox"
              checked={esCambioAceite}
              onChange={(e) => setEsCambioAceite(e.target.checked)}
            />
            ¿Este servicio es cambio de aceite? (Dispara alertas preventivas en Ficha Vehicular)
          </label>

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
                backgroundColor: '#10B981',
                color: '#FFFFFF',
                border: 'none',
                borderRadius: '8px',
                fontWeight: '700',
                cursor: 'pointer',
              }}
            >
              {guardando ? 'Guardando...' : servicioAEditar ? 'Guardar Cambios' : 'Crear Servicio'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};
