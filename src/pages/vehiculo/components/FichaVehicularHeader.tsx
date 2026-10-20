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
    <header className="fv-header">
      <div className="fv-heading">
        <h2>Ficha vehicular</h2>
        <p>Consulta vehículos y su historial de mantenimiento.</p>
      </div>
      <div className="fv-toolbar">
        <form onSubmit={onBuscar} className="fv-search-form">
          <label className="fv-search-label">
            <span>Placa</span>
            <span className="fv-search-input">
              <Search size={17} aria-hidden="true" />
              <input type="text" placeholder="Ej. ABC123" maxLength={6}
                value={placaBusqueda}
                onChange={(e) => setPlacaBusqueda(e.target.value.replace(/[^a-zA-Z0-9]/g, '').toUpperCase())}
                style={inputBusquedaStyle} />
            </span>
          </label>
          <button type="submit" disabled={cargando} className="fv-button fv-primary">
            {cargando ? 'Buscando...' : 'Buscar'}
          </button>
        </form>
        {puedeCrear && (
          <div className="fv-new-actions">
            {onAbrirCrearCliente && (
              <button onClick={onAbrirCrearCliente} className="fv-button">
                <UserPlus size={16} /> Nuevo cliente
              </button>
            )}
            <button onClick={onAbrirCrear} className="fv-button">
              <Plus size={16} /> Nuevo vehículo
            </button>
          </div>
        )}
      </div>
    </header>
  );
};
