import { useCallback, useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { RefreshCw, ShoppingCart, ClipboardList, ReceiptText } from 'lucide-react';
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
import { ModalPos } from './components/ModalPos';
import { AutorizacionPos } from './components/AutorizacionPos';
import { decimalBase, soles } from './posUtils';
import { CapturasPosContext } from './CapturasPos';
import './posVisual.css';

export function PosPage({
  otInicial,
  onConsumirOt,
}: {
  otInicial?: number | null;
  onConsumirOt?: () => void;
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
    [procesando, Sprocesando] = useState(false),
    [abrirCaja, SabrirCaja] = useState(false),
    [fondo, Sfondo] = useState('0');
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
        .seleccionarOt({ id: otInicial, codigo: '', placa: '', cliente: '', fecha: '' })
        .then(() => onConsumirOt?.())
        .catch((e) => Serror(mensajePos(e)));
    }
  }, [otInicial, !!inicio]);
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
  async function abrirTurno() {
    if (lock.current) return;
    lock.current = true;
    Sprocesando(true);
    try {
      const monto = decimalBase(fondo, 2, true);
      await invoke('abrir_caja_cmd', { payload: { id_usuario: id, monto_apertura: monto } });
      SabrirCaja(false);
      await actualizar();
      notificar('Turno propio abierto correctamente.', 'EXITO');
    } catch (e) {
      Serror(mensajePos(e));
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
  return (
    <CapturasPosContext.Provider value={registrarCaptura}>
      <main className="pos-module">
        <header className="pos-main-header">
          <div>
            <span className="pos-eyebrow">OPERACIÓN COMERCIAL</span>
            <h1>Punto de Venta</h1>
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
        <div className="pos-statusbar">
          <span>
            <i className={inicio?.caja_propia ? 'pos-dot' : 'pos-dot pos-dot-off'} />
            {inicio?.caja_propia
              ? `Tu caja: ${inicio.caja?.codigo}`
              : inicio?.caja
                ? `Caja de ${inicio.caja.titular}`
                : 'Sin caja abierta'}
          </span>
          <span>{usuario?.nombre_completo}</span>
          {inicio?.caja_propia ? (
            <span>
              Efectivo disponible: <strong>{soles(inicio.caja?.efectivo_cent ?? 0)}</strong>
            </span>
          ) : (
            !inicio?.caja && (
              <button className="pos-link" onClick={() => SabrirCaja(true)}>
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
          <div className="pos-warning">
            Completa la configuración de empresa e integración en Administración para habilitar el
            cobro. Puedes consultar el catálogo y preparar la venta.
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
              onClick={() => cambiarVista('venta')}
            >
              <ShoppingCart size={17} />
              Venta
            </button>
            <button
              className={vista === 'listado' ? 'active' : ''}
              onClick={() => cambiarVista('listado')}
            >
              <ReceiptText size={17} />
              Ventas y comprobantes
            </button>
          </nav>
          {vista === 'venta' && (
            <button disabled={!!pendiente} onClick={() => ScargarOt(true)}>
              <ClipboardList size={17} />
              Cargar OT finalizada
            </button>
          )}
        </div>
        {vista === 'venta' ? (
          <div className="pos-workspace">
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
              descuento={venta.descuento}
              motivo={venta.motivo}
              onDescuento={venta.Sdescuento}
              onMotivo={venta.Smotivo}
              onCantidad={venta.cambiarCantidad}
              onQuitar={venta.quitar}
              onVaciar={vaciar}
              onCobrar={() => venta.Scobro(true)}
              puedeCobrar={!!inicio?.caja_propia && !!inicio?.configurada && !pendiente}
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
        {abrirCaja && (
          <ModalPos
            captura
            titulo="Abrir turno propio"
            onCerrar={() => {
              if (fondo === '0' || window.confirm('¿Descartar el fondo inicial sin abrir turno?'))
                SabrirCaja(false);
            }}
            ocupado={procesando}
          >
            <p className="pos-hint">
              Registra el efectivo inicial recibido. La gestión completa del turno corresponde a
              Caja Chica.
            </p>
            <label>
              Fondo inicial (S/)
              <input inputMode="decimal" value={fondo} onChange={(e) => Sfondo(e.target.value)} />
            </label>
            <footer className="pos-modal-footer">
              <button
                disabled={procesando}
                onClick={() => {
                  if (
                    fondo === '0' ||
                    window.confirm('¿Descartar el fondo inicial sin abrir turno?')
                  )
                    SabrirCaja(false);
                }}
              >
                Cancelar
              </button>
              <button
                className="pos-primary"
                disabled={procesando}
                onClick={() => void abrirTurno()}
              >
                {procesando ? 'Abriendo…' : 'Abrir turno'}
              </button>
            </footer>
          </ModalPos>
        )}
        {aviso && (
          <div role={aviso.tipo === 'ERROR' ? 'alert' : 'status'}>
            <ToastNotification
              key={aviso.id}
              mensaje={aviso.mensaje}
              tipo={aviso.tipo}
              onCerrar={cerrarAviso}
            />
          </div>
        )}
      </main>
    </CapturasPosContext.Provider>
  );
}
