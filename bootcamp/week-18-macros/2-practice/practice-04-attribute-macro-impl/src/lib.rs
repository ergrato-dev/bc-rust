// practice-04-attribute-macro-impl/src/lib.rs
//
// Implementa el attribute macro `#[log_call]` que envuelve una función
// con trazas de entrada y salida:
//
//   #[log_call]
//   fn suma(a: i32, b: i32) -> i32 { a + b }
//
// Se transforma en:
//
//   fn suma(a: i32, b: i32) -> i32 {
//       println!("[LOG] → suma()");
//       let __result = { a + b };
//       println!("[LOG] ← suma() = {:?}", __result);
//       __result
//   }
//
// También implementa `#[rename(nuevo_nombre)]` que cambia el identificador
// de la función manteniendo el resto sin cambios.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};

// ── #[log_call] ──────────────────────────────────────────────────────────────

/// Envuelve la función con `println!` de entrada y salida.
///
/// # Pasos a implementar:
///
/// 1. Parsear `item` como `ItemFn` con `parse_macro_input!`.
/// 2. Extraer el nombre: `&func.sig.ident` y su `to_string()`.
/// 3. Extraer el bloque original: `&func.block`.
/// 4. Preservar atributos, visibilidad y firma:
///    `&func.attrs`, `&func.vis`, `&func.sig`.
/// 5. Generar con `quote!`:
///    ```
///    #(#attrs)* #vis #sig {
///        println!("[LOG] → {}()", #nombre_str);
///        let __result = #bloque;
///        println!("[LOG] ← {}() = {:?}", #nombre_str, __result);
///        __result
///    }
///    ```
#[proc_macro_attribute]
pub fn log_call(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    impl_log_call(func).into()
}

fn impl_log_call(func: ItemFn) -> TokenStream2 {
    // TODO: implementar según los pasos del docstring
    let _ = func;
    todo!("implementar #[log_call]")
}

// ── #[rename(nuevo_nombre)] ───────────────────────────────────────────────────

/// Renombra la función al identificador dado en el atributo.
///
/// Ejemplo:
///   `#[rename(calcular)]`
///   `fn suma(a: i32, b: i32) -> i32 { a + b }`
///
/// Genera: `fn calcular(a: i32, b: i32) -> i32 { a + b }`
///
/// # Pasos a implementar:
///
/// 1. Parsear `attr` como `LitStr` para obtener el nuevo nombre.
/// 2. Crear el nuevo ident: `syn::Ident::new(&nuevo_nombre, func.sig.ident.span())`.
/// 3. Parsear `item` como `ItemFn`.
/// 4. Reemplazar `func.sig.ident` con el nuevo ident.
/// 5. Retornar `quote! { #func }`.
#[proc_macro_attribute]
pub fn rename(attr: TokenStream, item: TokenStream) -> TokenStream {
    let nuevo_nombre = parse_macro_input!(attr as LitStr).value();
    let mut func = parse_macro_input!(item as ItemFn);
    impl_rename(&nuevo_nombre, &mut func).into()
}

fn impl_rename(nuevo_nombre: &str, func: &mut ItemFn) -> TokenStream2 {
    // TODO: implementar según los pasos del docstring
    let _ = nuevo_nombre;
    let _ = func;
    todo!("implementar #[rename]")
}
