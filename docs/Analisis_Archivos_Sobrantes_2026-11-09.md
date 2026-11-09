# Análisis de archivos sobrantes — 9 de noviembre de 2026

No se eliminó ni modificó código del proyecto. Esta revisión agrega este informe y un inventario CSV.

## Alcance y criterio

Se inventariaron 4.939 archivos propios y auxiliares, incluidos archivos ocultos, antes de crear este informe. Se excluyeron del análisis de contenido `.git`, dependencias instaladas y resultados de compilación; estos últimos se midieron por separado. Se analizaron las conexiones de los 142 archivos de `src`, los módulos de los 91 archivos Rust de `src-tauri/src`, los puntos de entrada, las configuraciones, las pruebas, las referencias documentales y el manifiesto de carga de datos.

«Sin uso» significa sin consumidores encontrados en este repositorio y su configuración actual. No prueba que una persona no ejecute una herramienta manualmente o use archivos desde fuera del repositorio. Un archivo que solo sirve para pruebas, documentación, herramientas o respaldos no es automáticamente basura.

## 1. Candidatos claros para eliminar sin modificar la aplicación actual

| Archivo | Evidencia |
|---|---|
| `src/assets/logo.png` | Copia idéntica, comprobada por SHA-256, de `src/assets/logo-gloria-login.png`. Login y Sidebar importan el segundo; el primero no tiene consumidores. |
| `src/assets/react.svg` | Recurso de plantilla sin importaciones ni referencias encontradas. |
| `public/tauri.svg` | Recurso de plantilla sin referencias encontradas. Vite lo copia a la distribución por estar en `public`, pero la aplicación no lo solicita. |
| `src/components/ReautenticarModal.tsx` | No se importa desde la aplicación ni desde las pruebas. Esto no significa que el servicio de reautenticación sobre: el servicio sigue usándose. |
| `src/pages/inventario/components/ModalDespacho.tsx` | Componente sin consumidores en el grafo de importaciones ni en las pruebas. |
| `src/pages/ordenes/components/modalDetalle/index.ts` | Reexportaciones sin consumidores. Los componentes se importan directamente desde sus archivos. Eliminar solamente este `index.ts`, no la carpeta. |
| `src/pages/ordenes/ordenesStyles.ts` | Archivo de estilos sin importaciones; la página utiliza su implementación visual actual. |
| `src/pages/vehiculo/vehiculoStyles.ts` | Archivo de estilos sin importaciones desde aplicación o pruebas. |
| `src/styles/loginStyles.ts` | Archivo de estilos sin importaciones desde aplicación o pruebas. |
| `src/types/types.ts` | Definiciones sin importaciones; no confundir con los otros archivos activos de `src/types`. |
| `src-tauri/tarpaulin-report.html` | Informe de cobertura ya generado; no interviene en compilación, ejecución o pruebas configuradas. Se pierde el informe histórico si se elimina. |
| `docs/.gitkeep` | Marcador innecesario: la carpeta ya tiene contenido versionado. |
| `src/components/.gitkeep` | Marcador innecesario: la carpeta ya tiene contenido versionado. |
| `src/pages/.gitkeep` | Marcador innecesario: la carpeta ya tiene contenido versionado. |
| `src-tauri/migrations/.gitkeep` | Marcador innecesario: existe la migración SQL real. Conservar esa migración. |
| `tests/.gitkeep` | Único contenido de la carpeta `tests` de la raíz; no contiene pruebas ejecutables. Eliminarlo hace que Git deje de conservar esa carpeta vacía. Las pruebas reales están en `src` y `src-tauri/tests`. |

Son 16 archivos candidatos. Siete son módulos TypeScript/TSX sin consumidores. Su eliminación no requiere eliminar las funciones o dependencias que esos módulos importan, porque estas pueden tener otros consumidores.

## 2. Limpieza de archivos regenerables

Estas carpetas sí participan en el trabajo del proyecto. Se pueden limpiar cuando no hay compilaciones, servidores o aplicaciones del proyecto en ejecución; posteriormente hay que regenerarlas.

| Carpeta | Archivos observados | Tamaño observado | Consecuencia de limpiarla |
|---|---:|---:|---|
| `src-tauri/target` | 23.308 | 23.283.035.138 bytes, aproximadamente 21,68 GiB | Cargo recompila el backend. También se pierden ejecutables e instaladores que estén dentro. Conservar fuera cualquier entrega necesaria. |
| `node_modules` | 7.852 | 180.324.051 bytes, aproximadamente 171,97 MiB | Hay que reinstalar con `npm ci`; puede requerir conexión. |
| `dist` | 11 | 1.224.308 bytes, aproximadamente 1,17 MiB | Hay que ejecutar `npm run build` para volver a previsualizar o empaquetar el frontend. Tauri declara `frontendDist: ../dist`. |
| `src-tauri/gen/schemas` | 4 | 331.134 bytes, aproximadamente 323,37 KiB | Tauri regenera los esquemas. `capabilities/default.json` referencia uno para validación/autocompletado. |

El mayor ahorro está en `src-tauri/target`, no en borrar componentes. No borrar `.git`: contiene el historial y la información del repositorio.

## 3. Archivos externos al arranque normal: archivar o eliminar solo si ya no necesitas su función

| Ruta | Función real y condición |
|---|---|
| `ingesta_peru_v8/` | Herramienta manual para cargar datos generados. Contiene 4.580 archivos y ocupa 9.270.341 bytes. No se invoca desde la aplicación, npm, CI ni el empaquetado actual. Sin embargo, sus 4.571 SQL sí están declarados en `plan.json`, todos existen y todos coinciden con sus hashes. No son SQL huérfanos: conservar el paquete completo si quieres repetir o auditar la carga. Retirarlo del repositorio no elimina los datos que ya se hayan cargado en una BD externa. |
| `exportar_bd.cjs` | Herramienta ejecutable manualmente con Node para leer la BD de AppData y generar `dump_completo_bd.sql`. No forma parte de los scripts npm ni del arranque de Tauri. Conservar si usas esa exportación. No se ejecutó durante esta revisión. |
| `dump_completo_bd.sql` | Salida del exportador anterior. No es la migración de la aplicación ni se carga automáticamente. Eliminar solo si no necesitas esos datos exportados. |
| `lubricentro_demo_poblada.db` | Base de datos auxiliar local, de 2.322.432 bytes. No hay referencia a ella como base activa de la aplicación. Conservar si usas su contenido para consultas o pruebas manuales; no se abrió ni se alteró. |
| `lubricentro_demo_poblada.db.validacion.json` | Informe asociado a datos de demostración. Está mencionado en `docs/Reevaluacion_Ficha_Vehicular_v7_2026-10-18.md`. Eliminarlo afecta esa evidencia documental, aunque no el arranque. |
| `Respaldo_Original_20261018_143904/` | Contiene `lubricentro_secure.db`, `lubricentro_secure.db-shm` y `lubricentro_secure.db-wal`, con 696.320 bytes en total. No se encontró uso desde la aplicación, pero es un respaldo de datos. No recomendar borrarlo sin verificar que tienes otra copia íntegra y recuperable. No se validó la integridad de esa BD. |
| `docs/diagnosticos-*/` | Diez archivos `.rs.txt` y `.test.tsx.txt` conservados como evidencia de revisiones. Las pruebas normales no ejecutan estos textos; los informes explican cómo reutilizarlos. Archivarlos junto con los informes si no quieres conservar ese historial dentro del repositorio. |
| Informes de auditoría y reevaluación de `docs/` | Historial técnico, no código de ejecución. Su antigüedad no demuestra que sobren. Hay referencias entre informes y desde README. |

El backend obtiene su directorio de datos mediante `app.path().app_data_dir()`. En modo `PRUEBA` utiliza una subcarpeta `entornos/prueba`; el servicio de backup puede seleccionar generaciones activas. Por eso no se debe confundir una copia `.db` en la raíz del repositorio con la BD activa, ni borrar bases reales en AppData a partir de este informe.

## 4. Casos que necesitan otro cambio o una decisión

- `public/vite.svg` **sí se usa**: `index.html:5` lo declara como favicon. Para quitarlo, primero retirar o sustituir esa etiqueta. Tiene aspecto de residuo de plantilla, pero hoy tiene referencia.
- `.eslintrc.json` no se ejecuta desde npm ni CI: `npm run lint` ejecuta TypeScript, no ESLint, y `package.json` no declara ESLint ni los plugins que exige esta configuración. Es candidata a retirarse si no la usa una instalación externa del editor; también podría conservarse para configurar ESLint correctamente.
- `.prettierrc` puede usarse automáticamente por el editor aunque no haya script npm. No clasificarla como sobrante solo por no tener importaciones.
- `src/components/AutorizarDescuentoModal.tsx` no se alcanza desde `src/main.tsx`, pero sí desde `src/components/__tests__/AutorizarDescuentoModal.test.tsx`. Borrarlo aisladamente rompe esas pruebas. Decidir por separado si se conserva como componente disponible o se retira junto con sus pruebas.
- `src-tauri/examples/generar_hash.rs` y `src-tauri/src/bin/generar_hash.rs` **no son duplicados**: el ejemplo usa bcrypt y el binario usa Argon2. Cargo reconoce automáticamente ejemplos y binarios; además, el binario está declarado en `Cargo.toml`. Para retirarlos hay que decidir que ya no se necesitan esas herramientas y, en el caso del binario declarado, modificar `Cargo.toml`.
- `src-tauri/src/bin/guardar_token_nubefact.rs` es otra herramienta reconocida por Cargo; sirve para guardar un token. No se vuelve sobrante por no tener un `mod` en la biblioteca.
- Los iconos adicionales `Square*Logo.png`, `StoreLogo.png` e `icon.png` no aparecen en la lista explícita `bundle.icon`, pero pertenecen al conjunto de recursos de empaquetado. No se ha construido cada formato/plataforma para demostrar que se pueden retirar. Conservarlos en esta limpieza conservadora.

## 5. Archivos que sí hay que conservar

- `package-lock.json` y `src-tauri/Cargo.lock`: fijan las dependencias; `npm ci` en CI necesita el primero.
- `src-tauri/migrations/0001_initial_schema_v7.sql`: se incluye desde `db.rs`, pruebas y servicios de backup/restauración.
- `src-tauri/src/services/backup_service/tests.rs`: está declarado mediante `#[cfg(test)] mod tests;` en el módulo de backups. Sí participa en las pruebas.
- Todos los archivos Rust de `src-tauri/src` están conectados a módulos o ejecutables. Esto no significa que cada función sea utilizada, pero ninguno resulta un archivo aislado que pueda borrarse directamente.
- Las 26 capturas de `docs/images/`: todas están referenciadas desde `README.md`.
- `src/assets/logo-gloria-login.png`: usado por Login y Sidebar.
- `src/styles/gloriaTaller.css`, los CSS importados, `src/App.css`, `src/test/setup.ts` y `src/vite-env.d.ts`: tienen uso de aplicación, pruebas o tipos.
- Las pruebas de `src` y `src-tauri/tests`: Vitest y Cargo las descubren sin requerir importaciones desde la aplicación.
- `src-tauri/.env`: el backend llama a `dotenvy::dotenv()`. No se imprimió ni modificó su contenido; puede afectar integraciones locales.
- Configuraciones de compilación, CI, capacidades Tauri y módulos de comandos. Un comando registrado o un módulo público no es eliminable simplemente porque no se haya encontrado una llamada literal desde React.

## Comprobaciones y límites

- `npm run typecheck`: correcto.
- `npm test -- --reporter=dot`: 19 archivos de pruebas y 128 pruebas aprobadas. El primer intento estuvo limitado por `spawn EPERM`; la repetición con permiso de ejecución terminó correctamente.
- Grafo de importaciones estáticas y dinámicas literales de TypeScript, reexportaciones y dependencias CSS: sin rutas relativas sin resolver. Se incluyeron como raíces las pruebas, su setup y la declaración de tipos de Vite.
- No se detectó un mecanismo de carga de archivos del frontend por `import.meta.glob` que invalide los candidatos listados.
- Grafo de módulos Rust: 91 de 91 archivos de `src-tauri/src` alcanzados, incluyendo binarios y el módulo de pruebas de backup.
- Verificación de `ingesta_peru_v8/plan.json`: 4.571 SQL declarados, ninguno faltante, ninguno fuera del plan y ningún hash discrepante. No se ejecutó la ingesta.
- Comparación de archivos propios por SHA-256: el único grupo duplicado con contenido útil fue el par de logos indicado; también coinciden tres marcadores `.gitkeep`.
- No se ejecutaron pruebas Rust, no se compiló cada instalador ni se hizo una eliminación de prueba. Las pruebas aprobadas describen el estado actual: no son una garantía absoluta de un estado posterior a futuras eliminaciones.
- No se analizaron como parte del proyecto las copias abiertas en pestañas bajo `C:/Users/lozay/AppData/Local/Temp/`: están fuera de esta carpeta. No deben confundirse con los archivos activos del repositorio.

El archivo `Inventario_Archivos_2026-11-09.csv` enumera los 4.939 archivos inventariados con tamaño, clasificación y motivo. Excluye los dos informes creados por esta revisión y agrupa dependencias y compilaciones en la tabla anterior para evitar mezclar miles de archivos generados con código propio.
