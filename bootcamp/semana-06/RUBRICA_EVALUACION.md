# 📊 Rúbrica de Evaluación - Semana 06

## Manejo de Errores

### Competencias a Evaluar

| Competencia | Peso | Descripción |
|-------------|------|-------------|
| Uso de Result | 25% | Retornar y manejar Result correctamente |
| Propagación con ? | 25% | Usar el operador ? idiomáticamente |
| Errores personalizados | 20% | Crear tipos de error apropiados |
| Conversión de errores | 15% | Implementar From/Into |
| Proyecto integrador | 15% | Aplicar todos los conceptos |

---

## Niveles de Desempeño

### 1. Uso de Result (25%)

#### Excelente (90-100%)
```rust
fn dividir(a: f64, b: f64) -> Result<f64, DivisionError> {
    if b == 0.0 {
        Err(DivisionError::DivisionPorCero)
    } else if b.is_nan() || a.is_nan() {
        Err(DivisionError::ValorInvalido("NaN no permitido".into()))
    } else {
        Ok(a / b)
    }
}
```
- Result con tipo de error apropiado
- Maneja múltiples casos de error
- Mensajes descriptivos

#### Satisfactorio (70-89%)
```rust
fn dividir(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("División por cero".to_string())
    } else {
        Ok(a / b)
    }
}
```
- Result funcional
- Error como String (menos ideal)

#### En Desarrollo (50-69%)
- Usa unwrap() frecuentemente
- No maneja todos los casos de error

---

### 2. Propagación con ? (25%)

#### Excelente (90-100%)
```rust
fn procesar_archivo(ruta: &str) -> Result<Config, AppError> {
    let contenido = std::fs::read_to_string(ruta)?;
    let datos: Datos = serde_json::from_str(&contenido)?;
    let config = validar(datos)?;
    Ok(config)
}
```
- Cadena fluida de operaciones
- Tipos de error compatibles
- Código limpio y legible

#### Satisfactorio (70-89%)
```rust
fn procesar_archivo(ruta: &str) -> Result<Config, Box<dyn Error>> {
    let contenido = std::fs::read_to_string(ruta)?;
    // ... resto
    Ok(config)
}
```
- Usa Box<dyn Error> (funcional pero menos específico)

#### En Desarrollo (50-69%)
- Match anidados en lugar de ?
- Propagación manual verbose

---

### 3. Errores Personalizados (20%)

#### Excelente (90-100%)
```rust
#[derive(Debug)]
enum ConfigError {
    ArchivoNoEncontrado(PathBuf),
    ParseError { linea: usize, mensaje: String },
    ValidacionFallida(Vec<String>),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ArchivoNoEncontrado(p) => 
                write!(f, "Archivo no encontrado: {}", p.display()),
            Self::ParseError { linea, mensaje } => 
                write!(f, "Error en línea {}: {}", linea, mensaje),
            Self::ValidacionFallida(errores) => 
                write!(f, "Validación fallida: {}", errores.join(", ")),
        }
    }
}

impl std::error::Error for ConfigError {}
```
- Enum con variantes descriptivas
- Implementa Display y Error
- Datos contextuales en variantes

#### Satisfactorio (70-89%)
```rust
#[derive(Debug)]
struct MiError(String);

impl std::fmt::Display for MiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
```
- Tipo de error básico funcional
- Falta contexto estructurado

---

### 4. Conversión de Errores (15%)

#### Excelente (90-100%)
```rust
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Parse(err.to_string())
    }
}
```
- Implementaciones From para cada fuente
- Preserva información del error original
- Permite uso fluido de ?

#### Satisfactorio (70-89%)
- Usa map_err manualmente
- Conversiones funcionan pero son verbose

---

### 5. Proyecto Integrador (15%)

#### Excelente (90-100%)
- Validador completo funcionando
- Errores descriptivos y útiles
- Todos los tests pasan
- Código bien estructurado

#### Satisfactorio (70-89%)
- Funcionalidad básica completa
- Algunos casos edge no manejados
- Tests principales pasan

---

## Ejercicios de Evaluación

### Ejercicio 1: Result Básico
Implementa una función que parsee coordenadas "x,y":

```rust
fn parsear_coordenadas(s: &str) -> Result<(i32, i32), ParseError> {
    // Debe manejar:
    // - Formato inválido (no tiene coma)
    // - Números inválidos
    // - Espacios extras
}
```

### Ejercicio 2: Propagación
Completa la función que lee y procesa múltiples archivos:

```rust
fn procesar_directorio(dir: &Path) -> Result<Vec<Resultado>, Error> {
    // Usar ? para propagar errores
    // Acumular resultados exitosos
}
```

### Ejercicio 3: Error Custom
Crea un tipo de error para un sistema de autenticación:

```rust
enum AuthError {
    // - Usuario no encontrado
    // - Contraseña incorrecta
    // - Sesión expirada
    // - Token inválido
}
// Implementar Display y Error
```

---

## Distribución de Notas

| Componente | Peso |
|------------|------|
| Conocimiento (teoría) | 30% |
| Desempeño (prácticas) | 40% |
| Producto (proyecto) | 30% |

## Anti-patrones a Evitar

❌ **No hacer:**
```rust
// Usar unwrap en producción
let valor = resultado.unwrap();

// Ignorar errores
let _ = operacion_puede_fallar();

// panic! para errores recuperables
panic!("Archivo no encontrado");

// Strings como único tipo de error
Result<T, String>
```

✅ **Hacer:**
```rust
// Manejar o propagar
let valor = resultado?;

// Registrar o manejar errores
if let Err(e) = operacion() {
    log::error!("Falló: {}", e);
}

// Result para errores recuperables
fn leer() -> Result<Data, IoError>

// Tipos de error específicos
Result<T, MiErrorEspecifico>
```
