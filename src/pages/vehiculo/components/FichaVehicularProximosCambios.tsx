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
  AlertCircle,
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
      <div className="fv-list-empty fv-loading" role="status">
        <span className="gt-spinner" aria-hidden="true" /> Cargando vehículos candidatos a cambio de aceite…
      </div>
    );
  }

  if (error) {
    return (
      <div className="gt-notice gt-notice--error fv-lookup-error" role="alert">
        <AlertCircle size={17} aria-hidden="true" />
        <div><strong>No se pudieron consultar los próximos cambios</strong><p>{error}</p></div>
        <button type="button" className="fv-button fv-small" onClick={cargarProximosCambios}>
          <RefreshCw size={14} aria-hidden="true" /> Reintentar
        </button>
      </div>
    );
  }

  return (
    <section className="fv-upcoming" aria-labelledby="fv-upcoming-title">
      <div className="fv-section-title fv-upcoming-heading">
        <div>
          <h3 id="fv-upcoming-title">Próximos cambios de aceite <span className="fv-count">{totalRegistros}</span></h3>
          <p>Vehículos que se acercan al próximo mantenimiento o que ya superaron el intervalo.</p>
        </div>
        <button type="button" onClick={cargarProximosCambios} className="fv-button">
          <RefreshCw size={15} aria-hidden="true" /> Actualizar lista
        </button>
      </div>

      <div className="fv-upcoming-filters">
        <div className="fv-search-input">
          <Search size={16} aria-hidden="true" />
          <input
            type="text"
            className="gt-input gt-input--leading"
            aria-label="Buscar por placa o cliente"
            placeholder="Buscar por placa o cliente..."
            value={busqueda}
            onChange={(e) => handleBusquedaChange(e.target.value)}
          />
        </div>

        <div className="fv-filter-buttons" role="group" aria-label="Motivo del cambio">
          <Filter size={15} aria-hidden="true" />
          {(['TODOS', 'POR_KM', 'POR_FECHA', 'AMBOS'] as TipoFiltroCausa[]).map((causa) => (
            <button type="button" key={causa} className="fv-button fv-small"
              aria-pressed={filtroCausa === causa} onClick={() => handleCausaChange(causa)}>
              {causa === 'TODOS' ? 'Todos' : causa === 'POR_KM' ? 'Por km' : causa === 'POR_FECHA' ? 'Por fecha' : 'Km y fecha'}
            </button>
          ))}
        </div>
      </div>

      {lista.length === 0 ? (
        <div className="fv-list-empty" role="status">
          No se encontraron vehículos que coincidan con los criterios de búsqueda.
        </div>
      ) : (
        <div className="fv-upcoming-grid">
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
        <div className="fv-pagination fv-upcoming-pagination">
          <span>Página <strong>{paginaActual}</strong> de <strong>{totalPaginas}</strong></span>
          <div className="fv-pagination-controls">
            <button type="button" className="fv-button fv-small" disabled={paginaActual === 1}
              onClick={() => setPaginaActual((p) => Math.max(1, p - 1))}>Anterior</button>
            <button type="button" className="fv-button fv-small" disabled={paginaActual === totalPaginas}
              onClick={() => setPaginaActual((p) => Math.min(totalPaginas, p + 1))}>Siguiente</button>
          </div>
        </div>
      )}
    </section>
  );
};
