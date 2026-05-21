//! # rwc — Contador de palabras/líneas/bytes (RIIR de `wc`)
//!
//! Reemplazo de la utilidad `wc` de Unix, implementado en Rust.
//!
//! ## Uso
//!
//! ```bash
//! rwc archivo.txt           # líneas, palabras, bytes
//! rwc -l archivo.txt        # solo líneas
//! rwc -w archivo.txt        # solo palabras
//! rwc -c archivo.txt        # solo bytes
//! rwc -l -w archivo.txt     # líneas y palabras
//! echo "hola mundo" | rwc   # leer desde stdin
//! ```

use clap::Parser;
use std::fs;
use std::io::{self, Read};

/// rwc — Reescritura de `wc` en Rust
#[derive(Parser, Debug)]
#[command(
    name = "rwc",
    version = "0.1.0",
    about = "Cuenta líneas, palabras y bytes de archivos de texto"
)]
struct Args {
    /// Archivos a procesar (si no se especifica, lee de stdin)
    archivos: Vec<String>,

    /// Contar líneas
    #[arg(short = 'l', long)]
    lineas: bool,

    /// Contar palabras
    #[arg(short = 'w', long)]
    palabras: bool,

    /// Contar bytes
    #[arg(short = 'c', long)]
    bytes: bool,
}

/// Resultado del conteo de un texto.
#[derive(Debug, Default, PartialEq)]
struct Conteo {
    /// Número de líneas.
    lineas: usize,
    /// Número de palabras.
    palabras: usize,
    /// Número de bytes.
    bytes: usize,
}

/// Cuenta líneas, palabras y bytes de un texto.
fn contar(texto: &str) -> Conteo {
    Conteo {
        lineas: texto.lines().count(),
        palabras: texto.split_whitespace().count(),
        bytes: texto.len(),
    }
}

fn imprimir_conteo(conteo: &Conteo, nombre: &str, args: &Args) {
    let mostrar_todo = !args.lineas && !args.palabras && !args.bytes;
    let mut partes = Vec::new();

    if args.lineas || mostrar_todo {
        partes.push(format!("{:8}", conteo.lineas));
    }
    if args.palabras || mostrar_todo {
        partes.push(format!("{:8}", conteo.palabras));
    }
    if args.bytes || mostrar_todo {
        partes.push(format!("{:8}", conteo.bytes));
    }

    println!("{} {nombre}", partes.join(""));
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if args.archivos.is_empty() {
        // Leer de stdin
        let mut contenido = String::new();
        io::stdin().read_to_string(&mut contenido)?;
        let c = contar(&contenido);
        imprimir_conteo(&c, "", &args);
    } else {
        let mut total = Conteo::default();
        let mostrar_total = args.archivos.len() > 1;

        for archivo in &args.archivos {
            match fs::read_to_string(archivo) {
                Ok(contenido) => {
                    let c = contar(&contenido);
                    total.lineas += c.lineas;
                    total.palabras += c.palabras;
                    total.bytes += c.bytes;
                    imprimir_conteo(&c, archivo, &args);
                }
                Err(e) => {
                    eprintln!("rwc: {archivo}: {e}");
                }
            }
        }

        if mostrar_total {
            imprimir_conteo(&total, "total", &args);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contar_vacio() {
        let c = contar("");
        assert_eq!(c, Conteo { lineas: 0, palabras: 0, bytes: 0 });
    }

    #[test]
    fn contar_una_linea() {
        let c = contar("hola mundo\n");
        assert_eq!(c.lineas, 1);
        assert_eq!(c.palabras, 2);
        assert_eq!(c.bytes, 11);
    }

    #[test]
    fn contar_multiples_lineas() {
        let texto = "hola\nmundo\nRust\n";
        let c = contar(texto);
        assert_eq!(c.lineas, 3);
        assert_eq!(c.palabras, 3);
    }

    #[test]
    fn contar_espacios_multiples() {
        let c = contar("  hola   mundo  ");
        assert_eq!(c.palabras, 2);
    }
}
