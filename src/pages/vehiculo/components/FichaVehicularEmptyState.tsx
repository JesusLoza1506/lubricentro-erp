import React from 'react';
import { Car, ShieldCheck, SearchX } from 'lucide-react';
import { btnPlacaPruebaStyle } from '../vehiculoStyles';

interface Props {
  buscado: boolean;
  placaBusqueda: string;
  puedeCrear: boolean;
  onEjecutarBusqueda: (placa: string) => void;
  onAbrirCrear: () => void;
}

export const FichaVehicularEmptyState: React.FC<Props> = ({
  buscado,
  placaBusqueda,
  puedeCrear,
  onEjecutarBusqueda,
  onAbrirCrear,
}) => {
  if (!buscado) {
    return (
      <div
        style={{
          flex: 1,
          display: 'flex',
          flexDirection: 'column',
          justifyContent: 'center',
          alignItems: 'center',
          padding: '60px 20px',
          backgroundColor: 'rgba(15, 23, 42, 0.4)',
          border: '2px dashed #334155',
          borderRadius: '20px',
          textAlign: 'center',
          gap: '16px',
        }}
      >
        <div
          style={{
            padding: '20px',
            backgroundColor: 'rgba(37, 99, 235, 0.1)',
            borderRadius: '50%',
          }}
        >
          <Car size={48} color="#38BDF8" />
        </div>
        <div>
          <h3
            style={{ fontSize: '20px', fontWeight: '700', color: '#F8FAFC', margin: '0 0 8px 0' }}
          >
            Consulta la Hoja de Vida Vehicular
          </h3>
          <p
            style={{
              color: '#94A3B8',
              fontSize: '14px',
              maxWidth: '460px',
              margin: 0,
              lineHeight: '1.5',
            }}
          >
            Ingresa el número de placa en el buscador superior para verificar datos del propietario,
            kilometraje e historial de mantenimientos.
          </p>
        </div>

        <div
          style={{
            marginTop: '12px',
            display: 'flex',
            alignItems: 'center',
            gap: '8px',
            backgroundColor: 'rgba(30, 41, 59, 0.8)',
            padding: '8px 16px',
            borderRadius: '12px',
            border: '1px solid #334155',
          }}
        >
          <ShieldCheck size={16} color="#34D399" />
          <span style={{ fontSize: '13px', color: '#CBD5E1' }}>
            Prueba buscar placas registradas:
          </span>
          <button onClick={() => onEjecutarBusqueda('ABC-123')} style={btnPlacaPruebaStyle}>
            ABC-123
          </button>
          <button onClick={() => onEjecutarBusqueda('XYZ-789')} style={btnPlacaPruebaStyle}>
            XYZ-789
          </button>
        </div>
      </div>
    );
  }

  return (
    <div
      style={{
        padding: '48px',
        textAlign: 'center',
        backgroundColor: 'rgba(15, 23, 42, 0.4)',
        border: '1px dashed #334155',
        borderRadius: '16px',
        color: '#94A3B8',
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        gap: '12px',
      }}
    >
      <SearchX size={40} color="#64748B" />
      <span style={{ fontSize: '16px', fontWeight: '600', color: '#E2E8F0' }}>
        Vehículo no encontrado
      </span>
      <p style={{ margin: 0, fontSize: '14px' }}>
        No existe registro para la placa{' '}
        <strong style={{ color: '#38BDF8' }}>{placaBusqueda}</strong>.
      </p>
      {puedeCrear && (
        <button
          onClick={onAbrirCrear}
          style={{
            marginTop: '12px',
            padding: '10px 20px',
            backgroundColor: '#2563EB',
            color: '#FFFFFF',
            border: 'none',
            borderRadius: '10px',
            fontWeight: '700',
            cursor: 'pointer',
          }}
        >
          Registrar {placaBusqueda} Ahora
        </button>
      )}
    </div>
  );
};
