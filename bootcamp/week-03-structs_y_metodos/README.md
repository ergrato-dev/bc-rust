# 📦 Semana 03: Structs y Métodos

## 🎯 Objetivos de Aprendizaje

Al finalizar esta semana, serás capaz de:

- Definir y crear estructuras (`struct`) para agrupar datos
- Implementar métodos y funciones asociadas con `impl`
- Usar diferentes tipos de structs (named, tuple, unit)
- Aplicar el patrón constructor con `new()`
- Entender `self`, `&self` y `&mut self`

---

## 📚 Contenido

| Sección | Tema | Duración |
|---------|------|----------|
| 1 | Definición de Structs | 30 min |
| 2 | Instanciación y Acceso | 20 min |
| 3 | Métodos con impl | 40 min |
| 4 | Funciones Asociadas | 20 min |
| 5 | Tipos de Structs | 30 min |

---

## 📁 Estructura de la Semana

```
semana-03/
├── README.md                    # Este archivo
├── rubrica-evaluacion.md        # Criterios de evaluación
├── 0-assets/                    # Diagramas SVG
├── 1-teoria/                    # Material teórico
│   ├── 01-definicion-structs.md
│   ├── 02-instanciacion-acceso.md
│   ├── 03-metodos-impl.md
│   ├── 04-funciones-asociadas.md
│   └── 05-tipos-structs.md
├── 2-practica/                  # Ejercicios guiados
│   ├── practica-01-struct-basico/
│   ├── practica-02-metodos/
│   ├── practica-03-constructores/
│   └── practica-04-structs-avanzados/
├── 3-proyecto/                  # Proyecto semanal
│   └── proyecto-sistema-usuarios/
├── 4-recursos/                  # Material adicional
│   ├── ebook-free/
│   ├── videografia/
│   └── webgrafia/
└── 5-glosario/                  # Términos clave
```

---

## ⏱️ Distribución del Tiempo (4 horas)

| Actividad | Tiempo | Descripción |
|-----------|--------|-------------|
| **Teoría** | 60 min | Lectura y comprensión de conceptos |
| **Prácticas** | 90 min | Ejercicios guiados (4 prácticas) |
| **Proyecto** | 60 min | Sistema de gestión de usuarios |
| **Repaso** | 30 min | Glosario y recursos adicionales |

---

## 🔑 Conceptos Clave

### Struct Básico

```rust
struct Usuario {
    nombre: String,
    email: String,
    edad: u32,
    activo: bool,
}
```

### Métodos con impl

```rust
impl Usuario {
    // Función asociada (constructor)
    fn new(nombre: String, email: String) -> Self {
        Self {
            nombre,
            email,
            edad: 0,
            activo: true,
        }
    }

    // Método (usa &self)
    fn saludar(&self) {
        println!("Hola, soy {}", self.nombre);
    }

    // Método que modifica (&mut self)
    fn cumplir_anios(&mut self) {
        self.edad += 1;
    }
}
```

### Uso

```rust
let mut usuario = Usuario::new(
    String::from("Ana"),
    String::from("ana@email.com")
);

usuario.saludar();
usuario.cumplir_anios();
```

---

## ✅ Requisitos Previos

- [x] Semana 01: Setup completado
- [x] Semana 02: Variables y tipos de datos
- [x] Entender ownership básico (se profundiza en Semana 04)

---

## 📋 Evaluación

| Componente | Peso | Criterio |
|------------|------|----------|
| **Conocimiento** | 30% | Quiz sobre structs y métodos |
| **Desempeño** | 40% | Prácticas completadas |
| **Producto** | 30% | Proyecto funcional con tests |

---

## 🚀 Próxima Semana

**Semana 04**: Ownership y Borrowing - El corazón de Rust

---

*Bootcamp Rust: Zero to Hero*
