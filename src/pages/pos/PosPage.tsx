import { useCallback, useEffect, useRef, useState } from 'react';
import {
  RefreshCw,
  ShoppingCart,
  ClipboardList,
  ReceiptText,
  AlertTriangle,
  UserRound,
  Wallet,
} from 'lucide-react';
import { useAuth } from '../../context/AuthContext';
import { ToastNotification } from '../../components/ToastNotification';
import type { CredencialesPos, InicioPos, ResultadoPos } from '../../types/pos';
import {
  documentoPos,
  ejecutarPos,
  leerPendientePos,
  mensajePos,
  posApi,
  type SolicitudPos,
} from '../../services/posService';
import { useVentaPos } from './hooks/useVentaPos';
import { CatalogoPos } from './components/CatalogoPos';
import { CarritoPos } from './components/CarritoPos';
import { ReceptorPos } from './components/ReceptorPos';
import { CobroPos } from './components/CobroPos';
import { CargarOtPos } from './components/CargarOtPos';
import { VentasPos } from './components/VentasPos';
import { DetalleVentaPos } from './components/DetalleVentaPos';
import { AutorizacionPos } from './components/AutorizacionPos';
import { soles } from './posUtils';
import { CapturasPosContext } from './CapturasPos';
import './posVisual.css';

export function PosPage({
  otInicial,
  onConsumirOt,
  ventaInicial,
  onConsumirVenta,
  onIrCaja,
}: {
  otInicial?: number | null;
  onConsumirOt?: () => void;
  ventaInicial?: number | null;
  onConsumirVenta?: () => void;
  onIrCaja?: () => void;
}) {
  const { usuario, ambienteActual, setHayCambiosSinGuardar } = useAuth();
  useEffect(() => {
    document.body.classList.add('pos-active');
    return () => document.body.classList.remove('pos-active');
  }, []);
  const [inicio, Sinicio] = useState<InicioPos | null>(null),
    [error, Serror] = useState(''),
    [vista, Svista] = useState<'venta' | 'listado'>('venta'),
    [cargarOt, ScargarOt] = useState(false),
    [detalle, Sdetalle] = useState<number | null>(null),
    [revision, Srevision] = useState(0),
    [aviso, Saviso] = useState<{ id: number; mensaje: string; tipo: 'EXITO' | 'ERROR' } | null>(
      null
    ),
    [pendiente, Spendiente] = useState<SolicitudPos | null>(null),
    [cred, Scred] = useState<CredencialesPos>({ username: '', password: '' }),
    [procesando, Sprocesando] = useState(false);
  const [capturas, Scapturas] = useState<Record<string, boolean>>({});
  const registrarCaptura = useCallback(
    (id: string, dirty: boolean) =>
      Scapturas((p) => {
        const n = { ...p };
        if (dirty) n[id] = true;
        else delete n[id];
        return n;
      }),
    []
  );
  const lock = useRef(false);
  const montaje = useRef(true);
  const id = usuario?.id_usuario ?? 0;
  const admin = usuario?.rol === 'ADMINISTRADOR';
  const notificar = useCallback(
    (mensaje: string, tipo: 'EXITO' | 'ERROR') => Saviso({ id: Date.now(), mensaje, tipo }),
    []
  );
  const actualizar = useCallback(async () => {
    try {
      const r = await posApi<InicioPos>('inicio');
      if (montaje.current) {
        Sinicio(r);
        Serror('');
      }
    } catch (e) {
      if (montaje.current) Serror(mensajePos(e));
    }
  }, []);
  const pendienteActual = useCallback(() => {
    try {
      Spendiente(leerPendientePos(id, ambienteActual));
    } catch (e) {
      Serror(mensajePos(e));
    }
  }, [id, ambienteActual]);
  const guardado = useCallback(
    async (r: ResultadoPos) => {
      notificar('Venta registrada correctamente.', 'EXITO');
      Sdetalle(r.id_venta);
      Srevision((n) => n + 1);
      await actualizar();
      try {
        const d = await documentoPos(r.id_documento, false, true, r.token_envio);
        if (d.estado !== 'ACEPTADO') notificar(`Venta guardada. ${d.mensaje}`, 'ERROR');
      } catch (e) {
        notificar(`Venta guardada; documento pendiente. ${mensajePos(e)}`, 'ERROR');
      }
      Srevision((n) => n + 1);
      pendienteActual();
    },
    [actualizar, notificar, pendienteActual]
  );
  const venta = useVentaPos(id, ambienteActual, guardado);
  useEffect(() => {
    montaje.current = true;
    void actualizar();
    pendienteActual();
    return () => {
      montaje.current = false;
    };
  }, [actualizar, pendienteActual]);
  useEffect(() => {
    setHayCambiosSinGuardar(venta.dirty || Object.values(capturas).some(Boolean));
    return () => setHayCambiosSinGuardar(false);
  }, [venta.dirty, capturas, setHayCambiosSinGuardar]);
  useEffect(() => {
    if (otInicial && inicio) {
      void venta
        .seleccionarOt(otInicial)
        .then(() => onConsumirOt?.())
        .catch((e) => Serror(mensajePos(e)));
    }
  }, [otInicial, !!inicio]);
  useEffect(() => {
    if (ventaInicial) {
      Svista('listado');
      Sdetalle(ventaInicial);
      onConsumirVenta?.();
    }
  }, [ventaInicial, onConsumirVenta]);
  const cerrarAviso = useCallback(() => Saviso(null), []);
  async function reintentar() {
    if (!pendiente || lock.current) return;
    lock.current = true;
    Sprocesando(true);
    try {
      const r = await ejecutarPos(
        id,
        ambienteActual,
        pendiente.accion,
        pendiente.payload,
        !admin ? cred : undefined
      );
      Spendiente(null);
      venta.limpiar();
      await guardado(r);
    } catch (e) {
      Serror(mensajePos(e));
    } finally {
      lock.current = false;
      Sprocesando(false);
      Scred({ username: '', password: '' });
      pendienteActual();
    }
  }
  async function revisarPendientes() {
    if (lock.current) return;
    lock.current = true;
    Sprocesando(true);
    let completados = 0;
    try {
      const ds = await posApi<{ id: number }[]>('pendientes');
      for (const d of ds) {
        let r = await documentoPos(d.id);
        if (r.estado === 'PENDIENTE') r = await documentoPos(d.id);
        if (r.estado === 'ACEPTADO' || r.estado === 'RECHAZADO') completados++;
      }
      notificar(
        `Revisión terminada: ${ds.length} documentos atendidos; ${completados} con respuesta definitiva.`,
        'EXITO'
      );
      Srevision((n) => n + 1);
    } catch (e) {
      notificar(mensajePos(e), 'ERROR');
    } finally {
      lock.current = false;
      Sprocesando(false);
    }
  }
  function cambiarVista(v: 'venta' | 'listado') {
    Svista(v);
  }
  const vaciar = () => {
    if (window.confirm('¿Descartar la venta actual sin cobrarla?')) venta.limpiar();
  };
  const bloqueoCobro = pendiente
    ? 'Recupera la operación pendiente antes de cobrar.'
    : !inicio
      ? 'El estado del punto de venta aún no está disponible.'
      : !inicio.configurada
        ? 'Cobro pendiente de configuración.'
        : !inicio.caja_propia
          ? 'Necesitas un turno propio abierto para cobrar.'
          : undefined;

  return (
    <CapturasPosContext.Provider value={registrarCaptura}>
      <section className="gloria-theme pos-module" aria-labelledby="pos-module-title">
        <header className="pos-main-header gt-stage">
          <div>
            <span className="pos-eyebrow">OPERACIÓN COMERCIAL</span>
            <h1 id="pos-module-title">Punto de Venta</h1>
            <p>Ventas directas y cobro de trabajos terminados.</p>
          </div>
          <div className="pos-header-actions">
            <span className="pos-badge">{ambienteActual}</span>
            <button
              disabled={procesando}
              onClick={() => {
                void actualizar();
                Srevision((n) => n + 1);
              }}
            >
              <RefreshCw size={16} />
              Actualizar
            </button>
          </div>
        </header>
        <div className="pos-statusbar" aria-label="Estado de caja">
          <span className="pos-shift">
            <Wallet size={16} aria-hidden="true" />
            <i
              className={inicio?.caja_propia ? 'pos-dot' : 'pos-dot pos-dot-off'}
              aria-hidden="true"
            />
            {inicio?.caja_propia
              ? `Tu caja: ${inicio.caja?.codigo}`
              : inicio?.caja_ocupada || inicio?.caja
                ? 'Caja ocupada'
                : inicio
                  ? 'Sin caja abierta'
                  : 'Consultando caja…'}
          </span>
          <span className="pos-operator">
            <UserRound size={15} aria-hidden="true" /> {usuario?.nombre_completo}
          </span>
          {inicio?.caja_propia ? (
            <span>
              Efectivo disponible: <strong>{soles(inicio.caja?.efectivo_cent ?? 0)}</strong>
            </span>
          ) : (
            inicio &&
            !inicio.caja_ocupada &&
            !inicio.caja &&
            onIrCaja && (
              <button className="pos-link" onClick={onIrCaja}>
                Abrir turno propio
              </button>
            )
          )}
        </div>
        {error && (
          <div role="alert" className="pos-error">
            {error}
            <button onClick={() => void actualizar()}>Reintentar consulta</button>
          </div>
        )}
        {inicio && !inicio.configurada && (
          <div className="pos-warning pos-configuration" role="status">
            <AlertTriangle size={16} aria-hidden="true" />
            <div>
              <strong>Cobro todavía no habilitado</strong>
              <p>
                Completa la configuración de empresa e integración en Administración. Mientras
                tanto, puedes preparar la venta.
              </p>
            </div>
          </div>
        )}
        {pendiente && (
          <section className="pos-warning">
            <strong>Existe una solicitud sin respuesta confirmada</strong>
            <p>
              Se conservaron sus datos originales. Recupera su resultado antes de iniciar otra
              operación.
            </p>
            <AutorizacionPos
              necesaria={
                !admin &&
                (pendiente.accion === 'devolver' || Number(pendiente.payload.descuento_bp) > 500)
              }
              valor={cred}
              onChange={Scred}
            />
            <button disabled={procesando} onClick={() => void reintentar()}>
              {procesando ? 'Recuperando…' : 'Recuperar resultado original'}
            </button>
          </section>
        )}
        <div className="pos-navigation">
          <nav aria-label="Vistas de Punto de Venta">
            <button
              className={vista === 'venta' ? 'active' : ''}
              aria-current={vista === 'venta' ? 'page' : undefined}
              onClick={() => cambiarVista('venta')}
            >
              <ShoppingCart size={17} />
              Venta
            </button>
            <button
              className={vista === 'listado' ? 'active' : ''}
              aria-current={vista === 'listado' ? 'page' : undefined}
              onClick={() => cambiarVista('listado')}
            >
              <ReceiptText size={17} />
              Ventas y comprobantes
            </button>
          </nav>
          {vista === 'venta' && (
            <button className="pos-load-ot" disabled={!!pendiente} onClick={() => ScargarOt(true)}>
              <ClipboardList size={17} />
              Cargar OT finalizada
            </button>
          )}
        </div>
        {vista === 'venta' ? (
          <div className="pos-workspace">
            <nav className="pos-compact-links" aria-label="Secciones de la venta">
              <a href="#pos-catalog-title">Ir al catálogo</a>
              <a href="#pos-cart-title">Ir a venta actual</a>
            </nav>
            <CatalogoPos
              onAgregar={(p) => {
                if (!pendiente) venta.agregar(p);
              }}
            />
            <CarritoPos
              lineas={venta.lineas}
              totales={venta.totales}
              cargando={venta.calculando}
              error={venta.error}
              ot={venta.ot}
              otContexto={venta.otContexto}
              descuento={venta.descuento}
              motivo={venta.motivo}
              onDescuento={venta.Sdescuento}
              onMotivo={venta.Smotivo}
              onCantidad={venta.cambiarCantidad}
              onQuitar={venta.quitar}
              onVaciar={vaciar}
              onCobrar={() => venta.Scobro(true)}
              puedeCobrar={!!inicio?.caja_propia && !!inicio?.configurada && !pendiente}
              bloqueoCobro={bloqueoCobro}
            >
              <ReceptorPos
                cliente={venta.cliente}
                onSeleccionar={venta.Scliente}
                tipo={venta.tipo}
                onTipo={venta.Stipo}
              />
            </CarritoPos>
          </div>
        ) : (
          inicio && (
            <VentasPos
              hoy={inicio.hoy}
              admin={admin}
              revision={revision}
              onAbrir={Sdetalle}
              onPendientes={revisarPendientes}
            />
          )
        )}{' '}
        {cargarOt && (
          <CargarOtPos
            onCerrar={() => ScargarOt(false)}
            onSeleccionar={async (ot) => {
              await venta.seleccionarOt(ot);
              ScargarOt(false);
            }}
          />
        )}
        {venta.cobro && venta.totales && (
          <CobroPos
            totales={venta.totales}
            requiereAdmin={!admin && Number(venta.descuento.replace(',', '.')) > 5}
            onCerrar={() => {
              venta.Scobro(false);
              pendienteActual();
            }}
            onConfirmar={async (p, c) => {
              try {
                await venta.confirmar(p, c);
              } finally {
                pendienteActual();
              }
            }}
          />
        )}{' '}
        {detalle && (
          <DetalleVentaPos
            id={detalle}
            revisionExterna={revision}
            usuario={id}
            ambiente={ambienteActual}
            admin={admin}
            cajaPropia={!!inicio?.caja_propia}
            empresa={inicio?.empresa ?? 'Lubricentro Gloria'}
            onCerrar={() => {
              Sdetalle(null);
              pendienteActual();
            }}
            onOperacion={() => {
              Srevision((n) => n + 1);
              void actualizar();
            }}
            onAviso={notificar}
          />
        )}{' '}
        {aviso && (
          <div
            className="pos-toast"
            data-kind={aviso.tipo}
            role={aviso.tipo === 'ERROR' ? 'alert' : 'status'}
            aria-atomic="true"
          >
            <ToastNotification
              key={aviso.id}
              mensaje={aviso.mensaje}
              tipo={aviso.tipo}
              onCerrar={cerrarAviso}
            />
          </div>
        )}
      </section>
    </CapturasPosContext.Provider>
  );
}
