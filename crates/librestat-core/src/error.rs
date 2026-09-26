//! Tipos de error unificados para el motor estadístico de LibreStat.

/// Errores posibles durante la ejecución de procedimientos estadísticos o manipulación de datos.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatisticalError {
    /// Tamaño de muestra insuficiente para el procedimiento (ej. n < 2 para varianza muestral).
    InsufficientSampleSize {
        /// Tamaño de muestra mínimo requerido.
        required: usize,
        /// Tamaño de muestra real disponible.
        actual: usize,
    },
    /// Varianza muestral cero o vector con valores idénticos.
    ZeroVariance,
    /// Grados de libertad inválidos (<= 0).
    InvalidDegreesOfFreedom,
    /// Nivel de significancia o probabilidad fuera del intervalo (0, 1).
    InvalidProbability(String),
    /// Dimensiones incompatibles entre vectores o matrices.
    DimensionMismatch {
        /// Dimensión esperada.
        expected: usize,
        /// Dimensión real obtenida.
        actual: usize,
    },
    /// Matriz singular o no invertible en descomposición lineal.
    SingularMatrix,
    /// Columna no encontrada en la hoja de trabajo.
    ColumnNotFound(String),
    /// Error de tipo de datos en columna.
    TypeMismatch {
        /// Tipo de dato esperado.
        expected: &'static str,
        /// Tipo de dato real recibido.
        actual: &'static str,
    },
}

impl std::fmt::Display for StatisticalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InsufficientSampleSize { required, actual } => {
                write!(f, "Tamaño de muestra insuficiente: se requieren {required}, pero se encontraron {actual}")
            }
            Self::ZeroVariance => write!(
                f,
                "La varianza de la muestra es cero (todos los valores son idénticos)"
            ),
            Self::InvalidDegreesOfFreedom => {
                write!(f, "Grados de libertad inválidos (deben ser mayores a cero)")
            }
            Self::InvalidProbability(msg) => write!(f, "Probabilidad inválida: {msg}"),
            Self::DimensionMismatch { expected, actual } => {
                write!(f, "Discrepancia de dimensiones: se esperaban {expected}, pero se obtuvieron {actual}")
            }
            Self::SingularMatrix => write!(f, "La matriz es singular y no admite solución única"),
            Self::ColumnNotFound(name) => write!(f, "Columna '{name}' no encontrada"),
            Self::TypeMismatch { expected, actual } => {
                write!(
                    f,
                    "Discrepancia de tipo: se esperaba '{expected}', se obtuvo '{actual}'"
                )
            }
        }
    }
}

impl std::error::Error for StatisticalError {}
