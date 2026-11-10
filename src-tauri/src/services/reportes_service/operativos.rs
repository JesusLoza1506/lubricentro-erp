use super::*;

pub(super) fn stock(db: &Connection, f: &Filtros) -> R<Datos> {
    let mut d = Datos::nuevo(vec![
        "Código",
        "Producto",
        "Categoría",
        "Estado",
        "Unidad",
        "Stock",
        "Mínimo",
        "Crítico",
        "Faltante",
        "Costo promedio (S/)",
        "Valor (S/)",
    ]);
    let mut total = 0_i128;
    consulta(db, "SELECT id_producto,codigo,nombre,categoria,activo,unidad_medida,stock_actual,stock_minimo,valor_stock_micro
        FROM productos WHERE (?1='' OR instr(lower(codigo),lower(?1))>0 OR instr(lower(nombre),lower(?1))>0)
        AND (?2 IS NULL OR categoria=?2)
        AND ((activo=1 AND ?3<>'INACTIVOS') OR (activo=0 AND stock_actual>0 AND ?3<>'ACTIVOS'))
        AND (NOT ?4 OR (activo=1 AND stock_actual<=stock_minimo)) ORDER BY codigo,id_producto",
        &[&f.buscar.trim(),&f.categoria,&f.estado.as_deref().unwrap_or("AMBOS"),&f.solo_criticos], |r| {
        let id = r.get(0)?;
        let activo: bool = r.get(4)?;
        let unidad: String = r.get(5)?;
        let q: i128 = i128::from(r.get::<_, i64>(6)?);
        let minimo: i128 = i128::from(r.get::<_, i64>(7)?);
        let v: i128 = i128::from(r.get::<_, i64>(8)?);
        total += v;
        let cantidad = |q| comercial::cantidad(q,&unidad);
        let promedio = if q == 0 { Celda::NoAplica } else {
            decimal((v * if unidad == "ML" { 1000 } else { 1 } + q / 2) / q, 6)
        };
        d.agregar(Fila::nueva(vec![txt(r,1)?,txt(r,2)?,txt(r,3)?,t(if activo {"Activo"} else {"Inactivo"}),
            t(comercial::unidad_visible(&unidad)),cantidad(q),cantidad(minimo),t(if activo && q<=minimo {"Sí"} else {"No"}),
            cantidad((minimo-q).max(0)),promedio,valor(v)],[Origen::nuevo(Destino::Inventario,id)]),
            Detalle::pares(vec![("Código",txt(r,1)?),("Producto",txt(r,2)?),("Valor al corte (S/)",valor(v))]));
        Ok(())
    })?;
    d.totales = vec![
        ("Productos", n(d.filas.len())),
        ("Valor total (S/)", valor(total)),
    ];
    Ok(d)
}

pub(super) fn cierres(
    db: &Connection,
    amb: &str,
    desde: &str,
    hasta: &str,
    f: &Filtros,
) -> R<Datos> {
    let mut d = Datos::nuevo(vec![
        "Turno",
        "Titular",
        "Apertura",
        "Cierre",
        "Cerrado por",
        "Causa",
        "Esperado (S/)",
        "Contado (S/)",
        "Diferencia (S/)",
        "Fondo siguiente (S/)",
        "Retiro (S/)",
    ]);
    let mut tot = [0_i128; 5];
    consulta(db, "SELECT c.id_caja,c.codigo_caja,u.nombre_completo,c.fecha_apertura,c.fecha_cierre,a.nombre_completo,c.tipo_cierre,
      c.efectivo_esperado_cent,c.efectivo_contado_cent,c.diferencia_efectivo_cent,c.fondo_siguiente_cent,c.retiro_cierre_cent,
      c.motivo_cierre,c.aclaracion,c.tarjeta_neto_cent,c.yape_neto_cent,c.plin_neto_cent,c.transferencia_neto_cent
      FROM caja_chica c JOIN usuarios u ON u.id_usuario=c.id_usuario LEFT JOIN usuarios a ON a.id_usuario=c.id_usuario_cierre
      WHERE c.ambiente=?1 AND c.estado='CERRADA' AND c.fecha_cierre>=?2 AND c.fecha_cierre<?3
        AND (?4 IS NULL OR c.id_usuario=?4) AND (NOT ?5 OR c.diferencia_efectivo_cent<>0)
      ORDER BY c.fecha_cierre DESC,c.id_caja DESC", &[&amb,&desde,&hasta,&f.usuario,&f.con_diferencia], |r| {
        let mut c = (1..=6).map(|i| txt(r,i)).collect::<R<Vec<_>>>()?;
        for (i, s) in tot.iter_mut().enumerate() {
            let x: i64 = r.get(i+7)?; *s += i128::from(x); c.push(soles(x));
        }
        d.agregar(Fila::nueva(c,[Origen::nuevo(Destino::Caja,r.get(0)?)]), Detalle::pares(vec![
            ("Turno",txt(r,1)?),("Motivo de cierre",txt(r,12)?),("Aclaración",txt(r,13)?),
            ("Tarjeta neto (S/)",moneda(r,14)?),("Yape neto (S/)",moneda(r,15)?),
            ("Plin neto (S/)",moneda(r,16)?),("Transferencia neto (S/)",moneda(r,17)?),
        ]));
        Ok(())
    })?;
    d.totales.push(("Cierres", n(d.filas.len())));
    d.totales.extend(
        [
            "Esperado (S/)",
            "Contado (S/)",
            "Diferencia (S/)",
            "Fondo siguiente (S/)",
            "Retiro (S/)",
        ]
        .into_iter()
        .zip(tot.map(soles)),
    );
    Ok(d)
}

pub(super) fn duracion_ot(inicio: Option<&str>, fin: Option<&str>) -> Celda {
    let parse = |s: Option<&str>| {
        s.and_then(|v| NaiveDateTime::parse_from_str(v, "%Y-%m-%d %H:%M:%S").ok())
    };
    match (parse(inicio), parse(fin)) {
        (Some(a), Some(b)) if b >= a => {
            let segundos = (b - a).num_seconds();
            t(format!(
                "{}:{:02}:{:02}",
                segundos / 3600,
                segundos / 60 % 60,
                segundos % 60
            ))
        }
        _ => Celda::NoAplica,
    }
}

pub(super) fn taller(
    db: &Connection,
    desde: &str,
    hasta: &str,
    f: &Filtros,
    recibidas: bool,
) -> R<Datos> {
    let (fecha, responsable, estado) = if recibidas {
        ("fecha_ingreso", "id_mecanico", "1=1")
    } else {
        ("fecha_fin", "id_mecanico_terminal", "o.estado='FINALIZADO'")
    };
    let mut d = Datos::nuevo(vec![
        "OT",
        if recibidas {
            "Ingreso"
        } else {
            "Finalización"
        },
        "Vehículo",
        "Cliente",
        "Responsable",
        "Zanja",
        "Estado",
    ]);
    if !recibidas {
        d.columnas.extend(["Inicio", "Fin", "Duración (h:mm:ss)"]);
    }
    consulta(db, &format!("SELECT o.id_ot,o.codigo_ot,o.{fecha},o.placa_snapshot,
      coalesce(json_extract(o.cliente_snapshot,'$.nombre'),json_extract(o.cliente_snapshot,'$.nombre_razon_social'),
      json_extract(o.cliente_snapshot,'$.razon_social')),u.nombre_completo,CASE WHEN z.id_zanja IS NOT NULL THEN 'Zanja '||z.id_zanja END,o.estado,o.fecha_inicio,o.fecha_fin
      FROM ordenes_trabajo o LEFT JOIN usuarios u ON u.id_usuario=o.{responsable}
      LEFT JOIN zanjas z ON z.id_zanja=o.id_zanja_ejecucion
      WHERE {estado} AND o.{fecha}>=?1 AND o.{fecha}<?2 AND (?3 IS NULL OR o.{responsable}=?3)
      ORDER BY o.{fecha} DESC,o.id_ot DESC"), &[&desde,&hasta,&f.usuario], |r| {
        let mut c=(1..=7).map(|i| txt(r,i)).collect::<R<Vec<_>>>()?;
        let inicio: Option<String> = r.get(8)?;
        let fin: Option<String> = r.get(9)?;
        let duracion = duracion_ot(inicio.as_deref(), fin.as_deref());
        if !recibidas { c.extend([txt(r,8)?,txt(r,9)?,duracion.clone()]); }
        d.agregar(Fila::nueva(c,[Origen::nuevo(Destino::Taller,r.get(0)?)]),
            Detalle::pares(vec![("OT",txt(r,1)?),("Inicio",txt(r,8)?),("Fin",txt(r,9)?),("Duración",duracion)]));
        Ok(())
    })?;
    d.totales.push((
        if recibidas {
            "OT recibidas"
        } else {
            "OT finalizadas"
        },
        n(d.filas.len()),
    ));
    Ok(d)
}

pub(super) fn documentos(
    db: &Connection,
    amb: &str,
    desde: &str,
    hasta: &str,
    f: &Filtros,
) -> R<Datos> {
    let mut d = Datos::nuevo(vec![
        "Tipo",
        "Serie / número",
        "Emisión",
        "Venta",
        "Receptor",
        "Total (S/)",
        "Estado",
    ]);
    consulta(db, "SELECT c.tipo_comprobante,s.serie||'-'||printf('%08d',c.correlativo),c.fecha_emision,c.id_venta,
      json_extract(c.receptor_snapshot,'$.nombre'),c.total_cent,c.estado_envio,c.relacion,
      sa.serie||'-'||printf('%08d',a.correlativo),sr.serie||'-'||printf('%08d',r.correlativo),c.fecha_ultimo_intento,
      c.codigo_respuesta,c.respuesta_resumida
      FROM comprobantes c JOIN series_comprobante s ON s.id_serie=c.id_serie
      LEFT JOIN comprobantes a ON a.id_comprobante=c.id_documento_anterior LEFT JOIN series_comprobante sa ON sa.id_serie=a.id_serie
      LEFT JOIN comprobantes r ON r.id_comprobante=c.id_documento_referenciado LEFT JOIN series_comprobante sr ON sr.id_serie=r.id_serie
      WHERE c.ambiente=?1 AND c.fecha_emision>=?2 AND c.fecha_emision<?3 AND (?4 IS NULL OR c.estado_envio=?4)
      AND (?5='' OR instr(lower(s.serie||'-'||printf('%08d',c.correlativo)),lower(?5))>0
        OR instr(lower(s.serie||'-'||c.correlativo),lower(?5))>0)
      ORDER BY c.fecha_emision DESC,c.id_comprobante DESC", &[&amb,&desde,&hasta,&f.estado,&f.buscar.trim()], |r| {
        let id: i64=r.get(3)?;
        let estado: String=r.get(6)?;
        let resumen: Option<String>=r.get(12)?;
        let resumen=resumen.and_then(|s| auditoria::texto_seguro(&s)).map(|s| s.chars().take(160).collect::<String>());
        d.agregar(Fila::nueva(vec![txt(r,0)?,txt(r,1)?,txt(r,2)?,n(id),txt(r,4)?,moneda(r,5)?,t(&estado)],
            [Origen::nuevo(Destino::Pos,id)]), Detalle::pares(vec![("Documento",txt(r,1)?),("Relación",txt(r,7)?),
            ("Documento reemplazado",txt(r,8)?),("Documento referenciado",txt(r,9)?),("Último intento",txt(r,10)?),
            ("Código del resultado",txt(r,11)?),("Último resultado",dato(resumen))]));
        Ok(())
    })?;
    d.totales.push(("Comprobantes", n(d.filas.len())));
    Ok(d)
}
