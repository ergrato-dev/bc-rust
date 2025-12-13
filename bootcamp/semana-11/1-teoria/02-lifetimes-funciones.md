# 📚 Lifetimes en Funciones

## 🎯 Objetivos de Aprendizaje

- Anotar lifetimes en parámetros de funciones
- Relacionar lifetimes de entrada con salidas
- Resolver errores comunes de lifetime en funciones

---

## 🔧 Anotando Lifetimes en Funciones

### Sintaxis Básica

```rust
fn funcion<'a>(param: &'a str) -> &'a str {
    param
}
```

**Desglose:**
- `<'a>` - Declara el lifetime genérico
- `param: &'a str` - El parámetro tiene lifetime `'a`
- `-> &'a str` - El retorno tiene el mismo lifetime

### El Ejemplo Clásico: longest

```rust
// ✅ Ahora compila
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

**¿Qué significa `'a` aquí?**

> "El lifetime del valor retornado es el **más corto** de los lifetimes de `x` e `y`"

---

## 📊 Visualizando Lifetimes

```
fn main() {
    let string1 = String::from("largo");     // ────┬─── 'a
    {                                        //     │
        let string2 = String::from("xy");    // ─┬──│─── 'b
        let result = longest(&string1,       //  │  │
                             &string2);      //  │  │
        println!("{}", result);              //  │  │
    }                                        // ─┴──│─── 'b termina
}                                            // ────┴─── 'a termina
```

El resultado tiene lifetime `'b` (el más corto), por eso solo puede usarse dentro del bloque.

---

## ⚠️ Errores Comunes y Soluciones

### Error 1: Retornar referencia que excede lifetime

```rust
// ❌ NO compila
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    let string1 = String::from("largo");
    {
        let string2 = String::from("xy");
        let result = longest(&string1, &string2);
        result  // Intentamos usar result fuera del scope de string2
    }
    // string2 ya no existe aquí
}
```

### Error 2: Diferentes lifetimes cuando deberían ser iguales

```rust
// ❌ NO compila - El retorno no dice de dónde viene
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```

```rust
// ✅ Solución: usar el mismo lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

---

## 🎯 Patrones Comunes

### Patrón 1: Un Parámetro, Un Retorno

```rust
fn first_word<'a>(s: &'a str) -> &'a str {
    match s.find(' ') {
        Some(pos) => &s[..pos],
        None => s,
    }
}
```

### Patrón 2: Múltiples Parámetros, Uno Determina Retorno

```rust
// Solo x determina el lifetime del retorno
fn usar_primero<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}
```

### Patrón 3: Múltiples Lifetimes Independientes

```rust
fn independientes<'a, 'b>(x: &'a str, y: &'b str) {
    println!("x: {}, y: {}", x, y);
    // No retornamos referencias, no hay conflicto
}
```

---

## 📝 Ejemplos Prácticos

### Encontrar Subcadena

```rust
fn find_substring<'a>(haystack: &'a str, needle: &str) -> Option<&'a str> {
    haystack.find(needle).map(|i| &haystack[i..i + needle.len()])
}

fn main() {
    let texto = String::from("Hola mundo Rust");
    let encontrado = find_substring(&texto, "mundo");
    println!("{:?}", encontrado);  // Some("mundo")
}
```

**Nota:** `needle` no necesita lifetime porque no afecta al retorno.

### Dividir en Partes

```rust
fn split_at_char<'a>(s: &'a str, c: char) -> (&'a str, &'a str) {
    match s.find(c) {
        Some(pos) => (&s[..pos], &s[pos + c.len_utf8()..]),
        None => (s, ""),
    }
}

fn main() {
    let texto = "clave=valor";
    let (clave, valor) = split_at_char(texto, '=');
    println!("Clave: {}, Valor: {}", clave, valor);
}
```

---

## 🔍 Lifetimes con Genéricos

Puedes combinar lifetimes con tipos genéricos:

```rust
fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Anuncio: {}", ann);
    if x.len() > y.len() { x } else { y }
}
```

---

## ✅ Verificación de Comprensión

### Ejercicio 1
¿Por qué este código no compila?

```rust
fn get_str<'a>() -> &'a str {
    let s = String::from("hola");
    &s
}
```

<details>
<summary>Ver respuesta</summary>

`s` se crea dentro de la función y se destruye al salir. No podemos retornar una referencia a algo que no existirá.

**Solución:** Retornar `String` en lugar de `&str`, o usar `&'static str`.

</details>

### Ejercicio 2
Anota los lifetimes correctamente:

```rust
fn first_or_second(first: &str, second: &str, use_first: bool) -> &str {
    if use_first { first } else { second }
}
```

<details>
<summary>Ver respuesta</summary>

```rust
fn first_or_second<'a>(first: &'a str, second: &'a str, use_first: bool) -> &'a str {
    if use_first { first } else { second }
}
```

</details>

---

## 📌 Puntos Clave

1. Declarar lifetimes con `<'a>` después del nombre de función
2. El retorno debe tener un lifetime que aparezca en los parámetros
3. Cuando múltiples referencias pueden retornarse, usar el mismo lifetime
4. El lifetime resultante es el **más corto** de los involucrados
5. No se puede retornar referencia a datos creados dentro de la función

---

## 🔗 Próximo Tema

Ahora que sabemos anotar lifetimes en funciones, veremos cómo usarlos en **structs**.

→ [03 - Lifetimes en Structs](03-lifetimes-structs.md)
