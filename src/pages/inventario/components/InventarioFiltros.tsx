import React from 'react';
import { Search, AlertTriangle, RefreshCw } from 'lucide-react';

interface InventarioFiltrosProps {
  busqueda: string;
  setBusqueda: (val: string) => void;
  filtroCritico: boolean;
  setFiltroCritico: (val: boolean) => void;
  onRefresh: () => void;
}

export const InventarioFiltros: React.FC<InventarioFiltrosProps> = ({
  busqueda,
  setBusqueda,
  filtroCritico,
  setFiltroCritico,
  onRefresh,
}) => {
  return (
    <div style={{ display: 'flex', gap: '16px', alignItems: 'center' }}>
      <div style={{ position: 'relative', flex: 1 }}>
        <Search
          size={18}
          color="#64748B"
          style={{
            position: 'absolute',
            left: '14px',
            top: '50%',
            transform: 'translateY(-50%)',
          }}
        />
        <input
          type="text"
          placeholder="Buscar por nombre o código de barras..."
          value={busqueda}
          onChange={(e) => setBusqueda(e.target.value)}
          style={{
            width: '100%',
            padding: '12px 16px 12px 42px',
            backgroundColor: 'rgba(15, 23, 42, 0.6)',
            border: '1px solid #334155',
            borderRadius: '10px',
            color: '#F8FAFC',
            fontSize: '14px',
            outline: 'none',
            boxSizing: 'border-box',
          }}
        />
      </div>

      <button
        onClick={() => setFiltroCritico(!filtroCritico)}
        style={{
          display: 'flex',
          alignItems: 'center',
          gap: '8px',
          padding: '12px 16px',
          backgroundColor: filtroCritico ? 'rgba(239, 68, 68, 0.2)' : 'rgba(30, 41, 59, 0.6)',
          color: filtroCritico ? '#FCA5A5' : '#CBD5E1',
          border: filtroCritico ? '1px solid #EF4444' : '1px solid #334155',
          borderRadius: '10px',
          fontWeight: '600',
          cursor: 'pointer',
        }}
      >
        <AlertTriangle size={18} color={filtroCritico ? '#EF4444' : '#94A3B8'} />
        {filtroCritico ? 'Mostrando Stock Crítico' : 'Ver Stock Crítico'}
      </button>

      <button
        onClick={onRefresh}
        style={{
          padding: '12px',
          backgroundColor: 'rgba(30, 41, 59, 0.6)',
          border: '1px solid #334155',
          borderRadius: '10px',
          color: '#CBD5E1',
          cursor: 'pointer',
        }}
      >
        <RefreshCw size={18} />
      </button>
    </div>
  );
};
