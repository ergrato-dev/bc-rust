# Proyecto: Pipeline de Procesamiento de Datos

## 🎯 Descripción

Implementa un sistema de **pipeline configurable** que procese datos usando closures e iteradores. El proyecto integra todos los conceptos de la semana:

- **Closures** para definir transformaciones
- **Fn traits** para aceptar diferentes tipos de closures
- **Iteradores** para procesamiento lazy y encadenado
- **Box<dyn Fn>** para almacenar closures dinámicamente

## 📋 Requisitos

### Parte 1: Pipeline de Números (8 puntos)

Implementa `procesar_numeros` que:
1. Filtre números pares
2. Multiplique cada uno por 2
3. Sume 10 a cada resultado

```rust
fn procesar_numeros(numeros: &[i32]) -> Vec<i32> {
    numeros.iter()
        .filter(...)
        .map(...)
        .map(...)
        .collect()
}
```

### Parte 2: Pipeline de Strings (8 puntos)

Implementa `limpiar_textos` que:
1. Aplique `trim()` a cada string
2. Filtre strings vacíos
3. Convierta a minúsculas
4. Ordene por longitud

```rust
fn limpiar_textos(textos: &[&str]) -> Vec<String>
```

### Parte 3: Estadísticas (7 puntos)

Implementa `calcular_estadisticas` usando iteradores:

```rust
struct Estadisticas {
    cantidad: usize,
    suma: i32,
    promedio: f64,
    minimo: i32,
    maximo: i32,
}

fn calcular_estadisticas(datos: &[i32]) -> Option<Estadisticas>
```

### Parte 4: Pipeline Configurable (7 puntos)

Implementa una estructura `Pipeline<T>` que:
- Almacene etapas como `Vec<Box<dyn Fn(T) -> T>>`
- Permita añadir etapas con `agregar_etapa`
- Ejecute el pipeline con `ejecutar`

```rust
struct Pipeline<T> {
    etapas: Vec<Box<dyn Fn(T) -> T>>,
}

impl<T: Copy> Pipeline<T> {
    fn new() -> Self { ... }
    fn agregar_etapa(&mut self, etapa: Box<dyn Fn(T) -> T>) -> &mut Self { ... }
    fn ejecutar(&self, datos: &[T]) -> Vec<T> { ... }
}
```

## 💡 Conceptos Aplicados

| Concepto | Uso en el Proyecto |
|----------|-------------------|
| Closures | Definir transformaciones inline |
| `Fn` trait | Closures que solo leen |
| `Box<dyn Fn>` | Almacenar closures en Vec |
| `map` | Transformar elementos |
| `filter` | Filtrar elementos |
| `fold` | Aplicar etapas secuencialmente |
| `collect` | Materializar resultados |

## 🧪 Tests

```bash
cargo test
```

## ✅ Criterios de Evaluación

| Parte | Puntos | Criterio |
|-------|--------|----------|
| Números | 8 | Pipeline funcional con filter+map |
| Strings | 8 | Limpieza y ordenamiento correcto |
| Stats | 7 | Cálculos correctos, manejo de vacío |
| Pipeline | 7 | Estructura genérica y configurable |
| **Total** | **30** | |

## 📚 Recursos

- [Iterator in std::iter](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [Fn traits](https://doc.rust-lang.org/book/ch13-01-closures.html)
