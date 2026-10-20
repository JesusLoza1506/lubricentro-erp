import React from 'react';
import { Car, SearchX } from 'lucide-react';
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
      <div className="fv-empty">
        <div className="fv-empty-icon"><Car size={32} /></div>
        <h3>Busca un vehículo por placa</h3>
        <p>Consulta sus datos, propietario e historial de mantenimiento desde el buscador superior.</p>
        <details className="fv-examples">
          <summary>Ejemplos de búsqueda</summary>
          <div>
            <button onClick={() => onEjecutarBusqueda('ABC-123')} style={btnPlacaPruebaStyle}>ABC-123</button>
            <button onClick={() => onEjecutarBusqueda('XYZ-789')} style={btnPlacaPruebaStyle}>XYZ-789</button>
          </div>
        </details>
      </div>
    );
  }
  return (
    <div className="fv-empty fv-empty-not-found">
      <div className="fv-empty-icon"><SearchX size={32} /></div>
      <h3>Vehículo no encontrado</h3>
      <p>No existe registro para la placa <strong>{placaBusqueda}</strong>.</p>
      {puedeCrear && (
        <button onClick={onAbrirCrear} className="fv-button fv-primary">
          Registrar {placaBusqueda} Ahora
        </button>
      )}
    </div>
  );
};
