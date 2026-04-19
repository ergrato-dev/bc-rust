# Práctica 02: Pattern Matching

## 🎯 Objetivo

Dominar la expresión `match` y sus patrones avanzados.

## 📋 Instrucciones

### Ejercicio 1: Match Básico

Crea un enum `Moneda` (Peso, Dolar, Euro, Libra) e implementa una función `a_pesos` que convierta cualquier moneda a pesos.

### Ejercicio 2: Enums con Datos

Crea un enum `Figura` con variantes que contengan datos:
- `Circulo(f64)` - radio
- `Rectangulo { ancho: f64, alto: f64 }`
- `Triangulo(f64, f64)` - base, altura

Implementa `calcular_area`.

### Ejercicio 3: Patrones Avanzados

Implementa `clasificar_numero` que use guards y rangos:
- 0: "cero"
- 1-10: "pequeño"
- 11-100: "mediano"
- > 100: "grande"
- negativos: "negativo"

## 🧪 Tests

```bash
cargo test
```

## ✅ Criterios de Éxito

- [ ] Match exhaustivo en todos los casos
- [ ] Extracción correcta de datos
- [ ] Guards funcionando
- [ ] Todos los tests pasan
