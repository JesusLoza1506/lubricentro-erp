import { solicitudPendiente, solesAMicro } from '../../../services/operacionInventario';
import React, { useState, useEffect, useRef } from 'react';
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
  const enviando = useRef(false);
  const [incierta, setIncierta] = useState(false);
  const [errorLocal, setErrorLocal] = useState('');
  const [guardando, setGuardando] = useState<boolean>(false);

  useEffect(() => {
    if (producto && abierto) {
      setErrorLocal('');
      setIncierta(false);
      const stockUI =
        producto.unidad_medida === 'ML' ? producto.stock_actual / 1000 : producto.stock_actual;
      setStockFisico(stockUI);
      setStockObservado(stockUI);
      setMotivo('');
      setCostoSoles(0);
      try {
        const pendiente = solicitudPendiente('procesar_conteo_fisico_cmd')?.payload as
          | ConteoFisicoDTO
          | undefined;
        if (pendiente) {
          if (pendiente.id_producto !== producto.id_producto) {
            setErrorLocal(
              `Hay un conteo pendiente del producto ID ${pendiente.id_producto}. Reinténtalo antes de contar otro producto.`
            );
          } else {
            setStockFisico(pendiente.stock_fisico_contado);
            setStockObservado(pendiente.stock_observado ?? stockUI);
            setMotivo(pendiente.motivo);
            setCostoSoles((pendiente.costo_unitario_micro ?? 0) / 1000000);
          }
          setIncierta(true);
        }
      } catch (error) {
        setErrorLocal(String(error));
      }
    }
  }, [producto, abierto]);

  if (!abierto || !producto) return null;

  const esML = producto.unidad_medida === 'ML';
  const stockTeoricoUI = esML ? producto.stock_actual / 1000 : producto.stock_actual;
  const diferenciaUI = stockFisico - stockTeoricoUI;
  const requiereMotivo = diferenciaUI !== 0;

  const handleFormSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (enviando.current) return;
    if (requiereMotivo && motivo.trim().length < 5) return;

    try {
      enviando.current = true;
      setGuardando(true);
      setErrorLocal('');
      const payload: ConteoFisicoDTO = {
        id_producto: producto.id_producto,
        stock_fisico_contado: stockFisico,
        stock_observado: stockObservado,
        costo_unitario_micro:
          diferenciaUI > 0 && stockObservado === 0 && costoSoles > 0
            ? solesAMicro(costoSoles)
            : null,
        motivo: motivo.trim(),
      };
      const pendiente = solicitudPendiente('procesar_conteo_fisico_cmd');
      if (pendiente && (pendiente.payload as ConteoFisicoDTO).id_producto !== producto.id_producto)
        throw new Error('Reabre el conteo del producto indicado para reintentar su solicitud.');
      await onSubmit((pendiente?.payload as ConteoFisicoDTO) ?? payload);
      onClose();
    } catch (error) {
      setErrorLocal(String(error));
      setIncierta(!!solicitudPendiente('procesar_conteo_fisico_cmd'));
    } finally {
      enviando.current = false;
      setGuardando(false);
    }
  };

  return (
    <div
      className="gloria-theme inv-overlay"
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
        className="inv-dialog inv-dialog-compact"
        role="dialog"
        aria-modal="true"
        aria-label="Conteo físico"
        style={{
          width: '500px',
          backgroundColor: '#0F172A',
          border: '1px solid #A855F7',
          borderRadius: '16px',
          padding: '24px',
          boxShadow: '0 20px 40px rgba(168, 85, 247, 0.2)',
        }}
      >
        <header className="inv-dialog-header">
          <div><span className="inv-dialog-kicker">Control de existencias</span>
            <h3>Conteo físico de stock</h3>
            <p>Compara las existencias físicas con el saldo registrado.</p>
          </div>
        </header>

        <div className="inv-counted-product">
          <div><strong>{producto.nombre}</strong><span className="inv-code">{producto.codigo}</span></div>
          <div><span>Stock registrado</span><strong>{stockTeoricoUI} {esML ? 'Litros (L)' : 'UNIDADES'}</strong></div>
        </div>

        <form
          onSubmit={handleFormSubmit}
          style={{ display: 'flex', flexDirection: 'column', gap: '14px' }}
        >
          <fieldset
            disabled={guardando || incierta}
            style={{ border: 0, padding: 0, margin: 0, display: 'contents' }}
          >
            <div>
              <label htmlFor="inv-count-physical"
                style={{
                  display: 'block',
                  fontSize: '12px',
                  color: '#94A3B8',
                  marginBottom: '6px',
                }}
              >
                Stock Real en Almacén / Taller ({esML ? 'Litros' : 'Unidades'}) *
              </label>
              <input id="inv-count-physical"
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
              <div className="inv-stock-difference" data-change={diferenciaUI > 0 ? 'increase' : 'decrease'}>
                <span>
                  Diferencia a Ajustar: {diferenciaUI > 0 ? `+${diferenciaUI}` : diferenciaUI}{' '}
                  {esML ? 'L' : 'UND'} (
                  {diferenciaUI > 0 ? 'Sobrante - Entrada' : 'Faltante - Salida'})
                </span>
              </div>
            )}

            {diferenciaUI > 0 && stockTeoricoUI === 0 && (
              <div>
                <label htmlFor="inv-count-cost"
                  style={{
                    display: 'block',
                    fontSize: '12px',
                    color: '#94A3B8',
                    marginBottom: '6px',
                  }}
                >
                  Costo por unidad o litro (S/), si no existe costo previo
                </label>
                <input id="inv-count-cost"
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
              <label htmlFor="inv-count-reason"
                style={{
                  display: 'block',
                  fontSize: '12px',
                  color: '#94A3B8',
                  marginBottom: '6px',
                }}
              >
                Motivo de la Reconciliación{' '}
                {requiereMotivo ? '(mínimo 5 caracteres) *' : '(Opcional sin diferencias)'}
              </label>
              <input id="inv-count-reason"
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
          </fieldset>
          {errorLocal && (
            <p role="alert" className="inv-form-error">
              {errorLocal}
            </p>
          )}
          {incierta && (
            <p className="gt-notice">
              Reintenta la solicitud conservada para confirmar su resultado.
            </p>
          )}
          <footer className="inv-form-footer">
            <button type="button" className="inv-button" onClick={onClose} disabled={guardando}>
              {incierta ? 'Conservar y cerrar' : 'Cancelar'}
            </button>
            <button type="submit" className="inv-button inv-primary" aria-busy={guardando}
              disabled={guardando || (requiereMotivo && motivo.trim().length < 5)}>
              {guardando
                ? 'Procesando...'
                : incierta
                  ? 'Reintentar conteo pendiente'
                  : 'Aplicar Reconciliación'}
            </button>
          </footer>
        </form>
      </div>
    </div>
  );
};
