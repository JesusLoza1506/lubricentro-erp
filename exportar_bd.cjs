const { DatabaseSync } = require('node:sqlite');
const fs = require('fs');
const path = require('path');

const dbPath = path.join(process.env.APPDATA, 'com.lubricentro.app', 'lubricentro_secure.db');

if (!fs.existsSync(dbPath)) {
  console.error('❌ No se encontró el archivo de base de datos en:', dbPath);
  process.exit(1);
}

try {
  const db = new DatabaseSync(dbPath);
  let dump = '-- DUMP COMPLETO DE BASE DE DATOS LUBRICENTRO\n\n';

  const tables = db
    .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'")
    .all();

  tables.forEach((t) => {
    const tableName = t.name;
    const rows = db.prepare(`SELECT * FROM ${tableName}`).all();

    dump += `-- ==========================================\n`;
    dump += `-- TABLA: ${tableName} (${rows.length} registros)\n`;
    dump += `-- ==========================================\n`;

    if (rows.length > 0) {
      rows.forEach((row) => {
        const keys = Object.keys(row);
        const vals = keys.map((k) => {
          const v = row[k];
          if (v === null) return 'NULL';
          if (typeof v === 'string') return "'" + v.replace(/'/g, "''") + "'";
          return v;
        });
        dump += `INSERT INTO ${tableName} (${keys.join(', ')}) VALUES (${vals.join(', ')});\n`;
      });
    } else {
      dump += `-- (Tabla vacía)\n`;
    }
    dump += '\n';
  });

  fs.writeFileSync('dump_completo_bd.sql', dump, 'utf8');
  console.log('✅ EXPORTACIÓN EXITOSA: Se generó dump_completo_bd.sql en la raíz del proyecto.');
} catch (error) {
  console.error('Error al exportar la base de datos:', error);
}
