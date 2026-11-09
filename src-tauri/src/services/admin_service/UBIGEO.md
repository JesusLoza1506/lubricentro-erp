# Catálogo de ubigeos de Administración

El archivo `ubigeos_inei_2026.txt` contiene 1 893 códigos distritales del
[SISCONCODE del Instituto Nacional de Estadística e Informática del Perú](https://www.gob.pe/15028-consultar-codigos-estandarizados-del-inei),
versión 2026 (`versionCategoriaPK=9-1`). La aplicación incluye estos códigos en
el binario y no hace peticiones al guardar Empresa.

Se extrajo únicamente la columna `ubigeo` de las filas `nivel=distrito` del
[CSV de SISCONCODE reproducido por Peru-maps, revisión 72c23e7](https://github.com/Rodasluis/Peru-maps/blob/72c23e7a384be07d36575b64b2642f33f0021c5e/salida/ubigeos_2026.csv).
El listado resultante coincide con la
[consulta oficial del SISCONCODE para la versión 9-1](https://webapp.inei.gob.pe:8443/sisconcode/ubigeo/listaBusquedaUbigeoPorUbicacionGeografica.htm?versionCategoriaPK=9-1&strVersion=2026&nivel=1&strDpto=TODOS&strProv=TODOS&strDist=TODOS&flagDpto=&flagProv=&flagDist=).
SHA-256 del CSV de origen:
`d7192ef1ed4d62dd0828764a41f0c8847637e71231f8c762f3013e4d52146432`.

Al actualizar el catálogo, conservar los códigos como texto de seis dígitos,
sin duplicados y ordenados, uno por línea. Excluir los niveles departamental y
provincial. Este archivo solo valida nuevas escrituras desde Administración;
no modifica los datos ni los comprobantes existentes.
