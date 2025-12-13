# Práctica 03: I/O Asíncrono

## 🎯 Objetivo

Dominar las operaciones de I/O asíncrono con Tokio: archivos, TCP y streams.

## 📋 Ejercicios

### Ejercicio 1: Operaciones de Archivos

Implementa funciones para manipular archivos de forma asíncrona:

```rust
/// Lee un archivo y retorna su contenido.
async fn leer_archivo(path: &str) -> io::Result<String> {
    todo!()
}

/// Escribe contenido a un archivo.
async fn escribir_archivo(path: &str, contenido: &str) -> io::Result<()> {
    todo!()
}

/// Copia un archivo de origen a destino.
async fn copiar_archivo(origen: &str, destino: &str) -> io::Result<u64> {
    todo!()
}
```

### Ejercicio 2: Lectura por Líneas

Implementa un contador de líneas asíncrono:

```rust
/// Cuenta las líneas de un archivo.
async fn contar_lineas(path: &str) -> io::Result<usize> {
    // Usar BufReader y lines()
    todo!()
}

/// Lee líneas que contienen un patrón.
async fn filtrar_lineas(path: &str, patron: &str) -> io::Result<Vec<String>> {
    todo!()
}
```

### Ejercicio 3: Servidor Echo TCP

Implementa un servidor echo simple:

```rust
/// Servidor que devuelve lo que recibe.
async fn servidor_echo(addr: &str) -> io::Result<()> {
    // 1. Bind al address
    // 2. Loop de accept
    // 3. Spawn task por conexión
    // 4. Echo: leer y escribir de vuelta
    todo!()
}
```

### Ejercicio 4: Cliente TCP

Implementa un cliente que se conecta al servidor:

```rust
/// Cliente que envía un mensaje y espera respuesta.
async fn cliente_echo(addr: &str, mensaje: &str) -> io::Result<String> {
    // 1. Conectar
    // 2. Enviar mensaje
    // 3. Leer respuesta
    // 4. Retornar respuesta
    todo!()
}
```

## 🧪 Tests

```bash
cargo test
```

## ✅ Criterios de Evaluación

| Criterio | Puntos |
|----------|--------|
| Operaciones de archivo funcionan | 25 |
| Lectura por líneas correcta | 25 |
| Servidor echo maneja conexiones | 25 |
| Cliente se comunica correctamente | 25 |

## 💡 Pistas

- Usa `tokio::fs` para archivos
- `BufReader::new().lines()` para leer líneas
- `TcpListener::bind()` y `.accept()` para el servidor
- `TcpStream::connect()` para el cliente
