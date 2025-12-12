# 🎭 Semana 05: Enums y Pattern Matching

> **El poder expresivo de Rust** - Modelar estados y variantes de forma segura

## 🎯 Objetivos de Aprendizaje

Al finalizar esta semana, serás capaz de:

1. **Definir** enums con y sin datos asociados
2. **Usar** `match` para manejar todas las variantes exhaustivamente
3. **Aplicar** `if let` y `while let` para casos simples
4. **Dominar** `Option<T>` para valores opcionales
5. **Entender** `Result<T, E>` para manejo de errores (intro)

---

## 📚 Contenido

### 1. Teoría

| Archivo | Tema | Duración |
|---------|------|----------|
| [01-definicion-enums.md](1-teoria/01-definicion-enums.md) | Definición y variantes de enums | 20 min |
| [02-enums-con-datos.md](1-teoria/02-enums-con-datos.md) | Enums con datos asociados | 25 min |
| [03-pattern-matching.md](1-teoria/03-pattern-matching.md) | match y exhaustividad | 30 min |
| [04-if-let-while-let.md](1-teoria/04-if-let-while-let.md) | Atajos para pattern matching | 20 min |
| [05-option-result.md](1-teoria/05-option-result.md) | Option y Result en la stdlib | 25 min |

### 2. Práctica

| Ejercicio | Descripción | Dificultad |
|-----------|-------------|------------|
| [Práctica 01](2-practica/practica-01-enums-basicos/) | Enums simples y con datos | ⭐ |
| [Práctica 02](2-practica/practica-02-match/) | Pattern matching exhaustivo | ⭐⭐ |
| [Práctica 03](2-practica/practica-03-option/) | Trabajar con Option | ⭐⭐ |
| [Práctica 04](2-practica/practica-04-patrones-avanzados/) | Guards, bindings, destructuring | ⭐⭐⭐ |

### 3. Proyecto Semanal

| Proyecto | Descripción |
|----------|-------------|
| [Máquina de Estados](3-proyecto/proyecto-maquina-estados/) | Sistema de pedidos con estados tipados |

### 4. Recursos

- [📖 eBooks Gratuitos](4-recursos/ebook-free/README.md)
- [🎬 Videografía](4-recursos/videografia/README.md)
- [🌐 Webgrafía](4-recursos/webgrafia/README.md)

### 5. Glosario

- [📖 Términos de Enums](5-glosario/README.md)

---

## ⏱️ Distribución del Tiempo (4 horas)

| Actividad | Tiempo | Descripción |
|-----------|--------|-------------|
| Teoría | 80 min | Enums, match, Option |
| Práctica guiada | 60 min | Ejercicios con el instructor |
| Proyecto | 60 min | Máquina de estados |
| Revisión | 40 min | Patrones avanzados, Q&A |

---

## 🔑 Conceptos Clave

### Enum Básico vs Enum con Datos

```rust
// Sin datos
enum Direccion {
    Norte,
    Sur,
    Este,
    Oeste,
}

// Con datos asociados
enum Mensaje {
    Salir,
    Mover { x: i32, y: i32 },
    Escribir(String),
    Color(u8, u8, u8),
}
```

### Match Exhaustivo

```rust
match direccion {
    Direccion::Norte => println!("↑"),
    Direccion::Sur => println!("↓"),
    Direccion::Este => println!("→"),
    Direccion::Oeste => println!("←"),
}
// ¡Debe cubrir TODAS las variantes!
```

---

## ⚠️ Errores Comunes

| Error | Causa | Solución |
|-------|-------|----------|
| `non-exhaustive patterns` | Falta cubrir variantes | Agregar casos o usar `_` |
| `cannot move out of` | Mover valor de enum | Usar referencia o clone |
| `unused variable` | Variable en pattern no usada | Prefijar con `_` |

---

## 📋 Checklist de Competencias

- [ ] Puedo definir enums con y sin datos
- [ ] Uso match para manejar todas las variantes
- [ ] Sé cuándo usar `if let` vs `match`
- [ ] Manejo Option sin usar unwrap
- [ ] Entiendo la diferencia entre Option y Result

---

## 🔗 Navegación

| ← Anterior | Inicio | Siguiente → |
|------------|--------|-------------|
| [Semana 04: Ownership](../semana-04/README.md) | [Bootcamp](../README.md) | [Semana 06: Error Handling](../semana-06/README.md) |
