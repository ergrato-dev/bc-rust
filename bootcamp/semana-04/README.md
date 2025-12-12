# 📦 Semana 04: Ownership y Borrowing

> **El corazón de Rust** - El sistema que hace a Rust único y seguro

## 🎯 Objetivos de Aprendizaje

Al finalizar esta semana, serás capaz de:

1. **Comprender** el sistema de ownership y sus tres reglas fundamentales
2. **Aplicar** correctamente move semantics y copy semantics
3. **Usar** referencias inmutables (`&T`) y mutables (`&mut T`)
4. **Resolver** errores del borrow checker con confianza
5. **Diseñar** funciones que reciban y devuelvan ownership apropiadamente

---

## 📚 Contenido

### 1. Teoría

| Archivo | Tema | Duración |
|---------|------|----------|
| [01-sistema-ownership.md](1-teoria/01-sistema-ownership.md) | Las 3 reglas del ownership | 25 min |
| [02-move-copy.md](1-teoria/02-move-copy.md) | Move semantics vs Copy | 25 min |
| [03-referencias-borrowing.md](1-teoria/03-referencias-borrowing.md) | Referencias y préstamos | 30 min |
| [04-reglas-borrowing.md](1-teoria/04-reglas-borrowing.md) | Reglas del borrow checker | 25 min |
| [05-ownership-funciones.md](1-teoria/05-ownership-funciones.md) | Ownership en funciones | 20 min |

### 2. Práctica

| Ejercicio | Descripción | Dificultad |
|-----------|-------------|------------|
| [Práctica 01](2-practica/practica-01-ownership-basico/) | Ownership básico y moves | ⭐ |
| [Práctica 02](2-practica/practica-02-referencias/) | Referencias inmutables y mutables | ⭐⭐ |
| [Práctica 03](2-practica/practica-03-borrow-checker/) | Resolver errores del borrow checker | ⭐⭐ |
| [Práctica 04](2-practica/practica-04-ownership-funciones/) | Ownership en parámetros y retorno | ⭐⭐⭐ |

### 3. Proyecto Semanal

| Proyecto | Descripción |
|----------|-------------|
| [Sistema de Biblioteca](3-proyecto/proyecto-biblioteca/) | Gestión de libros con préstamos (borrowing real!) |

### 4. Recursos

- [📖 eBooks Gratuitos](4-recursos/ebook-free/README.md)
- [🎬 Videografía](4-recursos/videografia/README.md)
- [🌐 Webgrafía](4-recursos/webgrafia/README.md)

### 5. Glosario

- [📖 Términos de Ownership](5-glosario/README.md)

---

## ⏱️ Distribución del Tiempo (4 horas)

| Actividad | Tiempo | Descripción |
|-----------|--------|-------------|
| Teoría | 90 min | Ownership, borrowing, borrow checker |
| Práctica guiada | 60 min | Ejercicios con el instructor |
| Proyecto | 60 min | Sistema de biblioteca |
| Revisión | 30 min | Errores comunes, Q&A |

---

## 🔑 Conceptos Clave

### Las 3 Reglas del Ownership

```
1. Cada valor tiene UN único dueño (owner)
2. Solo puede haber UN dueño a la vez
3. Cuando el dueño sale del scope, el valor se elimina (drop)
```

### Las Reglas del Borrowing

```
EN CUALQUIER MOMENTO puedes tener:
  - UNA referencia mutable (&mut T)
  - O MUCHAS referencias inmutables (&T)
  - PERO NUNCA ambas al mismo tiempo
```

---

## ⚠️ Errores Comunes

| Error | Causa | Solución |
|-------|-------|----------|
| `value moved here` | Usar valor después de moverlo | Clonar o usar referencias |
| `cannot borrow as mutable` | Préstamo mutable mientras hay inmutables | Reorganizar el código |
| `does not live long enough` | Referencia a valor que ya no existe | Extender el lifetime |

---

## 📋 Checklist de Competencias

- [ ] Puedo explicar las 3 reglas del ownership
- [ ] Entiendo la diferencia entre move y copy
- [ ] Sé cuándo usar `&T` vs `&mut T`
- [ ] Puedo resolver errores del borrow checker
- [ ] Diseño funciones considerando ownership

---

## 🔗 Navegación

| ← Anterior | Inicio | Siguiente → |
|------------|--------|-------------|
| [Semana 03: Structs](../semana-03/README.md) | [Bootcamp](../README.md) | [Semana 05: Enums](../semana-05/README.md) |
