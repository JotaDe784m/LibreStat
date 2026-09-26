//! # LibreStat Core
//!
//! Motor estadístico y matemático puro para LibreStat.
//!
//! Este crate está completamente desacoplado de la interfaz gráfica y de Tauri,
//! proporcionando estructuras de datos columnares, algoritmos numéricamente estables
//! y procedimientos de inferencia estadística verificados.

pub mod algorithms;
pub mod data;
pub mod descriptive;
pub mod distributions;
pub mod error;
pub mod inference;
pub mod regression;

/// Versión del motor central de LibreStat
pub const CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
