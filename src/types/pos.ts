export type MedioPago = 'EFECTIVO' | 'TARJETA' | 'YAPE' | 'PLIN' | 'TRANSFERENCIA';
export interface ProductoPos {
  tipo: 'PRODUCTO' | 'SERVICIO';
  id: number;
  codigo: string;
  nombre: string;
  unidad: 'UND' | 'ML';
  precio_cent: number;
  stock: number | null;
}
export interface LineaDirecta {
  tipo: 'PRODUCTO' | 'SERVICIO';
  id: number;
  cantidad: number;
  precio_esperado_cent: number;
}
export interface PagoPos {
  medio: MedioPago;
  monto_cent: number;
  recibido_cent: number | null;
  referencia: string | null;
}
export interface VentaSolicitud {
  id_operacion: string;
  id_ot: number | null;
  id_cliente: number;
  tipo_comprobante: 'BOLETA' | 'FACTURA';
  descuento_bp: number;
  motivo: string;
  lineas: LineaDirecta[];
  pagos: PagoPos[];
}
export interface LineaPos {
  id_detalle?: number;
  id_producto: number | null;
  id_servicio?: number | null;
  id_consumo_ot?: number | null;
  id_servicio_ot?: number | null;
  codigo: string;
  nombre: string;
  unidad: 'UND' | 'ML';
  cantidad: number;
  precio_cent: number;
  total_cent: number;
  base_cent: number;
  igv_cent?: number;
  devuelta?: number;
  recuperada?: number;
  origen?: string;
}
export interface TotalesPos {
  ot?: OtPos;
  bruto_cent: number;
  descuento_cent: number;
  base_cent: number;
  igv_cent: number;
  total_cent: number;
  lineas: LineaPos[];
}
export interface DocumentoPos {
  id: number;
  tipo: string;
  serie: string;
  numero: number;
  estado: string;
  relacion: string;
  incierto: number;
  requiere_consulta: number;
  intento: number;
  mensaje: string | null;
  advertencias?: string[];
  pdf: string | null;
  xml: string | null;
  cdr: string | null;
  reemplazado: number;
  fecha: string;
}
export interface ReceptorPos {
  id?: number;
  tipo: string | null;
  documento: string | null;
  nombre: string;
  direccion: string | null;
  varios?: number;
}
export interface DetalleVentaPos extends TotalesPos {
  id: number;
  id_ot: number | null;
  id_emisor: number;
  emisor: string;
  caja: string;
  fecha: string;
  tipo: string;
  receptor: ReceptorPos;
  devuelto_cent: number;
  pagos: (PagoPos & { vuelto_cent: number | null })[];
  documentos: DocumentoPos[];
  devoluciones: { id: number; fecha: string; total_cent: number; motivo: string }[];
}
export interface InicioPos {
  ambiente: string;
  empresa: string | null;
  configurada: boolean;
  caja_propia: number | null;
  caja_ocupada: boolean;
  hoy: string;
  series: { id: number; tipo: string; serie: string }[];
  caja: {
    id: number;
    codigo: string;
    titular: string;
    propia: number;
    efectivo_cent: number;
  } | null;
}
export interface FilaVentaPos {
  id: number;
  fecha: string;
  cliente: string;
  total_cent: number;
  devuelto_cent: number;
  id_ot: number | null;
  emisor: string;
  estado: string;
}
export interface OtPos {
  total_cent: number;
  id: number;
  codigo: string;
  placa: string;
  cliente: string;
  fecha: string;
}
export interface LineaDevolucionPos {
  id_detalle: number;
  cantidad: number;
  recuperada: number;
}
export interface DevolucionSolicitud {
  id_operacion: string;
  id_venta: number;
  motivo: string;
  medio: MedioPago;
  referencia: string | null;
  lineas: LineaDevolucionPos[];
}
export interface ResultadoPos {
  token_envio?: string;
  id_venta: number;
  id_documento: number;
  total_cent?: number;
  id_devolucion?: number;
}
export interface CredencialesPos {
  username: string;
  password: string;
}
export interface PaginaPos<T> {
  filas: T[];
  total: number;
}
