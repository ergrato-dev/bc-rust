# 🎨 Proyecto: Sistema de Formas Geométricas

## Descripción

Sistema polimórfico para trabajar con formas geométricas usando traits.

## Objetivos

- Definir traits para comportamiento geométrico
- Implementar traits para múltiples formas
- Usar traits derivables apropiadamente
- Implementar Display y Default
- Crear funciones genéricas con trait bounds

## Estructura del Proyecto

```
proyecto-formas/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── traits.rs      # Definición de traits
│   ├── formas.rs      # Implementación de formas
│   └── canvas.rs      # Canvas para dibujar
└── README.md
```

## Traits Implementados

- `Forma`: área, perímetro, nombre
- `Dibujable`: dibujar en ASCII
- `Transformable`: escalar, trasladar
- `Display`: formato legible
- `Default`: valores por defecto

## Formas Implementadas

- Círculo
- Rectángulo
- Triángulo
- Cuadrado

## Ejecutar

```bash
cargo run --package proyecto-formas
```

## Tests

```bash
cargo test --package proyecto-formas
```

## Ejemplo de Uso

```rust
use proyecto_formas::*;

fn main() {
    let circulo = Circulo::new(5.0);
    let rectangulo = Rectangulo::new(10.0, 5.0);
    
    println!("Círculo: área = {:.2}", circulo.area());
    println!("Rectángulo: perímetro = {:.2}", rectangulo.perimetro());
    
    // Polimorfismo con trait bounds
    imprimir_info(&circulo);
    imprimir_info(&rectangulo);
}

fn imprimir_info(forma: &impl Forma) {
    println!("{}: área={:.2}, perímetro={:.2}", 
        forma.nombre(), forma.area(), forma.perimetro());
}
```
