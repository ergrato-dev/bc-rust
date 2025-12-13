# 📊 Rúbrica de Evaluación - Semana 11: Lifetimes

## 🎯 Competencias a Evaluar

| Competencia | Descripción |
|-------------|-------------|
| **C1** | Comprende el propósito de los lifetimes |
| **C2** | Aplica anotaciones de lifetime en funciones |
| **C3** | Implementa structs con referencias |
| **C4** | Reconoce cuándo aplicar elision rules |
| **C5** | Utiliza 'static y lifetime bounds correctamente |

---

## 📝 Práctica 01: Lifetimes Básicos (15 puntos)

### Criterios de Evaluación

| Criterio | Excelente (5) | Bueno (4) | Suficiente (3) | Insuficiente (0-2) |
|----------|---------------|-----------|----------------|-------------------|
| **Sintaxis** | Anotaciones correctas en todos los ejercicios | Pequeños errores de sintaxis | Varios errores pero compila | No compila |
| **Relaciones** | Entiende relación entrada/salida | Algunos errores en relaciones | Confusión en múltiples params | No relaciona lifetimes |
| **Tests** | Todos pasan | 80%+ pasan | 60%+ pasan | <60% pasan |

**Puntaje**: ___/15

---

## 📝 Práctica 02: Lifetimes en Structs (15 puntos)

### Criterios de Evaluación

| Criterio | Excelente (5) | Bueno (4) | Suficiente (3) | Insuficiente (0-2) |
|----------|---------------|-----------|----------------|-------------------|
| **Declaración** | Structs con lifetimes correctos | Pequeños errores | Confusión en anotaciones | No puede declarar |
| **Implementación** | impl blocks correctos | Errores menores en impl | Dificultad con impl<'a> | No implementa métodos |
| **Uso** | Instancia correctamente | Algunos errores de uso | Confusión frecuente | No puede instanciar |

**Puntaje**: ___/15

---

## 📝 Práctica 03: Elision Rules (15 puntos)

### Criterios de Evaluación

| Criterio | Excelente (5) | Bueno (4) | Suficiente (3) | Insuficiente (0-2) |
|----------|---------------|-----------|----------------|-------------------|
| **Identificación** | Identifica cuándo aplicar cada regla | Mayoría correcta | Aplica reglas básicas | No identifica reglas |
| **Simplificación** | Omite lifetimes cuando es posible | Algunos casos no simplificados | Sobreespecifica lifetimes | No simplifica |
| **Explicación** | Justifica por qué aplica/no aplica | Explicaciones parciales | Explicaciones confusas | No puede explicar |

**Puntaje**: ___/15

---

## 📝 Práctica 04: Lifetimes Avanzados (15 puntos)

### Criterios de Evaluación

| Criterio | Excelente (5) | Bueno (4) | Suficiente (3) | Insuficiente (0-2) |
|----------|---------------|-----------|----------------|-------------------|
| **'static** | Uso correcto y apropiado | Pequeños errores | Confusión sobre cuándo usar | Uso incorrecto |
| **Bounds** | T: 'a y 'a: 'b correctos | Mayoría correcta | Dificultad con bounds | No entiende bounds |
| **Patrones** | Aplica patrones avanzados | Algunos patrones | Solo patrones básicos | No aplica patrones |

**Puntaje**: ___/15

---

## 🏗️ Proyecto: Parser de Texto (30 puntos)

### Criterios de Evaluación

| Criterio | Peso | Excelente | Bueno | Suficiente | Insuficiente |
|----------|------|-----------|-------|------------|--------------|
| **Funcionalidad** | 10 | Parser completo y funcional | Funciona con limitaciones | Parsing básico | No funciona |
| **Eficiencia** | 8 | Sin copias innecesarias | Algunas copias | Muchas copias | Todo copiado |
| **Lifetimes** | 8 | Lifetimes óptimos | Algunos redundantes | Muchos redundantes | Incorrectos |
| **Tests** | 4 | Cobertura completa | Buena cobertura | Cobertura básica | Sin tests |

**Puntaje**: ___/30

---

## 📈 Resumen de Evaluación

| Componente | Puntaje Obtenido | Puntaje Máximo |
|------------|------------------|----------------|
| Práctica 01 | | 15 |
| Práctica 02 | | 15 |
| Práctica 03 | | 15 |
| Práctica 04 | | 15 |
| Proyecto | | 30 |
| Participación | | 10 |
| **Total** | | **100** |

---

## 🎖️ Escala de Calificación

| Rango | Calificación | Descripción |
|-------|--------------|-------------|
| 90-100 | A | Excelente - Dominio completo de lifetimes |
| 80-89 | B | Bueno - Sólida comprensión con áreas de mejora |
| 70-79 | C | Satisfactorio - Comprensión básica lograda |
| 60-69 | D | Mínimo - Necesita refuerzo significativo |
| <60 | F | Insuficiente - Requiere repetir el material |

---

## 💡 Errores Comunes a Observar

### En Funciones
- [ ] Olvidar anotar lifetime en el retorno
- [ ] Usar diferentes lifetimes cuando deberían ser iguales
- [ ] Sobreespecificar lifetimes innecesariamente

### En Structs
- [ ] No anotar lifetime en la definición del struct
- [ ] Olvidar `<'a>` en el bloque impl
- [ ] Crear referencias que outlive el struct

### En Elision
- [ ] No reconocer cuándo aplicar regla 3 (self)
- [ ] Forzar anotaciones donde no son necesarias
- [ ] No entender por qué el compilador infiere lifetimes

### Conceptuales
- [ ] Confundir lifetimes con "duración" en tiempo de ejecución
- [ ] Pensar que lifetimes "controlan" cuánto vive un valor
- [ ] No entender que lifetimes son verificación estática

---

## 📋 Observaciones del Evaluador

```
Fortalezas:


Áreas de mejora:


Recomendaciones:


```

---

**Fecha de evaluación:** _______________  
**Evaluador:** _______________  
**Firma:** _______________
