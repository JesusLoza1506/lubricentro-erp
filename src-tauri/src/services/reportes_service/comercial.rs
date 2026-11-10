use super::*;
use std::collections::BTreeMap;

pub const MEDIOS: [&str; 5] = ["EFECTIVO", "TARJETA", "YAPE", "PLIN", "TRANSFERENCIA"];
pub fn medio(v: &str) -> &str {
    match v {
        "EFECTIVO" => "Efectivo",
        "TARJETA" => "Tarjeta",
        "YAPE" => "Yape",
        "PLIN" => "Plin",
        "TRANSFERENCIA" => "Transferencia",
        _ => v,
    }
}

const VENTAS: [&str; 9] = [
    "Ventas",
    "Bruto (S/)",
    "Descuentos (S/)",
    "Vendido (S/)",
    "Devuelto (S/)",
    "Neto (S/)",
    "Base neta (S/)",
    "IGV neto (S/)",
    "Devoluciones",
];
fn ventas_celdas(v: [i128; 9]) -> Vec<Celda> {
    v.into_iter()
        .enumerate()
        .map(|(i, x)| if i == 0 || i == 8 { n(x) } else { soles(x) })
        .collect()
}

pub(super) fn ventas(db: &Connection, amb: &str, desde: &str, hasta: &str) -> R<Datos> {
    let mut grupos: BTreeMap<String, ([i128; 9], Vec<Fila>)> = BTreeMap::new();
    let mut total = [0_i128; 9];
    consulta(db, "SELECT fecha_operacion,id_venta,id_devolucion,bruto_cent,descuento_cent,base_cent,igv_cent,total_cent
        FROM v_operaciones_comerciales WHERE ambiente=?1 AND fecha_operacion>=?2 AND fecha_operacion<?3
        ORDER BY fecha_operacion,id_venta,id_devolucion", &[&amb,&desde,&hasta], |r| {
        let fecha: String = r.get(0)?;
        let dia = fecha.get(..10).ok_or_else(|| error("Fecha económica no válida."))?;
        let id: i64 = r.get(1)?;
        let dev: Option<i64> = r.get(2)?;
        let bruto: i64 = r.get(3)?;
        let dscto: i64 = r.get(4)?;
        let base: i64 = r.get(5)?;
        let igv: i64 = r.get(6)?;
        let imp: i64 = r.get(7)?;
        let valores = [i128::from(dev.is_none()), bruto.into(), dscto.into(),
            if dev.is_none() { imp.into() } else { 0 }, if dev.is_some() { -i128::from(imp) } else { 0 },
            imp.into(), base.into(), igv.into(), i128::from(dev.is_some())];
        let (suma, filas) = grupos.entry(dia.into()).or_default();
        for (i, x) in valores.iter().enumerate() { suma[i] += x; total[i] += x; }
        filas.push(Fila::nueva(vec![t(fecha), n(id), dev.map_or(Celda::NoAplica,n), soles(imp), soles(base), soles(igv)],
            [Origen::nuevo(Destino::Pos,id)]));
        Ok(())
    })?;
    let mut d = Datos::nuevo([vec!["Fecha (Lima)"], VENTAS.to_vec()].concat());
    d.totales = VENTAS.into_iter().zip(ventas_celdas(total)).collect();
    for (dia, (sumas, filas)) in grupos {
        d.agregar(
            Fila::nueva([vec![t(dia)], ventas_celdas(sumas)].concat(), []),
            Detalle {
                columnas: vec![
                    "Fecha y hora (Lima)",
                    "Venta",
                    "Devolución",
                    "Importe (S/)",
                    "Base (S/)",
                    "IGV (S/)",
                ],
                filas,
            },
        );
    }
    Ok(d)
}

const COBROS: [&str; 4] = ["Movimientos", "Cobrado (S/)", "Devuelto (S/)", "Neto (S/)"];
fn cobros_celdas(s: [i128; 4]) -> Vec<Celda> {
    vec![n(s[0]), soles(s[1]), soles(s[2]), soles(s[3])]
}

pub(super) fn cobros(
    db: &Connection,
    amb: &str,
    desde: &str,
    hasta: &str,
    filtro: Option<&str>,
) -> R<Datos> {
    let mut grupos: BTreeMap<String, ([i128; 4], Vec<Fila>)> = BTreeMap::new();
    let mut total = [0_i128; 4];
    consulta(db, "SELECT m.medio_pago,m.signo,m.monto_cent,m.fecha_operacion,
        coalesce(m.id_venta,d.id_venta),m.id_devolucion,c.codigo_caja,u.nombre_completo,a.nombre_completo,c.id_caja
        FROM movimientos_caja m JOIN caja_chica c ON c.id_caja=m.id_caja
        JOIN usuarios u ON u.id_usuario=m.id_usuario LEFT JOIN ventas v ON v.id_venta=m.id_venta
        LEFT JOIN devoluciones d ON d.id_devolucion=m.id_devolucion
        LEFT JOIN usuarios a ON a.id_usuario=coalesce(d.id_autorizante,v.id_autorizante)
        WHERE c.ambiente=?1 AND m.fecha_operacion>=?2 AND m.fecha_operacion<?3
          AND (?4 IS NULL OR m.medio_pago=?4)
          AND ((m.tipo='COBRO' AND v.estado_registro='CONFIRMADA')
            OR (m.tipo='REINTEGRO' AND d.estado_registro='CONFIRMADA'))
        ORDER BY m.fecha_operacion,m.id_movimiento", &[&amb,&desde,&hasta,&filtro], |r| {
        let signo: i64 = r.get(1)?;
        let imp: i128 = i128::from(r.get::<_, i64>(2)?);
        let id: i64 = r.get(4)?;
        let caja: i64 = r.get(9)?;
        let neto = imp * i128::from(signo);
        let (suma, filas) = grupos.entry(r.get(0)?).or_default();
        for (i, x) in [1, if signo > 0 { imp } else { 0 }, if signo < 0 { imp } else { 0 }, neto].into_iter().enumerate() {
            suma[i] += x; total[i] += x;
        }
        filas.push(Fila::nueva(vec![txt(r,3)?, n(id), r.get::<_,Option<i64>>(5)?.map_or(Celda::NoAplica,n),
            txt(r,6)?, txt(r,7)?, txt(r,8)?, soles(neto)], [Origen::nuevo(Destino::Pos,id), Origen::nuevo(Destino::Caja,caja)]));
        Ok(())
    })?;
    let mut d = Datos::nuevo([vec!["Medio de pago"], COBROS.to_vec()].concat());
    d.totales = COBROS.into_iter().zip(cobros_celdas(total)).collect();
    for m in MEDIOS {
        if let Some((s, filas)) = grupos.remove(m) {
            d.agregar(
                Fila::nueva([vec![t(medio(m))], cobros_celdas(s)].concat(), []),
                Detalle {
                    columnas: vec![
                        "Fecha y hora (Lima)",
                        "Venta",
                        "Devolución",
                        "Turno",
                        "Ejecutor",
                        "Autorizante",
                        "Aplicado (S/)",
                    ],
                    filas,
                },
            );
        }
    }
    Ok(d)
}

#[derive(Default)]
struct Item {
    codigo: String,
    nombre: String,
    fecha: String,
    cantidad: i128,
    base: i128,
    costo: i128,
    filas: Vec<Fila>,
}
pub(super) fn margen(
    db: &Connection,
    amb: &str,
    desde: &str,
    hasta: &str,
    servicio: bool,
    todos: bool,
) -> R<Datos> {
    let mut items: BTreeMap<(i64, String), Item> = BTreeMap::new();
    let columna = if servicio {
        "id_servicio"
    } else {
        "id_producto"
    };
    consulta(db, &format!("SELECT l.{columna},l.codigo_snapshot,l.nombre_snapshot,l.unidad_medida,
        l.cantidad,l.base_cent,coalesce(l.costo_total_micro,0),v.fecha_venta,v.id_venta,NULL
        FROM ventas v JOIN detalle_venta l ON l.id_venta=v.id_venta
        WHERE v.ambiente=?1 AND v.fecha_venta>=?2 AND v.fecha_venta<?3
          AND v.estado_registro='CONFIRMADA' AND l.{columna} IS NOT NULL
        UNION ALL SELECT l.{columna},l.codigo_snapshot,l.nombre_snapshot,l.unidad_medida,
        -dd.cantidad,-dd.base_cent,-dd.costo_revertido_micro,d.fecha_operacion,v.id_venta,d.id_devolucion
        FROM devoluciones d JOIN ventas v ON v.id_venta=d.id_venta
        JOIN detalle_devolucion dd ON dd.id_devolucion=d.id_devolucion
        JOIN detalle_venta l ON l.id_detalle=dd.id_detalle_venta
        WHERE v.ambiente=?1 AND d.fecha_operacion>=?2 AND d.fecha_operacion<?3
          AND d.estado_registro='CONFIRMADA' AND l.{columna} IS NOT NULL
        ORDER BY 8,9,10,1,2,3"), &[&amb,&desde,&hasta], |r| {
        let id: i64 = r.get(0)?;
        let unidad: String = r.get(3)?;
        let q: i64 = r.get(4)?;
        let base: i64 = r.get(5)?;
        let costo: i64 = r.get(6)?;
        let fecha: String = r.get(7)?;
        let venta: i64 = r.get(8)?;
        let item = items.entry((id, unidad.clone())).or_default();
        if fecha >= item.fecha {
            item.codigo = r.get(1)?;
            item.nombre = r.get(2)?;
            item.fecha = fecha.clone();
        }
        item.cantidad += i128::from(q); item.base += i128::from(base); item.costo += i128::from(costo);
        item.filas.push(Fila::nueva(vec![t(fecha), n(venta),r.get::<_,Option<i64>>(9)?.map_or(Celda::NoAplica,n),
            cantidad(q.into(),&unidad), t(unidad_visible(&unidad)), soles(base), if servicio { Celda::NoAplica } else { valor(costo.into()) }],
            [Origen::nuevo(Destino::Pos,venta)]));
        Ok(())
    })?;
    let mut items: Vec<_> = items.into_iter().collect();
    items.sort_by(|a, b| b.1.base.cmp(&a.1.base).then(a.0.cmp(&b.0)));
    let mut d = Datos::nuevo(vec![
        "Código",
        "Producto / servicio",
        "Unidad",
        "Cantidad neta",
        "Ingreso sin IGV (S/)",
        "Costo reconocido (S/)",
        "Margen (S/)",
    ]);
    d.disponibles = Some(items.len());
    if !todos {
        items.truncate(10);
    }
    let (mut base, mut costo, mut und, mut ml) = (0, 0, 0, 0);
    for ((_, unidad), item) in items {
        base += item.base;
        costo += item.costo;
        if unidad == "ML" {
            ml += item.cantidad;
        } else {
            und += item.cantidad;
        }
        d.agregar(
            Fila::nueva(
                vec![
                    t(item.codigo),
                    t(item.nombre),
                    t(unidad_visible(&unidad)),
                    cantidad(item.cantidad, &unidad),
                    soles(item.base),
                    if servicio {
                        Celda::NoAplica
                    } else {
                        valor(item.costo)
                    },
                    if servicio {
                        Celda::NoAplica
                    } else {
                        valor(item.base * 10_000 - item.costo)
                    },
                ],
                [],
            ),
            Detalle {
                columnas: vec![
                    "Fecha y hora (Lima)",
                    "Venta",
                    "Devolución",
                    "Cantidad",
                    "Unidad",
                    "Ingreso sin IGV (S/)",
                    "Costo reconocido (S/)",
                ],
                filas: item.filas,
            },
        );
    }
    d.totales.push((
        if todos {
            "Total del resultado sin IGV (S/)"
        } else {
            "Subtotal Top 10 sin IGV (S/)"
        },
        soles(base),
    ));
    d.totales
        .extend([("Cantidad (UND)", n(und)), ("Cantidad (L)", decimal(ml, 3))]);
    if !servicio {
        d.totales.extend([
            ("Costo reconocido (S/)", valor(costo)),
            ("Margen (S/)", valor(base * 10_000 - costo)),
        ]);
    }
    Ok(d)
}
pub(super) fn unidad_visible(u: &str) -> &str {
    if u == "ML" {
        "L"
    } else {
        u
    }
}
pub(super) fn cantidad(q: i128, u: &str) -> Celda {
    if u == "ML" {
        decimal(q, 3)
    } else {
        n(q)
    }
}
