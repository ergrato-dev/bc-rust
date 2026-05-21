---
applyTo: "**/*.svg"
---

# Reglas de Estilo SVG — Bootcamp bc-rust

## Tema obligatorio: Dark Mode

Todos los SVG del bootcamp usan tema oscuro consistente con VS Code Dark+.

## Paleta de colores

```
Fondos
  #1e1e1e   — fondo principal (body del SVG)
  #252526   — fondo secundario (paneles, cards)
  #2d2d2d   — fondo terciario (code blocks internos)

Texto
  #d4d4d4   — texto principal
  #808080   — texto secundario / labels
  #ffffff   — texto destacado / títulos principales

Acentos Rust
  #CE422B   — Rust Orange (acento principal — flechas, highlights, íconos)
  #A72145   — Rust Dark (hover states, bordes activos)
  #F46623   — Rust Light (énfasis suave)

Código (syntax highlighting)
  #569cd6   — keywords
  #ce9178   — strings
  #dcdcaa   — functions
  #4ec9b0   — types
  #6a9955   — comments

Bordes
  #3c3c3c   — borde normal
  #454545   — borde hover / activo
```

## Reglas absolutas

- ❌ **Prohibido** usar gradientes (`linearGradient`, `radialGradient`)
- ❌ **Prohibido** usar fuentes serif o display
- ❌ **Prohibido** fondos blancos o colores claros
- ✅ `viewBox` siempre definido (ej: `viewBox="0 0 800 400"`)
- ✅ `xmlns="http://www.w3.org/2000/svg"` siempre presente
- ✅ Primera capa: `<rect width="100%" height="100%" fill="#1e1e1e"/>`

## Tipografía

```xml
<!-- Títulos -->
font-family="'Segoe UI', 'Helvetica Neue', Arial, sans-serif"

<!-- Código -->
font-family="'Fira Code', 'JetBrains Mono', 'Consolas', monospace"

<!-- Texto general -->
font-family="'Segoe UI', 'Roboto', 'Open Sans', sans-serif"
```

## Estructura base

```xml
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 800 400">
  <style>
    .bg      { fill: #1e1e1e; }
    .panel   { fill: #252526; }
    .title   { font-family: 'Segoe UI', sans-serif; font-size: 22px; fill: #ffffff; font-weight: 600; }
    .label   { font-family: 'Segoe UI', sans-serif; font-size: 13px; fill: #808080; }
    .body    { font-family: 'Segoe UI', sans-serif; font-size: 14px; fill: #d4d4d4; }
    .code    { font-family: 'Fira Code', monospace; font-size: 12px; fill: #d4d4d4; }
    .accent  { fill: #CE422B; }
    .border  { fill: none; stroke: #3c3c3c; stroke-width: 1; }
    .arrow   { fill: none; stroke: #CE422B; stroke-width: 2; marker-end: url(#arrow); }
  </style>
  <!-- Fondo obligatorio -->
  <rect class="bg" width="100%" height="100%"/>
  <!-- Contenido -->
</svg>
```

## Flechas y conectores

```xml
<defs>
  <marker id="arrow" markerWidth="10" markerHeight="7"
          refX="10" refY="3.5" orient="auto">
    <polygon points="0 0, 10 3.5, 0 7" fill="#CE422B"/>
  </marker>
</defs>
<line x1="100" y1="50" x2="200" y2="50" class="arrow"/>
```

## Dimensiones recomendadas por tipo

| Tipo de diagrama      | viewBox            |
| --------------------- | ------------------- |
| Header / banner       | `0 0 1200 300`      |
| Diagrama de flujo     | `0 0 800 500`       |
| Comparativa / tabla   | `0 0 900 400`       |
| Diagrama de memoria   | `0 0 700 400`       |
| Timeline / secuencia  | `0 0 1000 350`      |

## generar variedad. que no se convierta un nuevo diagrama en una simple copia de los anteriores con modificaciones de textos