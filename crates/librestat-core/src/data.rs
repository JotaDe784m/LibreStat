//! Estructuras de datos columnares y manejo de valores faltantes.
//!
//! Implementación basada en columnas contiguas tipadas con soporte de máscaras
//! de bits para valores ausentes ('*').

/// Tipos de datos soportados para las columnas de una hoja de trabajo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// Número de coma flotante de doble precisión (64 bits).
    Float64,
    /// Entero con signo de 64 bits.
    Int64,
    /// Cadena de texto UTF-8.
    Text,
    /// Marca de tiempo en milisegundos UTC.
    DateTime,
}
