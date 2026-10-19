import React from 'react';
import { Search, Plus, UserPlus } from 'lucide-react';
import { inputBusquedaStyle } from '../vehiculoStyles';

interface Props {
  placaBusqueda: string;
  setPlacaBusqueda: (val: string) => void;
  cargando: boolean;
  puedeCrear: boolean;
  onBuscar: (e: React.FormEvent) => void;
  onAbrirCrear: () => void;
  onAbrirCrearCliente?: () => void;
}

export const FichaVehicularHeader: React.FC<Props> = ({
  placaBusqueda,
  setPlacaBusqueda,
  cargando,
  puedeCrear,
  onBuscar,
  onAbrirCrear,
  onAbrirCrearCliente,
}) => {
  return (
    <div
      style={{
        display: 'flex',
        justifyContent: 'space-between',
        alignItems: 'center',
        flexWrap: 'wrap',
        gap: '16px',
        paddingBottom: '8px',
        borderBottom: '1px solid rgba(51, 65, 85, 0.4)',
      }}
    >
      <div>
        <h2 style={{ fontSize: '26px', fontWeight: '800', color: '#F8FAFC', margin: '0 0 4px 0' }}>
          Ficha Vehicular
        </h2>
        <p style={{ color: '#94A3B8', fontSize: '14px', margin: 0 }}>
          Consulta las especificaciones, propietario y hoja de vida de mantenimientos por placa.
        </p>
      </div>

      <div style={{ display: 'flex', gap: '12px', alignItems: 'center', flexWrap: 'wrap' }}>
        <form onSubmit={onBuscar} style={{ display: 'flex', gap: '10px' }}>
          <div style={{ position: 'relative' }}>
            <input
              type="text"
              placeholder="Ej. ABC123"
              maxLength={6}
              value={placaBusqueda}
              onChange={(e) =>
                setPlacaBusqueda(e.target.value.replace(/[^a-zA-Z0-9]/g, '').toUpperCase())
              }
              style={inputBusquedaStyle}
            />
            <Search
              size={18}
              color="#94A3B8"
              style={{
                position: 'absolute',
                left: '14px',
                top: '50%',
                transform: 'translateY(-50%)',
              }}
            />
          </div>
          <button
            type="submit"
            disabled={cargando}
            style={{
              backgroundColor: '#2563EB',
              color: '#FFFFFF',
              border: 'none',
              borderRadius: '12px',
              padding: '0 20px',
              fontSize: '14px',
              fontWeight: '700',
              cursor: 'pointer',
              transition: 'background 0.2s',
            }}
          >
            {cargando ? 'Buscando...' : 'Buscar'}
          </button>
        </form>

        {puedeCrear && (
          <div
            style={{
              display: 'flex',
              gap: '10px',
              borderLeft: '1px solid #334155',
              paddingLeft: '12px',
            }}
          >
            {onAbrirCrearCliente && (
              <button
                onClick={onAbrirCrearCliente}
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '8px',
                  backgroundColor: 'rgba(37, 99, 235, 0.15)',
                  color: '#60A5FA',
                  border: '1px solid rgba(37, 99, 235, 0.4)',
                  borderRadius: '12px',
                  padding: '10px 16px',
                  fontSize: '13px',
                  fontWeight: '700',
                  cursor: 'pointer',
                }}
              >
                <UserPlus size={16} />
                Nuevo Cliente
              </button>
            )}

            <button
              onClick={onAbrirCrear}
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: '8px',
                backgroundColor: 'rgba(16, 185, 129, 0.15)',
                color: '#34D399',
                border: '1px solid rgba(16, 185, 129, 0.4)',
                borderRadius: '12px',
                padding: '10px 16px',
                fontSize: '13px',
                fontWeight: '700',
                cursor: 'pointer',
              }}
            >
              <Plus size={16} />
              Nuevo Vehículo
            </button>
          </div>
        )}
      </div>
    </div>
  );
};
