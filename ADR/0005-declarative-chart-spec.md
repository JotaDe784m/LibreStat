# ADR 0005: Especificación Declarativa de Gráficos (LibreStatChartSpec) y Renderizado Híbrido

* **Estado**: Aceptado
* **Fecha**: Septiembre 2026
* **Decisores**: Propietario del Proyecto, Arquitecto Principal

---

## Contexto
LibreStat necesita generar gráficos estadísticos científicos (histogramas con ajuste normal, boxplots, gráficos de dispersión, Q-Q plots) y diagramas de control de calidad (Xbar-R, Ishikawa) interactivos, reproducibles y con capacidad de exportación vectorial de alta resolución (SVG, PDF y PNG 300 DPI).

## Decisión
Se diseña una especificación declarativa propia en JSON denominada **`LibreStatChartSpec`** que describe semánticamente el gráfico (series de datos, tipos de ejes, líneas de referencia $UCL/\bar{X}/LCL$, regiones críticas sombreadas y etiquetas). El renderizado en la interfaz se delega a adaptadores sobre **Apache ECharts** (Canvas/SVG) y **D3.js** para gráficos especializados.

## Consecuencias
### Positivas
- Desacoplamiento total: la lógica de análisis genera una especificación abstracta sin acoplarse a ninguna biblioteca gráfica particular.
- Re-renderizado exacto e interactivo al reabrir proyectos antiguos.
- Exportación vectorial limpia a SVG nativo y PDF para publicaciones académicas y reportes industriales.
- Soporte para interactividad bidireccional en gráficos de distribución de probabilidad (marcado de áreas sombreadas y valores críticos).

### Negativas / Riesgos
- Requiere mantener la capa de adaptación entre `LibreStatChartSpec` y las opciones de configuración de Apache ECharts y D3.
