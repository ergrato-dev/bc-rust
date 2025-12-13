# 📝 Práctica 02: CRUD de Usuarios

## 📋 Objetivo

Implementar una API REST completa con operaciones CRUD (Create, Read, Update, Delete) usando estado en memoria.

## 🎯 Requisitos

| Método | Ruta | Descripción |
|--------|------|-------------|
| GET | `/usuarios` | Listar todos los usuarios |
| POST | `/usuarios` | Crear nuevo usuario |
| GET | `/usuarios/:id` | Obtener usuario por ID |
| PUT | `/usuarios/:id` | Actualizar usuario |
| DELETE | `/usuarios/:id` | Eliminar usuario |

## ▶️ Ejecutar

```bash
docker compose run --rm -p 3000:3000 rust-dev cargo run -p practica-02-crud
```

## 🧪 Probar con curl

```bash
# Listar usuarios (vacío inicialmente)
curl http://localhost:3000/usuarios

# Crear usuario
curl -X POST http://localhost:3000/usuarios \
  -H "Content-Type: application/json" \
  -d '{"nombre": "Ana García", "email": "ana@ejemplo.com"}'

# Crear otro usuario
curl -X POST http://localhost:3000/usuarios \
  -H "Content-Type: application/json" \
  -d '{"nombre": "Carlos López", "email": "carlos@ejemplo.com"}'

# Listar usuarios (ahora con datos)
curl http://localhost:3000/usuarios

# Obtener usuario específico
curl http://localhost:3000/usuarios/1

# Actualizar usuario
curl -X PUT http://localhost:3000/usuarios/1 \
  -H "Content-Type: application/json" \
  -d '{"nombre": "Ana García Ruiz", "activo": false}'

# Eliminar usuario
curl -X DELETE http://localhost:3000/usuarios/2

# Verificar eliminación (debe retornar 404)
curl http://localhost:3000/usuarios/2
```

## 📝 Tests

```bash
docker compose run --rm rust-dev cargo test -p practica-02-crud
```

## 💡 Conceptos Clave

- **State<T>** - Estado compartido entre handlers
- **Arc<RwLock<T>>** - Datos mutables thread-safe
- **Json<T>** - Extractor y respuesta JSON
- **StatusCode** - Códigos HTTP (200, 201, 404, etc.)
- **Result<T, E>** - Manejo de errores en handlers

## ✅ Criterios de Evaluación

- [ ] Todas las operaciones CRUD funcionan
- [ ] Estado persiste entre requests
- [ ] Códigos HTTP correctos (201 al crear, 204 al eliminar)
- [ ] 404 cuando no encuentra usuario
- [ ] Tests pasan
