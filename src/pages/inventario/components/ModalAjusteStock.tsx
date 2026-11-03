import React, { useState, useEffect } from 'react';
import { Producto, ConteoFisicoDTO } from '../../../types/inventario';
import { modalInputStyle } from '../inventarioStyles';

interface ModalAjusteStockProps {
  abierto: boolean;
  producto: Producto | null;
  onClose: () => void;
  onSubmit: (payload: ConteoFisicoDTO) => Promise<void>;
}

export const ModalAjusteStock: React.FC<ModalAjusteStockProps> = ({
  abierto,
  producto,
  onClose,
  onSubmit,
}) => {
  const [stockFisico, setStockFisico] = useState<number>(0);
  const [stockObservado, setStockObservado] = useState<number>(0);
  const [costoSoles, setCostoSoles] = useState<number>(0);
  const [motivo, setMotivo] = useState<string>('');
  const [guardando, setGuardando] = useState<boolean>(false);

  useEffect(() => {
    if (producto) {
      const stockUI =
        producto.unidad_medida === 'ML' ? producto.stock_actual / 1000 : producto.stock_actual;
      setStockFisico(stockUI);
      setStockObservado(stockUI);
      setMotivo('');
      setCostoSoles(0);
    }
  }, [producto]);

  if (!abierto || !producto) return null;

  const esML = producto.unidad_medida === 'ML';
  const stockTeoricoUI = esML ? producto.stock_actual / 1000 : producto.stock_actual;
  const diferenciaUI = stockFisico - stockTeoricoUI;
  const requiereMotivo = diferenciaUI !== 0;

  const handleFormSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (requiereMotivo && motivo.trim().length < 5) return;

    try {
      setGuardando(true);
      const payload: ConteoFisicoDTO = {
        id_producto: producto.id_producto,
        stock_fisico_contado: stockFisico,
        stock_observado: stockObservado,
        costo_unitario_micro:
          diferenciaUI > 0 && stockTeoricoUI === 0 ? Math.round(costoSoles * 1_000_000) : null,
        motivo: motivo.trim(),
      };
      await onSubmit(payload);
      onClose();
    } catch {
      // Error manejado en el componente superior
    } finally {
      setGuardando(false);
    }
  };

  return (
    <div
      style={{
        position: 'fixed',
        top: 0,
        left: 0,
        width: '100vw',
        height: '100vh',
        backgroundColor: 'rgba(0,0,0,0.75)',
        backdropFilter: 'blur(4px)',
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        zIndex: 1100,
      }}
    >
      <div
        style={{
          width: '500px',
          backgroundColor: '#0F172A',
          border: '1px solid #A855F7',
          borderRadius: '16px',
          padding: '24px',
          boxShadow: '0 20px 40px rgba(168, 85, 247, 0.2)',
        }}
      >
        <h3 style={{ fontSize: '18px', fontWeight: '800', color: '#F8FAFC', margin: '0 0 16px 0' }}>
          Conteo Físico Reconciliado (Auditoría)
        </h3>

        <div
          style={{
            backgroundColor: '#1E293B',
            padding: '12px 16px',
            borderRadius: '10px',
            marginBottom: '16px',
          }}
        >
          <p style={{ color: '#CBD5E1', fontSize: '13px', margin: '0 0 6px 0' }}>
            Producto: <strong style={{ color: '#F8FAFC' }}>{producto.nombre}</strong> (
            {producto.codigo})
          </p>
          <p style={{ color: '#94A3B8', fontSize: '13px', margin: 0 }}>
            Stock Registrado en Sistema:{' '}
            <strong style={{ color: '#38BDF8' }}>
              {stockTeoricoUI} {esML ? 'Litros (L)' : 'UNIDADES'}
            </strong>
          </p>
        </div>

        <form
          onSubmit={handleFormSubmit}
          style={{ display: 'flex', flexDirection: 'column', gap: '14px' }}
        >
          <div>
            <label
              style={{ display: 'block', fontSize: '12px', color: '#94A3B8', marginBottom: '6px' }}
            >
              Stock Real en Almacén / Taller ({esML ? 'Litros' : 'Unidades'}) *
            </label>
            <input
              required
              type="number"
              step={esML ? '0.001' : '1'}
              min="0"
              value={stockFisico}
              onChange={(e) => setStockFisico(parseFloat(e.target.value) || 0)}
              style={modalInputStyle}
            />
          </div>

          {diferenciaUI !== 0 && (
            <div
              style={{
                backgroundColor:
                  diferenciaUI > 0 ? 'rgba(16, 185, 129, 0.1)' : 'rgba(239, 68, 68, 0.1)',
                border: `1px solid ${diferenciaUI > 0 ? '#10B981' : '#EF4444'}`,
                padding: '12px',
                borderRadius: '8px',
              }}
            >
              <span
                style={{
                  fontSize: '13px',
                  fontWeight: '700',
                  color: diferenciaUI > 0 ? '#34D399' : '#F87171',
                }}
              >
                Diferencia a Ajustar: {diferenciaUI > 0 ? `+${diferenciaUI}` : diferenciaUI}{' '}
                {esML ? 'L' : 'UND'} (
                {diferenciaUI > 0 ? 'Sobrante - Entrada' : 'Faltante - Salida'})
              </span>
            </div>
          )}

          {diferenciaUI > 0 && stockTeoricoUI === 0 && (
            <div>
              <label
                style={{
                  display: 'block',
                  fontSize: '12px',
                  color: '#94A3B8',
                  marginBottom: '6px',
                }}
              >
                Costo Unitario en Soles (S/) *
              </label>
              <input
                required
                type="number"
                step="0.01"
                min="0.01"
                value={costoSoles}
                onChange={(e) => setCostoSoles(parseFloat(e.target.value) || 0)}
                style={modalInputStyle}
              />
            </div>
          )}

          <div>
            <label
              style={{ display: 'block', fontSize: '12px', color: '#94A3B8', marginBottom: '6px' }}
            >
              Motivo de la Reconciliación{' '}
              {requiereMotivo ? '(mínimo 5 caracteres) *' : '(Opcional sin diferencias)'}
            </label>
            <input
              required={requiereMotivo}
              minLength={requiereMotivo ? 5 : undefined}
              maxLength={250}
              type="text"
              placeholder={
                requiereMotivo
                  ? 'Ej. Conteo físico mensual de almacén con inventario real'
                  : 'Sin diferencias'
              }
              value={motivo}
              onChange={(e) => setMotivo(e.target.value)}
              style={modalInputStyle}
            />
          </div>

          <div
            style={{ display: 'flex', justifyContent: 'flex-end', gap: '12px', marginTop: '8px' }}
          >
            <button
              type="button"
              onClick={onClose}
              disabled={guardando}
              style={{
                padding: '10px 16px',
                backgroundColor: 'transparent',
                border: '1px solid #334155',
                color: '#CBD5E1',
                borderRadius: '8px',
                cursor: 'pointer',
              }}
            >
              Cancelar
            </button>
            <button
              type="submit"
              disabled={guardando || (requiereMotivo && motivo.trim().length < 5)}
              style={{
                padding: '10px 20px',
                backgroundColor: '#A855F7',
                color: '#FFFFFF',
                border: 'none',
                borderRadius: '8px',
                fontWeight: '700',
                cursor: guardando ? 'wait' : 'pointer',
                opacity: requiereMotivo && motivo.trim().length < 5 ? 0.6 : 1,
              }}
            >
              {guardando ? 'Procesando...' : 'Aplicar Reconciliación'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};
