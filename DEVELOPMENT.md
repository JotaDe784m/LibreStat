# DEVELOPMENT.md: Guía del Entorno de Desarrollo de LibreStat

Este documento proporciona las instrucciones paso a paso para configurar el entorno local, compilar el proyecto y ejecutar las auditorías de calidad en **LibreStat**.

---

## 1. Requisitos Previos del Sistema

### 1.1. Herramientas Generales
* **Rust**: Versión estable reciente ($\ge 1.80$). Instalar mediante [rustup](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  rustup component add clippy rustfmt
  ```
* **Node.js**: Versión LTS ($\ge 20.x$).
* **pnpm**: Gestor de paquetes obligatorio ($\ge 9.x$):
  ```bash
  corepack enable
  corepack prepare pnpm@latest --activate
  ```
* **Herramientas de Auditoría de Rust**:
  ```bash
  cargo install cargo-deny cargo-audit
  ```

### 1.2. Dependencias del Sistema para Tauri v2

#### Linux (Debian / Ubuntu)
```bash
sudo apt update
sudo apt install -y \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libwebkit2gtk-4.1-dev \
  javascriptcoregtk-4.1
```

#### Windows
* **Visual Studio C++ Build Tools** (con la carga de trabajo "Desarrollo para el escritorio con C++").
* **Microsoft Edge WebView2** (preinstalado en Windows 10/11).

#### macOS
* **Xcode Command Line Tools**:
  ```bash
  xcode-select --install
  ```

---

## 2. Configuración Inicial del Monorepo

```bash
# 1. Clonar el repositorio (si no estás dentro)
cd LibreStat

# 2. Instalar dependencias del frontend (pnpm workspace)
pnpm install

# 3. Compilar el workspace de Rust
cargo build --workspace
```

---

## 3. Comandos de Desarrollo y Ejecución

* **Ejecutar la aplicación en modo desarrollo (Hot-Reloading completo)**:
  ```bash
  pnpm tauri dev
  ```
* **Ejecutar solo el servidor frontend en el navegador (para prototipado rápido de UI)**:
  ```bash
  pnpm dev
  ```

---

## 4. Comandos de Auditoría y Verificación Obligatorios

Antes de proponer cualquier cambio o commit, es obligatorio ejecutar y aprobar la siguiente suite de auditorías:

### 4.1. Verificación en Rust
```bash
# Formateo de código
cargo fmt -- --check

# Análisis estático y linter (cero advertencias toleradas)
cargo clippy --workspace -- -D warnings

# Pruebas unitarias y matemáticas
cargo test --workspace

# Auditoría de licencias compatibles (GPLv3 / MIT / Apache-2.0)
cargo deny check licenses

# Auditoría de vulnerabilidades en dependencias
cargo audit
```

### 4.2. Verificación en Frontend
```bash
# Formateo y estilo
pnpm prettier --check .

# Linter de código
pnpm lint

# Verificación de tipos TypeScript estricta
pnpm tsc --noEmit

# Auditoría de código huérfano, exports muertos y dependencias sin uso
pnpm knip

# Pruebas unitarias de UI
pnpm test

# Auditoría de seguridad de paquetes
pnpm audit
```

---

## 5. Compilación de Paquetes de Distribución

Para compilar instaladores listos para producción:
```bash
pnpm tauri build
```
Los ejecutables se generarán en `target/release/bundle/`:
* En Linux: archivos `.AppImage` y `.deb`.
* En Windows: instaladores `.exe` (NSIS) y `.msi`.
* En macOS: archivos `.dmg`.
