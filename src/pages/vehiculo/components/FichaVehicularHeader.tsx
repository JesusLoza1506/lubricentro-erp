import React from 'react';
import { Search, Plus } from 'lucide-react';
import { inputBusquedaStyle } from '../vehiculoStyles';

interface Props {
  placaBusqueda: string;
  setPlacaBusqueda: (val: string) => void;
  cargando: boolean;
  puedeCrear: boolean;
  onBuscar: (e: React.FormEvent) => void;
  onAbrirCrear: () => void;
}

export const FichaVehicularHeader: React.FC<Props> = ({
  placaBusqueda,
  setPlacaBusqueda,
  cargando,
  puedeCrear,
  onBuscar,
  onAbrirCrear,
}) => {
  return (
    <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
      <div>
        <h2 style={{ fontSize: '26px', fontWeight: '800', color: '#F8FAFC', margin: '0 0 4px 0' }}>
          Ficha Vehicular
        </h2>
        <p style={{ color: '#94A3B8', fontSize: '14px', margin: 0 }}>
          Consulta las especificaciones, propietario y hoja de vida de mantenimientos por placa.
        </p>
      </div>

      <div style={{ display: 'flex', gap: '12px', alignItems: 'center' }}>
        <form onSubmit={onBuscar} style={{ display: 'flex', gap: '12px' }}>
          <div style={{ position: 'relative' }}>
            <input
              type="text"
              placeholder="Ej. ABC-123"
              value={placaBusqueda}
              onChange={(e) => setPlacaBusqueda(e.target.value.toUpperCase())}
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
            }}
          >
            {cargando ? 'Buscando...' : 'Buscar'}
          </button>
        </form>

        {puedeCrear && (
          <button
            onClick={onAbrirCrear}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              backgroundColor: 'rgba(16, 185, 129, 0.2)',
              color: '#34D399',
              border: '1px solid #10B981',
              borderRadius: '12px',
              padding: '12px 18px',
              fontSize: '14px',
              fontWeight: '700',
              cursor: 'pointer',
            }}
          >
            <Plus size={18} />
            Nuevo Vehículo
          </button>
        )}
      </div>
    </div>
  );
};
