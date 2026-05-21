// practice-03-custom-derive-macro/src/lib.rs
//
// Implementa el derive macro `Describe` que genera automáticamente:
//
//   impl Describe for MiStruct {
//       fn describe(&self) -> String {
//           "MiStruct { campo1: <valor>, campo2: <valor> }".to_string()
//       }
//   }
//
// Para structs con campos nombrados, posicionales y unit.
// Solo debes implementar `impl_describe`. La función pública ya está dada.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Punto de entrada del custom derive `#[derive(Describe)]`.
///
/// No modificar esta función — delega a `impl_describe`.
#[proc_macro_derive(Describe)]
pub fn describe_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_describe(&ast).into()
}

/// Genera la implementación de `Describe` para el tipo recibido.
///
/// # Pasos a implementar:
///
/// 1. Extraer `ast.ident` (nombre del tipo).
/// 2. Con `match &ast.data`:
///    - `Data::Struct` con `Fields::Named`: iterar campos con nombre,
///      generar `format!("campo: {:?}", self.campo)` para cada uno.
///    - `Data::Struct` con `Fields::Unnamed`: iterar por índice (0, 1, ...),
///      generar `format!("0: {:?}", self.0)` etc.
///    - `Data::Struct` con `Fields::Unit`: retornar solo el nombre.
///    - Cualquier otro caso: emitir error con `syn::Error::new_spanned`.
/// 3. Generar con `quote!` el bloque `impl Describe for #nombre { ... }`.
fn impl_describe(ast: &DeriveInput) -> TokenStream2 {
    let nombre = &ast.ident;

    let campos_expr: TokenStream2 = match &ast.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => {
                // TODO: iterar fields.named y generar expresiones de formato
                // Pista: para cada campo f:
                //   let campo_ident = f.ident.as_ref().unwrap();
                //   let campo_str   = campo_ident.to_string();
                //   quote! { format!("{}: {:?}", #campo_str, self.#campo_ident) }
                // Luego: quote! { vec![#(#partes),*].join(", ") }
                let _ = fields;
                todo!("implementar Fields::Named")
            }

            Fields::Unnamed(fields) => {
                // TODO: iterar fields.unnamed con enumerate()
                // Pista: let idx = syn::Index::from(i);
                //   quote! { format!("{}: {:?}", #i, self.#idx) }
                let _ = fields;
                todo!("implementar Fields::Unnamed")
            }

            Fields::Unit => {
                // Struct sin campos: solo retornar el nombre del tipo
                quote! { String::new() }
            }
        },

        _ => {
            return syn::Error::new_spanned(
                nombre,
                "#[derive(Describe)] solo puede aplicarse a structs",
            )
            .to_compile_error();
        }
    };

    let nombre_str = nombre.to_string();

    // TODO: completar el quote! con la lógica de formato condicional
    // Si campos_expr produce cadena vacía → solo "#nombre_str"
    // Si no → "#nombre_str { <campos> }"
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
