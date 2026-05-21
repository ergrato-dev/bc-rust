# Práctica 03 — Custom Derive con `syn` + `quote`

## 🎯 Objetivo

Implementar el derive macro `#[derive(Describe)]` que genera automáticamente el método `fn describe(&self) -> String` para cualquier struct, mostrando el nombre del tipo y sus campos con sus valores.

## 🗂️ Estructura del Ejercicio

Este ejercicio usa **dos crates** en el workspace:

| Crate | Ruta | Rol |
|-------|------|-----|
| `practice-03-custom-derive-macro` | `practice-03-custom-derive-macro/` | Proc-macro (editar aquí) |
| `practice-03-custom-derive` | `practice-03-custom-derive/` | Consumidor (tests aquí) |

## 📋 Instrucciones

Solo debes editar `practice-03-custom-derive-macro/src/lib.rs`, en la función `impl_describe`:

### Paso 1 — Fields::Named

```rust
Fields::Named(fields) => {
    let partes: Vec<TokenStream2> = fields.named.iter().map(|f| {
        let campo_ident = f.ident.as_ref().unwrap();
        let campo_str   = campo_ident.to_string();
        quote! { format!("{}: {:?}", #campo_str, self.#campo_ident) }
    }).collect();
    quote! { vec![#(#partes),*].join(", ") }
}
```

### Paso 2 — Fields::Unnamed

```rust
Fields::Unnamed(fields) => {
    let partes: Vec<TokenStream2> = fields.unnamed.iter().enumerate().map(|(i, _)| {
        let idx = syn::Index::from(i);
        quote! { format!("{}: {:?}", #i, self.#idx) }
    }).collect();
    quote! { vec![#(#partes),*].join(", ") }
}
```

### Paso 3 — Verificar

```bash
cargo expand -p practice-03-custom-derive
cargo test -p practice-03-custom-derive
```

## ✅ Criterios de Aceptación

- [ ] Compila sin warnings
- [ ] `cargo test -p practice-03-custom-derive` — todos los tests pasan
- [ ] `Usuario { nombre: "Ana", edad: 30, activo: true }` para campos nombrados
- [ ] `Punto { 0: 3.14, 1: 2.71 }` para campos posicionales
- [ ] `Marcador` para struct unit (sin campos)
- [ ] `derive(Describe)` no interfiere con otros derives (`Debug`, `Clone`)

## 💡 Pistas

<details>
<summary>Pista — Cómo acceder a un campo por índice en quote!</summary>

Para campos posicionales (`struct Punto(f64, f64)`), acceder a `self.0` y `self.1` requiere un `syn::Index`, no un `usize` directo:

```rust
let idx = syn::Index::from(0usize);
quote! { self.#idx }  // genera self.0
```
</details>

<details>
<summary>Pista — Por qué #(#partes),*</summary>

La sintaxis `#(#partes),*` en `quote!` equivale al `$()*` de `macro_rules!`: itera el Vec de TokenStreams y los separa con `,`. El resultado se puede pasar a `vec![...]`.
</details>

## 🔗 Referencias

- [syn::DeriveInput](https://docs.rs/syn/latest/syn/struct.DeriveInput.html)
- [quote! macro](https://docs.rs/quote/latest/quote/macro.quote.html)
- [Teoría: 04-derive-macros.md](../../1-theory/04-derive-macros.md)
