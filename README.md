# LibreStat

> **Software estadístico de escritorio, libre, local-first y multiplataforma inspirado en Minitab®.**

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)
[![Status: Planning & Phase 0](https://img.shields.io/badge/Status-Fase%200%20(Gobernanza)-orange.svg)](ROADMAP.md)
[![Rust](https://img.shields.io/badge/Rust-1.80+-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://v2.tauri.app/)
[![React](https://img.shields.io/badge/React-18%2F19-61dafb.svg)](https://react.dev/)

---

## 1. Visión y Propósito
**LibreStat** nace para ofrecer una alternativa moderna, accesible y de código abierto a las herramientas estadísticas comerciales privativas (especialmente Minitab®), orientada a:
- **Estudiantes y Docentes Universitarios**: aprendizaje intuitivo de estadística descriptiva, inferencial y probabilidad mediante hojas de trabajo interactivas, ventanas de sesión reproducibles y visualización gráfica.
- **Investigadores Académicos**: análisis de datos riguroso, trazable y verificable contra estándares internacionales.
- **Ingenieros de Calidad y Manufactura**: aplicación de control estadístico de procesos (SPC), análisis de capacidad ($C_p, C_{pk}$) y herramientas de mejora continua (Six Sigma).

---

## 2. Principios Fundamentales
* **Local-First y Privacidad Absoluta**: Los datos y los cálculos nunca salen de tu ordenador. Sin telemetría oculta, sin nube obligatoria y sin modelos de lenguaje en tiempo de ejecución.
* **Precisión Numérica Certificada**: Motor estadístico auditado contra los datasets de referencia certificados del **NIST StRD** y contrastado con **GNU R**, empleando algoritmos numéricamente estables (Welford para momentos, QR/SVD para regresión lineal).
* **Flujo de Trabajo Minitab**:
  - **Hojas de Trabajo Tabulares (Worksheets)**: edición reactiva de filas y columnas, tipos de datos (`Float64`, `Int64`, `Text`, `DateTime`) y soporte de valores faltantes (`*`).
  - **Ventana de Sesión (Session Window)**: historial inmutable y reproducible de tablas de resultados, pruebas de hipótesis y estimadores.
  - **Gráficos Científicos y de Distribución**: histogramas, diagramas de caja (boxplots), dispersión, gráficos de probabilidad normal (Q-Q plots) y curvas de distribución con sombreado de colas y regiones críticas.
* **Arquitectura de Alto Rendimiento**: Shell ligero basado en **Tauri v2**, interfaz reactiva en **React + TypeScript** y un motor de cómputo columnar puro e independiente en **Rust**.

---

## 3. Estado del Proyecto
Actualmente el proyecto se encuentra en la **Fase 0: Infraestructura, Gobernanza y Configuración Base**. No se ha iniciado la implementación de código de producto ni dependencias hasta consolidar el entorno de desarrollo y auditoría.

---

## 4. Documentación y Gobernanza del Proyecto

| Documento | Propósito |
| :--- | :--- |
| **[AGENTS.md](AGENTS.md)** | **Constitución obligatoria** para agentes de IA y desarrolladores (límites de código, higiene, auditorías y política estricta de commits). |
| **[DESIGN.md](DESIGN.md)** | **Sistema de diseño e identidad visual**: tokens semánticos, anatomía de diálogos, tipografía tabular y directrices UI. |
| **[ARCHITECTURE.md](ARCHITECTURE.md)** | Especificación técnica, límites de capas, diagramas C4, modelo de datos y contratos IPC. |
| **[ROADMAP.md](ROADMAP.md)** | Plan maestro de fases, alcance del MVP Ampliado Académico, entregables y criterios de aceptación. |
| **[TESTING.md](TESTING.md)** | Protocolo tripartito de validación matemática (NIST StRD, R de referencia y `proptest`). |
| **[DEVELOPMENT.md](DEVELOPMENT.md)** | Guía de configuración del entorno de desarrollo local, herramientas y comandos de auditoría. |
| **[CONTRIBUTING.md](CONTRIBUTING.md)** | Directrices para contribuciones, ramas, convenciones de commit y revisiones por pares. |
| **[ADR/](ADR/)** | Registro formal de decisiones arquitectónicas (*Architecture Decision Records*). |
| **[LICENSE](LICENSE)** | Licencia legal del proyecto (**GNU General Public License v3.0 o posterior**). |

---

## 5. Licencia
LibreStat es software libre publicado bajo los términos de la **[GNU General Public License v3.0 or later (GPL-3.0-or-later)](LICENSE)**.
