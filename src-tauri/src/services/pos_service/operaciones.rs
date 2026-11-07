use crate::{
    dto::{pos_dto::CredencialesPos, usuario_dto::AutorizarDescuentoRequestDto},
    errors::AppError,
    services::{
        auth_service::AuthService,
        inventario_service::idempotencia::IdempotenciaService,
        pos_service::{self as pos, consultas, devoluciones, documentos, ventas},
        session_manager::SessionState,
    },
};
use rusqlite::{params, Connection, OptionalExtension};
use serde_json::{json, Value};
pub fn ejecutar(
    conn: &mut Connection,
    session: &mut SessionState,
    accion: &str,
    payload: Value,
    credenciales: Option<CredencialesPos>,
) -> Result<Value, AppError> {
    let actor = session.validar_sesion_operativa(conn)?;
    let rol = pos::validar_rol(conn, actor)?;
    let admin = if rol == "CAJERO"
        && ((accion == "confirmar" && payload["descuento_bp"].as_i64().unwrap_or(0) > 500)
            || accion == "devolver")
    {
        // Un resultado previo idéntico se recupera sin consumir otra autorización.
        let op = payload["id_operacion"].as_str().unwrap_or("");
        let previo:Option<String>=conn.query_row("SELECT solicitud_sha256 FROM solicitudes_operacion WHERE id_operacion=?1 AND id_usuario=?2",params![op,actor],|r|r.get(0)).optional()?;
        if previo.is_some() {
            None
        } else {
            let c = credenciales.ok_or_else(|| {
                pos::error("Esta operación requiere autorización administrativa.")
            })?;
            match AuthService::autorizar_descuento(
                conn,
                session,
                AutorizarDescuentoRequestDto {
                    username_admin: c.username,
                    password_admin: c.password,
                    referencia_venta: IdempotenciaService::calcular_sha256(&payload)?,
                    descuento_bp: payload["descuento_bp"].as_i64().unwrap_or(0) as i32,
                },
            ) {
                Ok(id) => Some(id),
                Err(e) => {
                    pos::auditar_denegacion(conn, actor, accion, op)?;
                    return Err(e);
                }
            }
        }
    } else {
        None
    };
    match accion {
        "inicio" => consultas::inicio(conn, actor),
        "catalogo" => consultas::catalogo(
            conn,
            actor,
            payload["buscar"].as_str().unwrap_or(""),
            payload["tipo"].as_str().unwrap_or("TODOS"),
            payload["pagina"].as_i64().unwrap_or(1),
        ),
        "ots" => consultas::ots(
            conn,
            actor,
            payload["buscar"].as_str().unwrap_or(""),
            payload["pagina"].as_i64().unwrap_or(1),
        ),
        "listado" => consultas::listado(conn, actor, &payload),
        "detalle" => consultas::detalle(conn, actor, payload["id"].as_i64().unwrap_or(0)),
        "cotizar" => ventas::cotizar(
            conn,
            actor,
            &serde_json::from_value(payload).map_err(pos::json_error)?,
        ),
        "confirmar" => {
            let req: crate::dto::pos_dto::ConfirmarVentaPos =
                serde_json::from_value(payload).map_err(pos::json_error)?;
            let previa: bool = conn.query_row(
                "SELECT EXISTS(SELECT 1 FROM solicitudes_operacion WHERE id_operacion=?1)",
                [&req.id_operacion],
                |r| r.get(0),
            )?;
            let mut resultado = ventas::confirmar(conn, actor, admin, &req)?;
            if !previa && rol == "CAJERO" {
                let documento = resultado["id_documento"]
                    .as_i64()
                    .ok_or_else(|| pos::error("No se encontró el documento guardado."))?;
                let token = uuid::Uuid::new_v4().to_string();
                session.guardar_autorizacion(
                    token.clone(),
                    actor,
                    actor,
                    "POS_ENVIO_INICIAL".into(),
                    documento.to_string(),
                );
                resultado["token_envio"] = json!(token);
            }
            Ok(resultado)
        }
        "cotizar_devolucion" => devoluciones::cotizar(
            conn,
            actor,
            &serde_json::from_value(payload).map_err(pos::json_error)?,
        ),
        "devolver" => {
            let req: crate::dto::pos_dto::DevolverVentaPos =
                serde_json::from_value(payload).map_err(pos::json_error)?;
            let previa: bool = conn.query_row(
                "SELECT EXISTS(SELECT 1 FROM solicitudes_operacion WHERE id_operacion=?1)",
                [&req.id_operacion],
                |r| r.get(0),
            )?;
            let mut resultado = devoluciones::devolver(conn, actor, admin, &req)?;
            if !previa && rol == "CAJERO" {
                let documento = resultado["id_documento"]
                    .as_i64()
                    .ok_or_else(|| pos::error("No se encontró el documento guardado."))?;
                let token = uuid::Uuid::new_v4().to_string();
                session.guardar_autorizacion(
                    token.clone(),
                    actor,
                    actor,
                    "POS_ENVIO_INICIAL".into(),
                    documento.to_string(),
                );
                resultado["token_envio"] = json!(token);
            }
            Ok(resultado)
        }
        "reemitir" => documentos::reemitir(
            conn,
            actor,
            &serde_json::from_value(payload).map_err(pos::json_error)?,
        ),
        "pendientes" => {
            if rol != "ADMINISTRADOR" {
                return Err(AppError::PermisoDenegado);
            }
            Ok(json!(consultas::filas(conn,"SELECT json_object('id',id_comprobante) FROM comprobantes c WHERE estado_envio IN ('PENDIENTE','ERROR','ENVIANDO') AND NOT EXISTS(SELECT 1 FROM comprobantes x WHERE x.id_documento_anterior=c.id_comprobante) ORDER BY fecha_emision,id_comprobante LIMIT 20",&[])?))
        }
        _ => Err(pos::error("Operación POS desconocida.")),
    }
}
