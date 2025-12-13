# 📚 Lifetimes en Structs

![Lifetimes en Structs](../0-assets/03-lifetimes-structs.svg)

## 🎯 Objetivos de Aprendizaje

- Declarar structs que contienen referencias
- Implementar métodos en structs con lifetimes
- Entender la relación entre struct y sus referencias

---

## 🏗️ Structs con Referencias

### ¿Por Qué Lifetimes en Structs?

Cuando un struct contiene referencias, Rust necesita saber:
- ¿Cuánto tiempo vivirán los datos referenciados?
- ¿El struct puede outlive sus referencias?

```rust
// ❌ NO compila - falta lifetime
struct Excerpt {
    part: &str,
}
```

```
error[E0106]: missing lifetime specifier
 --> src/main.rs:2:11
  |
2 |     part: &str,
  |           ^ expected named lifetime parameter
```

### Sintaxis Correcta

```rust
// ✅ Compila
struct Excerpt<'a> {
    part: &'a str,
}
```

**Significado:** "Un `Excerpt` no puede vivir más que la referencia `part`"

---

## 📊 Visualización del Lifetime

```
fn main() {
    let novela = String::from("Llámame Ishmael. Hace años...");
    //           ├──────────────────────────────────────────┤
    //           │          Lifetime de novela ('a)         │
    //           │                                          │
    let primera_oracion;  //                                │
    {                     //                                │
        let excerpt = Excerpt {  // ┬── Lifetime de excerpt │
            part: &novela[..16],   // │  (limitado por 'a)  │
        };                         // │                     │
        primera_oracion = excerpt.part;  // │               │
    }                             // ┴──────────────────────│
    println!("{}", primera_oracion);  //                    │
}                                 // ───────────────────────┘
```

---

## 🔧 Implementando Métodos

### Sintaxis de impl con Lifetimes

```rust
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    // Método que no retorna referencia
    fn len(&self) -> usize {
        self.part.len()
    }
    
    // Método que retorna la referencia interna
    fn part(&self) -> &str {
        self.part
    }
}
```

**Nota:** En `impl<'a>`, declaramos `'a` para poder usarlo en `Excerpt<'a>`.

### Métodos que Retornan Referencias

```rust
impl<'a> Excerpt<'a> {
    // Retorna parte del contenido
    fn first_word(&self) -> &str {
        self.part.split_whitespace().next().unwrap_or("")
    }
    
    // Retorna referencia con el mismo lifetime
    fn get_part(&self) -> &'a str {
        self.part
    }
}
```

---

## 📝 Múltiples Referencias en Structs

### Mismo Lifetime

```rust
struct Pair<'a> {
    first: &'a str,
    second: &'a str,
}

fn main() {
    let s1 = String::from("hola");
    let s2 = String::from("mundo");
    
    let pair = Pair {
        first: &s1,
        second: &s2,
    };
    
    println!("{} {}", pair.first, pair.second);
}
```

### Diferentes Lifetimes

```rust
struct Context<'a, 'b> {
    text: &'a str,
    metadata: &'b str,
}

impl<'a, 'b> Context<'a, 'b> {
    fn new(text: &'a str, metadata: &'b str) -> Self {
        Context { text, metadata }
    }
}
```

---

## 🎯 Patrones Comunes

### Patrón 1: Parser/Tokenizer

```rust
struct Parser<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser { input, position: 0 }
    }
    
    fn remaining(&self) -> &'a str {
        &self.input[self.position..]
    }
    
    fn advance(&mut self, n: usize) {
        self.position += n;
    }
}
```

### Patrón 2: Vista de Datos

```rust
struct DataView<'a> {
    data: &'a [u8],
    name: &'a str,
}

impl<'a> DataView<'a> {
    fn first_n(&self, n: usize) -> &'a [u8] {
        &self.data[..n.min(self.data.len())]
    }
}
```

### Patrón 3: Cache/Buffer

```rust
struct LineBuffer<'a> {
    content: &'a str,
    lines: Vec<&'a str>,
}

impl<'a> LineBuffer<'a> {
    fn new(content: &'a str) -> Self {
        let lines = content.lines().collect();
        LineBuffer { content, lines }
    }
    
    fn get_line(&self, index: usize) -> Option<&'a str> {
        self.lines.get(index).copied()
    }
}
```

---

## ⚠️ Errores Comunes

### Error 1: Struct que Outlive su Referencia

```rust
// ❌ NO compila
fn create_excerpt() -> Excerpt {
    let text = String::from("hola mundo");
    Excerpt { part: &text }  // text se destruye aquí
}
```

**Solución:** El dato debe vivir fuera de la función.

### Error 2: Olvidar Lifetime en impl

```rust
// ❌ NO compila
impl Excerpt<'a> {  // 'a no está declarado
    fn len(&self) -> usize { self.part.len() }
}

// ✅ Correcto
impl<'a> Excerpt<'a> {
    fn len(&self) -> usize { self.part.len() }
}
```

---

## 🔄 Struct con Datos Propios y Referencias

Puedes combinar datos owned y referencias:

```rust
struct Article<'a> {
    title: String,           // Owned
    content: String,         // Owned  
    summary: &'a str,        // Referencia externa
}

impl<'a> Article<'a> {
    fn new(title: String, content: String, summary: &'a str) -> Self {
        Article { title, content, summary }
    }
}
```

---

## ✅ Verificación de Comprensión

### Ejercicio 1
¿Por qué este código no compila?

```rust
struct Holder<'a> {
    value: &'a i32,
}

fn main() {
    let holder;
    {
        let x = 5;
        holder = Holder { value: &x };
    }
    println!("{}", holder.value);
}
```

<details>
<summary>Ver respuesta</summary>

`x` se destruye al final del bloque interno, pero `holder` intenta usar `holder.value` después. El struct no puede outlive la referencia que contiene.

</details>

### Ejercicio 2
Completa los lifetimes:

```rust
struct Config {
    name: &str,
    value: &str,
}
```

<details>
<summary>Ver respuesta</summary>

```rust
struct Config<'a> {
    name: &'a str,
    value: &'a str,
}
// O con diferentes lifetimes:
struct Config<'a, 'b> {
    name: &'a str,
    value: &'b str,
}
```

</details>

---

## 📌 Puntos Clave

1. Structs con referencias **requieren** anotaciones de lifetime
2. El struct no puede vivir más que sus referencias
3. En `impl<'a>`, declaramos el lifetime antes de usarlo
4. Puedes usar múltiples lifetimes si son independientes
5. Combinar datos owned y referencias es válido

---

## 🔗 Próximo Tema

Hemos anotado lifetimes manualmente, pero Rust puede inferirlos en muchos casos. Aprenderemos las **reglas de elision**.

→ [04 - Reglas de Elision](04-elision-rules.md)
