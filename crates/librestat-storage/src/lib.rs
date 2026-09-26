//! # LibreStat Storage
//!
//! Persistencia híbrida para LibreStat:
//! - Base de datos de trabajo transaccional SQLite para auto-guardado y tolerancia a fallos.
//! - Empaquetador de proyectos `.lstat` basado en contenedores ZIP y manifest versionado.
//! - Importadores y exportadores de formatos externos (CSV, Excel .xlsx).

pub mod io;
pub mod project;
pub mod session_db;
