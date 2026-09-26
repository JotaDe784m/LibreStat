# ADR 0004: Ecosistema Matemático Pure-Rust y Protocolo Tripartito de Validación Numérica

* **Estado**: Aceptado
* **Fecha**: Septiembre 2026
* **Decisores**: Propietario del Proyecto, Arquitecto Principal

---

## Contexto
El software estadístico científico y de ingeniería no puede permitirse inestabilidades numéricas, cancelaciones catastróficas ni discrepancias con estándares de referencia. Además, enlazar bibliotecas heredadas en C/Fortran (como BLAS/LAPACK) complica gravemente la compilación cruzada en Windows y macOS.

## Decisión
1. Adoptar crates matemáticos en **Rust puro**:
   - `faer` para álgebra lineal de alto rendimiento (QR, SVD, Cholesky) sin toolchains externas de C/Fortran.
   - `statrs` para distribuciones estadísticas continuas y funciones especiales.
   - Algoritmos propios verificados (Welford para varianza muestral, Householder QR para regresión lineal, reglas de Nelson para SPC).
2. Establecer un **Protocolo Tripartito de Validación**:
   - Nivel 1: Pruebas unitarias contra datasets certificados del NIST StRD.
   - Nivel 2: Contraste automatizado en CI contra GNU R.
   - Nivel 3: Pruebas de propiedades matemáticas con `proptest`.

## Consecuencias
### Positivas
- Compilación cruzada estática y limpia sin dependencias nativas complejas en el pipeline de CI.
- Garantía matemática certificada de no regresión en la precisión de los cálculos.
- Manejo explícito de condiciones patológicas (matrices singulares, grados de libertad insuficientes) mediante tipos `Result<T, StatisticalError>`.

### Negativas / Riesgos
- Necesidad de calibrar y justificar documentalmente las tolerancias numéricas relativas y absolutas por algoritmo en lugar de aplicar un umbral uniforme.
