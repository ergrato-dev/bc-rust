# Recursos - Semana 11: Lifetimes

## 📚 Documentación Oficial

### The Rust Book
- [Capítulo 10.3: Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [Capítulo 19.2: Advanced Lifetimes](https://doc.rust-lang.org/book/ch19-02-advanced-lifetimes.html)

### Rust Reference
- [Lifetimes](https://doc.rust-lang.org/reference/lifetime-elision.html)
- [Subtyping and Variance](https://doc.rust-lang.org/reference/subtyping.html)

### Rust By Example
- [Lifetimes](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html)
- [Explicit Annotation](https://doc.rust-lang.org/rust-by-example/scope/lifetime/explicit.html)
- [Elision](https://doc.rust-lang.org/rust-by-example/scope/lifetime/elision.html)

---

## 📖 Artículos Recomendados

### Nivel Principiante
- [Understanding Rust Lifetimes](https://blog.logrocket.com/understanding-lifetimes-rust/)
- [Rust Lifetimes: A Complete Guide](https://www.shuttle.rs/blog/2022/02/16/rust-lifetimes)

### Nivel Intermedio
- [Common Rust Lifetime Misconceptions](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md)
- [Lifetimes in Rust](https://hashrust.com/blog/lifetimes-in-rust/)

### Nivel Avanzado
- [The Rustonomicon: Lifetimes](https://doc.rust-lang.org/nomicon/lifetimes.html)
- [Variance in Rust](https://doc.rust-lang.org/nomicon/subtyping.html)

---

## 🎥 Videos

### Tutoriales
- [Lifetimes in Rust - Let's Get Rusty](https://www.youtube.com/watch?v=juIINGuZyBc)
- [Rust Lifetimes Explained - Jon Gjengset](https://www.youtube.com/watch?v=rAl-9HwD858)

### Conferencias
- [RustConf: Lifetimes Workshop](https://www.youtube.com/results?search_query=rustconf+lifetimes)

---

## 🛠️ Herramientas

### Visualización
- [Rust Playground](https://play.rust-lang.org/) - Para experimentar con lifetimes
- [Compiler Explorer](https://godbolt.org/) - Ver cómo Rust maneja referencias

### Análisis
- `cargo clippy` - Detecta problemas de lifetimes
- `rust-analyzer` - Hints de lifetime en VSCode

---

## 📝 Ejercicios Adicionales

### Rustlings
```bash
# Ejercicios de lifetimes en rustlings
rustlings watch lifetimes
```

Ejercicios relacionados:
- `lifetimes1.rs`
- `lifetimes2.rs`
- `lifetimes3.rs`

### Exercism
- [Exercism Rust Track](https://exercism.org/tracks/rust)
- Ejercicios con lifetimes: "gigasecond", "custom-set", "circular-buffer"

---

## 📚 Libros

### The Rust Programming Language
- Capítulo 10: Generic Types, Traits, and Lifetimes
- Capítulo 19: Advanced Features

### Programming Rust (O'Reilly)
- Capítulo 5: References
- Capítulo 21: Unsafe Code (lifetimes avanzados)

### Rust in Action
- Capítulo 6: Memory Safety Without Garbage Collection

---

## 🔗 Enlaces Útiles

### Comunidad
- [r/rust](https://www.reddit.com/r/rust/) - Preguntas sobre lifetimes
- [Rust Users Forum](https://users.rust-lang.org/)
- [Rust Discord](https://discord.gg/rust-lang)

### Cheat Sheets
- [Rust Language Cheat Sheet - Lifetimes](https://cheats.rs/#lifetime-elision)
- [Rust Lifetime Cheat Sheet](https://github.com/usagi/rust-memory-container-cs)

---

## 🎯 Patrones Comunes

### Pattern: Borrowed Iterator
```rust
struct Iter<'a> {
    data: &'a [i32],
    index: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            let item = &self.data[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}
```

### Pattern: Builder with Lifetime
```rust
struct Config<'a> {
    name: &'a str,
    value: i32,
}

struct ConfigBuilder<'a> {
    name: Option<&'a str>,
    value: Option<i32>,
}

impl<'a> ConfigBuilder<'a> {
    fn new() -> Self {
        ConfigBuilder { name: None, value: None }
    }

    fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    fn build(self) -> Option<Config<'a>> {
        Some(Config {
            name: self.name?,
            value: self.value.unwrap_or(0),
        })
    }
}
```

---

## 📋 Checklist de Aprendizaje

- [ ] Entender por qué Rust necesita lifetimes
- [ ] Saber cuándo anotar lifetimes manualmente
- [ ] Comprender las reglas de elisión
- [ ] Usar lifetimes en funciones
- [ ] Usar lifetimes en structs
- [ ] Entender `'static`
- [ ] Aplicar lifetime bounds en generics
- [ ] Conocer Higher-Ranked Trait Bounds (HRTB)
