# ROADMAP.md: Plan de Fases y Entregables de LibreStat

Este documento establece la planificación cronológica de LibreStat, sus fases de desarrollo, los entregables concretos por subfase, los criterios de aceptación verificables (DoD) y la gestión de riesgos técnicos.

---

## 1. Visión General del Roadmap

```mermaid
gantt
    title Roadmap de Desarrollo LibreStat
    dateFormat  YYYY-MM-DD
    section Fase 0: Fundación
    Infraestructura, Tooling y AGENTS.md :done, f0_1, 2026-10-01, 7d
    Workspace Rust + Tauri v2 + React Setup :active, f0_2, after f0_1, 7d
    section Fase 1: MVP Académico
    Motor Columnar y BitMasks (librestat-core) :f1_1, after f0_2, 10d
    Hojas de Trabajo Virtualizadas y Edición :f1_2, after f1_1, 12d
    Importador/Exportador CSV y Excel (.xlsx) :f1_3, after f1_2, 7d
    Persistencia .lstat y SQLite de Sesión :f1_4, after f1_3, 10d
    Estadística Descriptiva Completa + NIST :f1_5, after f1_1, 14d
    Gráficos Esenciales (Histograma, Boxplot, Scatter, QQ) :f1_6, after f1_5, 12d
    Inferencia y Gráficos de Distribución :f1_7, after f1_5, 16d
    Regresión Lineal Simple y Diagnóstico 4-en-1 :f1_8, after f1_7, 12d
    Ventana de Sesión y Exportación HTML/PDF :f1_9, after f1_8, 8d
    CI Multiplataforma y Validación 1.0 :f1_10, after f1_9, 10d
    section Fase 2: ANOVA y SPC
    ANOVA 1 y 2 Factores + Tukey HSD :f2_1, 2027-01-15, 20d
    Regresión Múltiple y Selección de Modelos :f2_2, after f2_1, 15d
    Control de Calidad: Xbar-R, I-MR y Reglas Nelson :f2_3, after f2_1, 20d
    Capacidad de Proceso: Cp, Cpk, Pp, Ppk :f2_4, after f2_3, 10d
    section Fase 3: Calidad Avanzada & DOE
    Gráficos por Atributos (p, np, c, u) :f3_1, 2027-03-15, 15d
    Gage R&R (Sistemas de Medición) :f3_2, after f3_1, 15d
    Diagrama de Ishikawa y Pareto :f3_3, after f3_2, 12d
    Diseño de Experimentos Factoriales (2^k) :f3_4, after f3_3, 20d
```

---

## 2. Fase 0: Infraestructura, Gobernanza y Configuración Base (Actual)

### Objetivo
Establecer la estructura del monorepo, configurar los linters, formateadores, pipelines de CI y reglas de gobernanza sin implementar código de funcionalidades estadísticas.

### Entregables
1. **Documentación de Gobernanza**: `AGENTS.md`, `README.md`, `ARCHITECTURE.md`, `ROADMAP.md`, `TESTING.md`, `DEVELOPMENT.md`, `CONTRIBUTING.md`, `LICENSE` y primeros registros en `ADR/`.
2. **Scaffolding del Monorepo**:
   - Cargo Workspace configurado con `crates/librestat-core`, `crates/librestat-storage` y `crates/librestat-desktop`.
   - pnpm Workspace configurado con la aplicación React en `src/` y `packages/librestat-types/`.
3. **Herramientas de Auditoría**:
   - `cargo clippy -- -D warnings`, `cargo fmt`, `cargo deny`.
   - `pnpm lint`, `pnpm tsc --noEmit`, `pnpm knip`.
4. **CI Inicial**: Pipeline de GitHub Actions validando builds limpios en Linux, Windows y macOS.

### Criterio de Aceptación (DoD)
- Ejecución limpia de `cargo check`, `cargo test`, `pnpm tsc --noEmit`, `pnpm lint` y `pnpm knip` con cero advertencias o errores.

---

## 3. Fase 1: MVP Ampliado Académico (Hito 1.0)

### Objetivo
Proveer una herramienta estadística funcional, precisa y completa para la docencia universitaria y la investigación básica, abarcando hojas de trabajo, estadística descriptiva, inferencia básica, distribuciones de probabilidad y regresión lineal simple.

### Subfases y Entregables

#### Subfase 1.1: Motor de Datos Columnar y Hoja de Cálculo Virtualizada
- **Entregables**:
  - Estructuras columnares en `crates/librestat-core/src/data/` (`Float64`, `Int64`, `Text`, `DateTime`) con `BitVec` contiguo para missing values (`*`).
  - Tabla virtualizada en React capaz de navegar fluidamente hasta 500.000 filas a 60 FPS.
  - Edición en celda, renombrado de columnas, inserción/eliminación de filas/columnas y portapapeles (copiar/pegar TSV).
- **Criterio de Aceptación**: Tiempo de respuesta de scroll < 16ms por frame; consumo de memoria < 100MB con 100.000 filas.

#### Subfase 1.2: Importación, Exportación y Persistencia Híbrida
- **Entregables**:
  - Importador y exportador de CSV con detección automática de separador (`.`, `,`, `;`, tab) y codificación.
  - Importador de archivos Excel (`.xlsx`).
  - Base de datos SQLite local de trabajo para auto-guardado en segundo plano y recuperación tras cierres inesperados.
  - Empaquetador `.lstat` en formato ZIP con `manifest.json` y guardado atómico.
- **Criterio de Aceptación**: Cierre forzado del proceso recupera la sesión intacta; archivo `.lstat` es 100% interoperable entre Windows, Linux y macOS.

#### Subfase 1.3: Estadística Descriptiva y Gráficos Esenciales
- **Entregables**:
  - Algoritmo de Welford para media, varianza muestral insesgada y desviación estándar.
  - Cuartiles y percentiles (tipo 7 estándar R/NIST), mediana, moda, IQR, asimetría de Fisher-Pearson, curtosis, rango, suma, conteos válidos $N$ y faltantes $N^*$.
  - Descriptiva univariada con desglose por variables de grupo (categorías).
  - Gráficos esenciales: Histograma (con ajuste de curva normal opcional), Boxplot (con detección de outliers), Diagrama de dispersión (Scatter plot) y Gráfico de probabilidad normal (Normal Q-Q plot).
- **Criterio de Aceptación**: Aprobación de la suite de pruebas NIST StRD (`Mavro`, `Michelso`, `PiFive`) con error relativo $< 10^{-12}$.

#### Subfase 1.4: Inferencia Estadística y Distribuciones de Probabilidad
- **Entregables**:
  - **Pruebas de Hipótesis e Intervalos**:
    - Prueba Z (1 y 2 muestras).
    - Prueba t de Student (1 muestra, 2 muestras independientes con varianzas iguales).
    - Prueba t de Welch (varianzas desiguales con grados de libertad de Satterthwaite).
    - Prueba t pareada.
    - Pruebas de proporciones (1 y 2 proporciones con aproximación normal y cálculo exacto binomial).
    - Prueba Chi-cuadrado de independencia y bondad de ajuste.
    - Intervalos de confianza asociados al nivel $1-\alpha$.
  - **Cálculo de Distribuciones (`Calcular > Distribuciones de Probabilidad`)**:
    - Evaluación de PDF/PMF, CDF y función inversa/cuantiles para distribuciones Normal, Student-$t$, Chi-cuadrado, $F$ y Binomial.
  - **Gráfico de Distribución de Probabilidad (`Gráfica > Gráfica de Distribución de Probabilidad`)**:
    - Modo "Ver Distribución": Curva continua o barras discretas con parámetros.
    - Modo "Ver Probabilidad (Áreas Sombreadas)": Sombreado de cola izquierda, cola derecha, dos colas y área central entre dos valores con rotulado de valores críticos.
- **Criterio de Aceptación**: Concordancia de p-values con GNU R $\ge 8$ cifras significativas; gráfico con interactividad bidireccional (introducir valor $X$ para obtener probabilidad o introducir $\alpha$ para marcar valores críticos en el eje).

#### Subfase 1.5: Regresión Lineal Simple y Diagnóstico
- **Entregables**:
  - Ajuste por mínimos cuadrados $Y = \beta_0 + \beta_1 X + \epsilon$ mediante descomposición QR con `faer`.
  - Coeficientes con errores estándar, estadísticos t y p-values.
  - Tabla de análisis de varianza (ANOVA de regresión): SS, MS, F y p-value, $R^2$, $R^2$ ajustado y error típico $S$.
  - Panel de diagnóstico 4-en-1 de residuos: Normal de residuos, Residuos vs Ajustados, Histograma de residuos y Residuos vs Orden.
- **Criterio de Aceptación**: Aprobación de datasets certificados NIST `Norris` y `Pontius`.

#### Subfase 1.6: Ventana de Sesión, Reportes e Instaladores Multiplataforma
- **Entregables**:
  - Ventana de sesión con registro inmutable y reproducible de análisis ejecutados.
  - Exportación de resultados a HTML y PDF.
  - Empaquetado automático en CI: Windows (NSIS `.exe` y `.msi`), Linux (`.AppImage` y `.deb`) y macOS (`.dmg` universal).
- **Criterio de Aceptación**: Instaladores funcionales generados automáticamente en GitHub Actions.

---

## 4. Fases Posteriores

### Fase 2: ANOVA y Control Estadístico de Procesos (SPC)
- ANOVA de 1 y 2 factores (con términos de interacción).
- Comparaciones múltiples de Tukey (HSD), Fisher LSD y Dunnett.
- Regresión lineal múltiple y selección de variables.
- Gráficos de control por variables: $\bar{X}-R$, $\bar{X}-S$ e $I-MR$.
- Reglas de señales de control de Nelson (1 a 8).
- Análisis de capacidad del proceso: $C_p, C_{pk}, P_p, P_{pk}$ con límites de especificación ($LSL, USL$).

### Fase 3: Calidad Avanzada y Diseño de Experimentos (DOE)
- Gráficos de control por atributos: $p$, $np$, $c$ y $u$ (con tamaños de subgrupo variables).
- Análisis de Sistemas de Medición (Gage R&R: método cruzado y anidado ANOVA).
- Herramientas de calidad: Diagrama de Causa y Efecto (Ishikawa) interactivo y Diagrama de Pareto.
- Diseño de Experimentos (DOE): Diseños factoriales completos y fraccionados $2^k$.

### Fase 4: Ecosistema, Localización y Reportes Avanzados
- Localización ampliada (francés, portugués, alemán).
- Constructor visual de reportes ejecutivos interactivos.
- Extensibilidad para scripts de análisis y macros reproducibles.
