# 🌲 Proyecto Semanal: Árbol de Archivos Interactivo

## 📋 Descripción

Implementa un sistema de árbol de archivos similar al explorador de archivos de un sistema operativo. Cada nodo puede ser un directorio o un archivo, y se puede navegar bidireccional entre padres e hijos.

---

## 🎯 Objetivos de Aprendizaje

- Aplicar `Rc<RefCell<T>>` para estructuras mutables compartidas
- Usar `Weak<T>` para relaciones padre-hijo sin ciclos de memoria
- Implementar búsqueda recursiva en estructuras de árbol
- Manejar movimiento de nodos entre diferentes partes del árbol

---

## 📐 Arquitectura

```
┌─────────────────────────────────────────────────────────────┐
│                      Sistema de Archivos                     │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│   NodoRef = Rc<RefCell<NodoInner>>                          │
│                                                              │
│   ┌────────────────────────────────────────────────────┐    │
│   │                    NodoInner                        │    │
│   ├────────────────────────────────────────────────────┤    │
│   │  nombre: String                                     │    │
│   │  tipo: TipoNodo (Directorio | Archivo)             │    │
│   │  padre: Weak<RefCell<NodoInner>>    ← No ciclos    │    │
│   │  hijos: Vec<Rc<RefCell<NodoInner>>> ← Ownership    │    │
│   └────────────────────────────────────────────────────┘    │
│                                                              │
│   Relaciones:                                                │
│   • Padre → Hijo: Rc (fuerte) - padre mantiene vivos hijos  │
│   • Hijo → Padre: Weak (débil) - no previene liberación     │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## 📁 Estructura del Proyecto

```
proyecto-arbol/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs        # Implementación con placeholders
    └── solucion.rs    # Solución completa
```

---

## 🔧 Funcionalidades a Implementar

### 1. Creación de Nodos

```rust
// Crear directorio
let dir = FileNode::crear_directorio("documentos");

// Crear archivo con contenido
let archivo = FileNode::crear_archivo("readme.txt", "Contenido...");
```

### 2. Construcción del Árbol

```rust
// Agregar hijos a un directorio
FileNode::agregar_hijo(&raiz, &home);
FileNode::agregar_hijo(&home, &usuario);
```

### 3. Búsqueda

```rust
// Buscar por nombre (recursivo)
if let Some(nodo) = FileNode::buscar(&raiz, "config.toml") {
    println!("Encontrado: {}", FileNode::ruta_completa(&nodo));
}
```

### 4. Navegación

```rust
// Obtener ruta completa desde la raíz
let ruta = FileNode::ruta_completa(&nodo);
// "/home/usuario/documentos/archivo.txt"
```

### 5. Visualización

```rust
// Imprimir árbol con indentación
FileNode::imprimir_arbol(&raiz, 0);
// 📁 /
//   📁 home
//     📁 usuario
//       📄 archivo.txt
```

### 6. Estadísticas

```rust
let stats = FileNode::estadisticas(&raiz);
println!("Directorios: {}", stats.directorios);
println!("Archivos: {}", stats.archivos);
println!("Tamaño total: {} bytes", stats.tamanio_total);
```

### 7. Modificación

```rust
// Cambiar contenido de un archivo
FileNode::modificar_contenido(&archivo, "Nuevo contenido");
```

### 8. Movimiento

```rust
// Mover nodo a otro directorio
FileNode::mover(&archivo, &otro_directorio);
```

---

## 💡 Conceptos Clave

### ¿Por qué `Rc<RefCell<T>>`?

- **Rc**: Permite múltiples referencias al mismo nodo (necesario para que tanto el padre como la búsqueda puedan acceder)
- **RefCell**: Permite mutabilidad interior (modificar contenido, agregar hijos)

### ¿Por qué `Weak` para el padre?

```
Sin Weak (CICLO DE MEMORIA):
┌────────┐ Rc ┌────────┐
│ Padre  │───→│  Hijo  │
│        │←───│        │
└────────┘ Rc └────────┘
  ↑                    ↑
  │  Ambos con count=1 │
  │  Nunca se liberan  │

Con Weak (SIN CICLO):
┌────────┐ Rc  ┌────────┐
│ Padre  │────→│  Hijo  │
│        │←- - │        │
└────────┘Weak └────────┘
  ↑                    
  │ Cuando Padre.count=0, se libera
  │ Hijo también se libera
```

---

## 🧪 Tests

```bash
# Ejecutar tests
cargo test

# Tests incluidos:
# - test_crear_estructura
# - test_busqueda
# - test_ruta_completa
# - test_estadisticas
# - test_mover_nodo
```

---

## 📊 Rúbrica de Evaluación

| Criterio | Puntos |
|----------|--------|
| Crear directorios y archivos | 15 |
| Agregar hijos correctamente | 15 |
| Búsqueda recursiva funcional | 15 |
| Ruta completa correcta | 15 |
| Impresión del árbol | 10 |
| Cálculo de estadísticas | 10 |
| Modificación de contenido | 10 |
| Mover nodos entre directorios | 10 |
| **Total** | **100** |

---

## 🚀 Extensiones Opcionales

1. **Eliminar nodos**: Implementar `FileNode::eliminar(&nodo)`
2. **Copiar nodos**: Crear copia profunda de un subárbol
3. **Buscar con patrón**: Buscar usando glob patterns (`*.txt`)
4. **Historial de cambios**: Registrar modificaciones con timestamps
5. **Serialización**: Guardar/cargar árbol a/desde JSON

---

## ✅ Criterios de Éxito

- [ ] El programa compila sin warnings
- [ ] Todos los tests pasan
- [ ] No hay memory leaks (verificar con Rc::strong_count)
- [ ] La navegación padre-hijo funciona correctamente
- [ ] Mover nodos actualiza tanto origen como destino
