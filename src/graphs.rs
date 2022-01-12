
use std::cell::RefCell;
use typed_arena::Arena;

pub type NodeId = usize;

enum RelOpType {
    Ne, Eq
}

enum Expr {
    RelOp(RelOpType),
    Int(usize),
    Bool(bool),
    Var(String)
}

use Expr::*;
struct Node<'a> {
    pub inner: Expr,
    pub children: RefCell<Option<Vec<&'a Node<'a>>>>
}

impl<'a> Node<'a> {
    pub fn new(inner: Expr, children: Option<Vec<&'a Node<'a>>>) -> Self {
        Node {
            inner,
            children: RefCell::new(children),
        }
    }
}

#[test]
fn test() {    
    let arena = Arena::new();

    let int1 = arena.alloc(Node::new(Int(10), None));
    let int2 = arena.alloc(Node::new(Int(20), None));
    let relop = arena.alloc(Node::new(RelOp(RelOpType::Eq), Some(vec![int1, int2])));

    let v = relop.children.borrow().as_ref().unwrap()[0];

}




