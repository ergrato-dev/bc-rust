// project-macro-toolkit-derive/src/lib.rs
//
// Crate proc-macro del proyecto integrador.
// Contiene tres macros:
//   1. `#[derive(Describe)]`  — ya implementado como referencia
//   2. `#[log_call]`          — a implementar
//   3. `#[builder]`           — a implementar (patrón builder automático)

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, ItemFn, LitStr,
};

// ═══════════════════════════════════════════════════════════════════
// 1. #[derive(Describe)] — IMPLEMENTADO (referencia)
// ═══════════════════════════════════════════════════════════════════

/// Genera `fn describe(&self) -> String` para cualquier struct.
#[proc_macro_derive(Describe)]
pub fn describe_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_describe_full(&ast).into()
}

fn impl_describe_full(ast: &DeriveInput) -> TokenStream2 {
    let nombre = &ast.ident;
    let nombre_str = nombre.to_string();

    let campos_expr: TokenStream2 = match &ast.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => {
                let partes: Vec<TokenStream2> = fields
                    .named
                    .iter()
                    .map(|f| {
                        let id = f.ident.as_ref().unwrap();
                        let s = id.to_string();
                        quote! { format!("{}: {:?}", #s, self.#id) }
                    })
                    .collect();
                quote! { vec![#(#partes),*].join(", ") }
            }
            Fields::Unnamed(fields) => {
                let partes: Vec<TokenStream2> = fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, _)| {
                        let idx = syn::Index::from(i);
                        quote! { format!("{}: {:?}", #i, self.#idx) }
                    })
                    .collect();
                quote! { vec![#(#partes),*].join(", ") }
            }
            Fields::Unit => quote! { String::new() },
        },
        _ => {
            return syn::Error::new_spanned(
                nombre,
                "#[derive(Describe)] solo puede aplicarse a structs",
            )
            .to_compile_error();
        }
    };

    quote! {
        impl Describe for #nombre {
            fn describe(&self) -> String {
                let campos = #campos_expr;
                if campos.is_empty() {
                    #nombre_str.to_string()
                } else {
                    format!("{} {{ {} }}", #nombre_str, campos)
                }
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// 2. #[log_call] — A IMPLEMENTAR
// ═══════════════════════════════════════════════════════════════════

/// Registra entrada y salida de una función.
///
/// Transformación:
/// ```ignore
/// #[log_call]
/// fn suma(a: i32, b: i32) -> i32 { a + b }
/// // →
/// fn suma(a: i32, b: i32) -> i32 {
///     println!("[TOOLKIT] → suma()");
///     let __result = { a + b };
///     println!("[TOOLKIT] ← suma() retornó {:?}", __result);
///     __result
/// }
/// ```
///
/// Usa el prefijo `[TOOLKIT]` en lugar de `[LOG]`.
#[proc_macro_attribute]
pub fn log_call(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    impl_log_call_toolkit(func).into()
}

fn impl_log_call_toolkit(func: ItemFn) -> TokenStream2 {
    // TODO: implementar similar a practice-04 pero con prefijo [TOOLKIT]
    let _ = func;
    todo!("implementar #[log_call] en project-macro-toolkit-derive")
}

// ═══════════════════════════════════════════════════════════════════
// 3. #[builder] — A IMPLEMENTAR
// ═══════════════════════════════════════════════════════════════════

/// Genera automáticamente el patrón Builder para un struct con campos nombrados.
///
/// Dado:
/// ```ignore
/// #[builder]
/// pub struct Config {
///     host: String,
///     port: u16,
///     timeout: u64,
/// }
/// ```
///
/// Genera:
/// ```ignore
/// pub struct Config { host: String, port: u16, timeout: u64 }
///
/// pub struct ConfigBuilder {
///     host:    Option<String>,
///     port:    Option<u16>,
///     timeout: Option<u64>,
/// }
///
/// impl ConfigBuilder {
///     pub fn new() -> Self { Self { host: None, port: None, timeout: None } }
///     pub fn host(mut self, val: String) -> Self { self.host = Some(val); self }
///     pub fn port(mut self, val: u16) -> Self { self.port = Some(val); self }
///     pub fn timeout(mut self, val: u64) -> Self { self.timeout = Some(val); self }
///     pub fn build(self) -> Result<Config, String> {
///         Ok(Config {
///             host:    self.host.ok_or("host es requerido")?,
///             port:    self.port.ok_or("port es requerido")?,
///             timeout: self.timeout.ok_or("timeout es requerido")?,
///         })
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn builder(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    impl_builder(&ast).into()
}

fn impl_builder(ast: &DeriveInput) -> TokenStream2 {
    let nombre = &ast.ident;
    let builder_nombre = format_ident!("{}Builder", nombre);

    let fields = match &ast.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(f) => &f.named,
            _ => {
                return syn::Error::new_spanned(
                    nombre,
                    "#[builder] solo soporta structs con campos nombrados",
                )
                .to_compile_error();
            }
        },
        _ => {
            return syn::Error::new_spanned(nombre, "#[builder] solo puede aplicarse a structs")
                .to_compile_error();
        }
    };

    // TODO: implementar según el docstring
    // Pistas:
    //   - Para cada field: let campo = &f.ident; let tipo = &f.ty;
    //   - Builder fields: quote! { #campo: Option<#tipo> }
    //   - Builder::new(): quote! { #campo: None }
    //   - Setter: quote! { pub fn #campo(mut self, val: #tipo) -> Self { ... } }
    //   - build(): usa ok_or() para cada campo requerido
    let _ = fields;
    let _ = &builder_nombre;

    todo!("implementar #[builder]")
}
