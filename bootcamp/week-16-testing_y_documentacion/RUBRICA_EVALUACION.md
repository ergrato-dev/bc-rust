# Rubrica de Evaluacion - Semana 16

## Testing y Documentacion

### Distribucion de Puntos

| Categoria | Peso | Puntos |
|-----------|------|--------|
| Conocimiento | 30% | 30 pts |
| Desempeno | 40% | 40 pts |
| Producto | 30% | 30 pts |
| **Total** | **100%** | **100 pts** |

---

## 1. Conocimiento (30 pts)

### Tests Unitarios (10 pts)

| Criterio | Pts |
|----------|-----|
| Explica el atributo `#[test]` | 2 |
| Conoce macros assert | 3 |
| Entiende `#[should_panic]` | 2 |
| Sabe usar `#[ignore]` | 1 |
| Comprende tests con Result | 2 |

### Tests de Integracion (10 pts)

| Criterio | Pts |
|----------|-----|
| Entiende directorio `tests/` | 3 |
| Sabe compartir codigo con `common/` | 3 |
| Conoce `--test` flag | 2 |
| Diferencia unit vs integration | 2 |

### Documentacion (10 pts)

| Criterio | Pts |
|----------|-----|
| Usa `///` correctamente | 3 |
| Conoce secciones de rustdoc | 3 |
| Entiende doc tests | 2 |
| Sabe generar HTML con cargo doc | 2 |

---

## 2. Desempeno (40 pts)

### Practicas Completadas

| Practica | Pts |
|----------|-----|
| practica-01-unit-tests | 10 |
| practica-02-integration | 10 |
| practica-03-doctests | 10 |
| practica-04-tdd | 10 |

### Criterios de Evaluacion por Practica

- Tests compilan y pasan
- Cobertura de casos edge
- Nombres descriptivos
- Codigo limpio

---

## 3. Producto (30 pts)

### Proyecto: Calculadora Testeada

| Criterio | Pts |
|----------|-----|
| Tests unitarios completos | 8 |
| Tests de integracion | 6 |
| Documentacion rustdoc | 6 |
| Doc tests funcionales | 5 |
| Manejo de errores testeado | 5 |

---

## Escala de Calificacion

| Rango | Calificacion |
|-------|--------------|
| 90-100 | A - Excelente |
| 80-89 | B - Bueno |
| 70-79 | C - Satisfactorio |
| 60-69 | D - Necesita mejora |
| 0-59 | F - No aprobado |

---

## Checklist de Entrega

- [ ] Todos los tests pasan (`cargo test`)
- [ ] Documentacion generada (`cargo doc`)
- [ ] Sin warnings de clippy
- [ ] Codigo formateado (`cargo fmt`)
- [ ] Doc tests funcionan (`cargo test --doc`)
