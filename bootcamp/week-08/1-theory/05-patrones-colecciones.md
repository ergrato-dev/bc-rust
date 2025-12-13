# 📚 Patrones y Buenas Prácticas con Colecciones

> **Escribir código idiomático y eficiente**

## 🎯 Objetivos

- Elegir la colección correcta para cada caso
- Evitar errores comunes
- Escribir código idiomático
- Optimizar rendimiento

---

## 🗺️ Elegir la Colección Correcta

![Diagrama de Selección](../0-assets/05-patrones-colecciones.svg)

### ¿Cuándo usar cada colección?

| Colección | Caso de uso |
|-----------|-------------|
| `Vec<T>` | Lista ordenada, acceso por índice |
| `VecDeque<T>` | Cola doble, insertar/quitar al inicio |
| `HashMap<K, V>` | Búsqueda por clave, O(1) |
| `BTreeMap<K, V>` | Claves ordenadas, rangos |
| `HashSet<T>` | Elementos únicos, pertenencia |
| `BTreeSet<T>` | Elementos únicos y ordenados |

---

## 📝 Patrones con Vec

### Construir con map y collect

```rust
// ❌ Imperativo (válido pero no idiomático)
let mut cuadrados = Vec::new();
for i in 1..=5 {
    cuadrados.push(i * i);
}

// ✅ Funcional (idiomático)
let cuadrados: Vec<i32> = (1..=5).map(|i| i * i).collect();
```

### Transformar en lugar

```rust
let mut v = vec![1, 2, 3, 4, 5];

// ❌ Crear nuevo Vec
let v2: Vec<i32> = v.iter().map(|x| x * 2).collect();

// ✅ Modificar en lugar (más eficiente)
for n in &mut v {
    *n *= 2;
}
```

### Eliminar elementos que cumplen condición

```rust
let mut v = vec![1, 2, 3, 4, 5, 6];

// ❌ Eliminar mientras iteras (no compila)
// for (i, &n) in v.iter().enumerate() {
//     if n % 2 == 0 { v.remove(i); }
// }

// ✅ Usar retain
v.retain(|&n| n % 2 != 0);  // Solo impares

// ✅ O drain_filter (nightly) / extract_if
```

### Partir un Vec

```rust
let v = vec![1, 2, 3, 4, 5, 6];

// Partir por índice
let (izq, der) = v.split_at(3);
// izq = [1, 2, 3], der = [4, 5, 6]

// Partir por condición
let (pares, impares): (Vec<_>, Vec<_>) = v.iter()
    .partition(|&&n| n % 2 == 0);
```

### Deduplicar

```rust
let mut v = vec![1, 2, 2, 3, 3, 3, 4];

// dedup solo elimina CONSECUTIVOS duplicados
v.dedup();  // [1, 2, 3, 4]

// Para eliminar TODOS los duplicados, ordenar primero
let mut v = vec![1, 3, 2, 3, 1, 2, 4];
v.sort();
v.dedup();  // [1, 2, 3, 4]

// O usar HashSet
use std::collections::HashSet;
let unicos: Vec<i32> = v.into_iter()
    .collect::<HashSet<_>>()
    .into_iter()
    .collect();
```

---

## 📝 Patrones con String

### Preferir &str en funciones

```rust
// ❌ Requiere String, fuerza conversiones
fn procesar(s: String) { }

// ✅ Acepta &str Y &String (coercion)
fn procesar(s: &str) { }

// Uso
procesar("literal");              // &str
procesar(&String::from("owned")); // &String → &str
```

### Construir String eficientemente

```rust
// ❌ Múltiples realocaciones
let mut s = String::new();
s = s + "Hola";
s = s + " ";
s = s + "mundo";

// ✅ Pre-reservar o usar push_str
let mut s = String::with_capacity(11);
s.push_str("Hola");
s.push(' ');
s.push_str("mundo");

// ✅ O usar format! para legibilidad
let s = format!("{} {}", "Hola", "mundo");
```

### Parsear líneas

```rust
let texto = "línea 1\nlínea 2\nlínea 3";

// Procesar cada línea
for linea in texto.lines() {
    println!("{}", linea);
}

// Colectar en Vec
let lineas: Vec<&str> = texto.lines().collect();
```

### Validar entrada

```rust
fn validar_email(email: &str) -> bool {
    email.contains('@') && 
    email.contains('.') &&
    !email.starts_with('@') &&
    !email.ends_with('.')
}
```

---

## 📝 Patrones con HashMap

### Entry API para evitar doble lookup

```rust
use std::collections::HashMap;

// ❌ Dos lookups
if !mapa.contains_key(&clave) {
    mapa.insert(clave, valor);
}

// ✅ Un lookup con entry
mapa.entry(clave).or_insert(valor);
```

### Contador de frecuencias

```rust
fn contar_chars(s: &str) -> HashMap<char, usize> {
    let mut contador = HashMap::new();
    for c in s.chars() {
        *contador.entry(c).or_insert(0) += 1;
    }
    contador
}
```

### Agrupar elementos

```rust
fn agrupar_por<T, K, F>(items: Vec<T>, key_fn: F) -> HashMap<K, Vec<T>>
where
    K: Eq + std::hash::Hash,
    F: Fn(&T) -> K,
{
    let mut grupos = HashMap::new();
    for item in items {
        let key = key_fn(&item);
        grupos.entry(key).or_default().push(item);
    }
    grupos
}

// Uso
let palabras = vec!["rust", "es", "rápido", "y", "seguro"];
let por_longitud = agrupar_por(palabras, |s| s.len());
```

### Cache con computación lazy

```rust
use std::collections::HashMap;

fn get_or_compute<F>(
    cache: &mut HashMap<String, i32>,
    key: &str,
    compute: F,
) -> i32
where
    F: FnOnce() -> i32,
{
    *cache.entry(key.to_string()).or_insert_with(compute)
}
```

---

## 📝 Patrones con Iteradores

### Encadenar transformaciones

```rust
let resultado: Vec<String> = datos.iter()
    .filter(|x| x.es_valido())
    .map(|x| x.transformar())
    .take(10)
    .collect();
```

### zip para iterar en paralelo

```rust
let nombres = vec!["Ana", "Bob", "Carlos"];
let edades = vec![25, 30, 28];

for (nombre, edad) in nombres.iter().zip(edades.iter()) {
    println!("{} tiene {} años", nombre, edad);
}
```

### windows y chunks

```rust
let v = vec![1, 2, 3, 4, 5];

// Ventanas deslizantes
for ventana in v.windows(3) {
    println!("{:?}", ventana);  // [1,2,3], [2,3,4], [3,4,5]
}

// Chunks (grupos)
for chunk in v.chunks(2) {
    println!("{:?}", chunk);  // [1,2], [3,4], [5]
}
```

### fold vs for

```rust
// Cuando el resultado es un valor acumulado
let suma: i32 = v.iter().fold(0, |acc, x| acc + x);

// Equivalente con for (más verboso)
let mut suma = 0;
for x in &v {
    suma += x;
}
```

---

## ⚠️ Errores Comunes

### Error 1: Iterar con índices innecesariamente

```rust
// ❌ Estilo C (válido pero no idiomático)
for i in 0..vec.len() {
    println!("{}", vec[i]);
}

// ✅ Idiomático
for elemento in &vec {
    println!("{}", elemento);
}

// ✅ Si necesitas el índice
for (i, elemento) in vec.iter().enumerate() {
    println!("[{}] {}", i, elemento);
}
```

### Error 2: Clone innecesario

```rust
// ❌ Clonar cuando no es necesario
fn procesar(s: String) { }
let texto = String::from("hola");
procesar(texto.clone());  // Copia innecesaria si no usas texto después

// ✅ Pasar ownership si no necesitas el original
procesar(texto);

// ✅ O usar referencia
fn procesar(s: &str) { }
```

### Error 3: Collect intermedio innecesario

```rust
// ❌ collect() intermedio innecesario
let v = vec![1, 2, 3, 4, 5];
let temp: Vec<i32> = v.iter().map(|x| x * 2).collect();
let suma: i32 = temp.iter().sum();

// ✅ Encadenar directamente
let suma: i32 = v.iter().map(|x| x * 2).sum();
```

### Error 4: Mutar mientras iteras

```rust
// ❌ No compila
let mut v = vec![1, 2, 3, 4, 5];
for x in &v {
    if *x == 3 {
        v.push(6);  // ERROR: mutable borrow while borrowed
    }
}

// ✅ Separar las fases
let mut v = vec![1, 2, 3, 4, 5];
let debe_agregar = v.iter().any(|&x| x == 3);
if debe_agregar {
    v.push(6);
}
```

### Error 5: HashMap con claves String

```rust
// ❌ Requiere crear String para cada lookup
let mut mapa: HashMap<String, i32> = HashMap::new();
mapa.insert("clave".to_string(), 1);
let val = mapa.get(&"clave".to_string());  // Crea String innecesario

// ✅ get() acepta &str si K: Borrow<str>
let val = mapa.get("clave");  // Funciona directamente
```

---

## 🏎️ Optimizaciones

### Pre-reservar capacidad

```rust
// Si conoces el tamaño aproximado
let mut v = Vec::with_capacity(1000);
let mut s = String::with_capacity(100);
let mut m = HashMap::with_capacity(50);
```

### Usar referencias cuando sea posible

```rust
// ❌ Clonar datos
fn procesar(datos: Vec<String>) { }

// ✅ Usar referencias
fn procesar(datos: &[String]) { }
fn procesar(datos: &[&str]) { }
```

### Evitar collect() innecesarios

```rust
// ❌ Múltiples colecciones intermedias
let resultado = datos
    .iter()
    .map(|x| x * 2)
    .collect::<Vec<_>>()
    .iter()
    .filter(|x| **x > 5)
    .collect::<Vec<_>>();

// ✅ Una sola cadena
let resultado: Vec<_> = datos
    .iter()
    .map(|x| x * 2)
    .filter(|x| *x > 5)
    .collect();
```

---

## 📊 Complejidad de Operaciones

| Vec | Tiempo |
|-----|--------|
| `push` / `pop` | O(1) amortizado |
| `insert` / `remove` | O(n) |
| `get` | O(1) |
| `contains` | O(n) |
| `sort` | O(n log n) |

| HashMap | Tiempo |
|---------|--------|
| `insert` / `remove` | O(1) promedio |
| `get` | O(1) promedio |
| `contains_key` | O(1) promedio |

| String | Tiempo |
|--------|--------|
| `push` / `push_str` | O(1) amortizado |
| `insert` | O(n) |
| `len` | O(1) |
| `chars().count()` | O(n) |

---

## 🎯 Checklist de Código Idiomático

- [ ] ¿Uso `iter()` en lugar de índices?
- [ ] ¿Uso `&str` en lugar de `String` en parámetros?
- [ ] ¿Uso Entry API para HashMap?
- [ ] ¿Evito `collect()` intermedios?
- [ ] ¿Pre-reservo capacidad cuando conozco el tamaño?
- [ ] ¿Uso `for x in &v` en lugar de `for x in v.iter()`?
- [ ] ¿Uso adaptadores funcionales apropiados?

---

**Anterior**: [04 - Iteradores Básicos](04-iteradores-basicos.md)  
**Volver a**: [README](../README.md)
