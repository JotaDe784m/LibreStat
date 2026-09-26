/**
 * Tipos de datos soportados para las columnas de una hoja de trabajo.
 * Mapeo directo con DataType en librestat-core.
 */
export type DataType = 'Float64' | 'Int64' | 'Text' | 'DateTime';

/**
 * Metadatos de una columna tabular.
 */
export interface ColumnMetadata {
  id: string;
  name: string;
  dataType: DataType;
  index: number;
  missingCount: number;
}

/**
 * Metadatos de una hoja de trabajo.
 */
export interface WorksheetMetadata {
  id: string;
  name: string;
  rowCount: number;
  columnCount: number;
  columns: ColumnMetadata[];
}

/**
 * Manifiesto del archivo de proyecto .lstat.
 */
export interface ProjectManifest {
  schemaVersion: number;
  id: string;
  title: string;
  createdAt: string;
  modifiedAt: string;
  worksheets: Array<{
    id: string;
    name: string;
  }>;
}

/**
 * Estructura de error estadístico reportado desde el motor de Rust.
 */
export interface StatisticalErrorPayload {
  kind:
    | 'InsufficientSampleSize'
    | 'ZeroVariance'
    | 'InvalidDegreesOfFreedom'
    | 'InvalidProbability'
    | 'DimensionMismatch'
    | 'SingularMatrix'
    | 'ColumnNotFound'
    | 'TypeMismatch';
  message: string;
}
