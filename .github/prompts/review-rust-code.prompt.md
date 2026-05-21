---
mode: agent
description: "Revisa código Rust contra los estándares de calidad del bootcamp y sugiere mejoras."
---

Revisa el siguiente código Rust del bootcamp bc-rust y proporciona retroalimentación estructurada.

**Semana**: ${input:weekNumber}  
**Archivo**: ${input:filePath}  
**Contexto**: ${input:context}

## Criterios de revisión (en orden de prioridad)

### 1. Corrección
- [ ] ¿Compila sin errores?
- [ ] ¿Los tests pasan?
- [ ] ¿La lógica es correcta para todos los casos del enunciado?

### 2. Seguridad y robustez
- [ ] ¿Se usa `unwrap()`/`expect()` fuera de ejemplos simples? → Reemplazar con `?` o match
- [ ] ¿Hay `panic!` potenciales no documentados?
- [ ] Si hay `unsafe`: ¿tiene `// SAFETY:` comments?
- [ ] ¿Se validan inputs en los bordes del sistema?

### 3. Idiomaticidad Rust
- [ ] ¿Usa iteradores en lugar de loops manuales donde aplica?
- [ ] ¿Los errores usan tipos custom que implementan `std::error::Error`?
- [ ] ¿Se usa pattern matching exhaustivo en lugar de `if let` anidado?
- [ ] ¿Los tipos de datos son los más apropiados (`&str` vs `String`, etc.)?

### 4. Calidad del código
- [ ] ¿Pasa `cargo clippy -- -D warnings`? Listar lints específicos si no.
- [ ] ¿Cumple `cargo fmt --check`?
- [ ] ¿Los nombres de funciones/tipos son descriptivos en snake_case/PascalCase?

### 5. Documentación (solo para código `pub`)
- [ ] ¿Tienen `///` doc comments las funciones públicas?
- [ ] ¿Los ejemplos en doc comments son ejecutables (doctests)?

### 6. Tests
- [ ] ¿Hay tests para happy path, edge cases y casos de error?
- [ ] ¿Los nombres de los tests son descriptivos?
- [ ] ¿Los tests son independientes entre sí?

## Formato de respuesta

Para cada problema encontrado:

```
**[NIVEL]** Descripción del problema
Ubicación: `función_o_línea`
Problema: qué está mal y por qué importa
Sugerencia:
```rust
// código mejorado
```
```

Niveles: **[ERROR]** (rompe corrección), **[WARN]** (malas prácticas), **[STYLE]** (mejora de idioma), **[TIP]** (opcional, mejora la calidad)

## Al final de la revisión

Proporcionar:
1. **Resumen**: N errores, M warnings, K style notes
2. **Puntuación estimada** según la rúbrica del bootcamp (Conocimiento/Desempeño/Producto)
3. **Próximo paso recomendado** para el estudiante
