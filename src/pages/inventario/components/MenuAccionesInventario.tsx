import { useEffect, useId, useRef, useState } from 'react';
import { createPortal } from 'react-dom';
import { MoreHorizontal } from 'lucide-react';

export interface AccionInventario {
  texto: string;
  titulo?: string;
  ejecutar: () => void;
  peligro?: boolean;
}
export function MenuAccionesInventario({
  nombre,
  acciones,
}: {
  nombre: string;
  acciones: AccionInventario[];
}) {
  const [posicion, setPosicion] = useState<{ top: number; right: number } | null>(null);
  const boton = useRef<HTMLButtonElement>(null);
  const menu = useRef<HTMLDivElement>(null);
  const id = useId();
  useEffect(() => {
    if (!posicion) return;
    const fuera = (e: PointerEvent) => {
      if (!menu.current?.contains(e.target as Node) && !boton.current?.contains(e.target as Node))
        setPosicion(null);
    };
    const cerrar = () => setPosicion(null);
    document.addEventListener('pointerdown', fuera);
    window.addEventListener('resize', cerrar);
    window.addEventListener('scroll', cerrar, true);
    menu.current?.querySelector<HTMLButtonElement>('button')?.focus();
    return () => {
      document.removeEventListener('pointerdown', fuera);
      window.removeEventListener('resize', cerrar);
      window.removeEventListener('scroll', cerrar, true);
    };
  }, [posicion]);
  return (
    <>
      <button
        ref={boton}
        className="inv-icon-button"
        title={`Más acciones: ${nombre}`}
        aria-label={`Más acciones: ${nombre}`}
        aria-expanded={!!posicion}
        aria-haspopup="menu"
        aria-controls={posicion ? id : undefined}
        onClick={() => {
          if (posicion) {
            setPosicion(null);
            return;
          }
          const r = boton.current!.getBoundingClientRect();
          const altura = acciones.length * 40 + 12;
          setPosicion({
            top:
              r.bottom + altura + 12 < window.innerHeight
                ? r.bottom + 6
                : Math.max(8, r.top - altura - 6),
            right: Math.max(8, window.innerWidth - r.right),
          });
        }}
      >
        <MoreHorizontal size={18} />
      </button>
      {posicion &&
        createPortal(
          <div
            id={id}
            ref={menu}
            role="menu"
            aria-label={`Acciones de ${nombre}`}
            className="inv-action-menu"
            style={posicion}
            onKeyDown={(e) => {
              if (e.key === 'Escape') {
                setPosicion(null);
                boton.current?.focus();
              }
              if (['ArrowDown', 'ArrowUp', 'Home', 'End'].includes(e.key)) {
                e.preventDefault();
                const items = Array.from(
                  menu.current!.querySelectorAll<HTMLButtonElement>('button')
                );
                const actual = items.indexOf(document.activeElement as HTMLButtonElement);
                const siguiente =
                  e.key === 'Home'
                    ? 0
                    : e.key === 'End'
                      ? items.length - 1
                      : (actual + (e.key === 'ArrowDown' ? 1 : -1) + items.length) % items.length;
                items[siguiente]?.focus();
              }
              if (e.key === 'Tab') setPosicion(null);
            }}
          >
            {acciones.map((a) => (
              <button
                key={a.texto}
                role="menuitem"
                title={a.titulo}
                className={a.peligro ? 'is-danger' : ''}
                onClick={() => {
                  setPosicion(null);
                  boton.current?.focus();
                  a.ejecutar();
                }}
              >
                {a.texto}
              </button>
            ))}
          </div>,
          document.body
        )}
    </>
  );
}
