# 🧮 Proyecto Semanal: Calculadora de Tipos

## 📋 Descripción

Desarrollarás una calculadora interactiva que demuestra el uso de variables, tipos de datos y operaciones en Rust. El proyecto integra todos los conceptos de la Semana 02.

## 🎯 Objetivos de Aprendizaje

- Aplicar variables inmutables y mutables
- Usar diferentes tipos numéricos
- Implementar operaciones aritméticas
- Practicar shadowing y constantes
- Manejar conversiones de tipos

## 📁 Estructura del Proyecto

```
proyecto-calculadora-tipos/
├── Cargo.toml
├── src/
│   └── main.rs
└── README.md
```

## 🔧 Requisitos Funcionales

### Nivel 1: Básico (Obligatorio)

1. **Calculadora de área y perímetro**
   - Calcular área y perímetro de un rectángulo
   - Calcular área y circunferencia de un círculo
   - Usar constantes para PI

2. **Conversor de temperatura**
   - Celsius a Fahrenheit: `F = C * 9/5 + 32`
   - Fahrenheit a Celsius: `C = (F - 32) * 5/9`

3. **Estadísticas básicas**
   - Calcular suma, promedio, mínimo y máximo de 5 números

### Nivel 2: Intermedio (Recomendado)

4. **Explorador de tipos**
   - Mostrar el tamaño en bytes de cada tipo primitivo
   - Mostrar los valores mínimos y máximos de tipos enteros

5. **Calculadora de edad**
   - Calcular edad en días, horas y minutos
   - Usar tipos apropiados para cada cálculo

### Nivel 3: Avanzado (Opcional)

6. **Conversiones numéricas**
   - Decimal a binario (representación como String)
   - Demostrar overflow controlado

## ✅ Criterios de Evaluación

| Criterio | Peso | Descripción |
|----------|------|-------------|
| **Compilación** | 20% | Compila sin errores ni warnings |
| **Funcionalidad** | 30% | Todas las funciones operan correctamente |
| **Tipos correctos** | 25% | Uso apropiado de tipos para cada caso |
| **Código limpio** | 15% | Bien formateado, comentado |
| **Tests** | 10% | Tests unitarios para funciones clave |

## 🧪 Ejecutar

```bash
# Compilar y ejecutar
cargo run

# Ejecutar tests
cargo test

# Verificar con clippy
cargo clippy

# Formatear código
cargo fmt
```

## 💡 Pistas

### Constantes útiles

```rust
const PI: f64 = 3.14159265358979;
const DIAS_POR_ANIO: u32 = 365;
const HORAS_POR_DIA: u32 = 24;
```

### Obtener tamaño de tipos

```rust
use std::mem::size_of;
println!("i32: {} bytes", size_of::<i32>());
```

### Valores mínimos y máximos

```rust
println!("i8 max: {}", i8::MAX);
println!("u8 min: {}", u8::MIN);
```

## 📅 Entrega

- **Tiempo estimado**: 60-90 minutos
- **Formato**: Proyecto Cargo completo
- **Incluir**: Código funcionando + tests

---

¡Buena suerte! 🦀
