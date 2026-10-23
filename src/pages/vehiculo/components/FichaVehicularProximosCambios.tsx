import React, { useEffect, useState } from 'react';
import { ProximoCambioAceiteDTO } from '../../../types/vehiculo';
import { vehiculoService } from '../../../services/vehiculoService';
import {
  Calendar,
  Phone,
  Droplet,
  ArrowRight,
  Search,
  RefreshCw,
  Filter,
} from 'lucide-react';

export type TipoFiltroCausa = 'TODOS' | 'POR_KM' | 'POR_FECHA' | 'AMBOS';

export interface FiltrosProximosCambiosState {
  busqueda: string;
  filtroCausa: TipoFiltroCausa;
  paginaActual: number;
}

interface Props {
  onSeleccionarPlaca: (placa: string) => void;
  filtrosIniciales?: FiltrosProximosCambiosState;
  onFiltrosChange?: (filtros: FiltrosProximosCambiosState) => void;
}

export const FichaVehicularProximosCambios: React.FC<Props> = ({
  onSeleccionarPlaca,
  filtrosIniciales,
  onFiltrosChange,
}) => {
  const [lista, setLista] = useState<ProximoCambioAceiteDTO[]>([]);
  const [totalRegistros, setTotalRegistros] = useState<number>(0);
  const [totalPaginas, setTotalPaginas] = useState<number>(1);
  const [cargando, setCargando] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  const [busqueda, setBusqueda] = useState<string>(filtrosIniciales?.busqueda || '');
  const [filtroCausa, setFiltroCausa] = useState<TipoFiltroCausa>(
    filtrosIniciales?.filtroCausa || 'TODOS'
  );
  const [paginaActual, setPaginaActual] = useState<number>(filtrosIniciales?.paginaActual || 1);
  const elementosPorPagina = 20;

  useEffect(() => {
    if (onFiltrosChange) {
      onFiltrosChange({ busqueda, filtroCausa, paginaActual });
    }
  }, [busqueda, filtroCausa, paginaActual, onFiltrosChange]);

  const cargarProximosCambios = async () => {
    setCargando(true);
    setError(null);
    try {
      const data = await vehiculoService.obtenerProximosCambiosAceite(
        paginaActual,
        elementosPorPagina,
        busqueda,
        filtroCausa
      );
      setLista(data.items || []);
      setTotalRegistros(data.total_registros || 0);
      setTotalPaginas(data.total_paginas || 1);
    } catch (err: unknown) {
      if (typeof err === 'string') setError(err);
      else if (err && typeof err === 'object' && 'message' in err) {
        setError(String((err as { message: unknown }).message));
      } else {
        setError('No se pudo cargar la lista de próximos cambios de aceite.');
      }
    } finally {
      setCargando(false);
    }
  };

  useEffect(() => {
    cargarProximosCambios();
  }, [paginaActual, busqueda, filtroCausa]);

  const handleBusquedaChange = (val: string) => {
    setBusqueda(val);
    setPaginaActual(1);
  };

  const handleCausaChange = (causa: TipoFiltroCausa) => {
    setFiltroCausa(causa);
    setPaginaActual(1);
  };

  if (cargando) {
    return (
      <div style={{ padding: '40px', textAlign: 'center', color: '#94A3B8' }}>
        Cargando vehículos candidatos a cambio de aceite...
      </div>
    );
  }

  if (error) {
    return (
      <div
        style={{
          padding: '16px',
          backgroundColor: 'rgba(239, 68, 68, 0.1)',
          border: '1px solid rgba(239, 68, 68, 0.3)',
          borderRadius: '12px',
          color: '#FCA5A5',
          fontSize: '14px',
        }}
      >
        {error}
      </div>
    );
  }

  return (
    <section
      className="fv-upcoming"
      style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}
    >
      <div
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          flexWrap: 'wrap',
          gap: '12px',
        }}
      >
        <div>
          <h3 style={{ fontSize: '18px', fontWeight: '700', color: '#F8FAFC', margin: 0 }}>
            Próximos Cambios de Aceite ({totalRegistros})
          </h3>
          <p style={{ color: '#94A3B8', fontSize: '13px', margin: '4px 0 0 0' }}>
            Vehículos próximos a cumplir o vencidos en su intervalo de mantenimiento preventivo.
          </p>
        </div>
        <button
          onClick={cargarProximosCambios}
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: '6px',
            padding: '8px 14px',
            backgroundColor: 'rgba(30, 41, 59, 0.8)',
            border: '1px solid #334155',
            borderRadius: '8px',
            color: '#38BDF8',
            fontSize: '13px',
            fontWeight: '600',
            cursor: 'pointer',
          }}
        >
          <RefreshCw size={14} /> Actualizar Lista
        </button>
      </div>

      <div
        className="fv-upcoming-filters"
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          gap: '12px',
          padding: '12px',
          backgroundColor: 'rgba(15, 23, 42, 0.6)',
          border: '1px solid #334155',
          borderRadius: '12px',
          flexWrap: 'wrap',
        }}
      >
        <div style={{ position: 'relative', flex: '1', minWidth: '220px' }}>
          <Search
            size={16}
            color="#94A3B8"
            style={{
              position: 'absolute',
              left: '12px',
              top: '50%',
              transform: 'translateY(-50%)',
            }}
          />
          <input
            type="text"
            placeholder="Buscar por placa o cliente..."
            value={busqueda}
            onChange={(e) => handleBusquedaChange(e.target.value)}
            style={{
              width: '100%',
              padding: '8px 12px 8px 36px',
              backgroundColor: '#0F172A',
              border: '1px solid #334155',
              borderRadius: '8px',
              color: '#F8FAFC',
              fontSize: '13px',
              outline: 'none',
              boxSizing: 'border-box',
            }}
          />
        </div>

        <div
          className="fv-filter-buttons"
          style={{ display: 'flex', gap: '6px', alignItems: 'center' }}
        >
          <Filter size={14} color="#94A3B8" />
          {(['TODOS', 'POR_KM', 'POR_FECHA', 'AMBOS'] as TipoFiltroCausa[]).map((causa) => (
            <button
              key={causa}
              onClick={() => handleCausaChange(causa)}
              style={{
                padding: '6px 12px',
                borderRadius: '6px',
                fontSize: '11px',
                fontWeight: '700',
                border: '1px solid',
                cursor: 'pointer',
                backgroundColor:
                  filtroCausa === causa ? 'rgba(37, 99, 235, 0.2)' : 'rgba(30, 41, 59, 0.5)',
                color: filtroCausa === causa ? '#38BDF8' : '#94A3B8',
                borderColor: filtroCausa === causa ? '#2563EB' : '#334155',
              }}
            >
              {causa === 'TODOS'
                ? 'Todos'
                : causa === 'POR_KM'
                  ? 'Por Km'
                  : causa === 'POR_FECHA'
                    ? 'Por Fecha'
                    : 'Km y Fecha'}
            </button>
          ))}
        </div>
      </div>

      {lista.length === 0 ? (
        <div
          style={{
            padding: '32px',
            textAlign: 'center',
            backgroundColor: 'rgba(15, 23, 42, 0.4)',
            border: '1px solid #334155',
            borderRadius: '16px',
            color: '#94A3B8',
            fontSize: '14px',
          }}
        >
          No se encontraron vehículos que coincidan con los criterios de búsqueda.
        </div>
      ) : (
        <div
          style={{
            display: 'grid',
            gridTemplateColumns: 'repeat(auto-fill, minmax(min(100%, 340px), 1fr))',
            gap: '16px',
          }}
        >
          {lista.map((item) => (
            <article className="fv-upcoming-card" key={`${item.id_vehiculo}-${item.id_ot}`}>
              <div className="fv-upcoming-card-heading">
                <div><strong className="fv-upcoming-plate">{item.placa}</strong><p>{item.nombre_razon_social}</p></div>
                <span className="fv-upcoming-cause">{item.causa === 'POR_KM' ? 'Por kilometraje' : item.causa === 'POR_FECHA' ? 'Por fecha' : 'Por km y fecha'}</span>
              </div>
              <div className="fv-upcoming-mileage">
                <div><span>Km actual</span><strong>{(item.kilometraje_actual ?? 0).toLocaleString()} <small>km</small></strong></div>
                <ArrowRight size={16} aria-hidden="true" />
                <div><span>Próximo cambio</span><strong>{item.km_proximo !== null && item.km_proximo !== undefined ? `${item.km_proximo.toLocaleString()} km` : 'No registrado'}</strong></div>
              </div>
              <div className="fv-upcoming-meta">
                <span><Calendar size={14} />Fecha de servicio: {item.fecha_servicio ? new Date(item.fecha_servicio).toLocaleDateString('es-PE') : 'N/A'}</span>
                {item.tipo_aceite && <span><Droplet size={14} />{item.tipo_aceite} · Intervalo: {item.intervalo_aceite_km || 0} km</span>}
                {item.telefono && <span><Phone size={14} />{item.telefono}</span>}
              </div>
              <button onClick={() => onSeleccionarPlaca(item.placa)} className="fv-button fv-open-record">Ver ficha vehicular <ArrowRight size={15} /></button>
            </article>
          ))}
        </div>
      )}

      {totalPaginas > 1 && (
        <div
          style={{
            display: 'flex',
            justifyContent: 'center',
            alignItems: 'center',
            gap: '12px',
            marginTop: '12px',
          }}
        >
          <button
            disabled={paginaActual === 1}
            onClick={() => setPaginaActual((p) => Math.max(1, p - 1))}
            style={{
              padding: '6px 14px',
              backgroundColor: 'rgba(30, 41, 59, 0.6)',
              border: '1px solid #334155',
              borderRadius: '6px',
              color: paginaActual === 1 ? '#64748B' : '#F8FAFC',
              cursor: paginaActual === 1 ? 'not-allowed' : 'pointer',
            }}
          >
            Anterior
          </button>
          <span style={{ fontSize: '13px', color: '#94A3B8' }}>
            Página <strong>{paginaActual}</strong> de <strong>{totalPaginas}</strong>
          </span>
          <button
            disabled={paginaActual === totalPaginas}
            onClick={() => setPaginaActual((p) => Math.min(totalPaginas, p + 1))}
            style={{
              padding: '6px 14px',
              backgroundColor: 'rgba(30, 41, 59, 0.6)',
              border: '1px solid #334155',
              borderRadius: '6px',
              color: paginaActual === totalPaginas ? '#64748B' : '#F8FAFC',
              cursor: paginaActual === totalPaginas ? 'not-allowed' : 'pointer',
            }}
          >
            Siguiente
          </button>
        </div>
      )}
    </section>
  );
};
