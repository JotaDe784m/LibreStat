import React from 'react';
import { BarChart2 } from 'lucide-react';
import { useWorksheetStore } from './stores/useWorksheetStore';

/**
 * Componente principal de LibreStat (Placeholder de Fase 0).
 * Presenta el marco de trabajo inicial para el montaje de la interfaz.
 */
const App: React.FC = () => {
  const currentWorksheet = useWorksheetStore((state) => state.currentWorksheet);

  return (
    <div
      style={{
        fontFamily: 'system-ui, sans-serif',
        padding: '2rem',
        color: '#1a1a1a',
      }}
    >
      <header
        style={{
          borderBottom: '1px solid #e5e7eb',
          paddingBottom: '1rem',
          marginBottom: '1.5rem',
          display: 'flex',
          alignItems: 'center',
          gap: '0.75rem',
        }}
      >
        <BarChart2 size={28} color="#2563eb" />
        <div>
          <h1 style={{ fontSize: '1.5rem', fontWeight: 600, margin: 0 }}>LibreStat</h1>
          <p
            style={{
              margin: '0.25rem 0 0',
              color: '#6b7280',
              fontSize: '0.875rem',
            }}
          >
            Software estadístico de escritorio libre, local-first y multiplataforma
          </p>
        </div>
      </header>
      <main
        style={{
          background: '#f9fafb',
          border: '1px dashed #d1d5db',
          borderRadius: '0.5rem',
          padding: '2rem',
          textAlign: 'center',
        }}
      >
        <h2 style={{ fontSize: '1.125rem', fontWeight: 500, color: '#374151' }}>
          Fase 0: Infraestructura y Gobernanza
        </h2>
        <p
          style={{
            color: '#4b5563',
            maxWidth: '600px',
            margin: '0.5rem auto 0',
            fontSize: '0.875rem',
          }}
        >
          El entorno base del monorepo ha sido inicializado. La lógica de hojas de cálculo, ventana
          de sesión y cálculos estadísticos se incorporará progresivamente a partir de la Fase 1.
        </p>
        {currentWorksheet && (
          <p style={{ fontSize: '0.75rem', color: '#9ca3af' }}>Hoja: {currentWorksheet.name}</p>
        )}
      </main>
    </div>
  );
};

export default App;
