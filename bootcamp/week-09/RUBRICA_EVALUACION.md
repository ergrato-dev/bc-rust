# 📊 Rúbrica de Evaluación - Semana 09: Traits Básicos

## 🎯 Competencias a Evaluar

| Competencia | Descripción |
|-------------|-------------|
| **Definición** | Crear traits con métodos requeridos y default |
| **Implementación** | Implementar traits para tipos propios |
| **Derivación** | Usar correctamente `#[derive()]` |
| **Traits Estándar** | Implementar Display, Default, From/Into |
| **Bounds** | Usar trait bounds en funciones genéricas |

---

## 📝 Criterios de Evaluación

### 1. Definición de Traits (20 puntos)

| Nivel | Puntos | Criterios |
|-------|--------|-----------|
| **Excelente** | 18-20 | Define traits claros con métodos bien nombrados, incluye métodos default útiles |
| **Bueno** | 14-17 | Define traits funcionales, algunos métodos default |
| **Suficiente** | 10-13 | Define traits básicos sin métodos default |
| **Insuficiente** | 0-9 | No logra definir traits correctamente |

### 2. Implementación de Traits (25 puntos)

| Nivel | Puntos | Criterios |
|-------|--------|-----------|
| **Excelente** | 22-25 | Implementa traits para múltiples tipos, código idiomático |
| **Bueno** | 17-21 | Implementa traits correctamente, buen manejo de `self` |
| **Suficiente** | 12-16 | Implementa traits básicos con ayuda |
| **Insuficiente** | 0-11 | No logra implementar traits |

### 3. Traits Derivables (20 puntos)

| Nivel | Puntos | Criterios |
|-------|--------|-----------|
| **Excelente** | 18-20 | Usa derive apropiadamente, entiende cuándo derivar vs implementar manual |
| **Bueno** | 14-17 | Usa derive correctamente para Debug, Clone, PartialEq |
| **Suficiente** | 10-13 | Usa derive básico, no siempre el más apropiado |
| **Insuficiente** | 0-9 | No entiende cómo usar derive |

### 4. Traits de la Biblioteca Estándar (20 puntos)

| Nivel | Puntos | Criterios |
|-------|--------|-----------|
| **Excelente** | 18-20 | Implementa Display, Default, From/Into correctamente |
| **Bueno** | 14-17 | Implementa Display y Default, intenta From |
| **Suficiente** | 10-13 | Implementa Display básico |
| **Insuficiente** | 0-9 | No logra implementar traits estándar |

### 5. Trait Bounds (15 puntos)

| Nivel | Puntos | Criterios |
|-------|--------|-----------|
| **Excelente** | 13-15 | Usa bounds y where clauses, entiende `impl Trait` |
| **Bueno** | 10-12 | Usa trait bounds básicos correctamente |
| **Suficiente** | 7-9 | Usa bounds con ayuda |
| **Insuficiente** | 0-6 | No logra usar trait bounds |

---

## 🏆 Escala de Calificación

| Rango | Calificación | Descripción |
|-------|--------------|-------------|
| 90-100 | ⭐⭐⭐⭐⭐ | Excelente dominio de traits |
| 80-89 | ⭐⭐⭐⭐ | Buen manejo, pequeños detalles a mejorar |
| 70-79 | ⭐⭐⭐ | Competente, necesita más práctica |
| 60-69 | ⭐⭐ | Básico, requiere refuerzo |
| 0-59 | ⭐ | Insuficiente, necesita repetir conceptos |

---

## 📋 Lista de Verificación del Proyecto

### Proyecto: Sistema de Formas Geométricas

- [ ] Define trait `Forma` con área() y perímetro()
- [ ] Define trait `Dibujable` con dibujar()
- [ ] Implementa para Círculo, Rectángulo, Triángulo
- [ ] Usa `#[derive(Debug, Clone, PartialEq)]`
- [ ] Implementa `Display` para formato legible
- [ ] Implementa `Default` para valores por defecto
- [ ] Función genérica que acepta cualquier `Forma`
- [ ] Código compila sin warnings
- [ ] Tests unitarios pasan
- [ ] Código formateado con `rustfmt`

---

## 💡 Indicadores de Comprensión

### El estudiante ENTIENDE traits si:

1. Explica la diferencia entre trait e impl
2. Sabe cuándo usar derive vs implementación manual
3. Entiende la regla del huérfano
4. Puede escribir funciones con trait bounds
5. Diferencia entre `&impl Trait` y `&dyn Trait`

### Preguntas de Verificación:

1. ¿Qué es un trait y para qué sirve?
2. ¿Cuál es la diferencia entre `Debug` y `Display`?
3. ¿Por qué no puedo implementar `Display` para `Vec<T>`?
4. ¿Qué significa `T: Clone + Debug`?
5. ¿Cuándo usar `impl Trait` vs generics explícitos?
