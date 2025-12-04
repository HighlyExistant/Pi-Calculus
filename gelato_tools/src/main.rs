use std::{alloc::Layout, collections::BinaryHeap, ffi::CStr};

use crate::{allocators::{arena::MemoryArena, pool::{Pool, PoolAllocator}}, nodes::BinaryTreeNode};

mod allocators;
mod nodes;
fn main() {
    let mut root = BinaryTreeNode::new("A");
    let left = BinaryTreeNode::new("B");
    let right = BinaryTreeNode::new("C");
    let right_left = BinaryTreeNode::new("D");
    let right_right = BinaryTreeNode::new("E");
    right.set_left(Some(right_left));
    right.set_right(Some(right_right));
    root.set_left(Some(left));
    root.set_right(Some(right));
    println!("{}", root);
    root.rotate_left();
    println!("{}", root);
    root.rotate_left();
    println!("{}", root);
}
