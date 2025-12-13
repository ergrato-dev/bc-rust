//! Solución - Proyecto: Árbol de Archivos Interactivo

use std::rc::{Rc, Weak};
use std::cell::RefCell;

fn main() {
    println!("=== 🌲 Proyecto: Árbol de Archivos - SOLUCIÓN ===\n");

    // Crear estructura de árbol
    let raiz = FileNode::crear_directorio("/");
    
    let home = FileNode::crear_directorio("home");
    let etc = FileNode::crear_directorio("etc");
    let var = FileNode::crear_directorio("var");
    
    FileNode::agregar_hijo(&raiz, &home);
    FileNode::agregar_hijo(&raiz, &etc);
    FileNode::agregar_hijo(&raiz, &var);
    
    let usuario = FileNode::crear_directorio("usuario");
    let documentos = FileNode::crear_directorio("documentos");
    
    FileNode::agregar_hijo(&home, &usuario);
    FileNode::agregar_hijo(&usuario, &documentos);
    
    // Crear algunos archivos
    let readme = FileNode::crear_archivo("README.md", "# Mi Proyecto\n");
    let config = FileNode::crear_archivo(".gitignore", "target/\n*.log\n");
    
    FileNode::agregar_hijo(&documentos, &readme);
    FileNode::agregar_hijo(&documentos, &config);
    
    // Mostrar árbol
    println!("📂 Estructura del árbol:");
    FileNode::imprimir_arbol(&raiz, 0);
    
    // Buscar archivos
    println!("\n🔍 Búsqueda:");
    if let Some(nodo) = FileNode::buscar(&raiz, "README.md") {
        println!("Encontrado: {}", FileNode::ruta_completa(&nodo));
    }
    
    // Estadísticas
    println!("\n📊 Estadísticas:");
    let stats = FileNode::estadisticas(&raiz);
    println!("Directorios: {}", stats.directorios);
    println!("Archivos: {}", stats.archivos);
    println!("Tamaño total: {} bytes", stats.tamanio_total);
    
    // Modificar archivo
    println!("\n✏️ Modificación:");
    if let Some(nodo) = FileNode::buscar(&raiz, "README.md") {
        FileNode::modificar_contenido(&nodo, "# Mi Proyecto Actualizado\n\nDescripción...\n");
        println!("Archivo modificado");
    }
    
    // Mover nodo
    println!("\n📦 Mover archivo:");
    if let Some(gitignore) = FileNode::buscar(&raiz, ".gitignore") {
        FileNode::mover(&gitignore, &usuario);
        println!("Nueva ubicación: {}", FileNode::ruta_completa(&gitignore));
    }
    
    // Mostrar árbol actualizado
    println!("\n📂 Árbol actualizado:");
    FileNode::imprimir_arbol(&raiz, 0);
    
    println!("\n✅ Proyecto completado!");
}

// ============================================================
// TIPOS Y ESTRUCTURAS
// ============================================================

type NodoRef = Rc<RefCell<NodoInner>>;
type NodoWeak = Weak<RefCell<NodoInner>>;

enum TipoNodo {
    Directorio,
    Archivo { contenido: String },
}

struct NodoInner {
    nombre: String,
    tipo: TipoNodo,
    padre: NodoWeak,
    hijos: Vec<NodoRef>,
}

struct Estadisticas {
    directorios: usize,
    archivos: usize,
    tamanio_total: usize,
}

struct FileNode;

impl FileNode {
    fn crear_directorio(nombre: &str) -> NodoRef {
        Rc::new(RefCell::new(NodoInner {
            nombre: nombre.to_string(),
            tipo: TipoNodo::Directorio,
            padre: Weak::new(),
            hijos: Vec::new(),
        }))
    }
    
    fn crear_archivo(nombre: &str, contenido: &str) -> NodoRef {
        Rc::new(RefCell::new(NodoInner {
            nombre: nombre.to_string(),
            tipo: TipoNodo::Archivo { 
                contenido: contenido.to_string() 
            },
            padre: Weak::new(),
            hijos: Vec::new(),
        }))
    }
    
    fn agregar_hijo(padre: &NodoRef, hijo: &NodoRef) {
        // Verificar que el padre es un directorio
        let es_directorio = matches!(padre.borrow().tipo, TipoNodo::Directorio);
        
        if es_directorio {
            hijo.borrow_mut().padre = Rc::downgrade(padre);
            padre.borrow_mut().hijos.push(Rc::clone(hijo));
        }
    }
    
    fn buscar(raiz: &NodoRef, nombre: &str) -> Option<NodoRef> {
        // Si este nodo tiene el nombre buscado
        if raiz.borrow().nombre == nombre {
            return Some(Rc::clone(raiz));
        }
        
        // Buscar en los hijos
        for hijo in &raiz.borrow().hijos {
            if let Some(encontrado) = Self::buscar(hijo, nombre) {
                return Some(encontrado);
            }
        }
        
        None
    }
    
    fn ruta_completa(nodo: &NodoRef) -> String {
        let mut partes = vec![nodo.borrow().nombre.clone()];
        let mut actual = nodo.borrow().padre.upgrade();
        
        while let Some(padre) = actual {
            let nombre = padre.borrow().nombre.clone();
            partes.push(nombre);
            actual = padre.borrow().padre.upgrade();
        }
        
        partes.reverse();
        
        // Si la raíz es "/", manejar caso especial
        if partes.first() == Some(&"/".to_string()) {
            format!("/{}", partes[1..].join("/"))
        } else {
            partes.join("/")
        }
    }
    
    fn imprimir_arbol(nodo: &NodoRef, nivel: usize) {
        let nodo_ref = nodo.borrow();
        let indent = "  ".repeat(nivel);
        
        let icono = match nodo_ref.tipo {
            TipoNodo::Directorio => "📁",
            TipoNodo::Archivo { .. } => "📄",
        };
        
        println!("{}{} {}", indent, icono, nodo_ref.nombre);
        
        for hijo in &nodo_ref.hijos {
            Self::imprimir_arbol(hijo, nivel + 1);
        }
    }
    
    fn estadisticas(raiz: &NodoRef) -> Estadisticas {
        let mut stats = Estadisticas {
            directorios: 0,
            archivos: 0,
            tamanio_total: 0,
        };
        
        Self::calcular_stats_recursivo(raiz, &mut stats);
        stats
    }
    
    fn calcular_stats_recursivo(nodo: &NodoRef, stats: &mut Estadisticas) {
        let nodo_ref = nodo.borrow();
        
        match &nodo_ref.tipo {
            TipoNodo::Directorio => {
                stats.directorios += 1;
            }
            TipoNodo::Archivo { contenido } => {
                stats.archivos += 1;
                stats.tamanio_total += contenido.len();
            }
        }
        
        for hijo in &nodo_ref.hijos {
            Self::calcular_stats_recursivo(hijo, stats);
        }
    }
    
    fn modificar_contenido(nodo: &NodoRef, nuevo_contenido: &str) {
        let mut nodo_mut = nodo.borrow_mut();
        
        if let TipoNodo::Archivo { ref mut contenido } = nodo_mut.tipo {
            *contenido = nuevo_contenido.to_string();
        }
    }
    
    fn mover(nodo: &NodoRef, nuevo_padre: &NodoRef) {
        // Remover del padre actual
        if let Some(padre_actual) = nodo.borrow().padre.upgrade() {
            padre_actual.borrow_mut().hijos.retain(|h| !Rc::ptr_eq(h, nodo));
        }
        
        // Establecer nuevo padre y agregar a sus hijos
        nodo.borrow_mut().padre = Rc::downgrade(nuevo_padre);
        nuevo_padre.borrow_mut().hijos.push(Rc::clone(nodo));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_estructura() {
        let raiz = FileNode::crear_directorio("/");
        let home = FileNode::crear_directorio("home");
        
        FileNode::agregar_hijo(&raiz, &home);
        
        assert_eq!(raiz.borrow().hijos.len(), 1);
    }
    
    #[test]
    fn test_busqueda() {
        let raiz = FileNode::crear_directorio("/");
        let home = FileNode::crear_directorio("home");
        let archivo = FileNode::crear_archivo("test.txt", "contenido");
        
        FileNode::agregar_hijo(&raiz, &home);
        FileNode::agregar_hijo(&home, &archivo);
        
        let encontrado = FileNode::buscar(&raiz, "test.txt");
        assert!(encontrado.is_some());
    }
    
    #[test]
    fn test_ruta_completa() {
        let raiz = FileNode::crear_directorio("/");
        let home = FileNode::crear_directorio("home");
        let usuario = FileNode::crear_directorio("usuario");
        
        FileNode::agregar_hijo(&raiz, &home);
        FileNode::agregar_hijo(&home, &usuario);
        
        let ruta = FileNode::ruta_completa(&usuario);
        assert_eq!(ruta, "/home/usuario");
    }
    
    #[test]
    fn test_estadisticas() {
        let raiz = FileNode::crear_directorio("/");
        let archivo1 = FileNode::crear_archivo("a.txt", "123");
        let archivo2 = FileNode::crear_archivo("b.txt", "12345");
        
        FileNode::agregar_hijo(&raiz, &archivo1);
        FileNode::agregar_hijo(&raiz, &archivo2);
        
        let stats = FileNode::estadisticas(&raiz);
        assert_eq!(stats.directorios, 1);
        assert_eq!(stats.archivos, 2);
        assert_eq!(stats.tamanio_total, 8);
    }
    
    #[test]
    fn test_mover_nodo() {
        let raiz = FileNode::crear_directorio("/");
        let dir1 = FileNode::crear_directorio("dir1");
        let dir2 = FileNode::crear_directorio("dir2");
        let archivo = FileNode::crear_archivo("test.txt", "contenido");
        
        FileNode::agregar_hijo(&raiz, &dir1);
        FileNode::agregar_hijo(&raiz, &dir2);
        FileNode::agregar_hijo(&dir1, &archivo);
        
        assert_eq!(dir1.borrow().hijos.len(), 1);
        assert_eq!(dir2.borrow().hijos.len(), 0);
        
        FileNode::mover(&archivo, &dir2);
        
        assert_eq!(dir1.borrow().hijos.len(), 0);
        assert_eq!(dir2.borrow().hijos.len(), 1);
    }
    
    #[test]
    fn test_modificar_contenido() {
        let archivo = FileNode::crear_archivo("test.txt", "original");
        FileNode::modificar_contenido(&archivo, "modificado");
        
        if let TipoNodo::Archivo { contenido } = &archivo.borrow().tipo {
            assert_eq!(contenido, "modificado");
        } else {
            panic!("Debería ser un archivo");
        }
    }
}
