# Semana 19 — Teoría: `unsafe` Rust y Raw Pointers

Esta carpeta contiene los cinco archivos de teoría de la semana 19.
Cada archivo tiene ~180 líneas e incluye diagramas ASCII, ejemplos comentados
y comparaciones con otros lenguajes.

## Índice

| # | Archivo | Tema | Diagrama |
|---|---------|------|----------|
| 1 | [01-intro-unsafe.md](01-intro-unsafe.md) | Los 5 superpoderes de `unsafe`, UB, SAFETY comment | [SVG](../0-assets/01-intro-unsafe.svg) |
| 2 | [02-raw-pointers.md](02-raw-pointers.md) | `*const T`, `*mut T`, aritmética, `NonNull<T>` | [SVG](../0-assets/02-raw-pointers.svg) |
| 3 | [03-unsafe-funciones.md](03-unsafe-funciones.md) | `unsafe fn`, safe abstractions, `# Safety` en rustdoc | [SVG](../0-assets/03-unsafe-funciones.svg) |
| 4 | [04-unsafe-traits.md](04-unsafe-traits.md) | `Send`, `Sync`, `unsafe impl`, `!Send`, `PhantomData` | [SVG](../0-assets/04-unsafe-traits.svg) |
| 5 | [05-std-mem.md](05-std-mem.md) | `transmute`, `size_of`, `forget`, `MaybeUninit` | [SVG](../0-assets/05-std-mem.svg) |

## Ruta de Aprendizaje Recomendada

```
01-intro-unsafe  →  02-raw-pointers  →  03-unsafe-funciones
        ↓
04-unsafe-traits  →  05-std-mem
```

Leer en orden: cada archivo asume familiaridad con el anterior.

## Conceptos Clave de la Semana

| Concepto | Descripción breve |
|----------|-------------------|
| `unsafe` | Habilita 5 operaciones extra; no desactiva el borrow checker |
| UB (Undefined Behavior) | Violación de invariantes que produce comportamiento arbitrario |
| `// SAFETY:` | Comentario obligatorio que justifica cada bloque `unsafe` |
| `*const T` / `*mut T` | Raw pointers: sin lifetime, sin null-check, sin aliasing rules |
| `NonNull<T>` | Raw pointer con garantía de no-nulidad |
| `unsafe fn` | Función con precondiciones que el caller debe cumplir |
| `# Safety` | Sección de rustdoc que documenta las precondiciones de `unsafe fn` |
| `Send` | Tipo puede transferirse entre threads |
| `Sync` | `&T` puede compartirse entre threads |
| `unsafe impl` | Override manual de auto traits con justificación semántica |
| `mem::transmute` | Reinterpreta bits de un tipo como otro — la operación más peligrosa |
| `mem::forget` | Evita el Drop — produce memory leak intencional |
| `MaybeUninit<T>` | Forma correcta de manejar valores no inicializados |
