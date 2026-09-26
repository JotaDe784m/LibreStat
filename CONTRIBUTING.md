# CONTRIBUTING.md: Directrices de Contribución para LibreStat

Agradecemos el interés en contribuir a **LibreStat**. Este proyecto persigue democratizar la estadística profesional mediante software libre, determinista y auditado.

---

## 1. Principios para Contribuidores

1. **Rigor Matemático**: Cualquier algoritmo añadido debe contar con pruebas unitarias, referencias bibliográficas contrastadas y verificación con datos certificados (NIST StRD o R).
2. **Respeto a AGENTS.md**: Todas las reglas de modularidad, tamaño de componentes (<250 líneas en React), higiene de código y prohibición de código huérfano aplican rigurosamente tanto a agentes de IA como a contribuidores humanos.
3. **Licencia Libre (GPL-3.0-or-later)**: Toda contribución se incorporará bajo los términos de la GPLv3 o posterior. Asegúrate de que cualquier biblioteca externa que propongas cuente con una licencia compatible (MIT, Apache-2.0, BSD).

---

## 2. Flujo de Trabajo con Git

1. **Ramas**:
   - Nunca trabajes directamente sobre `main`.
   - Crea una rama descriptiva a partir de `main`:
     * Funcionalidades: `feat/nombre-de-funcionalidad`
     * Corrección de errores: `fix/descripcion-del-bug`
     * Documentación: `docs/tema-documentado`
     * Pruebas: `test/cobertura-modulo`
2. **Formato de Commits (Conventional Commits)**:
   - Los mensajes de commit deben seguir el estándar [Conventional Commits](https://www.conventionalcommits.org/):
     * `feat: añadir cálculo de prueba t pareada y grados de libertad`
     * `fix: corregir redondeo en percentiles de muestra impar`
     * `test: incorporar dataset de referencia Pontius del NIST`
     * `docs: actualizar matriz de tolerancias en TESTING.md`
     * `refactor: descomponer diálogo de regresión en subcomponentes`

---

## 3. Lista de Verificación Pre-Pull Request (PR Checklist)

Antes de enviar un Pull Request o solicitar un commit, verifica que:
- [ ] `cargo fmt -- --check` y `pnpm prettier --check .` pasan sin diferencias.
- [ ] `cargo clippy --workspace -- -D warnings` no emite ninguna advertencia.
- [ ] `pnpm tsc --noEmit` compila con cero errores de tipo.
- [ ] `pnpm lint` pasa con cero errores de ESLint.
- [ ] `pnpm knip` no detecta código huérfano, exports muertos ni dependencias sin uso.
- [ ] `cargo test --workspace` y `pnpm test` pasan al 100%.
- [ ] Los componentes React modificados respetan la guía de tamaño ($\le 250$ líneas).
- [ ] `cargo deny check licenses` certifica compatibilidad de licencias.

---

## 4. Proceso de Revisión de Cambios

1. Todo PR debe incluir una descripción clara del problema que resuelve, los módulos afectados y la estrategia de prueba utilizada.
2. Si el PR afecta a cálculos estadísticos, es obligatorio adjuntar el contraste con los valores esperados de R o NIST.
3. Se requiere al menos una revisión aprobada antes de fusionar (*merge*) en la rama `main`.
