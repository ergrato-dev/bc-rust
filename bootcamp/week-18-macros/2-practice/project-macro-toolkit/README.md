# Proyecto — Macro Toolkit

## 🎯 Objetivo

Construir una librería utilitaria completa que combine las tres técnicas de macros vistas en la semana:
- **`#[derive(Describe)]`** — ya implementado como referencia
- **`#[log_call]`** — a implementar (version avanzada con prefijo `[TOOLKIT]`)
- **`#[builder]`** — a implementar (patrón Builder automático)
- **`map!` y `assert_matches!`** — macros declarativas (ya implementadas en `src/lib.rs`)

## 🗂️ Estructura del Proyecto

```
project-macro-toolkit-derive/    ← proc-macro crate (EDITAR)
├── Cargo.toml
└── src/lib.rs

project-macro-toolkit/           ← librería + demo + tests
├── Cargo.toml
├── src/
│   ├── lib.rs                   ← re-exporta todo
│   └── main.rs                  ← demo interactivo
└── tests/
    └── integration_test.rs      ← tests de integración
```

## 📋 Instrucciones

### Tarea 1 — Implementar `#[log_call]` (versión Toolkit)

En `project-macro-toolkit-derive/src/lib.rs`, función `impl_log_call_toolkit`:

```rust
fn impl_log_call_toolkit(func: ItemFn) -> TokenStream2 {
    let nombre_str = func.sig.ident.to_string();
    let bloque     = &func.block;
    let attrs      = &func.attrs;
    let vis        = &func.vis;
    let sig        = &func.sig;

    quote! {
        #(#attrs)* #vis #sig {
            println!("[TOOLKIT] → {}()", #nombre_str);
            let __result = #bloque;
            println!("[TOOLKIT] ← {}() retornó {:?}", #nombre_str, __result);
            __result
        }
    }
}
```

### Tarea 2 — Implementar `#[builder]`

En `project-macro-toolkit-derive/src/lib.rs`, función `impl_builder`:

```rust
fn impl_builder(ast: &DeriveInput) -> TokenStream2 {
    let nombre = &ast.ident;
    let builder_nombre = format_ident!("{}Builder", nombre);
    // (extraer fields como en el código existente)

    let builder_fields: Vec<_> = fields.iter().map(|f| {
        let campo = &f.ident;
        let tipo  = &f.ty;
        quote! { #campo: Option<#tipo> }
    }).collect();

    let builder_defaults: Vec<_> = fields.iter().map(|f| {
        let campo = &f.ident;
        quote! { #campo: None }
    }).collect();

    let setters: Vec<_> = fields.iter().map(|f| {
        let campo = &f.ident;
        let tipo  = &f.ty;
        quote! {
            pub fn #campo(mut self, val: #tipo) -> Self {
                self.#campo = Some(val);
                self
            }
        }
    }).collect();

    let build_assigns: Vec<_> = fields.iter().map(|f| {
        let campo     = &f.ident;
        let campo_str = campo.as_ref().unwrap().to_string();
        let err_msg   = format!("{} es requerido", campo_str);
        quote! { #campo: self.#campo.ok_or(#err_msg)? }
    }).collect();

    quote! {
        #ast  // mantener el struct original

        pub struct #builder_nombre {
            #(#builder_fields),*
        }

        impl #builder_nombre {
            pub fn new() -> Self {
                Self { #(#builder_defaults),* }
            }
            #(#setters)*
            pub fn build(self) -> Result<#nombre, String> {
                Ok(#nombre { #(#build_assigns),* })
            }
        }
    }
}
```

> **Nota**: El `#ast` en el `quote!` final re-emite el struct original. Necesitas cambiar la firma de `impl_builder` para que reciba el struct como `DeriveInput` y lo incluya en el output.

## Verificar

```bash
# Compilar
cargo build -p project-macro-toolkit

# Expandir macros
cargo expand -p project-macro-toolkit

# Ejecutar demo
cargo run -p project-macro-toolkit

# Tests de integración
cargo test -p project-macro-toolkit
```

## ✅ Criterios de Aceptación

- [ ] `cargo build -p project-macro-toolkit` sin errores ni warnings
- [ ] `cargo test -p project-macro-toolkit` — todos los tests pasan
- [ ] `#[log_call]` imprime prefijo `[TOOLKIT]`
- [ ] `#[builder]` genera `XxxBuilder` con setters fluent y `build() -> Result<Xxx, String>`
- [ ] `build()` retorna `Err` cuando falta un campo requerido
- [ ] `map!` y `assert_matches!` funcionan en los integration tests

## 💡 Consejos

- Estudia primero el `#[derive(Describe)]` ya implementado — sigue el mismo patrón
- Usa `cargo expand` para ver el código generado antes de testear
- Si hay errores de tipo en el builder, verifica que los tipos de los campos sean `Clone` o que los valores se muevan correctamente

## 🔗 Referencias

- [Teoría: 03-proc-macros-intro.md](../../1-theory/03-proc-macros-intro.md)
- [Teoría: 04-derive-macros.md](../../1-theory/04-derive-macros.md)
- [Teoría: 05-macros-avanzadas.md](../../1-theory/05-macros-avanzadas.md)
- [proc-macro workshop (David Tolnay)](https://github.com/dtolnay/proc-macro-workshop)
