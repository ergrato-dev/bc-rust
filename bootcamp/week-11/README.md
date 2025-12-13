# 🦀 Semana 11: Lifetimes

## 📋 Información General

| Campo | Detalle |
|-------|---------|
| **Tema** | Lifetimes (Tiempos de Vida) |
| **Duración** | 4 horas |
| **Nivel** | Avanzado |
| **Prerequisitos** | Semanas 01-10 (especialmente ownership, borrowing y genéricos) |

## 🎯 Objetivos de Aprendizaje

Al finalizar esta semana, serás capaz de:

1. **Comprender** qué son los lifetimes y por qué existen
2. **Aplicar** anotaciones de lifetime en funciones
3. **Usar** lifetimes en structs que contienen referencias
4. **Reconocer** las reglas de elision de lifetimes
5. **Distinguir** entre `'static` y otros lifetimes
6. **Resolver** errores comunes de lifetimes

## 📚 Contenido

### Teoría (1.5 horas)

| Archivo | Tema | Duración |
|---------|------|----------|
| [01-introduccion-lifetimes.md](1-teoria/01-introduccion-lifetimes.md) | ¿Qué son y por qué existen? | 20 min |
| [02-lifetimes-funciones.md](1-teoria/02-lifetimes-funciones.md) | Anotaciones en funciones | 25 min |
| [03-lifetimes-structs.md](1-teoria/03-lifetimes-structs.md) | Referencias en estructuras | 20 min |
| [04-elision-rules.md](1-teoria/04-elision-rules.md) | Reglas de elision | 15 min |
| [05-lifetimes-avanzados.md](1-teoria/05-lifetimes-avanzados.md) | 'static, bounds y patrones | 20 min |

### Práctica (2 horas)

| Ejercicio | Tema | Dificultad |
|-----------|------|------------|
| [practica-01](2-practica/practica-01-lifetimes-basicos/) | Lifetimes básicos en funciones | ⭐⭐ |
| [practica-02](2-practica/practica-02-lifetimes-structs/) | Structs con referencias | ⭐⭐⭐ |
| [practica-03](2-practica/practica-03-elision/) | Reglas de elision | ⭐⭐ |
| [practica-04](2-practica/practica-04-lifetimes-avanzados/) | Patrones avanzados | ⭐⭐⭐⭐ |

### Proyecto (30 min)

| Proyecto | Descripción |
|----------|-------------|
| [proyecto-parser](3-proyecto/proyecto-parser/) | Parser de texto con referencias eficientes |

## 🗺️ Mapa Conceptual

```
                    LIFETIMES
                        │
        ┌───────────────┼───────────────┐
        ▼               ▼               ▼
   ¿Por qué?      Anotaciones      Elision
        │               │               │
   ┌────┴────┐    ┌─────┴─────┐    ┌────┴────┐
   │ Dangling│    │ Funciones │    │ 3 Reglas│
   │   refs  │    │ Structs   │    │  Input  │
   │ Validez │    │ impl      │    │  Output │
   └─────────┘    └───────────┘    └─────────┘
        │               │               │
        └───────────────┴───────┬───────┘
                                ▼
                    ┌───────────────────┐
                    │  'static y Bounds │
                    │  'a: 'b (outlives)│
                    │  T: 'a            │
                    └───────────────────┘
```

## ⏱️ Distribución del Tiempo

| Actividad | Tiempo | Porcentaje |
|-----------|--------|------------|
| Teoría | 1.5 horas | 37.5% |
| Práctica guiada | 1.5 horas | 37.5% |
| Proyecto | 0.5 horas | 12.5% |
| Revisión y dudas | 0.5 horas | 12.5% |
| **Total** | **4 horas** | **100%** |

## 🔑 Conceptos Clave

### Sintaxis de Lifetimes

```rust
// En funciones
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// En structs
struct Excerpt<'a> {
    part: &'a str,
}

// En impl
impl<'a> Excerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

### Las 3 Reglas de Elision

1. Cada referencia de entrada obtiene su propio lifetime
2. Si hay exactamente un lifetime de entrada, se aplica a todas las salidas
3. Si hay `&self` o `&mut self`, su lifetime se aplica a todas las salidas

### Lifetime 'static

```rust
// Vive durante toda la ejecución del programa
let s: &'static str = "Hola mundo";
```

## 📊 Evaluación

| Componente | Peso |
|------------|------|
| Prácticas (4) | 60% |
| Proyecto Parser | 30% |
| Participación | 10% |

## 🔗 Recursos Adicionales

- [4-recursos/RECURSOS.md](4-recursos/RECURSOS.md) - Enlaces y material extra
- [5-glosario/GLOSARIO.md](5-glosario/GLOSARIO.md) - Términos clave

## ❓ Preguntas Frecuentes

### ¿Por qué Rust necesita lifetimes?
Para garantizar en tiempo de compilación que las referencias siempre apuntan a datos válidos, evitando dangling references.

### ¿Siempre tengo que escribir lifetimes?
No, las reglas de elision permiten omitirlos en la mayoría de casos comunes.

### ¿Qué significa 'a: 'b?
Que el lifetime 'a debe vivir al menos tanto como 'b (outlives).

## 📝 Notas del Instructor

- Lifetimes es uno de los conceptos más desafiantes de Rust
- Enfatizar que lifetimes son **verificación**, no **control**
- Usar diagramas visuales para mostrar alcances
- Comparar con garbage collection y manual memory management
- Los errores del compilador son muy informativos - aprovecharlos

---

**Navegación:**
← [Semana 10: Genéricos](../semana-10/README.md) | [Semana 12: Closures e Iteradores](../semana-12/README.md) →
