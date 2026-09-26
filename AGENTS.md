# AGENTS.md: Directrices Obligatorias para Agentes de IA y Contribuidores de LibreStat

Este documento establece las reglas inviolables para cualquier agente de inteligencia artificial (incluyendo Antigravity, subagentes de investigación y agentes de edición) y desarrolladores humanos que interactúen con el repositorio de **LibreStat**.

---

## 1. Naturaleza del Proyecto y Filosofía Operativa
1. **Local-First Estricto**: Todo cálculo y dato se procesa localmente en el dispositivo del usuario. Queda terminantemente prohibido introducir llamadas a servicios externos, telemetría, analítica en la nube o servicios de inteligencia artificial generativa en tiempo de ejecución.
2. **Determinismo Estadístico**: Todos los procedimientos estadísticos deben ser deterministas, reproducibles, auditables y numéricamente estables. Jamás se inventarán resultados, algoritmos, tolerancias ni valores p.
3. **Integridad de Datos**: Los datos originales del usuario en las hojas de trabajo son sagrados. Ningún análisis o procedimiento modificará las columnas crudas de entrada sin una acción expresa y confirmada del usuario.

---

## 2. Política de Commits y Operaciones Git
1. **PROHIBICIÓN TOTAL DE COMMITS O PUSHES NO AUTORIZADOS**:
   - Ningún agente ejecutará `git commit`, `git push`, `git merge`, `git rebase` ni creará tags o releases sin la autorización explícita y afirmativa del propietario en la conversación inmediata.
   - La orden de "implementar una función", "corregir un error" o "avanzar una fase" **NUNCA** autoriza a hacer commit.
2. **Protocolo Pre-Commit Obligatorio**:
   - Antes de solicitar autorización para un commit, el agente debe presentar un informe detallado con:
     a) Listado exacto de archivos modificados/creados (`git status`).
     b) Resumen del `git diff`.
     c) Resultado exitoso de `cargo test` y suite matemática (incluyendo pruebas NIST cuando aplique).
     d) Resultado exitoso de `cargo clippy -- -D warnings`.
     e) Resultado exitoso de `pnpm tsc --noEmit` y `pnpm lint`.
     f) Verificación de código muerto mediante `pnpm knip`.
     g) Verificación de licencias mediante `cargo deny check licenses`.
     h) Lista explícita de deuda técnica, limitaciones o advertencias pendientes.
3. **No Reescritura de Historia**: Queda prohibido alterar la historia de git o ejecutar comandos destructivos (`git reset --hard`, `git clean -fd`) sin una instrucción unívoca y justificada del propietario.

---

## 3. Modularidad, Separación de Capas y Límites de Archivo
1. **Límite de Tamaño de Componentes React**:
   - Como regla guía estricta, ningún componente de React debe superar las **250 líneas de código**.
   - Si un componente excede ese límite o acumula múltiples responsabilidades (ej. renderizado + estado + llamadas IPC), debe descomponerse obligatoriamente en subcomponentes, custom hooks (ej. `useWorksheetVirtualizer.ts`), o funciones utilitarias puras.
2. **Separación de Responsabilidades por Capas**:
   - **UI (`src/components/`, `src/features/`)**: Presentación e interacción del usuario. CERO lógica matemática estadística pesada.
   - **Estado (`src/stores/`)**: Stores modulares de Zustand organizados por dominio funcional. Prohibido un store monolítico global.
   - **Servicios (`src/services/`)**: Adaptadores de comunicación IPC con Tauri.
   - **Backend Desktop (`crates/librestat-desktop/`)**: Enrutamiento de comandos IPC, menús y diálogos nativos del sistema operativo.
   - **Persistencia (`crates/librestat-storage/`)**: Transacciones SQLite, guardado atómico en `.lstat`, parsing de CSV y Excel.
   - **Motor Estadístico (`crates/librestat-core/`)**: Cálculos matemáticos y estructuras columnares puras. **PROHIBIDO** importar Tauri o crates de interfaz de usuario aquí.
3. **Cero Dependencias Circulares**: Mantener una jerarquía acíclica estricta de dependencias en Rust y TypeScript.
4. **Adherencia Estricta al Sistema de Diseño (`DESIGN.md`)**:
   - Todo componente de interfaz (UI), diálogo modal de análisis, cuadrícula de hoja de trabajo o gráfico debe apegarse rigurosamente a las especificaciones, tokens semánticos y patrones anatómicos de `DESIGN.md`.
   - Queda terminantemente prohibido utilizar colores "hardcodeados" arbitrarios, estilos ad-hoc que rompan la coherencia visual o diálogos que violen el patrón estándar de dos columnas.

---

## 4. Higiene del Código y Eliminación de Código Muerto
1. **Prohibición de Código Huérfano**:
   - Prohibido dejar componentes sin uso, imports no utilizados, funciones no referenciadas, tipos muertos, variables huérfanas o bloques comentados de implementaciones previas.
   - Si se reemplaza una implementación, el código viejo debe eliminarse completamente si ya no se usa y su eliminación es segura.
2. **Cero Supresiones Injustificadas**:
   - Prohibido el uso de `any` indiscriminado en TypeScript. Todo dato debe estar fuertemente tipado.
   - Prohibido silenciar el compilador con `@ts-ignore`, `eslint-disable` o `#[allow(...)]` en Rust sin una justificación técnica formal y documentada en comentario.
   - Prohibido capturar excepciones vacías (`catch (e) {}` o `let _ = result;` sin log ni manejo).

---

## 5. Auditorías y Gestión de Dependencias
1. **Criterio de Dependencias Nuevas**:
   - No se añadirá ninguna dependencia a `Cargo.toml` ni `package.json` sin previa evaluación de su tamaño, necesidad, licencia (compatible con GPLv3 / MIT / Apache-2.0) y aprobación del usuario.
   - Prohibido instalar paquetes por anticipado para funciones futuras no incluidas en la tarea actual.
2. **Comandos de Auditoría Obligatorios**:
   - Rust: `cargo clippy -- -D warnings`, `cargo test`, `cargo fmt -- --check`, `cargo audit`, `cargo deny check licenses`.
   - Frontend: `pnpm lint`, `pnpm tsc --noEmit`, `pnpm test`, `pnpm knip`, `pnpm audit`.

---

## 6. Rigor Estadístico y Precisión Numérica
1. **Estabilidad Numérica Obligatoria**:
   - Para la media y varianza muestral, se prohíbe la fórmula ingenua $\sum x^2 - (\sum x)^2 / n$. Es obligatorio usar el **algoritmo de Welford** o el de dos pasadas para prevenir cancelación catastrófica de punto flotante.
   - Para regresión lineal y mínimos cuadrados, se prohíbe la inversión directa de la matriz normal $(X^T X)^{-1}$. Es obligatorio usar descomposición **QR (Householder)** o **SVD**.
2. **Validación contra Referencias**:
   - Cada procedimiento estadístico debe acompañarse de pruebas unitarias que incluyan:
     a) Casos de referencia NIST StRD correspondientes.
     b) Casos límite: $n=1$, $n=2$, varianza cero (todos los valores idénticos), datos con valores faltantes (`*`), números extremadamente grandes o pequeños.
     c) Pruebas de propiedades (`proptest` en Rust): invariantes como $\text{Var}(X) \ge 0$, $\text{CDF}(x) \in [0, 1]$.
   - Las tolerancias relativas y absolutas deben estar documentadas y justificadas por la condición numérica del problema.

---

## 7. Comportamiento y Seguridad de Agentes de IA
1. **Cero Alucinaciones**: El agente nunca asumirá que un archivo o API existe sin verificarlo previamente en el árbol de trabajo. No se inventarán comandos de compilador ni resultados de tests.
2. **Edición Localizada**: Se prohíbe sobreescribir archivos enteros cuando basta con una modificación quirúrgica y localizada.
3. **Entradas No Confiables**: Cualquier archivo importado por el usuario (CSV, Excel, .lstat) debe ser tratado como dato no confiable y validado rigurosamente contra ataques de desbordamiento, rutas relativas maliciosas (`../`) o fórmulas peligrosas.
4. **Subagentes**: Todos los subagentes creados en el entorno heredarán estas directrices sin excepción.
