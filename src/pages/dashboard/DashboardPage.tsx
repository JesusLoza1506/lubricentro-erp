import React, { useEffect, useState } from 'react';
import {
  ArrowRight,
  Car,
  ClipboardList,
  Clock,
  DatabaseBackup,
  Package,
  Receipt,
  ShieldCheck,
  ShoppingCart,
  Wallet,
  Wrench,
} from 'lucide-react';
import { useAuth } from '../../context/AuthContext';
import { obtenerDatosDashboard } from '../../services/dashboardService';
import type { DashboardDTO } from '../../services/dashboardService';
import { backup, fechaBackup, type EstadoBackup } from '../backup/backupService';
import '../../styles/gloriaTaller.css';
import './dashboardVisual.css';

interface DashboardPageProps {
  setModuloActual: (modulo: string) => void;
}

const accesos = [
  {
    permiso: 'OT',
    destino: 'taller',
    icono: Wrench,
    titulo: 'Órdenes de Trabajo',
    descripcion: 'Revisa las zanjas y las órdenes del taller.',
  },
  {
    permiso: 'POS',
    destino: 'pos',
    icono: ShoppingCart,
    titulo: 'Punto de Venta',
    descripcion: 'Registra ventas y consulta comprobantes.',
  },
  {
    permiso: 'FICHA_VEHICULAR',
    destino: 'ficha_vehicular',
    icono: Car,
    titulo: 'Ficha Vehicular',
    descripcion: 'Busca por placa y consulta el historial del vehículo.',
  },
  {
    permiso: 'INVENTARIO',
    destino: 'inventario',
    icono: Package,
    titulo: 'Inventario y Stock',
    descripcion: 'Consulta productos y existencias.',
  },
  {
    permiso: 'CAJA',
    destino: 'caja',
    icono: Wallet,
    titulo: 'Caja Chica',
    descripcion: 'Abre tu turno, consulta movimientos y guarda el arqueo.',
  },
  {
    permiso: 'BACKUP',
    destino: 'backup',
    icono: DatabaseBackup,
    titulo: 'Backup y Copias',
    descripcion: 'Genera, verifica y restaura respaldos.',
  },
  {
    permiso: 'ADMIN',
    destino: 'administracion',
    icono: ShieldCheck,
    titulo: 'Administración',
    descripcion: 'Gestiona usuarios, empresa, facturación y taller.',
  },
] as const;

interface IndicadorProps {
  titulo: string;
  icono: typeof Wallet;
  valor: string;
  detalle: string;
  tono?: 'aviso' | 'exito';
}

function Indicador({ titulo, icono: Icono, valor, detalle, tono }: IndicadorProps) {
  return (
    <div className="inicio-indicador" data-tono={tono}>
      <dt className="inicio-indicador-label">
        {titulo}
        <Icono size={17} aria-hidden="true" />
      </dt>
      <dd className="inicio-indicador-valor">{valor}</dd>
      <dd className="inicio-indicador-detalle">{detalle}</dd>
    </div>
  );
}

export const DashboardPage: React.FC<DashboardPageProps> = ({ setModuloActual }) => {
  const { usuario, tienePermiso } = useAuth();
  const [metrics, setMetrics] = useState<DashboardDTO | null>(null);
  const [cargandoMetrics, setCargandoMetrics] = useState(false);
  const [estadoBackup, setEstadoBackup] = useState<EstadoBackup | null>(null);
  const [cargandoBackup, setCargandoBackup] = useState(true);
  const puedeVerBackup = usuario?.rol === 'ADMINISTRADOR' && tienePermiso('BACKUP', 'ver');
  const sesionBackup = usuario?.session_id;

  useEffect(() => {
    if (!puedeVerBackup || !sesionBackup) return;
    let vigente = true;
    setEstadoBackup(null);
    setCargandoBackup(true);
    void backup<EstadoBackup>('estado')
      .then((datos) => { if (vigente) setEstadoBackup(datos); })
      .catch(() => { if (vigente) setEstadoBackup(null); })
      .finally(() => { if (vigente) setCargandoBackup(false); });
    return () => { vigente = false; };
  }, [puedeVerBackup, sesionBackup]);

  const puedeVerKPIsGlobales = tienePermiso('BACKUP', 'ver') && tienePermiso('COLA_SUNAT', 'ver');
  const esOperativoTallerOnly = tienePermiso('OT', 'ver') && !tienePermiso('POS', 'ver');

  useEffect(() => {
    if (usuario) {
      setCargandoMetrics(true);
      obtenerDatosDashboard(usuario.id_usuario)
        .then((data) => setMetrics(data))
        .catch((err) => {
          setMetrics(null);
          console.error('Error al cargar métricas del dashboard:', err);
        })
        .finally(() => setCargandoMetrics(false));
    }
  }, [usuario]);

  const visibles = accesos.filter(({ permiso }) => {
    if (permiso === 'BACKUP') return puedeVerBackup;
    if (permiso === 'ADMIN') return usuario?.rol === 'ADMINISTRADOR' && tienePermiso('ADMIN', 'ver');
    return tienePermiso(permiso, 'ver');
  });
  const respaldo = cargandoBackup ? 'Consultando respaldo…'
    : !estadoBackup ? 'Estado no disponible'
    : estadoBackup.recuperacion.pendiente ? 'Recuperación pendiente'
    : !estadoBackup.configurada ? 'Protección sin configurar'
    : !estadoBackup.ultima ? 'Sin copia válida'
    : estadoBackup.vencida ? 'Copia de más de 24 h'
    : 'Copia al día';
  const tonoRespaldo = cargandoBackup || !estadoBackup ? undefined
    : !estadoBackup.configurada || !estadoBackup.ultima || estadoBackup.vencida || estadoBackup.recuperacion.pendiente
      ? 'aviso' : 'exito';
  const nombre = usuario?.nombre_completo.trim().split(/\s+/)[0];
  const estadoCaja = metrics?.estado_caja?.trim();
  const cajaNormalizada = estadoCaja?.toLowerCase();
  const ots = metrics?.ots_activas ?? 0;
  const stockCritico = metrics?.productos_stock_critico ?? 0;
  const valor = (texto: string) => (cargandoMetrics || !metrics ? '—' : texto);
  const detalle = (texto: string) =>
    cargandoMetrics ? 'Cargando…' : metrics ? texto : 'Dato no disponible';

  return (
    <div className="gloria-theme inicio-page">
      <div className="inicio-content gt-stage">
        <header className="inicio-bienvenida">
          <div className="inicio-bienvenida-texto">
            <p className="inicio-kicker">Resumen del taller</p>
            <h2>{nombre ? `Hola, ${nombre}` : 'Inicio'}</h2>
            <p className="inicio-descripcion">
              {puedeVerKPIsGlobales
                ? 'Consulta la actividad del día y entra a tus módulos de trabajo.'
                : esOperativoTallerOnly
                  ? 'Consulta tus órdenes activas y entra al taller.'
                  : 'Entra a tus módulos de trabajo desde aquí.'}
            </p>
          </div>
          {usuario?.rol && <span className="inicio-rol">{usuario.rol}</span>}
        </header>

        {puedeVerKPIsGlobales && (
          <section className="inicio-seccion" aria-labelledby="inicio-hoy-titulo">
            <h3 id="inicio-hoy-titulo">Resumen de hoy</h3>
            <dl className="inicio-indicadores" aria-busy={cargandoMetrics}>
              <Indicador
                titulo="Ventas del día"
                icono={Receipt}
                valor={valor(`S/ ${(metrics?.ventas_del_dia ?? 0).toFixed(2)}`)}
                detalle={detalle(
                  metrics?.comprobantes_emitidos === 1
                    ? '1 comprobante emitido'
                    : `${metrics?.comprobantes_emitidos ?? 0} comprobantes emitidos`
                )}
              />
              <Indicador
                titulo="Órdenes activas"
                icono={ClipboardList}
                valor={valor(String(ots))}
                detalle={detalle('Órdenes en taller')}
              />
              <Indicador
                titulo="Stock crítico"
                icono={Package}
                valor={valor(String(stockCritico))}
                detalle={detalle(
                  stockCritico === 1 ? '1 producto en umbral mínimo' : 'Productos en umbral mínimo'
                )}
                tono={!cargandoMetrics && stockCritico > 0 ? 'aviso' : undefined}
              />
              <Indicador
                titulo="Estado de caja"
                icono={Wallet}
                valor={valor(estadoCaja || '—')}
                detalle={detalle(
                  cajaNormalizada === 'abierta'
                    ? 'Turno en curso'
                    : cajaNormalizada === 'cerrada'
                      ? 'Sin turno abierto'
                      : estadoCaja
                        ? 'Estado registrado'
                        : 'Dato no disponible'
                )}
                tono={!cargandoMetrics && cajaNormalizada === 'abierta' ? 'exito' : undefined}
              />
            </dl>
          </section>
        )}

        {esOperativoTallerOnly && (
          <section className="inicio-seccion" aria-labelledby="inicio-ordenes-titulo">
            <h3 id="inicio-ordenes-titulo">Mis órdenes activas</h3>
            <div className="inicio-ordenes" aria-busy={cargandoMetrics}>
              <span className="inicio-icono">
                <Clock size={20} aria-hidden="true" />
              </span>
              <div>
                <p className="inicio-ordenes-cifra">{valor(String(ots))}</p>
                <p className="inicio-ordenes-texto">
                  {detalle(
                    ots > 0 ? 'Órdenes activas asignadas' : 'No tienes órdenes activas asignadas.'
                  )}
                </p>
              </div>
            </div>
          </section>
        )}

        <nav className="inicio-seccion" aria-labelledby="inicio-accesos-titulo">
          <h3 id="inicio-accesos-titulo">Accesos rápidos</h3>
          {visibles.length > 0 ? (
            <div className="inicio-accesos">
              {visibles.map(({ destino, icono: Icono, titulo, descripcion }) => (
                <button
                  key={destino}
                  type="button"
                  className="inicio-acceso"
                  aria-labelledby={`inicio-acceso-${destino}`}
                  aria-describedby={`inicio-descripcion-${destino}`}
                  onClick={() => setModuloActual(destino)}
                >
                  <span className="inicio-icono">
                    <Icono size={20} aria-hidden="true" />
                  </span>
                  <span className="inicio-acceso-texto">
                    <span id={`inicio-acceso-${destino}`} className="inicio-acceso-titulo">
                      {titulo}
                    </span>
                    <span
                      id={`inicio-descripcion-${destino}`}
                      className="inicio-acceso-descripcion"
                    >
                      {destino === 'taller' && esOperativoTallerOnly
                        ? 'Revisa tus servicios asignados y tu zanja.'
                        : destino === 'backup' && estadoBackup?.ultima
                          ? `Última copia válida: ${fechaBackup(estadoBackup.ultima)} (Lima)`
                          : descripcion}
                      {destino === 'backup' && (
                        <span className="inicio-acceso-estados">
                          <span className="inicio-acceso-estado" data-tono={tonoRespaldo}>{respaldo}</span>
                          {!cargandoBackup && estadoBackup?.ultima && estadoBackup.vencida &&
                            respaldo !== 'Copia de más de 24 h' && (
                              <span className="inicio-acceso-estado" data-tono="aviso">Copia de más de 24 h</span>
                            )}
                        </span>
                      )}
                    </span>
                  </span>
                  <ArrowRight className="inicio-acceso-flecha" size={19} aria-hidden="true" />
                </button>
              ))}
            </div>
          ) : (
            <p className="inicio-sin-accesos">No hay accesos rápidos disponibles para tu cuenta.</p>
          )}
        </nav>
      </div>
    </div>
  );
};
