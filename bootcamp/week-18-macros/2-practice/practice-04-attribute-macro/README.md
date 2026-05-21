# Práctica 04 — Attribute Macros: `#[log_call]` y `#[rename]`

## 🎯 Objetivo

Implementar dos attribute macros de proc-macro:
- `#[log_call]` — registra la entrada y salida de cualquier función
- `#[rename("nuevo")]` — cambia el identificador de una función en tiempo de compilación

## 🗂️ Estructura del Ejercicio

| Crate | Ruta | Rol |
|-------|------|-----|
| `practice-04-attribute-macro-impl` | `practice-04-attribute-macro-impl/` | Proc-macro (editar aquí) |
| `practice-04-attribute-macro` | `practice-04-attribute-macro/` | Consumidor (tests aquí) |

## 📋 Instrucciones

Edita **`practice-04-attribute-macro-impl/src/lib.rs`** e implementa las dos funciones:

### 1. `impl_log_call`

```rust
fn impl_log_call(func: ItemFn) -> TokenStream2 {
    let nombre_str = func.sig.ident.to_string();
    let bloque     = &func.block;
    let attrs      = &func.attrs;
    let vis        = &func.vis;
    let sig        = &func.sig;

    quote! {
        #(#attrs)* #vis #sig {
            println!("[LOG] → {}()", #nombre_str);
            let __result = #bloque;
            println!("[LOG] ← {}() = {:?}", #nombre_str, __result);
            __result
        }
    }
}
```

> **Nota**: el tipo de retorno debe implementar `Debug` para usar `{:?}`. Las funciones en los tests lo hacen.

### 2. `impl_rename`

```rust
fn impl_rename(nuevo_nombre: &str, func: &mut ItemFn) -> TokenStream2 {
    func.sig.ident = syn::Ident::new(nuevo_nombre, func.sig.ident.span());
    quote! { #func }
}
```

## Verificar

```bash
cargo build  -p practice-04-attribute-macro
cargo test   -p practice-04-attribute-macro
cargo expand -p practice-04-attribute-macro
```

## ✅ Criterios de Aceptación

- [ ] `cargo test -p practice-04-attribute-macro` — todos los tests pasan
- [ ] `suma(3, 4)` retorna `7` (el valor no cambia)
- [ ] `#[log_call]` imprime `[LOG] → suma()` y `[LOG] ← suma() = 7`
- [ ] `#[rename("adicion")]` permite llamar a la función como `adicion()`
- [ ] La función original (`add`) ya no es accesible tras el rename
- [ ] No hay warnings de compilación

## 💡 Pistas

<details>
<summary>Pista — Firma de #[proc_macro_attribute]</summary>

Un attribute macro siempre recibe dos `TokenStream`:
- `attr` — los argumentos dentro de los paréntesis: `#[rename("nuevo")]` → `"nuevo"`
- `item` — el elemento sobre el que se aplica (la función completa)

```rust
#[proc_macro_attribute]
pub fn mi_macro(attr: TokenStream, item: TokenStream) -> TokenStream { ... }
```
</details>

<details>
<summary>Pista — Preservar atributos y visibilidad</summary>

`ItemFn` tiene campos `attrs` (Vec de atributos), `vis` (pub/pub(crate)/nada) y `sig` (firma). Deben incluirse en el output para no perder `pub`, `#[inline]`, etc.:

```rust
let attrs = &func.attrs;
let vis   = &func.vis;
let sig   = &func.sig;
quote! { #(#attrs)* #vis #sig { ... } }
```
</details>

## 🔗 Referencias

- [syn::ItemFn](https://docs.rs/syn/latest/syn/struct.ItemFn.html)
- [Rust Reference — Attribute Macros](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros)
- [Teoría: 05-macros-avanzadas.md](../../1-theory/05-macros-avanzadas.md)
