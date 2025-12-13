//! Solución - Práctica 01: Box y Tipos Recursivos

fn main() {
    println!("=== Práctica 01: Box y Tipos Recursivos ===\n");

    println!("--- Ejercicio 1: Lista Enlazada ---");
    let list = List::new()
        .prepend(3)
        .prepend(2)
        .prepend(1);
    println!("Lista: {:?}", list);
    println!("Longitud: {}", list.len());
    println!("Suma: {}", list.sum());

    println!("\n--- Ejercicio 2: Árbol Binario ---");
    let tree = create_example_tree();
    println!("Árbol: {:?}", tree);
    println!("Profundidad: {}", tree.depth());
    println!("Suma: {}", tree.sum());

    println!("\n--- Ejercicio 3: Expresiones ---");
    let expr = create_example_expr();
    println!("Resultado: {}", expr.eval());

    println!("\n✅ Todos los ejercicios completados!");
}

// EJERCICIO 1: Lista Enlazada

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn new() -> Self {
        List::Nil
    }

    fn prepend(self, value: i32) -> Self {
        List::Cons(value, Box::new(self))
    }

    fn len(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, tail) => 1 + tail.len(),
        }
    }

    fn sum(&self) -> i32 {
        match self {
            List::Nil => 0,
            List::Cons(val, tail) => *val + tail.sum(),
        }
    }
}

// EJERCICIO 2: Árbol Binario

#[derive(Debug)]
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    fn with_children(value: i32, left: Option<TreeNode>, right: Option<TreeNode>) -> Self {
        TreeNode {
            value,
            left: left.map(Box::new),
            right: right.map(Box::new),
        }
    }

    fn depth(&self) -> usize {
        let left_depth = self.left.as_ref().map_or(0, |n| n.depth());
        let right_depth = self.right.as_ref().map_or(0, |n| n.depth());
        1 + left_depth.max(right_depth)
    }

    fn sum(&self) -> i32 {
        let left_sum = self.left.as_ref().map_or(0, |n| n.sum());
        let right_sum = self.right.as_ref().map_or(0, |n| n.sum());
        self.value + left_sum + right_sum
    }
}

fn create_example_tree() -> TreeNode {
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

// EJERCICIO 3: Expresiones Matemáticas

#[derive(Debug)]
enum Expr {
    Num(i32),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn eval(&self) -> i32 {
        match self {
            Expr::Num(n) => *n,
            Expr::Add(left, right) => left.eval() + right.eval(),
            Expr::Mul(left, right) => left.eval() * right.eval(),
        }
    }
}

fn create_example_expr() -> Expr {
    // (2 + 3) * 4
    Expr::Mul(
        Box::new(Expr::Add(
            Box::new(Expr::Num(2)),
            Box::new(Expr::Num(3)),
        )),
        Box::new(Expr::Num(4)),
    )
}

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
        let tree = create_example_tree();
        assert_eq!(tree.depth(), 3);
        assert_eq!(tree.sum(), 17);
    }

    #[test]
    fn test_expr_eval() {
        let expr = create_example_expr();
        assert_eq!(expr.eval(), 20);
    }
}
