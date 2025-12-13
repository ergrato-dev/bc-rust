# 🔐 Práctica 04: Middleware

## 📋 Objetivo

Implementar middleware de logging, autenticación y request ID.

## 🎯 Requisitos

1. **Logging Middleware** - Registrar cada request con tiempo de respuesta
2. **Auth Middleware** - Verificar token Bearer en rutas protegidas
3. **Request ID** - Añadir ID único a cada request/response
4. **Separación** - Rutas públicas vs protegidas

## ▶️ Ejecutar

```bash
docker compose run --rm -p 3000:3000 rust-dev cargo run -p practica-04-middleware
```

## 🧪 Probar con curl

### Rutas públicas (sin autenticación)
```bash
# Info pública
curl http://localhost:3000/

# Health check
curl http://localhost:3000/health
```

### Rutas protegidas (requieren token)
```bash
# Sin token (401 Unauthorized)
curl http://localhost:3000/privado

# Con token inválido (401)
curl http://localhost:3000/privado \
  -H "Authorization: Bearer token-incorrecto"

# Con token válido (200 OK)
curl http://localhost:3000/privado \
  -H "Authorization: Bearer mi-token-secreto"

# Admin con token
curl http://localhost:3000/admin \
  -H "Authorization: Bearer mi-token-secreto"
```

### Ver headers de respuesta (Request ID)
```bash
curl -v http://localhost:3000/ 2>&1 | grep -i x-request-id
```

## 📝 Tests

```bash
docker compose run --rm rust-dev cargo test -p practica-04-middleware
```

## 💡 Conceptos Clave

- **middleware::from_fn()** - Crear middleware desde función
- **Next** - Continuar la cadena de middleware
- **TraceLayer** - Logging automático de tower-http
- **layer()** - Aplicar middleware al router
- **Orden de capas** - Se ejecutan en orden inverso

## 📊 Flujo de Middleware

```
Request
   │
   ▼
┌──────────────┐
│ TraceLayer   │  ← Logging automático
└──────┬───────┘
       │
┌──────▼───────┐
│ LoggingMW    │  ← Nuestro logging
└──────┬───────┘
       │
┌──────▼───────┐
│ RequestIdMW  │  ← Añade X-Request-Id
└──────┬───────┘
       │
┌──────▼───────┐
│ AuthMW       │  ← Solo en rutas protegidas
└──────┬───────┘
       │
┌──────▼───────┐
│ Handler      │
└──────────────┘
```

## ✅ Criterios de Evaluación

- [ ] Logging muestra método, ruta y tiempo
- [ ] Rutas públicas accesibles sin token
- [ ] Rutas protegidas requieren token
- [ ] Token inválido retorna 401
- [ ] Request ID en headers
- [ ] Tests pasan
