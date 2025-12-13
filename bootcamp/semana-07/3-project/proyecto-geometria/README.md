# Proyecto: Biblioteca de Geometría Modular

## 🎯 Objetivo

Crear una biblioteca de geometría bien estructurada usando módulos, visibilidad apropiada y organización de archivos profesional.

## 📁 Estructura del Proyecto

```
proyecto-geometria/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Raíz de la biblioteca
│   ├── formas.rs           # Declara submódulos de formas
│   ├── formas/
│   │   ├── circulo.rs
│   │   ├── rectangulo.rs
│   │   └── triangulo.rs
│   ├── calculos.rs         # Declara submódulos de cálculos
│   ├── calculos/
│   │   ├── area.rs
│   │   └── perimetro.rs
│   ├── formato.rs          # Formateo y display
│   └── main.rs             # Demo de la biblioteca
└── tests/
    └── integration_test.rs
```

## 📋 Requisitos

### 1. Módulo `formas`

Cada forma debe implementar:

```rust
pub trait Forma {
    fn nombre(&self) -> &str;
    fn es_valida(&self) -> bool;
}

pub struct Circulo {
    pub radio: f64,
}

pub struct Rectangulo {
    pub ancho: f64,
    pub alto: f64,
}

pub struct Triangulo {
    pub lado_a: f64,
    pub lado_b: f64,
    pub lado_c: f64,
}
```

### 2. Módulo `calculos`

Funciones de cálculo separadas:

```rust
// calculos/area.rs
pub fn area_circulo(circulo: &Circulo) -> f64;
pub fn area_rectangulo(rect: &Rectangulo) -> f64;
pub fn area_triangulo(tri: &Triangulo) -> f64;

// calculos/perimetro.rs
pub fn perimetro_circulo(circulo: &Circulo) -> f64;
pub fn perimetro_rectangulo(rect: &Rectangulo) -> f64;
pub fn perimetro_triangulo(tri: &Triangulo) -> f64;
```

### 3. Módulo `formato`

Formateo de salida:

```rust
pub fn formatear_resultado(nombre: &str, area: f64, perimetro: f64) -> String;
pub fn formatear_tabla(formas: &[(&str, f64, f64)]) -> String;
```

### 4. Visibilidad

- `Forma` trait: público
- Structs de formas: públicos con campos públicos
- Funciones de cálculo: públicas
- Constantes internas (PI): `pub(crate)`
- Helpers de validación: privados

### 5. Re-exports en `lib.rs`

```rust
// lib.rs - API pública limpia
pub mod formas;
pub mod calculos;
pub mod formato;

// Re-exports para uso simplificado
pub use formas::{Circulo, Rectangulo, Triangulo, Forma};
pub use calculos::{area, perimetro};
```

## ✅ Tests Requeridos

### Tests Unitarios (en cada módulo)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circulo_area() {
        let c = Circulo { radio: 1.0 };
        let area = area::area_circulo(&c);
        assert!((area - std::f64::consts::PI).abs() < 0.0001);
    }
}
```

### Tests de Integración

```rust
// tests/integration_test.rs
use proyecto_geometria::*;

#[test]
fn test_biblioteca_completa() {
    let circulo = Circulo { radio: 5.0 };
    let rect = Rectangulo { ancho: 4.0, alto: 3.0 };
    
    assert!(circulo.es_valida());
    assert!(rect.es_valida());
    
    let area_c = calculos::area::area_circulo(&circulo);
    let area_r = calculos::area::area_rectangulo(&rect);
    
    assert!(area_c > 0.0);
    assert_eq!(area_r, 12.0);
}
```

## 🎯 Criterios de Evaluación

| Criterio | Puntos |
|----------|--------|
| Estructura de archivos correcta | 25% |
| Visibilidad apropiada | 20% |
| Trait Forma implementado | 20% |
| Cálculos correctos | 20% |
| Tests completos | 15% |

## 💡 Pistas

1. Usa `std::f64::consts::PI` para cálculos con π
2. Para el área del triángulo, usa la fórmula de Herón
3. Valida que las dimensiones sean positivas en `es_valida()`
4. Un triángulo es válido si cumple la desigualdad triangular

## 📝 Entrega

1. El proyecto debe compilar sin warnings
2. `cargo test` debe pasar todos los tests
3. `cargo clippy` sin errores
4. `cargo doc` debe generar documentación válida
