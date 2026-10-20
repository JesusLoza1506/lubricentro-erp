import React, { useEffect, useState, useMemo } from 'react';
import { ProximoCambioAceiteDTO } from '../../../types/vehiculo';
import { vehiculoService } from '../../../services/vehiculoService';
import {
  Calendar,
  Gauge,
  Phone,
  Droplet,
  ArrowRight,
  Search,
  RefreshCw,
  Filter,
} from 'lucide-react';

interface Props {
  onSeleccionarPlaca: (placa: string) => void;
}

type TipoFiltroCausa = 'TODOS' | 'POR_KM' | 'POR_FECHA' | 'AMBOS';

export const FichaVehicularProximosCambios: React.FC<Props> = ({ onSeleccionarPlaca }) => {
  const [lista, setLista] = useState<ProximoCambioAceiteDTO[]>([]);
  const [cargando, setCargando] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  // Filtros UI y Paginación
  const [busqueda, setBusqueda] = useState<string>('');
  const [filtroCausa, setFiltroCausa] = useState<TipoFiltroCausa>('TODOS');
  const [paginaActual, setPaginaActual] = useState<number>(1);
  const elementosPorPagina = 20;

  const cargarProximosCambios = async () => {
    setCargando(true);
    setError(null);
    try {
      const data = await vehiculoService.obtenerProximosCambiosAceite();
      setLista(data);
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
  }, []);

  // Filtrado reactivo en cliente
  const listaFiltrada = useMemo(() => {
    return lista.filter((item) => {
      const coincideBusqueda =
        item.placa.toLowerCase().includes(busqueda.toLowerCase()) ||
        item.nombre_razon_social.toLowerCase().includes(busqueda.toLowerCase());

      const coincideCausa = filtroCausa === 'TODOS' || item.causa === filtroCausa;

      return coincideBusqueda && coincideCausa;
    });
  }, [lista, busqueda, filtroCausa]);

  // Paginación
  const totalPaginas = Math.ceil(listaFiltrada.length / elementosPorPagina) || 1;
  const listaPaginada = useMemo(() => {
    const inicio = (paginaActual - 1) * elementosPorPagina;
    return listaFiltrada.slice(inicio, inicio + elementosPorPagina);
  }, [listaFiltrada, paginaActual]);

  useEffect(() => {
    setPaginaActual(1);
  }, [busqueda, filtroCausa]);

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
    <section className="fv-upcoming" style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
      {/* HEADER DE LA SECCIÓN */}
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
            Próximos Cambios de Aceite ({listaFiltrada.length})
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

      {/* BARRA DE FILTROS Y BÚSQUEDA */}
      <div className="fv-upcoming-filters"
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
        {/* BUSCADOR */}
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
            onChange={(e) => setBusqueda(e.target.value)}
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

        {/* BOTONES DE FILTRO CAUSA */}
        <div className="fv-filter-buttons" style={{ display: 'flex', gap: '6px', alignItems: 'center' }}>
          <Filter size={14} color="#94A3B8" />
          {(['TODOS', 'POR_KM', 'POR_FECHA', 'AMBOS'] as TipoFiltroCausa[]).map((causa) => (
            <button
              key={causa}
              onClick={() => setFiltroCausa(causa)}
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

      {/* GRILLA DE TARJETAS */}
      {listaPaginada.length === 0 ? (
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
          {listaPaginada.map((item) => (
            <div
              className="fv-upcoming-card"
              key={`${item.id_vehiculo}-${item.id_ot}`}
              style={{
                padding: '16px',
                backgroundColor: 'rgba(15, 23, 42, 0.6)',
                border: '1px solid #334155',
                borderRadius: '14px',
                display: 'flex',
                flexDirection: 'column',
                gap: '10px',
              }}
            >
              <div
                style={{
                  display: 'flex',
                  justifyContent: 'space-between',
                  alignItems: 'flex-start',
                  flexWrap: 'wrap',
                  gap: '8px',
                }}
              >
                <div>
                  <span style={{ fontSize: '18px', fontWeight: '700', color: '#F8FAFC' }}>
                    {item.placa}
                  </span>
                  <p
                    style={{
                      margin: '2px 0 0 0',
                      fontSize: '13px',
                      color: '#94A3B8',
                      fontWeight: '600',
                    }}
                  >
                    {item.nombre_razon_social}
                  </p>
                </div>
                <span
                  style={{
                    fontSize: '11px',
                    fontWeight: '700',
                    padding: '3px 8px',
                    borderRadius: '8px',
                    backgroundColor:
                      item.causa === 'AMBOS' ? 'rgba(239, 68, 68, 0.2)' : 'rgba(251, 191, 36, 0.2)',
                    color: item.causa === 'AMBOS' ? '#EF4444' : '#FBBF24',
                    border:
                      item.causa === 'AMBOS'
                        ? '1px solid rgba(239, 68, 68, 0.4)'
                        : '1px solid rgba(251, 191, 36, 0.4)',
                  }}
                >
                  {item.causa === 'POR_KM'
                    ? 'VENCIMIENTO KM'
                    : item.causa === 'POR_FECHA'
                      ? 'VENCIMIENTO FECHA'
                      : 'KM Y FECHA CRÍTICOS'}
                </span>
              </div>

              <div
                style={{
                  padding: '10px 12px',
                  backgroundColor: 'rgba(30, 41, 59, 0.5)',
                  borderRadius: '10px',
                  display: 'flex',
                  flexDirection: 'column',
                  gap: '6px',
                  fontSize: '12px',
                }}
              >
                <div
                  style={{ display: 'flex', alignItems: 'center', flexWrap: 'wrap', gap: '6px', color: '#CBD5E1' }}
                >
                  <Gauge size={14} color="#94A3B8" />
                  <span>
                    Km Actual: <strong>{(item.kilometraje_actual ?? 0).toLocaleString()} km</strong>
                  </span>
                  {item.km_proximo !== null && item.km_proximo !== undefined && (
                    <span style={{ color: '#94A3B8' }}>
                      (Próx: {item.km_proximo.toLocaleString()} km)
                    </span>
                  )}
                </div>

                <div
                  style={{ display: 'flex', alignItems: 'center', flexWrap: 'wrap', gap: '6px', color: '#CBD5E1' }}
                >
                  <Calendar size={14} color="#94A3B8" />
                  <span>
                    Último Servicio:{' '}
                    {item.fecha_servicio
                      ? new Date(item.fecha_servicio).toLocaleDateString('es-PE')
                      : 'N/A'}
                  </span>
                </div>

                {item.tipo_aceite && (
                  <div
                    style={{ display: 'flex', alignItems: 'center', flexWrap: 'wrap', gap: '6px', color: '#CBD5E1' }}
                  >
                    <Droplet size={14} color="#94A3B8" />
                    <span>
                      Aceite: {item.tipo_aceite} ({item.intervalo_aceite_km || 0} km)
                    </span>
                  </div>
                )}

                {item.telefono && (
                  <div
                    style={{ display: 'flex', alignItems: 'center', flexWrap: 'wrap', gap: '6px', color: '#CBD5E1' }}
                  >
                    <Phone size={14} />
                    <span>Teléfono: {item.telefono}</span>
                  </div>
                )}
              </div>

              <button
                onClick={() => onSeleccionarPlaca(item.placa)}
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'center',
                  gap: '6px',
                  width: '100%',
                  padding: '8px 12px',
                  backgroundColor: '#1B2A40',
                  color: '#BDD5F4',
                  border: '1px solid #35465D',
                  borderRadius: '8px',
                  fontSize: '13px',
                  fontWeight: '700',
                  cursor: 'pointer',
                }}
              >
                Ver Ficha Vehicular <ArrowRight size={14} />
              </button>
            </div>
          ))}
        </div>
      )}

      {/* CONTROLES DE PAGINACIÓN */}
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
