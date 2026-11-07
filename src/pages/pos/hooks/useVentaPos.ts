import { useCallback, useEffect, useRef, useState } from 'react';
import type { ClienteDTO } from '../../../types/cliente';
import type {
  CredencialesPos,
  LineaDirecta,
  LineaPos,
  OtPos,
  PagoPos,
  ProductoPos,
  ResultadoPos,
  TotalesPos,
  VentaSolicitud,
} from '../../../types/pos';
import { ejecutarPos, mensajePos, posApi } from '../../../services/posService';
import { decimalBase } from '../posUtils';
export function useVentaPos(
  usuario: number,
  ambiente: string,
  onGuardado: (r: ResultadoPos) => Promise<void>
) {
  const [directas, Sdirectas] = useState<(LineaDirecta & { producto: ProductoPos })[]>([]),
    [ot, Sot] = useState<number | null>(null),
    [otContexto, SotContexto] = useState<OtPos | null>(null),
    [cliente, Scliente] = useState<ClienteDTO | null>(null),
    [tipo, Stipo] = useState<'BOLETA' | 'FACTURA'>('BOLETA'),
    [descuento, Sdescuento] = useState('0'),
    [motivo, Smotivo] = useState(''),
    [totales, Stotales] = useState<TotalesPos | null>(null),
    [error, Serror] = useState(''),
    [calculando, Scalculando] = useState(false),
    [lineas, Slineas] = useState<LineaPos[]>([]),
    [cobro, Scobro] = useState(false);
  const sec = useRef(0);
  const envio = useRef(false);
  const construir = useCallback(
    (pagos: PagoPos[] = []): VentaSolicitud => ({
      id_operacion: 'cotizacion-sin-efectos',
      id_ot: ot,
      id_cliente: cliente?.id_cliente ?? 1,
      tipo_comprobante: tipo,
      descuento_bp: decimalBase(descuento || '0', 2, true),
      motivo,
      lineas: directas.map(({ producto: _, ...l }) => l),
      pagos,
    }),
    [ot, cliente, tipo, descuento, motivo, directas]
  );
  const limpiar = useCallback(() => {
    Sdirectas([]);
    Sot(null);
    SotContexto(null);
    Scliente(null);
    Stipo('BOLETA');
    Sdescuento('0');
    Smotivo('');
    Stotales(null);
    Slineas([]);
    Serror('');
    Scobro(false);
  }, []);
  useEffect(() => {
    const id = ++sec.current;
    if (!directas.length && !ot) {
      Stotales(null);
      Slineas([]);
      Serror('');
      Scalculando(false);
      return;
    }
    Scalculando(true);
    Stotales(null);
    const t = setTimeout(() => {
      try {
        void posApi<TotalesPos>('cotizar', construir())
          .then((r) => {
            if (id === sec.current) {
              Stotales(r);
              Slineas(r.lineas);
              Serror('');
            }
          })
          .catch((e) => {
            if (id === sec.current) Serror(mensajePos(e));
          })
          .finally(() => {
            if (id === sec.current) Scalculando(false);
          });
      } catch (e) {
        Serror(mensajePos(e));
        Scalculando(false);
      }
    }, 180);
    return () => {
      clearTimeout(t);
      sec.current++;
    };
  }, [directas, ot, construir]);
  function agregar(p: ProductoPos) {
    Sdirectas((ls) => {
      const existe = ls.find((l) => l.tipo === p.tipo && l.id === p.id);
      return existe
        ? ls.map((l) =>
            l === existe ? { ...l, cantidad: l.cantidad + (p.unidad === 'ML' ? 1000 : 1) } : l
          )
        : [
            ...ls,
            {
              tipo: p.tipo,
              id: p.id,
              cantidad: p.unidad === 'ML' ? 1000 : 1,
              precio_esperado_cent: p.precio_cent,
              producto: p,
            },
          ];
    });
  }
  // La captura permanece visible aunque la cotización falle (por ejemplo, stock cambiado).
  const mostradas: LineaPos[] = [
    ...lineas.filter((l) => ot !== null && (l.id_consumo_ot != null || l.id_servicio_ot != null)),
    ...directas.map((d) => {
      const calculada = lineas.find(
        (l) =>
          l.id_consumo_ot == null &&
          l.id_servicio_ot == null &&
          (d.tipo === 'PRODUCTO' ? l.id_producto === d.id : l.id_servicio === d.id) &&
          l.cantidad === d.cantidad
      );
      return (
        calculada ?? {
          id_producto: d.tipo === 'PRODUCTO' ? d.id : null,
          id_servicio: d.tipo === 'SERVICIO' ? d.id : null,
          codigo: d.producto.codigo,
          nombre: d.producto.nombre,
          unidad: d.producto.unidad,
          cantidad: d.cantidad,
          precio_cent: d.precio_esperado_cent,
          total_cent: Math.round(
            (d.cantidad * d.precio_esperado_cent) / (d.producto.unidad === 'ML' ? 1000 : 1)
          ),
          base_cent: 0,
        }
      );
    }),
  ];
  function cambiarCantidad(indice: number, texto: string) {
    const l = mostradas[indice];
    if (!l) return;
    try {
      const q = l.unidad === 'ML' ? decimalBase(texto, 3) : decimalBase(texto, 0);
      Sdirectas((ls) =>
        ls.map((d) =>
          d.tipo === (l.id_producto ? 'PRODUCTO' : 'SERVICIO') &&
          d.id === (l.id_producto ?? l.id_servicio)
            ? { ...d, cantidad: q }
            : d
        )
      );
    } catch (e) {
      Serror(mensajePos(e));
    }
  }
  function quitar(indice: number) {
    const l = mostradas[indice];
    Sdirectas((ls) =>
      ls.filter(
        (d) =>
          !(
            d.tipo === (l.id_producto ? 'PRODUCTO' : 'SERVICIO') &&
            d.id === (l.id_producto ?? l.id_servicio)
          )
      )
    );
  }
  async function seleccionarOt(o: OtPos | number) {
    const idOt = typeof o === 'number' ? o : o.id;
    const p = { ...construir(), id_ot: idOt };
    const calculada = await posApi<TotalesPos>('cotizar', p);
    Slineas(calculada.lineas);
    Sot(idOt);
    SotContexto(calculada.ot ?? (typeof o === 'number' ? null : o));
  }
  async function confirmar(pagos: PagoPos[], cred?: CredencialesPos) {
    if (envio.current) return;
    envio.current = true;
    try {
      const payload = { ...construir(pagos), id_operacion: crypto.randomUUID() };
      const resultado = await ejecutarPos(
        usuario,
        ambiente,
        'confirmar',
        payload as unknown as Record<string, unknown>,
        cred
      );
      limpiar();
      await onGuardado(resultado);
    } finally {
      envio.current = false;
    }
  }
  return {
    directas,
    ot,
    otContexto,
    cliente,
    tipo,
    descuento,
    motivo,
    totales,
    lineas: mostradas,
    error,
    calculando,
    cobro,
    dirty:
      directas.length > 0 ||
      ot !== null ||
      cliente !== null ||
      descuento !== '0' ||
      motivo !== '' ||
      tipo !== 'BOLETA',
    Scliente,
    Stipo,
    Sdescuento,
    Smotivo,
    Scobro,
    agregar,
    cambiarCantidad,
    quitar,
    seleccionarOt,
    confirmar,
    limpiar,
  };
}
