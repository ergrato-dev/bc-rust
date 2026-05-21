---
mode: agent
description: "Crea un diagrama SVG educativo con el estilo dark mode del bootcamp."
---

Crea un diagrama SVG educativo para el bootcamp bc-rust.

**Tema del diagrama**: ${input:topic}  
**Tipo**: ${input:diagramType} (flujo / comparativa / memoria / timeline / arquitectura)  
**Semana**: ${input:weekNumber}  
**Descripción de lo que debe ilustrar**: ${input:description}

## Reglas de estilo obligatorias

Seguir **exactamente** las reglas de `svg-style.instructions.md`:
- Fondo principal: `#1e1e1e`
- Sin gradientes
- Solo fuentes sans-serif para texto, monospace para código
- Acento principal: `#CE422B` (Rust Orange)
- Bordes: `#3c3c3c`

## Estructura del SVG

```xml
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 [W] [H]">
  <style>
    /* Definir clases CSS para colores y fuentes */
  </style>
  
  <!-- 1. Fondo obligatorio -->
  <rect width="100%" height="100%" fill="#1e1e1e"/>
  
  <!-- 2. Título del diagrama -->
  <!-- 3. Contenido principal -->
  <!-- 4. Leyenda (si aplica) -->
  <!-- 5. Footer con "Semana XX — bc-rust" en #808080 -->
</svg>
```

## Dimensiones según tipo

| Tipo | viewBox recomendado |
|------|---------------------|
| Diagrama de memoria (stack/heap) | `0 0 800 450` |
| Flujo de ownership/borrowing | `0 0 900 500` |
| Comparativa lado a lado | `0 0 1000 400` |
| Timeline de lifetimes | `0 0 900 350` |
| Arquitectura de sistema | `0 0 1000 600` |
| Header de semana | `0 0 1200 280` |

## Elementos visuales a usar

Para **diagramas de memoria**:
- Rectángulos con borde `#3c3c3c` para stack frames y heap blocks
- Flechas en `#CE422B` para punteros/referencias
- Labels en `#808080` para direcciones de memoria
- Valores en `#d4d4d4`

Para **comparativas** (Rust vs otro lenguaje):
- Dos columnas con header diferenciado
- Código en `font-family: 'Fira Code', monospace`
- Highlighting de keywords en `#569cd6`

Para **flujos y estados**:
- Nodos con `fill: #252526`, `stroke: #3c3c3c`
- Nodo activo/destacado: `stroke: #CE422B`, `stroke-width: 2`
- Flechas con marcador `#CE422B`

## Accesibilidad

- Incluir `<title>Descripción del diagrama</title>` como primer hijo
- Asegurar contraste mínimo 4.5:1 entre texto y fondo
- No comunicar información únicamente por color

## Entregable

El SVG debe guardarse en `bootcamp/week-${weekNumber}-*/0-assets/diagrama-${input:slug}.svg`
y referenciarse desde el `1-theory/README.md` de la semana.
