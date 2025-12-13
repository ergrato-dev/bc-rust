# 📚 Reglas de Elision de Lifetimes

![Reglas de Elision](../0-assets/04-elision-rules.svg)

## 🎯 Objetivos de Aprendizaje

- Conocer las 3 reglas de elision
- Identificar cuándo Rust infiere lifetimes
- Saber cuándo es necesario anotar explícitamente

---

## 🤔 ¿Qué es Lifetime Elision?

**Elision** = omisión de anotaciones de lifetime cuando Rust puede inferirlas.

```rust
// Con anotaciones explícitas
fn first_word<'a>(s: &'a str) -> &'a str { ... }

// Con elision (Rust infiere lo mismo)
fn first_word(s: &str) -> &str { ... }
```

Las reglas de elision hacen el código más legible sin perder seguridad.

---

## 📜 Las 3 Reglas de Elision

### Terminología

- **Input lifetimes**: lifetimes en parámetros
- **Output lifetimes**: lifetimes en valores de retorno

### Regla 1: Cada Referencia de Entrada Obtiene su Propio Lifetime

```rust
// Lo que escribes:
fn foo(x: &str, y: &str) { }

// Lo que Rust interpreta:
fn foo<'a, 'b>(x: &'a str, y: &'b str) { }
```

### Regla 2: Si Hay Exactamente Un Input Lifetime, Se Aplica a Todos los Outputs

```rust
// Lo que escribes:
fn foo(x: &str) -> &str { }

// Lo que Rust interpreta:
fn foo<'a>(x: &'a str) -> &'a str { }
```

### Regla 3: Si Hay `&self` o `&mut self`, Su Lifetime Se Aplica a Outputs

```rust
impl Foo {
    // Lo que escribes:
    fn method(&self, x: &str) -> &str { }
    
    // Lo que Rust interpreta:
    fn method<'a, 'b>(&'a self, x: &'b str) -> &'a str { }
}
```

---

## 📊 Aplicando las Reglas

### Ejemplo 1: Una Referencia de Entrada

```rust
fn first_word(s: &str) -> &str
```

| Paso | Regla | Resultado |
|------|-------|-----------|
| 1 | Regla 1 | `fn first_word<'a>(s: &'a str) -> &str` |
| 2 | Regla 2 | `fn first_word<'a>(s: &'a str) -> &'a str` |

✅ Completo - No necesita anotaciones manuales.

### Ejemplo 2: Dos Referencias de Entrada

```rust
fn longest(x: &str, y: &str) -> &str
```

| Paso | Regla | Resultado |
|------|-------|-----------|
| 1 | Regla 1 | `fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str` |
| 2 | Regla 2 | No aplica (hay 2 lifetimes) |
| 3 | Regla 3 | No aplica (no hay self) |

❌ Incompleto - El output no tiene lifetime. **Requiere anotación manual**.

### Ejemplo 3: Método con &self

```rust
impl Parser {
    fn parse(&self, input: &str) -> &str
}
```

| Paso | Regla | Resultado |
|------|-------|-----------|
| 1 | Regla 1 | `fn parse<'a, 'b>(&'a self, input: &'b str) -> &str` |
| 2 | Regla 2 | No aplica (hay 2 lifetimes) |
| 3 | Regla 3 | `fn parse<'a, 'b>(&'a self, input: &'b str) -> &'a str` |

✅ Completo - El lifetime de `&self` se usa para el output.

---

## 🔍 Casos que Requieren Anotación Manual

### Caso 1: Múltiples Inputs, Retorno Podría Venir de Cualquiera

```rust
// ❌ Elision no puede determinar el lifetime
fn choose(a: &str, b: &str, pick_first: bool) -> &str {
    if pick_first { a } else { b }
}

// ✅ Anotación manual necesaria
fn choose<'a>(a: &'a str, b: &'a str, pick_first: bool) -> &'a str {
    if pick_first { a } else { b }
}
```

### Caso 2: Retorno de Referencia Diferente a &self

```rust
impl<'a> Parser<'a> {
    // ❌ Elision asignaría lifetime de &self
    fn get_input(&self) -> &str { self.input }
    
    // ✅ Explícito: retorna con lifetime del campo
    fn get_input(&self) -> &'a str { self.input }
}
```

### Caso 3: Structs con Referencias

```rust
// ❌ Structs SIEMPRE requieren lifetimes explícitos
struct Excerpt {
    part: &str,  // Error!
}

// ✅ Siempre anotar
struct Excerpt<'a> {
    part: &'a str,
}
```

---

## ✅ Cuándo la Elision Funciona

| Firma | ¿Elision? | Razón |
|-------|-----------|-------|
| `fn f(x: &str) -> &str` | ✅ | Regla 2 |
| `fn f(&self) -> &str` | ✅ | Regla 3 |
| `fn f(&self, x: &str) -> &str` | ✅ | Regla 3 |
| `fn f(x: &str, y: &str) -> &str` | ❌ | Ambiguo |
| `fn f(x: &str, y: i32) -> &str` | ✅ | Solo x es ref |
| `fn f() -> &str` | ❌ | No hay input ref |

---

## 📝 Ejemplos Prácticos

### Funciones que NO Necesitan Anotación

```rust
// Una referencia de entrada
fn trim(s: &str) -> &str {
    s.trim()
}

// Método con &self
impl Text {
    fn first_line(&self) -> &str {
        self.content.lines().next().unwrap_or("")
    }
}

// Solo una referencia importa para el retorno
fn process(data: &[u8], _config: &Config) -> &[u8] {
    // Retorna slice de data, no de config
    &data[..10]
}
```

### Funciones que SÍ Necesitan Anotación

```rust
// Múltiples referencias, ambas pueden retornarse
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

// Retorno no viene de self
impl<'a> Container<'a> {
    fn data(&self) -> &'a [u8] {
        self.buffer
    }
}
```

---

## 💡 Consejo Práctico

> **Empieza sin anotaciones.** Si el compilador se queja, agrégalas.

```rust
// 1. Prueba sin anotaciones
fn process(input: &str) -> &str { ... }

// 2. Si hay error, el compilador te dirá qué falta
// error[E0106]: missing lifetime specifier

// 3. Agrega las anotaciones necesarias
fn process<'a>(input: &'a str) -> &'a str { ... }
```

---

## ✅ Verificación de Comprensión

### Ejercicio 1
¿Esta función necesita anotaciones de lifetime?

```rust
fn get_first(items: &[String]) -> &String {
    &items[0]
}
```

<details>
<summary>Ver respuesta</summary>

**No**, la elision funciona. Regla 2: una referencia de entrada, su lifetime se aplica a la salida.

</details>

### Ejercicio 2
¿Por qué esta función necesita anotaciones?

```rust
fn pick(a: &str, b: &str, first: bool) -> &str
```

<details>
<summary>Ver respuesta</summary>

Porque hay dos referencias de entrada (`a` y `b`) y el retorno podría venir de cualquiera. Regla 2 no aplica, y no hay `&self` para Regla 3.

</details>

---

## 📌 Puntos Clave

1. **Regla 1**: Cada input ref obtiene su propio lifetime
2. **Regla 2**: Un solo input lifetime → se aplica a outputs
3. **Regla 3**: `&self`/`&mut self` → su lifetime va a outputs
4. Si las 3 reglas no determinan todos los lifetimes → anotación manual
5. **Structs siempre** requieren anotaciones explícitas

---

## 🔗 Próximo Tema

Ahora veremos conceptos avanzados: `'static`, lifetime bounds, y patrones especiales.

→ [05 - Lifetimes Avanzados](05-lifetimes-avanzados.md)
