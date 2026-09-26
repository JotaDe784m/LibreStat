# ADR 0002: Almacenamiento Columnar Nativo en Rust con Máscaras de Bits y Virtualización en UI

* **Estado**: Aceptado
* **Fecha**: Septiembre 2026
* **Decisores**: Propietario del Proyecto, Arquitecto Principal

---

## Contexto
El flujo de trabajo interactivo inspirado en Minitab requiere gestionar hojas de cálculo con columnas de distintos tipos (`Float64`, `Int64`, `Text`, `DateTime`), edición dinámica celda por celda y manejo eficiente de valores faltantes (`*`), soportando hasta 500.000 filas sin degradación de memoria.

## Decisión
Se implementa un motor columnar propio en Rust basado en vectores contiguos tipados con máscaras de bits (`BitVec`) para la validez de los datos, complementado con una tabla virtualizada en React para la presentación.

## Consecuencias
### Positivas
- Acceso contiguo en memoria para operaciones estadísticas vectorizadas y optimizaciones SIMD.
- Cero sobrecarga de dependencias complejas de bases de datos analíticas como Arrow o Polars en el MVP.
- Control total sobre la semántica de valores faltantes (representados como `*` en la UI y bits en 0 en Rust).
- Virtualización en el frontend que garantiza renderizado a 60 FPS transfiriendo solo las filas visibles por el canal IPC.

### Negativas / Riesgos
- Se debe mantener y probar minuciosamente la lógica de inserción, ordenamiento y tipado en el backend de Rust.
