# 🗃️ Semana 08: Colecciones

> **Estructuras de datos dinámicas en Rust**

## 🎯 Objetivos de Aprendizaje

Al finalizar esta semana, serás capaz de:

- Usar `Vec<T>` para almacenar colecciones dinámicas
- Manipular `String` y `&str` correctamente
- Utilizar `HashMap<K, V>` para mapeos clave-valor
- Iterar sobre colecciones de forma idiomática
- Elegir la colección adecuada según el caso de uso

## 📋 Contenido

### Teoría

| # | Tema | Archivo |
|---|------|---------|
| 1 | Vectores (Vec) | [01-vectores.md](1-teoria/01-vectores.md) |
| 2 | Strings | [02-strings.md](1-teoria/02-strings.md) |
| 3 | HashMaps | [03-hashmaps.md](1-teoria/03-hashmaps.md) |
| 4 | Iteradores Básicos | [04-iteradores-basicos.md](1-teoria/04-iteradores-basicos.md) |
| 5 | Patrones y Buenas Prácticas | [05-patrones-colecciones.md](1-teoria/05-patrones-colecciones.md) |

### Práctica

| # | Ejercicio | Descripción |
|---|-----------|-------------|
| 1 | [Vectores](2-practica/practica-01-vectores/) | Operaciones CRUD con Vec |
| 2 | [Strings](2-practica/practica-02-strings/) | Manipulación de texto |
| 3 | [HashMaps](2-practica/practica-03-hashmaps/) | Diccionarios y contadores |
| 4 | [Iteradores](2-practica/practica-04-iteradores/) | map, filter, fold |

### Proyecto Semanal

| Proyecto | Descripción |
|----------|-------------|
| [Sistema de Inventario](3-proyecto/proyecto-inventario/) | Gestión de productos con colecciones |

## ⏱️ Distribución del Tiempo

| Actividad | Duración |
|-----------|----------|
| Teoría (Vec, String) | 45 min |
| Teoría (HashMap, iteradores) | 45 min |
| Prácticas guiadas | 90 min |
| Proyecto semanal | 60 min |
| **Total** | **4 horas** |

## 🔑 Conceptos Clave

### Vec<T>
```rust
let mut v: Vec<i32> = Vec::new();
let v2 = vec![1, 2, 3];

v.push(42);
let elemento = v.get(0);  // Option<&T>
let elemento = &v[0];     // &T (puede panic)
```

### String
```rust
let mut s = String::from("Hola");
s.push_str(" mundo");
s.push('!');

let slice: &str = &s[0..4];
```

### HashMap<K, V>
```rust
use std::collections::HashMap;

let mut mapa = HashMap::new();
mapa.insert("clave", "valor");
let valor = mapa.get("clave");
```

## 📚 Recursos

- [The Rust Book - Colecciones](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
- [std::vec](https://doc.rust-lang.org/std/vec/)
- [std::string](https://doc.rust-lang.org/std/string/)
- [std::collections::HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)

## ✅ Checklist de la Semana

- [ ] Leer teoría de Vec y String
- [ ] Leer teoría de HashMap e iteradores
- [ ] Completar práctica 1: Vectores
- [ ] Completar práctica 2: Strings
- [ ] Completar práctica 3: HashMaps
- [ ] Completar práctica 4: Iteradores
- [ ] Desarrollar proyecto de inventario
- [ ] Revisar rúbrica de evaluación

---

**Semana anterior**: [Semana 07 - Módulos y Crates](../semana-07/)  
**Semana siguiente**: [Semana 09 - Traits Básicos](../semana-09/)
