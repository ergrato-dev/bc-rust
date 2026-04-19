# 📊 Rúbrica de Evaluación - Semana 05

## Enums y Pattern Matching

---

## 🎯 Competencias a Evaluar

| Competencia | Peso |
|-------------|------|
| Definición de enums | 20% |
| Pattern matching con match | 25% |
| Uso de Option | 25% |
| if let / while let | 15% |
| Proyecto: Máquina de Estados | 15% |

---

## 📝 Criterios de Evaluación

### 1. Conocimiento (30%)

#### Excelente (90-100%)
- Define enums con variantes simples y con datos
- Explica exhaustividad del match
- Conoce Option, Some, None y sus métodos
- Entiende cuándo usar if let vs match

#### Satisfactorio (70-89%)
- Crea enums básicos correctamente
- Usa match cubriendo todos los casos
- Trabaja con Option usando match
- Conoce if let para casos simples

#### En Desarrollo (50-69%)
- Define enums simples
- Usa match con ayuda del compilador
- Confunde Option con null de otros lenguajes
- Prefiere unwrap sobre pattern matching

#### Insuficiente (<50%)
- No entiende el concepto de enum
- No puede escribir expresiones match
- No maneja Option correctamente

---

### 2. Desempeño (40%)

#### Excelente (90-100%)
```rust
// Código idiomático con enums
enum Estado {
    Activo { desde: String },
    Pausado,
    Finalizado { resultado: Result<i32, String> },
}

fn procesar(estado: &Estado) -> String {
    match estado {
        Estado::Activo { desde } => format!("Activo desde {}", desde),
        Estado::Pausado => "En pausa".to_string(),
        Estado::Finalizado { resultado: Ok(v) } => format!("Éxito: {}", v),
        Estado::Finalizado { resultado: Err(e) } => format!("Error: {}", e),
    }
}
```

#### Satisfactorio (70-89%)
- Usa match correctamente
- Maneja Option sin unwrap
- Aplica if let cuando es apropiado

#### En Desarrollo (50-69%)
- Depende del compilador para exhaustividad
- Usa unwrap frecuentemente
- No aprovecha destructuring

#### Insuficiente (<50%)
- Código no compila
- No puede manejar enums

---

### 3. Producto (30%)

#### Proyecto: Máquina de Estados

##### Excelente (90-100%)
```rust
enum EstadoPedido {
    Creado { id: u32 },
    Pagado { id: u32, monto: f64 },
    Enviado { id: u32, tracking: String },
    Entregado { id: u32 },
    Cancelado { id: u32, razon: String },
}

impl EstadoPedido {
    fn transicion(self, evento: Evento) -> Result<Self, Error> {
        match (self, evento) {
            (EstadoPedido::Creado { id }, Evento::Pagar(monto)) => 
                Ok(EstadoPedido::Pagado { id, monto }),
            // ... otras transiciones válidas
            _ => Err(Error::TransicionInvalida),
        }
    }
}
```

##### Satisfactorio (70-89%)
- Estados modelados como enum
- Transiciones implementadas
- Match exhaustivo

##### En Desarrollo (50-69%)
- Enum básico sin datos
- Transiciones parciales
- Falta manejo de errores

---

## 🔍 Ejercicios de Evaluación

### Ejercicio 1: Definir Enum (10 pts)

Define un enum para representar una figura geométrica:
- Círculo con radio
- Rectángulo con ancho y alto
- Triángulo con base y altura

### Ejercicio 2: Match Exhaustivo (15 pts)

Implementa una función que calcule el área de cualquier figura:

```rust
fn area(figura: &Figura) -> f64 {
    // TODO
}
```

### Ejercicio 3: Option (20 pts)

Implementa una función que busque un elemento y retorne su índice:

```rust
fn buscar<T: PartialEq>(lista: &[T], elemento: &T) -> Option<usize> {
    // TODO
}
```

---

## 📈 Escala de Calificación

| Rango | Calificación | Descripción |
|-------|--------------|-------------|
| 90-100 | A | Excelente uso de enums y patrones |
| 80-89 | B | Buen manejo, detalles menores |
| 70-79 | C | Competente, necesita práctica |
| 60-69 | D | En desarrollo |
| <60 | F | No alcanza competencias mínimas |

---

## 🎯 Objetivos para Siguiente Semana

Si dominas esta semana, estarás listo para:
- Semana 06: Error Handling con Result
- Combinar Option y Result
- El operador ? para propagación de errores
