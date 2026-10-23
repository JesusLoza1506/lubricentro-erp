use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServicioOtDto {
    pub id_detalle: i64,
    pub id_servicio: i64,
    pub codigo_snapshot: String,
    pub nombre_snapshot: String,
    pub cantidad: i32,
    pub precio_unitario_cent: i64,
    pub precio_unitario_soles: f64,
    pub subtotal_cent: i64,
    pub subtotal_soles: f64,
    pub es_cambio_aceite: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InsumoOtDto {
    pub id_detalle: i64,
    pub id_producto: i64,
    pub codigo_snapshot: String,
    pub nombre_snapshot: String,
    pub unidad_medida: String,    // "UND" | "ML"
    pub cantidad: i64,            // Unidades enteras o ML
    pub cantidad_formateada: f64, // Unidades o Litros
    pub precio_unitario_cent: i64,
    pub precio_unitario_soles: f64,
    pub subtotal_cent: i64,
    pub subtotal_soles: f64,
    pub devuelto_ml_und: i64,
    pub neto_ml_und: i64,
    pub costo_unitario_micro: Option<i64>, // Exclusivo ADMIN
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrdenTrabajoHistorialDto {
    pub id_ot: i64,
    pub codigo_ot: String,
    pub id_vehiculo: i64,
    pub placa: String,
    pub id_cliente_atendido: i64,
    pub cliente_nombre: String,
    pub cliente_documento: Option<String>,
    pub id_creador: i64,
    pub nombre_creador: String,
    pub id_mecanico: Option<i64>,
    pub nombre_mecanico: Option<String>,
    pub id_mecanico_terminal: Option<i64>,
    pub id_zanja_ejecucion: Option<i32>,
    pub estado: String,
    pub kilometraje_ingreso: i64,
    pub kilometraje_salida: Option<i64>,
    pub pedido_cliente: Option<String>,
    pub resumen_publico: Option<String>,
    pub nota_interna: Option<String>, // Oculto para Cajero
    pub tipo_aceite: Option<String>,
    pub intervalo_aceite_km: Option<i32>,
    pub km_proximo: Option<i64>,
    pub corregido: i32,
    pub fecha_ingreso: String,
    pub fecha_inicio: Option<String>,
    pub fecha_fin: Option<String>,
    pub fecha_cancelacion: Option<String>,
    pub motivo_cancelacion: Option<String>,
    pub servicios: Vec<ServicioOtDto>,
    pub insumos: Vec<InsumoOtDto>,
    pub total_servicios_cent: i64,
    pub total_insumos_cent: i64,
    pub total_general_cent: i64,
    pub total_general_soles: f64,
}

#[derive(Debug, Deserialize)]
pub struct ServicioInicialDto {
    pub id_servicio: i64,
    pub cantidad: i32,
}

#[derive(Debug, Deserialize)]
pub struct CrearOrdenTrabajoDto {
    pub id_vehiculo: i64,
    pub id_cliente_atendido: i64,
    pub kilometraje_ingreso: i64,
    pub id_mecanico: Option<i64>, // Opcional en recepción
    pub pedido_cliente: Option<String>,
    pub servicios_iniciales: Vec<ServicioInicialDto>, // Mínimo 1
}

#[derive(Debug, Deserialize)]
pub struct IniciarTrabajoOtDto {
    pub id_ot: i64,
    pub id_zanja_ejecucion: i32,
}

#[derive(Debug, Deserialize)]
pub struct ReasignarMecanicoOtDto {
    pub id_ot: i64,
    pub nuevo_id_mecanico: i64,
    pub motivo: String, // 5 a 250 caracteres
}

#[derive(Debug, Deserialize)]
pub struct RegistrarInsumoOtDto {
    pub id_ot: i64,
    pub id_producto: i64,
    pub cantidad: f64, // Unidades o Litros desde el frontend
}

#[derive(Debug, Deserialize)]
pub struct DevolverInsumoOtDto {
    pub id_ot: i64,
    pub id_detalle_producto: i64,
    pub cantidad_a_devolver: f64, // Unidades o Litros
    pub motivo: String,           // 5 a 250 caracteres
}

#[derive(Debug, Deserialize)]
pub struct AgregarServicioOtDto {
    pub id_ot: i64,
    pub id_servicio: i64,
    pub cantidad: i32,
}

#[derive(Debug, Deserialize)]
pub struct ModificarServicioOtDto {
    pub id_ot: i64,
    pub id_detalle_servicio: i64,
    pub nueva_cantidad: i32,
}

#[derive(Debug, Deserialize)]
pub struct FinalizarOtDto {
    pub id_ot: i64,
    pub kilometraje_salida: i64,
    pub resumen_publico: String,
    pub tipo_aceite: Option<String>, // Obligatorio si hubo cambio de aceite
}

#[derive(Debug, Deserialize)]
pub struct CancelarOtDto {
    pub id_ot: i64,
    pub motivo: String, // 5 a 250 caracteres
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZanjaEstadoDto {
    pub id_zanja: i32,
    pub activa: i32,
    pub ocupada: bool,
    pub ot_actual: Option<OrdenTrabajoHistorialDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableroZanjasDto {
    pub zanjas: Vec<ZanjaEstadoDto>,
    pub cola_espera: Vec<OrdenTrabajoHistorialDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicioResumenDto {
    pub id_servicio: i64,
    pub codigo: String,
    pub nombre: String,
    pub precio_venta_cent: i64,
    pub precio_venta_soles: f64,
    pub es_cambio_aceite: i32,
}
