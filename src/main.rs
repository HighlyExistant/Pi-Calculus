use gelato_tools::nodes::BinaryTreeNode;
use petgraph::{Graph, dot::{Config, Dot}, graph::UnGraph};

use crate::app::{App, statement::operator::{Operator, OperatorNode, binary::{BinOp}}};


mod app;
fn main() {
    let mut app = App::new(String::from("(var l)f(x).g(y)|help(g)"));
    let mut op = OperatorNode::parse_next(&mut app.tokens).unwrap();
    println!("{:#?}", op);
    op.bubble();
    println!("{:#?}", op);
}