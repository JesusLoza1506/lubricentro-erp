export interface CajaTurno {
  id_caja: number;
  codigo_caja: string;
  id_usuario: number;
  titular: string;
  ambiente: string;
  monto_apertura_cent: number;
  estado: 'ABIERTA' | 'CERRADA';
  por_conciliar: boolean;
  fecha_apertura: string;
  fecha_cierre: string | null;
  quien_cerro: string | null;
  tipo_cierre: CajaModo | null;
  motivo_cierre: string | null;
  efectivo_esperado_cent: number | null;
  efectivo_contado_cent: number | null;
  diferencia_efectivo_cent: number | null;
  fondo_siguiente_cent: number | null;
  retiro_cierre_cent: number | null;
  aclaracion: string | null;
  autor_aclaracion: string | null;
  fecha_aclaracion: string | null;
  version: number;
}
export type CajaModo = 'NORMAL' | 'AUSENCIA' | 'RECUPERACION';
interface TotalesMedio {
  cobros_cent: number;
  reintegros_cent: number;
  neto_cent: number;
}
export interface CajaResumen {
  efectivo_esperado_cent: number;
  ingresos_manuales_cent: number;
  egresos_manuales_cent: number;
  efectivo: TotalesMedio;
  tarjeta: TotalesMedio;
  yape: TotalesMedio;
  plin: TotalesMedio;
  transferencia: TotalesMedio;
}
export interface CajaDetalle {
  turno: CajaTurno;
  resumen: CajaResumen;
  puede_mover: boolean;
  modo_cierre: CajaModo | null;
  puede_aclarar: boolean;
}
export interface CajaInicio {
  ambiente: string;
  hoy: string;
  ocupada: boolean;
  puede_abrir: boolean;
  recuperacion: boolean;
  fondo_sugerido_cent: number;
  detalle: CajaDetalle | null;
  responsables: { id: number; nombre: string }[];
}
export interface CajaMovimiento {
  id_movimiento: number;
  tipo: 'COBRO' | 'REINTEGRO' | 'INGRESO_MANUAL' | 'EGRESO_MANUAL' | 'REVERSO_MANUAL';
  medio_pago: string;
  monto_cent: number;
  signo: 1 | -1;
  fecha: string;
  concepto: string;
  categoria: string | null;
  referencia: string | null;
  motivo: string | null;
  id_venta: number | null;
  id_devolucion: number | null;
  id_movimiento_original: number | null;
  id_reverso: number | null;
  ejecutor: string;
  autorizante: string | null;
}
export interface CajaPagina<T> {
  datos: T[];
  total: number;
  pagina: number;
  por_pagina: number;
}
