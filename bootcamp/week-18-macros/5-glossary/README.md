# Glosario — Semana 18: Macros

## A

**AST (Abstract Syntax Tree)**
Árbol que representa la estructura sintáctica de un programa. `syn` parsea código Rust en un AST en memoria que el macro puede navegar y transformar.

**Attribute Macro**
Tipo de proc-macro invocado como `#[mi_macro]` sobre un elemento (función, struct, etc.). Recibe dos `TokenStream`: los atributos del macro y el elemento sobre el que se aplica. Puede transformar o reemplazar completamente el elemento.

## D

**DeriveInput**
Tipo de `syn` que representa la estructura completa que recibe `#[derive(...)]`. Contiene `ident` (nombre), `data` (cuerpo: struct/enum/union) y `generics`.

**Derive Macro**
Tipo de proc-macro invocado via `#[derive(MiMacro)]`. Solo puede *añadir* código (no modificar el tipo anotado). Genera impls de traits automáticamente.

**Designator (macro_rules!)**
Categoría que indica qué tipo de fragmento acepta un placeholder en `macro_rules!`. Ejemplos: `expr`, `ident`, `ty`, `stmt`, `pat`, `tt`, `literal`. Ver tabla completa en [02-macro-rules.md](../1-theory/02-macro-rules.md).

## F

**`format_ident!`**
Macro de `quote` que crea un `syn::Ident` compuesto. Ejemplo: `format_ident!("{}Builder", nombre)` genera el identificador `MiTipoBuilder`.

**Function-like Macro**
Tipo de proc-macro invocado con sintaxis de llamada de función: `mi_macro!(...)`. A diferencia de `macro_rules!`, puede ejecutar lógica Rust arbitraria para generar el output.

## H

**Hygiene**
Propiedad de los macros que garantiza que los identificadores definidos dentro del macro no colisionan con los del código del invocador. En Rust, `macro_rules!` es *hygiénico* por defecto (cada invocación tiene su propio ámbito para variables locales).

## I

**`Ident`**
Tipo de `proc-macro2`/`syn` que representa un identificador Rust (nombre de variable, función, tipo, etc.). Se puede crear con `syn::Ident::new("nombre", span)`.

## M

**`macro_rules!`**
Sistema de macros declarativas (macros por ejemplo) de Rust. Define patrones de entrada (matcher) y plantillas de expansión. Más simple que proc-macros pero limitado a transformaciones de tokens sin lógica Rust completa.

**Macro Expansion**
Fase del compilador en la que los macros son evaluados y reemplazados por el código que generan. Ocurre antes del type-checking y la compilación final.

## P

**`parse_macro_input!`**
Macro de `syn` que parsea un `TokenStream` en un tipo syn específico. Si el parse falla, emite un error de compilación automáticamente.

**proc-macro**
Sistema de macros procedurales en Rust. Son funciones Rust que operan sobre `TokenStream`s en tiempo de compilación. Requieren un crate separado con `[lib] proc-macro = true`.

**proc-macro2**
Crate que provee una API compatible con `proc_macro` pero utilizable fuera del contexto de compilación (en tests, etc.). Recomendado sobre `proc_macro` directamente.

## Q

**`quote!`**
Macro del crate `quote` que convierte código Rust en un `TokenStream2`. Soporta interpolación (`#variable`) y repetición (`#(#items),*`).

## S

**Span**
Metadato que indica la posición en el código fuente de un token (archivo, línea, columna). Usado para generar mensajes de error que señalan la ubicación correcta.

**`syn`**
Crate para parsear código Rust en un AST navegable. El punto de entrada más común es `syn::parse_macro_input!(input as syn::DeriveInput)`.

## T

**`TokenStream`**
Secuencia de tokens que representa código Rust en tiempo de compilación. Es la unidad de comunicación entre el compilador y los proc-macros. Existen dos versiones: `proc_macro::TokenStream` (del compilador) y `proc_macro2::TokenStream` (del crate `proc-macro2`).

**TT (Token Tree)**
Unidad básica de un TokenStream. Puede ser un solo token (`ident`, literal, puntuación) o un grupo delimitado por `()`, `[]`, `{}`. El designator `tt` en `macro_rules!` acepta cualquier token tree.

## V

**Variadic Macro**
Macro que acepta un número variable de argumentos. En `macro_rules!`, se logra con `$($x:expr),*` (cero o más) o `$($x:expr),+` (uno o más).
