# 📋 Rúbrica de Evaluación - Semana 08

## Colecciones

### Competencias a Evaluar

| Competencia | Peso | Descripción |
|-------------|------|-------------|
| Vectores | 25% | Uso de Vec<T> |
| Strings | 25% | Manipulación de String y &str |
| HashMaps | 25% | Uso de HashMap<K, V> |
| Iteradores | 15% | Iteración idiomática |
| Proyecto | 10% | Sistema de inventario |

---

## 1. Vectores (25%)

### Nivel Avanzado (90-100%)
- Usa Vec con ownership correcto
- Entiende la diferencia entre `get()` y `[]`
- Itera con referencias sin mover valores
- Maneja capacidad y realocaciones

### Nivel Intermedio (70-89%)
- Crea y manipula Vec sin errores
- Usa push, pop, get correctamente
- Itera con `for` y `iter()`

### Nivel Básico (50-69%)
- Crea Vec básicos
- Confusión ocasional con ownership
- Necesita ayuda con iteración

### Nivel Insuficiente (<50%)
- No puede crear o manipular Vec
- Errores frecuentes de borrow checker

---

## 2. Strings (25%)

### Nivel Avanzado (90-100%)
- Entiende String vs &str profundamente
- Manipula UTF-8 correctamente
- Usa slices y métodos de String fluídamente

### Nivel Intermedio (70-89%)
- Convierte entre String y &str
- Usa concatenación y métodos básicos
- Entiende que String es growable

### Nivel Básico (50-69%)
- Crea Strings básicos
- Confusión entre String y &str
- Dificultad con slicing

### Nivel Insuficiente (<50%)
- No distingue String de &str
- Errores constantes de tipos

---

## 3. HashMaps (25%)

### Nivel Avanzado (90-100%)
- Usa entry API eficientemente
- Entiende ownership en claves y valores
- Implementa patrones comunes (contadores, agrupación)

### Nivel Intermedio (70-89%)
- Inserta y consulta valores sin errores
- Usa `get()` con Option correctamente
- Itera sobre claves y valores

### Nivel Básico (50-69%)
- Crea y usa HashMaps simples
- Dificultad con el manejo de Option

### Nivel Insuficiente (<50%)
- No puede usar HashMaps básicos

---

## 4. Iteradores (15%)

### Nivel Avanzado (90-100%)
- Usa map, filter, fold con fluidez
- Entiende lazy evaluation
- Combina múltiples adaptadores

### Nivel Intermedio (70-89%)
- Usa iter(), into_iter(), iter_mut()
- Aplica map y filter básicos
- Colecta resultados correctamente

### Nivel Básico (50-69%)
- Itera con `for` correctamente
- Dificultad con adaptadores

### Nivel Insuficiente (<50%)
- Solo usa índices para iterar
- No entiende el patrón Iterator

---

## 5. Proyecto: Sistema de Inventario (10%)

### Nivel Avanzado (90-100%)
- CRUD completo de productos
- Búsquedas eficientes con HashMap
- Reportes usando iteradores
- Código bien organizado

### Nivel Intermedio (70-89%)
- Funcionalidad básica completa
- Uso correcto de colecciones
- Manejo de errores básico

### Nivel Básico (50-69%)
- Funcionalidad parcial
- Algunos errores de lógica

### Nivel Insuficiente (<50%)
- Proyecto incompleto o no funcional

---

## 📊 Escala de Calificación

| Porcentaje | Calificación | Descripción |
|------------|--------------|-------------|
| 90-100% | A | Excelente dominio |
| 80-89% | B | Buen desempeño |
| 70-79% | C | Competente |
| 60-69% | D | Necesita práctica |
| <60% | F | Requiere refuerzo |

---

## 🎯 Indicadores de Logro

### Conocimiento (30%)
- [ ] Explica cuándo usar Vec vs array
- [ ] Distingue String de &str
- [ ] Describe casos de uso de HashMap
- [ ] Conoce métodos principales de cada colección

### Desempeño (40%)
- [ ] Resuelve errores de ownership en colecciones
- [ ] Itera sin consumir la colección cuando es necesario
- [ ] Usa la entry API de HashMap
- [ ] Aplica iteradores funcionales

### Producto (30%)
- [ ] Código compila sin warnings
- [ ] Tests pasan exitosamente
- [ ] Uso idiomático de colecciones
- [ ] Proyecto funcional y organizado

---

## 📝 Errores Comunes a Evitar

```rust
// ❌ Mal: usar índices cuando se puede iterar
for i in 0..vec.len() {
    println!("{}", vec[i]);
}

// ✅ Bien: iterar directamente
for elemento in &vec {
    println!("{}", elemento);
}
```

```rust
// ❌ Mal: crear String innecesariamente
fn procesar(s: String) { }
procesar(texto.to_string());  // Copia innecesaria

// ✅ Bien: aceptar &str cuando no se necesita ownership
fn procesar(s: &str) { }
procesar(&texto);
```

```rust
// ❌ Mal: múltiples lookups en HashMap
if mapa.contains_key(&clave) {
    let valor = mapa.get(&clave).unwrap();
}

// ✅ Bien: usar entry o get directamente
if let Some(valor) = mapa.get(&clave) {
    // usar valor
}
```

---

**Fecha**: Semana 08 del Bootcamp  
**Tema**: Colecciones  
**Versión**: 1.0
