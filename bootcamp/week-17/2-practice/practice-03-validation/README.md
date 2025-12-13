# ✅ Práctica 03: Validación de Requests

## 📋 Objetivo

Implementar validación robusta de datos de entrada con mensajes de error descriptivos.

## 🎯 Requisitos

1. Validar campos requeridos
2. Validar formatos (email, longitudes)
3. Validar reglas de negocio (edad mínima, password fuerte)
4. Retornar errores estructurados en JSON
5. Usar códigos HTTP apropiados

## ▶️ Ejecutar

```bash
docker compose run --rm -p 3000:3000 rust-dev cargo run -p practica-03-validacion
```

## 🧪 Probar con curl

### Registro válido
```bash
curl -X POST http://localhost:3000/registro \
  -H "Content-Type: application/json" \
  -d '{
    "nombre": "Ana García",
    "email": "ana@ejemplo.com",
    "edad": 25,
    "password": "MiPassword123"
  }'
```

### Nombre vacío (error)
```bash
curl -X POST http://localhost:3000/registro \
  -H "Content-Type: application/json" \
  -d '{
    "nombre": "",
    "email": "ana@ejemplo.com",
    "edad": 25,
    "password": "MiPassword123"
  }'
```

### Email inválido (error)
```bash
curl -X POST http://localhost:3000/registro \
  -H "Content-Type: application/json" \
  -d '{
    "nombre": "Ana",
    "email": "email-sin-arroba",
    "edad": 25,
    "password": "MiPassword123"
  }'
```

### Menor de edad (error)
```bash
curl -X POST http://localhost:3000/registro \
  -H "Content-Type: application/json" \
  -d '{
    "nombre": "Ana",
    "email": "ana@ejemplo.com",
    "edad": 15,
    "password": "MiPassword123"
  }'
```

### Password débil (múltiples errores)
```bash
curl -X POST http://localhost:3000/registro \
  -H "Content-Type: application/json" \
  -d '{
    "nombre": "Ana",
    "email": "ana@ejemplo.com",
    "edad": 25,
    "password": "123"
  }'
```

### JSON malformado (error)
```bash
curl -X POST http://localhost:3000/registro \
  -H "Content-Type: application/json" \
  -d 'esto no es json'
```

## 📝 Tests

```bash
docker compose run --rm rust-dev cargo test -p practica-03-validacion
```

## 💡 Conceptos Clave

- **JsonRejection** - Capturar errores de parsing JSON
- **IntoResponse** - Implementar respuestas personalizadas
- **ErrorResponse** - Estructura de error estándar
- **Validación en capas** - Primero JSON, luego reglas de negocio

## 📊 Formato de Error

```json
{
  "error": "Error de validación",
  "codigo": 400,
  "detalles": [
    "El nombre es requerido",
    "El email no tiene un formato válido"
  ]
}
```

## ✅ Criterios de Evaluación

- [ ] Validación de campos requeridos
- [ ] Validación de formato de email
- [ ] Validación de edad mínima
- [ ] Validación de password (longitud, mayúscula, número)
- [ ] Errores JSON estructurados
- [ ] Tests pasan
