import { Plus, ShoppingCart, Package } from 'lucide-react';
interface InventarioHeaderProps {
  esAdmin: boolean;
  onAbrirModalCrear: () => void;
  onAbrirModalCompra?: () => void;
  seccion?: 'PRODUCTOS' | 'SERVICIOS' | 'ENTRADAS';
  onNuevoServicio?: () => void;
}
export function InventarioHeader({
  esAdmin,
  onAbrirModalCrear,
  onAbrirModalCompra,
  seccion = 'PRODUCTOS',
  onNuevoServicio,
}: InventarioHeaderProps) {
  return (
    <header className="inv-header gt-stage">
      <div className="inv-heading">
        <span className="inv-heading-icon">
          <Package size={22} aria-hidden="true" />
        </span>
        <div>
          <span className="inv-eyebrow">ALMACÉN Y CATÁLOGOS</span>
          <h2>Inventario y Stock</h2>
          <p>Consulta existencias, registra compras y mantén tus catálogos al día.</p>
        </div>
      </div>
      {esAdmin && (
        <div className="inv-header-actions">
          {seccion === 'PRODUCTOS' && (
            <>
              <button className="inv-button" onClick={onAbrirModalCompra}>
                <ShoppingCart size={16} />
                Registrar Compra
              </button>
              <button className="inv-button inv-primary" onClick={onAbrirModalCrear}>
                <Plus size={17} />
                Nuevo Producto
              </button>
            </>
          )}
          {seccion === 'SERVICIOS' && (
            <button className="inv-button inv-primary" onClick={onNuevoServicio}>
              <Plus size={17} />
              Nuevo Servicio
            </button>
          )}
          {seccion === 'ENTRADAS' && (
            <button className="inv-button inv-primary" onClick={onAbrirModalCompra}>
              <Plus size={17} />
              Registrar Compra
            </button>
          )}
        </div>
      )}
    </header>
  );
}
