# 🔧 Proyecto: Validador de Configuración

## 📋 Descripción

Sistema robusto para cargar, validar y procesar archivos de configuración con manejo completo de errores. El sistema reporta **todos** los errores encontrados, no solo el primero.

## 🎯 Objetivos de Aprendizaje

- Crear tipos de error expresivos con contexto
- Acumular múltiples errores en lugar de fallar en el primero
- Proporcionar valores por defecto cuando sea apropiado
- Validar tipos y rangos de valores
- Implementar Display para errores legibles

## 📁 Formato del Archivo

```ini
# Comentario
nombre = MiApp
version = 1.0.0
puerto = 8080
host = localhost
max_conexiones = 100
timeout_ms = 5000
debug = true
```

## 🏗️ Estructura

```
Config
├── nombre: String (requerido)
├── version: String (default: "0.0.0")
├── puerto: u16 (requerido)
├── host: String (default: "localhost")
├── max_conexiones: u32 (default: 100, rango: 1-10000)
├── timeout_ms: u64 (default: 3000)
└── debug: bool (default: false)
```

## 🔴 Tipos de Error

| Error | Descripción |
|-------|-------------|
| `IoError` | Error al leer el archivo |
| `FormatoInvalido` | Línea sin formato `clave=valor` |
| `CampoFaltante` | Campo requerido no presente |
| `ValorVacio` | Campo presente pero sin valor |
| `TipoIncorrecto` | Valor no es del tipo esperado |
| `FueraDeRango` | Valor fuera del rango permitido |

## ▶️ Ejecución

```bash
# Ejecutar demo
cargo run

# Ejecutar tests
cargo test

# Ver output detallado
cargo test -- --nocapture
```

## 📊 Ejemplo de Salida

### Configuración válida:
```
✓ Configuración cargada exitosamente:

  Nombre:          MiApp
  Versión:         1.0.0
  Host:            localhost
  Puerto:          8080
  Max conexiones:  100
  Timeout:         5000ms
  Debug:           true
```

### Configuración con errores:
```
✗ Se encontraron 3 errores:
  • Valor vacío para campo 'nombre'
  • Campo 'puerto': esperado número entero 0-65535, encontrado 'abc'
  • Campo 'max_conexiones': valor -50 fuera de rango (1-10000)
```

## ✅ Criterios de Evaluación

| Criterio | Peso | Descripción |
|----------|------|-------------|
| Tipos de error | 25% | Enum con variantes apropiadas |
| Display | 20% | Mensajes claros y útiles |
| Validación | 25% | Campos requeridos, tipos, rangos |
| Acumulación | 15% | Reporta todos los errores |
| Tests | 15% | Cobertura de casos edge |

## 💡 Extensiones Opcionales

1. **Soporte para includes**: `include = otro.config`
2. **Variables de entorno**: `puerto = ${PORT:8080}`
3. **Secciones**: `[database]`, `[server]`
4. **Serialización**: Guardar config modificada

## 📚 Conceptos Aplicados

- `enum` con datos asociados
- `impl Display` y `impl Error`
- `HashMap` para almacenamiento
- Pattern matching exhaustivo
- Validación de datos
