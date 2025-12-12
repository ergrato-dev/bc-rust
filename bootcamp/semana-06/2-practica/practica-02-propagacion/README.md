# Práctica 02: Propagación de Errores

## 🎯 Objetivo

Dominar el operador `?` para propagar errores de forma limpia y legible.

## 📋 Ejercicios

### Ejercicio 1: Leer y Sumar

Lee un archivo con números (uno por línea) y retorna su suma.

```rust
fn sumar_lineas_archivo(ruta: &str) -> Result<i64, String>
```

Practica:
- Leer archivo con `fs::read_to_string`
- Convertir errores con `map_err`
- Propagar con `?`

### Ejercicio 2: Validación Encadenada

Valida nombre y edad, propagando el primer error:

```rust
fn validar_usuario(nombre: &str, edad: i32) -> Result<Usuario, String>
```

### Ejercicio 3: Procesar Configuración

Extrae el puerto de un archivo de configuración:

```rust
fn obtener_puerto(ruta: &str) -> Result<u16, String>
```

## ▶️ Ejecución

```bash
cargo run
cargo test
```

## 💡 Pistas

```rust
// El operador ? hace esto automáticamente:
let contenido = match fs::read_to_string(ruta) {
    Ok(c) => c,
    Err(e) => return Err(e.into()),
};

// Se simplifica a:
let contenido = fs::read_to_string(ruta)?;
```

## ✅ Criterios de Éxito

- [ ] Todas las funciones usan `?` correctamente
- [ ] Tests pasan
- [ ] Mensajes de error son descriptivos
