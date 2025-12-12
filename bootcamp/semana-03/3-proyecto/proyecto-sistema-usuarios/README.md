# 👥 Proyecto Semanal: Sistema de Usuarios

## 📋 Descripción

Desarrollarás un sistema de gestión de usuarios que demuestra el uso de structs, métodos y funciones asociadas en Rust.

## 🎯 Objetivos de Aprendizaje

- Definir structs con campos apropiados
- Implementar constructores con `new()`
- Crear métodos de lectura y modificación
- Usar structs anidados
- Aplicar el patrón newtype

## 📁 Estructura del Proyecto

```
proyecto-sistema-usuarios/
├── Cargo.toml
├── src/
│   └── main.rs
└── README.md
```

## 🔧 Requisitos Funcionales

### Nivel 1: Básico (Obligatorio)

1. **Struct Usuario**
   - Campos: id, nombre, email, edad, activo
   - Constructor `new()`
   - Métodos: mostrar(), es_adulto(), desactivar()

2. **Struct Perfil**
   - Campos: bio, sitio_web, ubicacion
   - Constructor y métodos básicos

3. **Struct UsuarioCompleto**
   - Combina Usuario y Perfil
   - Métodos para acceder a ambos

### Nivel 2: Intermedio (Recomendado)

4. **Newtype para IDs**
   - `UserId(u64)` con validación
   
5. **Sistema de Roles**
   - Struct `Rol` o enum
   - Permisos asociados

### Nivel 3: Avanzado (Opcional)

6. **Gestor de Usuarios**
   - Struct que contiene Vec<Usuario>
   - Métodos: agregar, buscar, listar

## ✅ Criterios de Evaluación

| Criterio | Peso | Descripción |
|----------|------|-------------|
| Compilación | 20% | Sin errores ni warnings |
| Funcionalidad | 30% | Requisitos implementados |
| Diseño | 25% | Uso correcto de structs/métodos |
| Código limpio | 15% | Documentación, formato |
| Tests | 10% | Cobertura de casos |

## 🧪 Ejecutar

```bash
cargo run
cargo test
cargo clippy
```

## 💡 Pistas

```rust
// Struct básico
struct Usuario {
    id: u64,
    nombre: String,
    // ...
}

// Constructor
impl Usuario {
    fn new(nombre: String) -> Self {
        Self { id: 0, nombre, ... }
    }
}

// Método de lectura
fn es_adulto(&self) -> bool {
    self.edad >= 18
}

// Método de modificación
fn cumplir_anios(&mut self) {
    self.edad += 1;
}
```

## 📅 Entrega

- **Tiempo estimado**: 60-90 minutos
- **Formato**: Proyecto Cargo completo
