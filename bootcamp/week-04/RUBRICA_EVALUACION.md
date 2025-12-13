# 📊 Rúbrica de Evaluación - Semana 04

## Ownership y Borrowing

---

## 🎯 Competencias a Evaluar

| Competencia | Peso |
|-------------|------|
| Comprensión del ownership | 25% |
| Move vs Copy semantics | 20% |
| Uso correcto de referencias | 25% |
| Resolución de errores del borrow checker | 20% |
| Proyecto: Sistema de Biblioteca | 10% |

---

## 📝 Criterios de Evaluación

### 1. Conocimiento (30%)

#### Excelente (90-100%)
- Explica las 3 reglas del ownership con precisión
- Distingue claramente move de copy con ejemplos
- Conoce las reglas del borrowing y sus implicaciones
- Entiende por qué Rust necesita estas reglas (seguridad de memoria)

#### Satisfactorio (70-89%)
- Conoce las reglas del ownership
- Diferencia move de copy en casos básicos
- Entiende referencias inmutables y mutables
- Sabe que el borrow checker previene errores

#### En Desarrollo (50-69%)
- Recuerda algunas reglas del ownership
- Confunde move con copy ocasionalmente
- Usa referencias pero no siempre correctamente
- Depende de los mensajes del compilador

#### Insuficiente (<50%)
- No recuerda las reglas del ownership
- No diferencia move de copy
- No entiende el concepto de referencias
- No puede explicar para qué sirve el borrow checker

---

### 2. Desempeño (40%)

#### Excelente (90-100%)
- Resuelve ejercicios de ownership sin errores
- Elige correctamente entre `&T`, `&mut T`, y `T`
- Soluciona errores del borrow checker de forma elegante
- Refactoriza código para evitar clones innecesarios

#### Satisfactorio (70-89%)
- Completa ejercicios con mínima ayuda
- Usa referencias apropiadamente en mayoría de casos
- Corrige errores del borrow checker con algo de experimentación
- Usa `.clone()` cuando es necesario

#### En Desarrollo (50-69%)
- Necesita ayuda frecuente con ownership
- Abusa de `.clone()` para evitar errores
- Corrige errores por prueba y error
- Le cuesta elegir el tipo de referencia correcto

#### Insuficiente (<50%)
- No puede completar ejercicios básicos
- No entiende cuándo usar referencias
- No puede resolver errores del borrow checker
- Código no compila

---

### 3. Producto (30%)

#### Proyecto: Sistema de Biblioteca

##### Excelente (90-100%)
```rust
// ✅ Diseño que refleja borrowing real
struct Biblioteca {
    libros: Vec<Libro>,
}

impl Biblioteca {
    // Presta libro (referencia mutable)
    fn prestar(&mut self, isbn: &str) -> Option<&mut Libro>
    
    // Consulta libro (referencia inmutable)
    fn buscar(&self, isbn: &str) -> Option<&Libro>
    
    // Devuelve libro (modifica estado)
    fn devolver(&mut self, isbn: &str) -> Result<(), Error>
}
```
- Usa el sistema de tipos para modelar préstamos
- No hay `.clone()` innecesarios
- Manejo correcto de errores
- Tests que verifican el borrowing

##### Satisfactorio (70-89%)
- Funcionalidad completa
- Uso razonable de referencias
- Algunos `.clone()` justificados
- Tests básicos

##### En Desarrollo (50-69%)
- Funcionalidad parcial
- Abuso de `.clone()`
- Ownership confuso
- Pocos tests

##### Insuficiente (<50%)
- No compila o funcionalidad mínima
- No usa referencias
- No aplica conceptos de la semana

---

## 🔍 Ejercicios de Evaluación

### Ejercicio 1: Identificar el Error (10 pts)

```rust
fn main() {
    let s1 = String::from("hola");
    let s2 = s1;
    println!("{}", s1); // ¿Qué pasa aquí?
}
```

**Respuesta esperada:**
- Error: `value borrowed here after move`
- `s1` fue movido a `s2`
- Soluciones: usar `s1.clone()` o referencia `&s1`

### Ejercicio 2: Corregir Código (15 pts)

```rust
fn main() {
    let mut vec = vec![1, 2, 3];
    let first = &vec[0];
    vec.push(4);
    println!("{}", first);
}
```

**Respuesta esperada:**
```rust
fn main() {
    let mut vec = vec![1, 2, 3];
    vec.push(4);  // Mover antes
    let first = &vec[0];
    println!("{}", first);
}
```

### Ejercicio 3: Diseñar Función (20 pts)

Diseña una función que:
- Reciba un vector por referencia mutable
- Duplique todos sus elementos
- No transfiera ownership

**Respuesta esperada:**
```rust
fn duplicar_elementos(vec: &mut Vec<i32>) {
    for elem in vec.iter_mut() {
        *elem *= 2;
    }
}
```

---

## 📈 Escala de Calificación

| Rango | Calificación | Descripción |
|-------|--------------|-------------|
| 90-100 | A | Excelente dominio del ownership |
| 80-89 | B | Buen manejo, errores menores |
| 70-79 | C | Competente, necesita práctica |
| 60-69 | D | En desarrollo, requiere refuerzo |
| <60 | F | No alcanza competencias mínimas |

---

## 💡 Retroalimentación Común

### Fortalezas Típicas
- "Excelente comprensión de por qué Rust usa ownership"
- "Buen uso de referencias para evitar copias"
- "Código idiomático que aprovecha el borrow checker"

### Áreas de Mejora
- "Reducir uso de `.clone()` - analizar si es necesario"
- "Considerar lifetimes cuando uses referencias en structs"
- "Practicar más la lectura de errores del compilador"

---

## 🎯 Objetivos para Siguiente Semana

Si dominas esta semana, estarás listo para:
- Semana 05: Enums y Pattern Matching
- Usar `Option<&T>` y `Result<&T, E>` con confianza
- Entender por qué `match` consume o toma prestado
