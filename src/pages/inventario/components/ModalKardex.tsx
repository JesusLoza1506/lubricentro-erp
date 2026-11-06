import React, { useEffect, useState, useRef } from 'react';
import { X, ArrowLeft, ArrowRight, AlertCircle, Filter } from 'lucide-react';
import { Producto, MovimientoKardexDTO } from '../../../types/inventario';
import { inventarioService } from '../../../services/inventarioService';
import { ModalOrigenKardex, dineroMicro } from './ModalOrigenKardex';

interface ModalKardexProps {
  abierto: boolean;
  producto: Producto | null;
  onClose: () => void;
}

interface ResumenSaldos {
  saldo_inicial: number;
  saldo_final: number;
  saldo_actual: number;
}

export const ModalKardex: React.FC<ModalKardexProps> = ({ abierto, producto, onClose }) => {
  const [movimientos, setMovimientos] = useState<MovimientoKardexDTO[]>([]);
  const [cargando, setCargando] = useState<boolean>(false);
  const [pagina, setPagina] = useState<number>(0);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);

  // Filtros
  const [fechaInicio, setFechaInicio] = useState<string>('');
  const [fechaFin, setFechaFin] = useState<string>('');
  const [tipoMovimiento, setTipoMovimiento] = useState<string>('TODOS');

  // Resumen de saldos
  const [saldos, setSaldos] = useState<ResumenSaldos>({
    saldo_inicial: 0,
    saldo_final: 0,
    saldo_actual: 0,
  });

  const limite = 20;
  const consulta = useRef(0);
  const aplicados = useRef({ inicio: '', fin: '', tipo: 'TODOS' });
  const [origenSeleccionado, setOrigenSeleccionado] = useState<number | null>(null);

  useEffect(() => {
    if (abierto && producto) {
      setOrigenSeleccionado(null);
      setMovimientos([]);
      setPagina(0);
      setFechaInicio('');
      setFechaFin('');
      setTipoMovimiento('TODOS');
      setErrorMsg(null);
      cargarKardex(0, '', '', 'TODOS');
    }
    return () => {
      consulta.current += 1;
    };
  }, [abierto, producto]);

  const cargarKardex = async (numPagina: number, fInicio: string, fFin: string, tMov: string) => {
    if (!producto) return;
    const actual = ++consulta.current;
    setCargando(true);
    setErrorMsg(null);
    try {
      const [data, resSaldos] = await Promise.all([
        inventarioService.listarKardexProducto(
          producto.id_producto,
          limite,
          numPagina * limite,
          fInicio || undefined,
          fFin || undefined,
          tMov === 'TODOS' ? undefined : tMov
        ),
        inventarioService.obtenerResumenSaldosKardex(
          producto.id_producto,
          fInicio || undefined,
          fFin || undefined,
          tMov === 'TODOS' ? undefined : tMov
        ),
      ]);
      if (actual !== consulta.current) return;
      aplicados.current = { inicio: fInicio, fin: fFin, tipo: tMov };
      setMovimientos(data || []);
      setSaldos(resSaldos);
      setPagina(numPagina);
    } catch (err: unknown) {
      if (actual !== consulta.current) return;
      const msg =
        typeof err === 'string'
          ? err
          : (err as { message?: string })?.message || 'Error al obtener datos del Kárdex';
      setErrorMsg(msg);
    } finally {
      if (actual === consulta.current) setCargando(false);
    }
  };

  const handleAplicarFiltros = (e: React.FormEvent) => {
    e.preventDefault();
    setPagina(0);
    cargarKardex(0, fechaInicio, fechaFin, tipoMovimiento);
  };

  if (!abierto || !producto) return null;

  const esML = producto.unidad_medida === 'ML';
  const unidadTexto = esML ? 'L' : 'UND';
  const formatearCant = (val: number) => (esML ? (val / 1000).toFixed(3) : val.toString());

  return (
    <div
      className="inv-overlay"
      style={{
        position: 'fixed',
        top: 0,
        left: 0,
        width: '100vw',
        height: '100vh',
        backgroundColor: 'rgba(0,0,0,0.8)',
        backdropFilter: 'blur(4px)',
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        zIndex: 1100,
      }}
    >
      <div
        className="inv-dialog inv-dialog-wide"
        role="dialog"
        aria-modal="true"
        aria-label="Kardex"
        style={{
          width: 'min(1400px, 96vw)',
          boxSizing: 'border-box',
          backgroundColor: '#0F172A',
          border: '1px solid #334155',
          borderRadius: '16px',
          padding: '24px',
          boxShadow: '0 20px 40px rgba(0,0,0,0.5)',
          maxHeight: '90vh',
          display: 'flex',
          flexDirection: 'column',
        }}
      >
        {/* Cabecera del Modal */}
        <div
          style={{
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
            marginBottom: '16px',
          }}
        >
          <div>
            <h3 style={{ fontSize: '18px', fontWeight: '800', color: '#F8FAFC', margin: 0 }}>
              Kardex · {producto.nombre}
            </h3>
            <span style={{ fontSize: '12px', color: '#94A3B8' }}>Código: {producto.codigo}</span>
          </div>
          <button
            aria-label="Cerrar Kardex"
            onClick={onClose}
            style={{ background: 'none', border: 'none', color: '#94A3B8', cursor: 'pointer' }}
          >
            <X size={20} />
          </button>
        </div>

        {/* Panel de Filtros por Rango de Fechas y Tipo de Movimiento */}
        <form
          onSubmit={handleAplicarFiltros}
          style={{
            display: 'flex',
            gap: '12px',
            alignItems: 'flex-end',
            backgroundColor: '#1E293B',
            padding: '12px 16px',
            borderRadius: '10px',
            marginBottom: '16px',
            flexWrap: 'wrap',
          }}
        >
          <div>
            <label
              style={{ display: 'block', fontSize: '11px', color: '#94A3B8', marginBottom: '4px' }}
            >
              Desde
            </label>
            <input
              type="date"
              value={fechaInicio}
              onChange={(e) => setFechaInicio(e.target.value)}
              style={filterInputStyle}
            />
          </div>

          <div>
            <label
              style={{ display: 'block', fontSize: '11px', color: '#94A3B8', marginBottom: '4px' }}
            >
              Hasta
            </label>
            <input
              type="date"
              value={fechaFin}
              onChange={(e) => setFechaFin(e.target.value)}
              style={filterInputStyle}
            />
          </div>

          <div>
            <label
              style={{ display: 'block', fontSize: '11px', color: '#94A3B8', marginBottom: '4px' }}
            >
              Tipo de Movimiento
            </label>
            <select
              value={tipoMovimiento}
              onChange={(e) => setTipoMovimiento(e.target.value)}
              style={filterInputStyle}
            >
              <option value="TODOS">TODOS LOS MOVIMIENTOS</option>
              <option value="COMPRA">COMPRA</option>
              <option value="CONSUMO_OT">CONSUMO EN OT</option>
              <option value="REPOSICION_OT">DEVOLUCIÓN EN OT</option>
              <option value="STOCK_INICIAL">STOCK INICIAL</option>
              <option value="AJUSTE_ENTRADA">AJUSTE ENTRADA</option>
              <option value="AJUSTE_SALIDA">AJUSTE SALIDA</option>
              <option value="ANULACION_INGRESO">ANULACIÓN INGRESO</option>
              <option value="VENTA_DIRECTA">VENTA DIRECTA</option>
              <option value="DEVOLUCION_VENTA">DEVOLUCIÓN DE VENTA</option>
            </select>
          </div>

          <button
            type="submit"
            disabled={cargando}
            style={{
              padding: '8px 16px',
              backgroundColor: '#38BDF8',
              color: '#0F172A',
              border: 'none',
              borderRadius: '6px',
              fontWeight: '700',
              cursor: 'pointer',
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              fontSize: '12px',
            }}
          >
            <Filter size={14} /> Filtrar
          </button>
        </form>

        <p style={{ color: '#94A3B8', fontSize: 12 }}>
          Saldos reales del período, considerando todos los movimientos. El filtro de tipo solo
          limita las filas mostradas. Importes históricos en S/ con seis decimales; costo unitario
          en UND o ML.
        </p>
        {/* Panel de Saldos Inicial, Final y Stock Actual */}
        <div
          style={{
            display: 'flex',
            justifyContent: 'space-between',
            backgroundColor: 'rgba(56, 189, 248, 0.1)',
            border: '1px solid rgba(56, 189, 248, 0.3)',
            padding: '10px 16px',
            borderRadius: '8px',
            marginBottom: '16px',
            fontSize: '13px',
          }}
        >
          <div>
            <span style={{ color: '#94A3B8' }}>Saldo Inicial Período: </span>
            <strong style={{ color: '#38BDF8' }}>
              {formatearCant(saldos.saldo_inicial)} {unidadTexto}
            </strong>
          </div>
          <div>
            <span style={{ color: '#94A3B8' }}>Saldo Final Período: </span>
            <strong style={{ color: '#34D399' }}>
              {formatearCant(saldos.saldo_final)} {unidadTexto}
            </strong>
          </div>
          <div>
            <span style={{ color: '#94A3B8' }}>Stock Actual Catálogo: </span>
            <strong style={{ color: '#F59E0B' }}>
              {formatearCant(saldos.saldo_actual)} {unidadTexto}
            </strong>
          </div>
        </div>

        {/* Alerta de Error */}
        {errorMsg && (
          <div
            role="alert"
            style={{
              backgroundColor: 'rgba(239, 68, 68, 0.15)',
              border: '1px solid #EF4444',
              color: '#F87171',
              padding: '10px 14px',
              borderRadius: '8px',
              marginBottom: '16px',
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              fontSize: '13px',
            }}
          >
            <AlertCircle size={16} />
            <span>{errorMsg}</span>
          </div>
        )}

        {/* Tabla Kárdex */}
        <div
          style={{ flex: 1, overflow: 'auto', border: '1px solid #334155', borderRadius: '8px' }}
        >
          <table
            style={{
              width: '100%',
              minWidth: 1150,
              borderCollapse: 'collapse',
              fontSize: '12px',
              color: '#F8FAFC',
            }}
          >
            <thead>
              <tr style={{ backgroundColor: '#1E293B', color: '#94A3B8', textAlign: 'left' }}>
                <th style={{ padding: '10px' }}>Fecha</th>
                <th style={{ padding: '10px' }}>Tipo Movimiento</th>
                <th style={{ padding: '10px' }}>Stock Ant.</th>
                <th style={{ padding: '10px' }}>Delta</th>
                <th style={{ padding: '10px' }}>Stock Resultante</th>
                <th style={{ padding: '10px' }}>Costo / {esML ? 'ML' : 'UND'}</th>
                <th style={{ padding: '10px' }}>Valor anterior</th>
                <th style={{ padding: '10px' }}>Variación de valor</th>
                <th style={{ padding: '10px' }}>Valor resultante</th>
                <th style={{ padding: '10px' }}>Origen / Motivo / Usuario</th>
              </tr>
            </thead>
            <tbody>
              {cargando ? (
                <tr>
                  <td
                    colSpan={10}
                    style={{ textAlign: 'center', padding: '20px', color: '#CBD5E1' }}
                  >
                    Cargando movimientos de Kardex...
                  </td>
                </tr>
              ) : movimientos.length === 0 ? (
                <tr>
                  <td
                    colSpan={10}
                    style={{ textAlign: 'center', padding: '20px', color: '#94A3B8' }}
                  >
                    Sin movimientos registrados en este período / filtro
                  </td>
                </tr>
              ) : (
                movimientos.map((m) => (
                  <tr key={m.id_movimiento} style={{ borderBottom: '1px solid #1E293B' }}>
                    <td style={{ padding: '10px' }}>{m.fecha_movimiento}</td>
                    <td style={{ padding: '10px', fontWeight: '700', color: '#38BDF8' }}>
                      {m.tipo_movimiento}
                    </td>
                    <td style={{ padding: '10px' }}>{formatearCant(m.stock_anterior)}</td>
                    <td
                      style={{
                        padding: '10px',
                        fontWeight: '700',
                        color: m.diferencia >= 0 ? '#34D399' : '#F87171',
                      }}
                    >
                      {m.diferencia >= 0
                        ? `+${formatearCant(m.diferencia)}`
                        : formatearCant(m.diferencia)}
                    </td>
                    <td style={{ padding: '10px', fontWeight: '700' }}>
                      {formatearCant(m.stock_resultante)}
                    </td>
                    <td style={{ padding: '10px', whiteSpace: 'nowrap' }}>
                      {dineroMicro(m.costo_unitario_micro)}
                    </td>
                    <td style={{ padding: '10px', whiteSpace: 'nowrap' }}>
                      {dineroMicro(m.valor_anterior_micro)}
                    </td>
                    <td style={{ padding: '10px', whiteSpace: 'nowrap' }}>
                      {dineroMicro(m.valor_delta_micro)}
                    </td>
                    <td style={{ padding: '10px', whiteSpace: 'nowrap' }}>
                      {dineroMicro(m.valor_resultante_micro)}
                    </td>
                    <td style={{ padding: '10px', color: '#CBD5E1' }}>
                      {(m.id_ingreso || m.id_ot || m.id_venta || m.id_devolucion) && (
                        <>
                          <button
                            type="button"
                            onClick={() => setOrigenSeleccionado(m.id_movimiento)}
                            style={{
                              color: '#38BDF8',
                              background: 'transparent',
                              border: 0,
                              padding: '0 0 6px',
                              cursor: 'pointer',
                              textAlign: 'left',
                            }}
                          >
                            Ver {m.codigo_origen || 'origen'}
                          </button>
                          <br />
                        </>
                      )}
                      {m.motivo || 'Sin observación adicional'} <br />
                      <small style={{ color: '#64748B' }}>
                        Por: {m.usuario_nombre || 'Sistema'}
                      </small>
                    </td>
                  </tr>
                ))
              )}
            </tbody>
          </table>
        </div>

        {/* Paginador */}
        <div
          style={{
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
            marginTop: '16px',
          }}
        >
          <button
            disabled={pagina === 0 || cargando}
            onClick={() =>
              cargarKardex(
                pagina - 1,
                aplicados.current.inicio,
                aplicados.current.fin,
                aplicados.current.tipo
              )
            }
            style={{
              padding: '8px 12px',
              backgroundColor: '#1E293B',
              border: '1px solid #334155',
              color: '#F8FAFC',
              borderRadius: '6px',
              cursor: pagina === 0 || cargando ? 'not-allowed' : 'pointer',
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              opacity: pagina === 0 || cargando ? 0.5 : 1,
            }}
          >
            <ArrowLeft size={16} /> Anterior
          </button>
          <span style={{ fontSize: '13px', color: '#94A3B8' }}>Página {pagina + 1}</span>
          <button
            disabled={movimientos.length < limite || cargando}
            onClick={() =>
              cargarKardex(
                pagina + 1,
                aplicados.current.inicio,
                aplicados.current.fin,
                aplicados.current.tipo
              )
            }
            style={{
              padding: '8px 12px',
              backgroundColor: '#1E293B',
              border: '1px solid #334155',
              color: '#F8FAFC',
              borderRadius: '6px',
              cursor: movimientos.length < limite || cargando ? 'not-allowed' : 'pointer',
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              opacity: movimientos.length < limite || cargando ? 0.5 : 1,
            }}
          >
            Siguiente <ArrowRight size={16} />
          </button>
        </div>
      </div>
      {origenSeleccionado !== null && (
        <ModalOrigenKardex
          idMovimiento={origenSeleccionado}
          onClose={() => setOrigenSeleccionado(null)}
        />
      )}
    </div>
  );
};

const filterInputStyle: React.CSSProperties = {
  backgroundColor: '#0F172A',
  border: '1px solid #334155',
  borderRadius: '6px',
  color: '#F8FAFC',
  padding: '6px 10px',
  fontSize: '12px',
  outline: 'none',
};
