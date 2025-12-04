use gelato_tools::nodes::BinaryTreeNode;
use petgraph::{Graph, dot::{Config, Dot}, graph::UnGraph};

use crate::app::{App, statement::operator::{Operator, OperatorNode, binary::{BinOp}}};


mod app;
fn main() {
    let app = App::new(String::from("(var l)(g(x) | f(x))")).unwrap();
    if let Operator::Value { op } = app.program.ast.value() {
        println!("{:#?}", op.values);
    }
}