# 🏗️ Proyecto: Parser de Texto Eficiente

## 📋 Descripción

Implementa un **parser de texto** que utiliza lifetimes para evitar copias innecesarias. El parser trabaja con referencias al texto original, permitiendo un análisis eficiente sin duplicar datos.

## 🎯 Objetivos

1. Aplicar lifetimes en un proyecto real
2. Diseñar structs eficientes con referencias
3. Implementar parsing zero-copy
4. Practicar patrones de lifetimes avanzados

## 📦 Estructura del Proyecto

```
proyecto-parser/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs      # Demo y ejecución
    ├── lib.rs       # Módulo principal
    ├── lexer.rs     # Tokenizador
    ├── parser.rs    # Parser de expresiones
    └── ast.rs       # Árbol de sintaxis abstracta
```

## 🔧 Componentes a Implementar

### 1. Token (ast.rs)
```rust
pub enum Token<'a> {
    Word(&'a str),
    Number(&'a str),
    Symbol(&'a str),
    Whitespace(&'a str),
}
```

### 2. Lexer (lexer.rs)
```rust
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}
```

### 3. Parser (parser.rs)
```rust
pub struct KeyValue<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

pub struct Parser<'a> {
    input: &'a str,
}
```

## 📝 Funcionalidades

1. **Tokenización**: Dividir texto en tokens sin copiar
2. **Parsing key=value**: Parsear pares clave-valor
3. **Parsing CSV básico**: Parsear líneas CSV
4. **Expresiones**: Parsear expresiones simples

## 🏃 Ejecución

```bash
# Compilar y ejecutar
cargo run

# Ejecutar tests
cargo test

# Ver documentación
cargo doc --open
```

## ✅ Criterios de Éxito

- [ ] Todos los tests pasan
- [ ] No hay copias innecesarias (usa referencias)
- [ ] Lifetimes correctamente anotados
- [ ] Código documentado

## 💡 Pistas

1. **Zero-copy**: Todos los tokens deben ser referencias al input original
2. **Lifetime del struct**: El parser no puede vivir más que su input
3. **Métodos**: Los métodos que retornan referencias usan elision o lifetime explícito

## 📊 Evaluación

| Criterio | Puntos |
|----------|--------|
| Funcionalidad | 10 |
| Eficiencia (zero-copy) | 8 |
| Lifetimes correctos | 8 |
| Tests | 4 |
| **Total** | **30** |
