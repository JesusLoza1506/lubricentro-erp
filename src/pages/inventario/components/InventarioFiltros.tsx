import { Search, AlertTriangle, RefreshCw } from 'lucide-react';
interface InventarioFiltrosProps {
  busqueda: string;
  setBusqueda: (val: string) => void;
  filtroCritico: boolean;
  setFiltroCritico: (val: boolean) => void;
  onRefresh: () => void;
}
export function InventarioFiltros({
  busqueda,
  setBusqueda,
  filtroCritico,
  setFiltroCritico,
  onRefresh,
}: InventarioFiltrosProps) {
  return (
    <div className="inv-search-row">
      <div className="inv-search">
        <Search size={17} />
        <input
          aria-label="Buscar productos"
          placeholder="Buscar por nombre o código de barras..."
          value={busqueda}
          onChange={(e) => setBusqueda(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === 'Enter') onRefresh();
          }}
        />
      </div>
      <button
        className={`inv-button inv-critical-shortcut ${filtroCritico ? 'is-active' : ''}`}
        aria-pressed={filtroCritico}
        onClick={() => setFiltroCritico(!filtroCritico)}
      >
        <AlertTriangle size={16} />
        {filtroCritico ? 'Mostrando Stock Crítico' : 'Ver Stock Crítico'}
      </button>
      <button
        className="inv-icon-button"
        aria-label="Actualizar catálogo"
        title="Actualizar catálogo"
        onClick={onRefresh}
      >
        <RefreshCw size={17} />
      </button>
    </div>
  );
}
