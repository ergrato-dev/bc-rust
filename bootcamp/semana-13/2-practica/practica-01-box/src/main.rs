//! Práctica 01: Box y Tipos Recursivos
//!
//! En esta práctica aprenderás a:
//! - Usar Box para almacenar datos en el heap
//! - Implementar tipos recursivos (lista enlazada)
//! - Trabajar con árboles binarios

fn main() {
    println!("=== Práctica 01: Box y Tipos Recursivos ===\n");

    // Ejercicio 1: Lista enlazada básica
    println!("--- Ejercicio 1: Lista Enlazada ---");
    let lista = List::new()
        .prepend(3)
        .prepend(2)
        .prepend(1);
    println!("Lista: {:?}", lista);
    println!("Longitud: {}", lista.len());
    println!("Suma: {}", lista.sum());

    // Ejercicio 2: Árbol binario
    println!("\n--- Ejercicio 2: Árbol Binario ---");
    let arbol = crear_arbol_ejemplo();
    println!("Árbol: {:?}", arbol);
    println!("Profundidad: {}", arbol.depth());
    println!("Suma: {}", arbol.sum());

    // Ejercicio 3: Expresiones matemáticas
    println!("\n--- Ejercicio 3: Expresiones ---");
    // (2 + 3) * 4 = 20
    let expr = crear_expresion_ejemplo();
    println!("Resultado: {}", expr.eval());

    println!("\n✅ Todos los ejercicios completados!");
}

// ============================================================
// EJERCICIO 1: Lista Enlazada
// ============================================================

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn new() -> Self {
        // ↓ Implementa: retorna una lista vacía
        List::Nil
    }

    fn prepend(self, value: i32) -> Self {
        // ↓ Implementa: agrega un elemento al inicio
        // Pista: List::Cons(value, Box::new(self))
        List::Cons(value, Box::new(self))
    }

    fn len(&self) -> usize {
        // ↓ Implementa: retorna la longitud de la lista
        // Pista: usa match y recursión
        match self {
            List::Nil => 0,
            List::Cons(_, _tail) => 1, // ← Cambiar por: 1 + tail.len()
        }
    }

    fn sum(&self) -> i32 {
        // ↓ Implementa: retorna la suma de todos los elementos
        match self {
            List::Nil => 0,
            List::Cons(val, _tail) => *val, // ← Cambiar por: *val + tail.sum()
        }
    }
}

// ============================================================
// EJERCICIO 2: Árbol Binario
// ============================================================

#[derive(Debug)]
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        // ↓ Implementa: crea un nodo hoja (sin hijos)
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    fn with_children(value: i32, left: Option<TreeNode>, right: Option<TreeNode>) -> Self {
        // ↓ Implementa: crea un nodo con hijos opcionales
        // Pista: usa .map(Box::new) para convertir Option<TreeNode> a Option<Box<TreeNode>>
        TreeNode {
            value,
            left: None,  // ← Cambiar por: left.map(Box::new)
            right: None, // ← Cambiar por: right.map(Box::new)
        }
    }

    fn depth(&self) -> usize {
        // ↓ Implementa: retorna la profundidad del árbol
        // Pista: max de profundidad izquierda y derecha + 1
        1 // ← Implementar recursivamente
    }

    fn sum(&self) -> i32 {
        // ↓ Implementa: retorna la suma de todos los valores
        self.value // ← Sumar también los hijos
    }
}

fn crear_arbol_ejemplo() -> TreeNode {
    //       5
    //      / \
    //     3   8
    //    /
    //   1
    TreeNode::with_children(
        5,
        Some(TreeNode::with_children(
            3,
            Some(TreeNode::new(1)),
            None,
        )),
        Some(TreeNode::new(8)),
    )
}

// ============================================================
// EJERCICIO 3: Expresiones Matemáticas
// ============================================================

#[derive(Debug)]
enum Expr {
    Num(i32),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn eval(&self) -> i32 {
        // ↓ Implementa: evalúa la expresión
        match self {
            Expr::Num(n) => *n,
            Expr::Add(_left, _right) => 0, // ← Cambiar por: left.eval() + right.eval()
            Expr::Mul(_left, _right) => 0, // ← Cambiar por: left.eval() * right.eval()
        }
    }
}

fn crear_expresion_ejemplo() -> Expr {
    // (2 + 3) * 4
    Expr::Mul(
        Box::new(Expr::Add(
            Box::new(Expr::Num(2)),
            Box::new(Expr::Num(3)),
        )),
        Box::new(Expr::Num(4)),
    )
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_empty() {
        let list = List::new();
        assert_eq!(list.len(), 0);
        assert_eq!(list.sum(), 0);
    }

    #[test]
    fn test_list_prepend() {
        let list = List::new().prepend(3).prepend(2).prepend(1);
        assert_eq!(list.len(), 3);
        assert_eq!(list.sum(), 6);
    }

    #[test]
    fn test_tree_leaf() {
        let node = TreeNode::new(5);
        assert_eq!(node.depth(), 1);
        assert_eq!(node.sum(), 5);
    }

    #[test]
    fn test_tree_with_children() {
        let tree = crear_arbol_ejemplo();
        assert_eq!(tree.depth(), 3);
        assert_eq!(tree.sum(), 17);
    }

    #[test]
    fn test_expr_eval() {
        let expr = crear_expresion_ejemplo();
        assert_eq!(expr.eval(), 20);
    }
}
