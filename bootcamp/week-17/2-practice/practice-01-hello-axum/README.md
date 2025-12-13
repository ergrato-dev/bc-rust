# 🚀 Práctica 01: Hello Axum

## 📋 Objetivo

Crear tu primer servidor HTTP con Axum, definiendo rutas simples y usando parámetros de ruta.

## 🎯 Requisitos

1. **Ruta raíz** (`/`): Retorna un mensaje de bienvenida
2. **Health check** (`/health`): Retorna "OK"
3. **Info** (`/info`): Retorna JSON con información del API
4. **Saludo** (`/saludo/:nombre`): Saludo personalizado

## ▶️ Ejecutar

```bash
# Desde el contenedor Docker
docker compose run --rm -p 3000:3000 rust-dev cargo run -p practica-01-hello-axum
```

## 🧪 Probar

```bash
# Página principal
curl http://localhost:3000/

# Health check
curl http://localhost:3000/health

# Información
curl http://localhost:3000/info

# Saludo personalizado
curl http://localhost:3000/saludo/Rust
curl http://localhost:3000/saludo/Ana
```

## 📝 Tests

```bash
docker compose run --rm rust-dev cargo test -p practica-01-hello-axum
```

## 💡 Conceptos Clave

- `Router::new()` - Crea un nuevo router
- `.route("/path", get(handler))` - Define una ruta GET
- `Path<T>` - Extrae parámetros de la URL
- `tokio::net::TcpListener` - Escucha conexiones TCP
- `axum::serve()` - Inicia el servidor

## ✅ Criterios de Evaluación

- [ ] Servidor arranca sin errores
- [ ] Todas las rutas responden correctamente
- [ ] Parámetro de ruta funciona
- [ ] Tests pasan
