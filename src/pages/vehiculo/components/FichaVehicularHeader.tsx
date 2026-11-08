import React from 'react';
import { Search, Plus, UserPlus, XCircle } from 'lucide-react';

interface Props {
  placaBusqueda: string;
  setPlacaBusqueda: (val: string) => void;
  cargando: boolean;
  puedeCrear: boolean;
  onBuscar: (e: React.FormEvent) => void;
  onLimpiar?: () => void;
  onAbrirCrear: () => void;
  onAbrirCrearCliente?: () => void;
}

export const FichaVehicularHeader: React.FC<Props> = ({
  placaBusqueda,
  setPlacaBusqueda,
  cargando,
  puedeCrear,
  onBuscar,
  onLimpiar,
  onAbrirCrear,
  onAbrirCrearCliente,
}) => {
  const normalizarPlaca = (val: string) => {
    return val.replace(/[^a-zA-Z0-9]/g, '').toUpperCase();
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setPlacaBusqueda(normalizarPlaca(e.target.value));
  };

  const handlePaste = (e: React.ClipboardEvent<HTMLInputElement>) => {
    e.preventDefault();
    const textoPegado = e.clipboardData.getData('text');
    setPlacaBusqueda(normalizarPlaca(textoPegado));
  };

  return (
    <header className="fv-header">
      <div className="fv-heading">
        <h3>Buscar por placa</h3>
      </div>
      <div className="fv-toolbar">
        <form onSubmit={onBuscar} className="fv-search-form">
          <label className="fv-search-label">
            <span>Placa</span>
            <span className="fv-search-input">
              <Search size={17} aria-hidden="true" />
              <input
                type="text"
                placeholder="Ej. ABC123"
                maxLength={6}
                value={placaBusqueda}
                onChange={handleChange}
                onPaste={handlePaste}
                className="gt-input gt-input--leading"
              />
            </span>
          </label>
          <button
            type="submit"
            disabled={cargando}
            aria-busy={cargando}
            className="fv-button fv-primary"
          >
            {cargando ? 'Buscando...' : 'Buscar'}
          </button>
          {onLimpiar && (
            <button
              type="button"
              disabled={cargando}
              onClick={onLimpiar}
              className="fv-button"
              title="Limpiar búsqueda y ocultar ficha"
            >
              <XCircle size={16} aria-hidden="true" /> Limpiar
            </button>
          )}
        </form>
        {puedeCrear && (
          <div className="fv-new-actions">
            {onAbrirCrearCliente && (
              <button onClick={onAbrirCrearCliente} className="fv-button">
                <UserPlus size={16} aria-hidden="true" /> Nuevo cliente
              </button>
            )}
            <button onClick={onAbrirCrear} className="fv-button">
              <Plus size={16} aria-hidden="true" /> Nuevo vehículo
            </button>
          </div>
        )}
      </div>
    </header>
  );
};
