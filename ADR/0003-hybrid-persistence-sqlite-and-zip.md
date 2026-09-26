# ADR 0003: Persistencia Híbrida: SQLite de Sesión de Trabajo y Paquete ZIP .lstat

* **Estado**: Aceptado
* **Fecha**: Septiembre 2026
* **Decisores**: Propietario del Proyecto, Arquitecto Principal

---

## Contexto
Se requiere proteger la integridad de los datos del usuario frente a cortes de energía o cierres anómalos de la aplicación, al tiempo que se proporciona un formato de archivo de proyecto abierto, transparente, versionado y fácilmente transportable entre diferentes sistemas operativos.

## Decisión
Se adopta una arquitectura de persistencia híbrida:
1. **SQLite transaccional local** en modo WAL (*Write-Ahead Logging*) como base de datos de trabajo para auto-guardado en tiempo real y recuperación tras fallos.
2. **Paquete comprimido ZIP con extensión `.lstat`** conteniendo `manifest.json` versionado, hojas de trabajo, historial de comandos y especificaciones semánticas de gráficos, implementando guardado atómico mediante escritura en `.lstat.tmp` y renombrado atómico en el sistema de archivos.

## Consecuencias
### Positivas
- Resistencia total a la corrupción de datos ante caídas inesperadas del proceso.
- Formato `.lstat` abierto, transparente e inspeccionable con herramientas estándar de compresión.
- Facilidad para evolucionar el formato en versiones posteriores mediante migraciones explícitas de `schema_version`.

### Negativas / Riesgos
- Requiere mantener la lógica de sincronización entre el estado activo en SQLite y la exportación/importación del paquete `.lstat`.
