import { useEffect, useRef, type ReactNode } from 'react';
import { useCapturaPos } from '../CapturasPos';
import { X } from 'lucide-react';
export function ModalPos({
  titulo,
  onCerrar,
  ocupado,
  children,
  ancho = false,
  captura = false,
}: {
  titulo: string;
  onCerrar: () => void;
  ocupado?: boolean;
  children: ReactNode;
  ancho?: boolean;
  captura?: boolean;
}) {
  const marcar = useCapturaPos();
  const ref = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const anterior = document.activeElement as HTMLElement | null;
    ref.current?.focus();
    return () => anterior?.focus();
  }, []);
  return (
    <div
      className="gloria-theme pos-overlay"
      onChangeCapture={() => {
        if (captura) marcar(true);
      }}
      onKeyDown={(e) => {
        if (e.key === 'Escape' && !ocupado) {
          e.stopPropagation();
          onCerrar();
        }
        if (e.key === 'Tab') {
          const els = ref.current?.querySelectorAll<HTMLElement>(
            'button:not(:disabled),input:not(:disabled),select:not(:disabled),textarea:not(:disabled),[tabindex="0"]'
          );
          if (!els?.length) return;
          const primero = els[0],
            ultimo = els[els.length - 1];
          if (e.shiftKey && document.activeElement === primero) {
            e.preventDefault();
            ultimo.focus();
          } else if (!e.shiftKey && document.activeElement === ultimo) {
            e.preventDefault();
            primero.focus();
          }
        }
      }}
    >
      <div
        ref={ref}
        tabIndex={-1}
        role="dialog"
        aria-modal="true"
        aria-label={titulo}
        className={`pos-modal ${ancho ? 'pos-modal-wide' : ''}`}
      >
        <header>
          <div>
            <span className="pos-eyebrow">PUNTO DE VENTA</span>
            <h2>{titulo}</h2>
          </div>
          <button
            type="button"
            className="pos-icon"
            disabled={ocupado}
            aria-label="Cerrar ventana"
            onClick={onCerrar}
          >
            <X size={20} />
          </button>
        </header>
        <div className="pos-modal-body">{children}</div>
      </div>
    </div>
  );
}
