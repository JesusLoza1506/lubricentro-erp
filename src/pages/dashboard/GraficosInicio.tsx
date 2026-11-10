import { useState, type CSSProperties } from 'react';
import type { PuntoDashboard } from '../../services/dashboardService';

const soles = new Intl.NumberFormat('es-PE', {
  minimumFractionDigits: 2,
  maximumFractionDigits: 2,
});
export const monedaInicio = (cent: number) =>
  `${cent < 0 ? '−' : ''}S/ ${soles.format(Math.abs(cent) / 100)}`;

function fechaUTC(iso: string) {
  return new Date(`${iso.slice(0, 10)}T12:00:00Z`);
}
export function fechaInicio(iso: string) {
  return new Intl.DateTimeFormat('es-PE', {
    weekday: 'long',
    day: 'numeric',
    month: 'long',
    timeZone: 'UTC',
  }).format(fechaUTC(iso));
}
const fechaCorta = (iso: string) => `${iso.slice(8, 10)}/${iso.slice(5, 7)}`;

export function GraficoColumnas({
  titulo,
  subtitulo,
  puntos,
  formato,
  hoy,
  horas = false,
  dinero = false,
}: {
  titulo: string;
  subtitulo: string;
  puntos: PuntoDashboard[];
  formato: (valor: number) => string;
  hoy: string;
  horas?: boolean;
  dinero?: boolean;
}) {
  const [elegido, setElegido] = useState<string | null>(null);
  const ultimo = puntos[puntos.length - 1];
  const activo = puntos.find((p) => p.periodo === elegido) ?? ultimo;
  const conFechas = horas && puntos.some((p) => p.periodo.slice(0, 10) !== hoy);
  const etiqueta = (p: PuntoDashboard) =>
    horas
      ? `${conFechas ? fechaCorta(p.periodo) + ' · ' : ''}${p.periodo.slice(11, 13)}:00`
      : p.periodo === hoy
        ? 'Hoy'
        : fechaCorta(p.periodo);
  const valores = puntos.map((p) => p.valor / (dinero ? 100 : 1));
  const paso = dinero ? 250 : 1;
  const techo = Math.max(paso, Math.ceil(Math.max(0, ...valores) / paso) * paso);
  const piso = Math.min(0, Math.floor(Math.min(0, ...valores) / paso) * paso);
  const y = (valor: number) => ((valor - piso) / (techo - piso)) * 100;
  const intermedio = Math.floor(techo / 2);
  const niveles = Array.from(new Set([techo, intermedio, 0, piso]));
  const salto = Math.max(1, Math.ceil(Math.min(puntos.length, 24) / (conFechas ? 4 : 7)));
  const desplazable = puntos.length > 24;
  return (
    <section className="inicio-grafico" aria-label={titulo}>
      <h4>{titulo}</h4>
      <p className="inicio-subtitulo">{subtitulo}</p>
      {puntos.length === 0 ? (
        <p className="inicio-grafico-vacio">Sin cobros en este turno.</p>
      ) : (
        <figure>
          <div className="inicio-chart-scroll" data-desplazable={desplazable}>
            <div
              className="inicio-chart-area"
              style={desplazable ? { minWidth: puntos.length * 15 + 42 } : undefined}
            >
              <div
                className="inicio-chart"
                style={{ '--columnas': puntos.length } as CSSProperties}
              >
                {niveles.map((nivel) => (
                  <div
                    key={nivel}
                    className={`inicio-gridline${nivel === 0 ? ' inicio-gridline-cero' : ''}`}
                    style={{ bottom: `${y(nivel)}%` }}
                    aria-hidden="true"
                  >
                    <span>{nivel.toLocaleString('es-PE')}</span>
                  </div>
                ))}
                <div className="inicio-barras">
                  {puntos.map((p, i) => (
                    <button
                      key={p.periodo}
                      type="button"
                      aria-label={`${etiqueta(p)}: ${formato(p.valor)}`}
                      aria-pressed={p === activo}
                      onClick={() => setElegido(p.periodo)}
                    >
                      <span
                        aria-hidden="true"
                        data-negativo={p.valor < 0}
                        style={{
                          bottom: `${y(Math.min(0, valores[i]))}%`,
                          height: `${(Math.abs(valores[i]) / (techo - piso)) * 100}%`,
                        }}
                      />
                    </button>
                  ))}
                </div>
              </div>
              <div
                className="inicio-eje"
                aria-hidden="true"
                style={{ '--columnas': puntos.length } as CSSProperties}
              >
                {puntos.map((p, i) => (
                  <span key={p.periodo} data-hoy={!horas && p.periodo === hoy}>
                    {i % salto === 0 ||
                    (i === puntos.length - 1 && (puntos.length < 9 || i % salto > 1)) ? (
                      horas ? (
                        <>
                          {conFechas && <small>{fechaCorta(p.periodo)}</small>}
                          {p.periodo.slice(11, 13)}
                        </>
                      ) : (
                        etiqueta(p)
                      )
                    ) : (
                      ''
                    )}
                  </span>
                ))}
              </div>
            </div>
          </div>
          <figcaption aria-live="polite">
            {activo && (
              <>
                {etiqueta(activo)} · <strong>{formato(activo.valor)}</strong>
              </>
            )}
          </figcaption>
        </figure>
      )}
    </section>
  );
}

export type Grupo = { nombre: string; valor: number; color?: 'azul' | 'verde' | 'ambar' };

export function GraficoGrupos({
  titulo,
  subtitulo,
  grupos,
  segmentado,
  vacio,
}: {
  titulo: string;
  subtitulo: string;
  grupos: Grupo[];
  segmentado?: boolean;
  vacio: string;
}) {
  const maximo = Math.max(1, ...grupos.map((g) => g.valor));
  return (
    <section className="inicio-grafico" aria-label={titulo}>
      <h4>{titulo}</h4>
      <p className="inicio-subtitulo">{subtitulo}</p>
      {grupos.every((g) => g.valor === 0) ? (
        <p className="inicio-grafico-vacio">{vacio}</p>
      ) : (
        <figure className="inicio-grupos">
          {segmentado && (
            <div className="inicio-segmentos" aria-hidden="true">
              {grupos
                .filter((g) => g.valor > 0)
                .map((g) => (
                  <span key={g.nombre} data-color={g.color} style={{ flexGrow: g.valor }} />
                ))}
            </div>
          )}
          <dl>
            {grupos.map((g) => (
              <div key={g.nombre} className="inicio-grupo">
                <dt>
                  {segmentado && <span data-color={g.color} aria-hidden="true" />}
                  {g.nombre}
                </dt>
                <dd>{g.valor}</dd>
                {!segmentado && (
                  <div className="inicio-pista" aria-hidden="true">
                    <span data-color={g.color} style={{ width: `${(g.valor / maximo) * 100}%` }} />
                  </div>
                )}
              </div>
            ))}
          </dl>
        </figure>
      )}
    </section>
  );
}
