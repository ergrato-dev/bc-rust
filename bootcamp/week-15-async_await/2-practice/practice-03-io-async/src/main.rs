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
pub async fn read_file(path: &str) -> io::Result<String> {
    // TODO: Implementar usando tokio::fs::read_to_string
    todo!("Implementar read_file")
}

/// Escribe contenido a un archivo.
pub async fn write_file(path: &str, content: &str) -> io::Result<()> {
    // TODO: Implementar usando tokio::fs::write
    todo!("Implementar write_file")
}

/// Copia un archivo de origen a destino.
/// Retorna el número de bytes copiados.
pub async fn copy_file(source: &str, destination: &str) -> io::Result<u64> {
    // TODO: Implementar
    // 1. Abrir archivo origen para lectura
    // 2. Crear archivo destino para escritura
    // 3. Usar tokio::io::copy para copiar contenido
    todo!("Implementar copy_file")
}

// =============================================================================
// EJERCICIO 2: Lectura por Líneas
// =============================================================================

/// Cuenta el número de líneas en un archivo.
pub async fn count_lines(path: &str) -> io::Result<usize> {
    // TODO: Implementar
    // 1. Abrir archivo
    // 2. Crear BufReader
    // 3. Usar .lines() para iterar
    // 4. Contar líneas
    todo!("Implementar count_lines")
}

/// Retorna las líneas que contienen el patrón dado.
pub async fn filter_lines(path: &str, pattern: &str) -> io::Result<Vec<String>> {
    // TODO: Implementar
    // 1. Abrir archivo con BufReader
    // 2. Iterar líneas
    // 3. Filtrar las que contienen el patrón
    // 4. Retornar vector de líneas
    todo!("Implementar filter_lines")
}

// =============================================================================
// EJERCICIO 3: Servidor Echo TCP
// =============================================================================

/// Maneja una conexión individual (echo).
async fn handle_connection(mut socket: TcpStream, addr: std::net::SocketAddr) {
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
pub async fn echo_server(addr: &str) -> io::Result<()> {
    // TODO: Implementar
    // 1. Crear TcpListener con bind
    // 2. Loop infinito con accept
    // 3. Por cada conexión, spawn una task con handle_connection
    todo!("Implementar echo_server")
}

// =============================================================================
// EJERCICIO 4: Cliente TCP
// =============================================================================

/// Cliente echo que envía un mensaje y espera la respuesta.
pub async fn echo_client(addr: &str, message: &str) -> io::Result<String> {
    // TODO: Implementar
    // 1. Conectar con TcpStream::connect
    // 2. Escribir el mensaje
    // 3. Leer la respuesta
    // 4. Convertir a String y retornar
    todo!("Implementar echo_client")
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
    let content = "Línea 1\nLínea 2 con Rust\nLínea 3\nRust es genial";

    write_file(test_path, content).await?;
    println!("Archivo creado: {}", test_path);

    let read_content = read_file(test_path).await?;
    println!("Contenido leído:\n{}", read_content);

    let copy_path = "/tmp/test_async_copia.txt";
    let bytes = copy_file(test_path, copy_path).await?;
    println!("Copiados {} bytes a {}", bytes, copy_path);
    println!();

    // Ejercicio 2: Lectura por líneas
    println!("--- Ejercicio 2: Lectura por Líneas ---");

    let num_lines = count_lines(test_path).await?;
    println!("Número de líneas: {}", num_lines);

    let rust_lines = filter_lines(test_path, "Rust").await?;
    println!("Líneas con 'Rust': {:?}", rust_lines);
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
        echo_server(server_addr).await.unwrap();
    });

    // Dar tiempo al servidor para iniciar
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    // Probar cliente
    let response = echo_client(server_addr, "Hola Tokio!").await?;
    println!("Respuesta del servidor: {}", response);

    // El servidor corre indefinidamente, abortamos
    server_handle.abort();
    */

    // Limpiar archivos de prueba
    fs::remove_file(test_path).await.ok();
    fs::remove_file(copy_path).await.ok();

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
    async fn test_write_and_read() {
        let path = "/tmp/test_rw.txt";
        let content = "Test contenido";

        write_file(path, content).await.unwrap();
        let read_content = read_file(path).await.unwrap();

        assert_eq!(read_content, content);

        fs::remove_file(path).await.ok();
    }

    #[tokio::test]
    async fn test_copy_file() {
        let source = "/tmp/test_source.txt";
        let destination = "/tmp/test_destination.txt";
        let content = "Contenido para copiar";

        write_file(source, content).await.unwrap();
        let bytes = copy_file(source, destination).await.unwrap();

        assert_eq!(bytes, content.len() as u64);

        let read_content = read_file(destination).await.unwrap();
        assert_eq!(read_content, content);

        fs::remove_file(source).await.ok();
        fs::remove_file(destination).await.ok();
    }

    #[tokio::test]
    async fn test_count_lines() {
        let path = "/tmp/test_lines.txt";
        write_file(path, "L1\nL2\nL3\nL4\nL5").await.unwrap();

        let count = count_lines(path).await.unwrap();
        assert_eq!(count, 5);

        fs::remove_file(path).await.ok();
    }

    #[tokio::test]
    async fn test_filter_lines() {
        let path = "/tmp/test_filter.txt";
        write_file(path, "Rust rocks\nPython ok\nRust rules\nGo fast")
            .await
            .unwrap();

        let rust_lines = filter_lines(path, "Rust").await.unwrap();
        assert_eq!(rust_lines.len(), 2);
        assert!(rust_lines[0].contains("Rust"));
        assert!(rust_lines[1].contains("Rust"));

        fs::remove_file(path).await.ok();
    }

    #[tokio::test]
    async fn test_server_client_echo() {
        let addr = "127.0.0.1:18080"; // Puerto diferente para test

        // Iniciar servidor
        let server_handle = tokio::spawn(async move {
            echo_server(addr).await.ok();
        });

        // Dar tiempo al servidor
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Probar cliente
        let response = echo_client(addr, "Test mensaje").await.unwrap();
        assert_eq!(response, "Test mensaje");

        // Limpiar
        server_handle.abort();
    }
}
