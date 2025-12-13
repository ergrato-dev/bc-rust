//! Práctica 03: I/O Asíncrono
//!
//! Operaciones de archivo y networking con Tokio.

use std::io;
use tokio::fs;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

// =============================================================================
// EJERCICIO 1: Operaciones de Archivos
// =============================================================================

/// Lee un archivo y retorna su contenido como String.
pub async fn leer_archivo(path: &str) -> io::Result<String> {
    // TODO: Implementar usando tokio::fs::read_to_string
    todo!("Implementar leer_archivo")
}

/// Escribe contenido a un archivo.
pub async fn escribir_archivo(path: &str, contenido: &str) -> io::Result<()> {
    // TODO: Implementar usando tokio::fs::write
    todo!("Implementar escribir_archivo")
}

/// Copia un archivo de origen a destino.
/// Retorna el número de bytes copiados.
pub async fn copiar_archivo(origen: &str, destino: &str) -> io::Result<u64> {
    // TODO: Implementar
    // 1. Abrir archivo origen para lectura
    // 2. Crear archivo destino para escritura
    // 3. Usar tokio::io::copy para copiar contenido
    todo!("Implementar copiar_archivo")
}

// =============================================================================
// EJERCICIO 2: Lectura por Líneas
// =============================================================================

/// Cuenta el número de líneas en un archivo.
pub async fn contar_lineas(path: &str) -> io::Result<usize> {
    // TODO: Implementar
    // 1. Abrir archivo
    // 2. Crear BufReader
    // 3. Usar .lines() para iterar
    // 4. Contar líneas
    todo!("Implementar contar_lineas")
}

/// Retorna las líneas que contienen el patrón dado.
pub async fn filtrar_lineas(path: &str, patron: &str) -> io::Result<Vec<String>> {
    // TODO: Implementar
    // 1. Abrir archivo con BufReader
    // 2. Iterar líneas
    // 3. Filtrar las que contienen el patrón
    // 4. Retornar vector de líneas
    todo!("Implementar filtrar_lineas")
}

// =============================================================================
// EJERCICIO 3: Servidor Echo TCP
// =============================================================================

/// Maneja una conexión individual (echo).
async fn manejar_conexion(mut socket: TcpStream, addr: std::net::SocketAddr) {
    println!("Nueva conexión desde: {}", addr);

    let mut buffer = [0u8; 1024];

    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => {
                println!("Conexión cerrada: {}", addr);
                break;
            }
            Ok(n) => {
                // Echo: devolver lo recibido
                if socket.write_all(&buffer[..n]).await.is_err() {
                    println!("Error escribiendo a: {}", addr);
                    break;
                }
            }
            Err(e) => {
                println!("Error leyendo de {}: {}", addr, e);
                break;
            }
        }
    }
}

/// Servidor echo que devuelve lo que recibe.
///
/// El servidor corre indefinidamente hasta que se cancele.
pub async fn servidor_echo(addr: &str) -> io::Result<()> {
    // TODO: Implementar
    // 1. Crear TcpListener con bind
    // 2. Loop infinito con accept
    // 3. Por cada conexión, spawn una task con manejar_conexion
    todo!("Implementar servidor_echo")
}

// =============================================================================
// EJERCICIO 4: Cliente TCP
// =============================================================================

/// Cliente echo que envía un mensaje y espera la respuesta.
pub async fn cliente_echo(addr: &str, mensaje: &str) -> io::Result<String> {
    // TODO: Implementar
    // 1. Conectar con TcpStream::connect
    // 2. Escribir el mensaje
    // 3. Leer la respuesta
    // 4. Convertir a String y retornar
    todo!("Implementar cliente_echo")
}

// =============================================================================
// MAIN
// =============================================================================

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("=== Práctica 03: I/O Asíncrono ===\n");

    // Ejercicio 1: Archivos
    println!("--- Ejercicio 1: Operaciones de Archivos ---");

    // Crear archivo de prueba
    let test_path = "/tmp/test_async.txt";
    let contenido = "Línea 1\nLínea 2 con Rust\nLínea 3\nRust es genial";

    escribir_archivo(test_path, contenido).await?;
    println!("Archivo creado: {}", test_path);

    let leido = leer_archivo(test_path).await?;
    println!("Contenido leído:\n{}", leido);

    let copia_path = "/tmp/test_async_copia.txt";
    let bytes = copiar_archivo(test_path, copia_path).await?;
    println!("Copiados {} bytes a {}", bytes, copia_path);
    println!();

    // Ejercicio 2: Lectura por líneas
    println!("--- Ejercicio 2: Lectura por Líneas ---");

    let num_lineas = contar_lineas(test_path).await?;
    println!("Número de líneas: {}", num_lineas);

    let lineas_rust = filtrar_lineas(test_path, "Rust").await?;
    println!("Líneas con 'Rust': {:?}", lineas_rust);
    println!();

    // Ejercicios 3 y 4: Servidor y Cliente
    println!("--- Ejercicios 3-4: Servidor y Cliente Echo ---");
    println!("Para probar el servidor, ejecuta en otra terminal:");
    println!("  echo 'Hola' | nc localhost 8080");
    println!();
    println!("O descomenta el código del servidor abajo.");

    // Descomentar para probar servidor/cliente:
    /*
    let server_addr = "127.0.0.1:8080";

    // Iniciar servidor en background
    let server_handle = tokio::spawn(async move {
        servidor_echo(server_addr).await.unwrap();
    });

    // Dar tiempo al servidor para iniciar
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    // Probar cliente
    let respuesta = cliente_echo(server_addr, "Hola Tokio!").await?;
    println!("Respuesta del servidor: {}", respuesta);

    // El servidor corre indefinidamente, abortamos
    server_handle.abort();
    */

    // Limpiar archivos de prueba
    fs::remove_file(test_path).await.ok();
    fs::remove_file(copia_path).await.ok();

    Ok(())
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_escribir_y_leer() {
        let path = "/tmp/test_rw.txt";
        let contenido = "Test contenido";

        escribir_archivo(path, contenido).await.unwrap();
        let leido = leer_archivo(path).await.unwrap();

        assert_eq!(leido, contenido);

        fs::remove_file(path).await.ok();
    }

    #[tokio::test]
    async fn test_copiar_archivo() {
        let origen = "/tmp/test_origen.txt";
        let destino = "/tmp/test_destino.txt";
        let contenido = "Contenido para copiar";

        escribir_archivo(origen, contenido).await.unwrap();
        let bytes = copiar_archivo(origen, destino).await.unwrap();

        assert_eq!(bytes, contenido.len() as u64);

        let leido = leer_archivo(destino).await.unwrap();
        assert_eq!(leido, contenido);

        fs::remove_file(origen).await.ok();
        fs::remove_file(destino).await.ok();
    }

    #[tokio::test]
    async fn test_contar_lineas() {
        let path = "/tmp/test_lineas.txt";
        escribir_archivo(path, "L1\nL2\nL3\nL4\nL5").await.unwrap();

        let count = contar_lineas(path).await.unwrap();
        assert_eq!(count, 5);

        fs::remove_file(path).await.ok();
    }

    #[tokio::test]
    async fn test_filtrar_lineas() {
        let path = "/tmp/test_filtro.txt";
        escribir_archivo(path, "Rust rocks\nPython ok\nRust rules\nGo fast")
            .await
            .unwrap();

        let rust_lines = filtrar_lineas(path, "Rust").await.unwrap();
        assert_eq!(rust_lines.len(), 2);
        assert!(rust_lines[0].contains("Rust"));
        assert!(rust_lines[1].contains("Rust"));

        fs::remove_file(path).await.ok();
    }

    #[tokio::test]
    async fn test_servidor_cliente_echo() {
        let addr = "127.0.0.1:18080"; // Puerto diferente para test

        // Iniciar servidor
        let server_handle = tokio::spawn(async move {
            servidor_echo(addr).await.ok();
        });

        // Dar tiempo al servidor
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Probar cliente
        let respuesta = cliente_echo(addr, "Test mensaje").await.unwrap();
        assert_eq!(respuesta, "Test mensaje");

        // Limpiar
        server_handle.abort();
    }
}
