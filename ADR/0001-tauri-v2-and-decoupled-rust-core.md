# ADR 0001: Adopción de Tauri v2 y Motor Estadístico Desacoplado en Rust

* **Estado**: Aceptado
* **Fecha**: Septiembre 2026
* **Decisores**: Propietario del Proyecto, Arquitecto Principal

---

## Contexto
LibreStat requiere una arquitectura de escritorio multiplataforma (Windows, Linux, macOS), local-first, con alto rendimiento para cálculos estadísticos y mínimo consumo de recursos del sistema. Se evaluaron Electron, Tauri v2 y microservicios locales con Python.

## Decisión
Se adopta **Tauri v2** para el shell de escritorio con una interfaz de usuario en **React/TypeScript**, combinada con un crate independiente de Rust puro (**`crates/librestat-core`**) para el motor estadístico.

## Consecuencias
### Positivas
- Huella de memoria mínima (<50MB en reposo vs >180MB en Electron).
- Tamaño de instalador liviano (<20MB vs >100MB).
- El motor estadístico (`librestat-core`) carece de dependencias gráficas o de Tauri, permitiendo pruebas unitarias ultrarrápidas con `cargo test` y reutilización como CLI o WebAssembly en el futuro.
- IPC fuertemente tipado y seguro mediante el sistema de capacidades de Tauri v2.

### Negativas / Riesgos
- Diferencias menores de renderizado entre motores WebView nativos (WebKitGTK en Linux, WebView2 en Windows, WebKit en macOS), mitigadas mediante pruebas de regresión visual con Playwright.
