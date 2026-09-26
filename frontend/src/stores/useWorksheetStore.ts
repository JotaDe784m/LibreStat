import { create } from 'zustand';
import type { WorksheetMetadata } from '@librestat/types';

interface WorksheetState {
  currentWorksheet: WorksheetMetadata | null;
  setCurrentWorksheet: (sheet: WorksheetMetadata | null) => void;
}

export const useWorksheetStore = create<WorksheetState>((set) => ({
  currentWorksheet: null,
  setCurrentWorksheet: (sheet) => set({ currentWorksheet: sheet }),
}));
