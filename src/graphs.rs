#![warn(missing_debug_implementations)]

use std::{borrow::Borrow, cell::RefCell};
use typed_arena::Arena;

pub type NodeId = usize;

#[derive(Debug)]
enum RelOpType {
    Ne,
    Eq,
}

#[derive(Debug)]
enum Expr {
    RelOp(RelOpType),
    Int(usize),
    Bool(bool),
    Var(String),
    Star,
    Select
}

use Expr::*;
#[derive(Debug)]
struct Node<'a> {
    pub inner: Expr,
    pub children: RefCell<Vec<&'a Node<'a>>>,
}

impl<'a> Node<'a> {
    pub fn new(inner: Expr, children: Vec<&'a Node<'a>>) -> Self {
        Node {
            inner,
            children: RefCell::new(children),
        }
    }

    pub fn print(&self) {}
}

pub struct Foo { id: usize }


pub fn foo() {

}

#[test]
pub fn test() {
    let arena: Arena<Node> = Arena::new();

    populate(&arena);
}

fn add_node<'a>(arena: &'a Arena<Node<'a>>, inner: Expr, children: Vec<&'a Node<'a>>) -> &'a Node<'a> {
    //let children = children.unwrap_or(vec![]);
    //let node = Node::new(inner, children);
    arena.alloc(Node::new(inner, children))
}

fn populate<'a>(arena: &'a Arena<Node<'a>>) {
    // select col1, (1 == 2), *

    let col = arena.alloc(Node::new(Int(10), vec![]));
    let int1 = arena.alloc(Node::new(Int(10), vec![]));
    let int2 = arena.alloc(Node::new(Int(20), vec![]));
    let int3 = arena.alloc(Node::new(Int(30), vec![]));

    dbg!(&int1);
    dbg!(&int2);
    dbg!(&int3);

    let relop = arena.alloc(Node::new(RelOp(RelOpType::Eq), vec![int1, int2]));
    dbg!(&relop);

    let mut v = relop.children.borrow_mut();
    v.push(int3);
    v.push(relop);

    dbg!(&relop);
}
