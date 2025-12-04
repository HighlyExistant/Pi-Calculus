use std::{alloc::Layout, cell::{Cell, RefCell, UnsafeCell}, fmt::Display, ptr::NonNull, rc::Rc};

use crate::allocators::{arena::ArenaAllocator, pool::{MemoryPool, Pool}};
#[derive(Debug, Clone)]
struct BinaryTreeNodeInternal<T> {
    value: T,
    left: Option<BinaryTreeNode<T>>,
    right: Option<BinaryTreeNode<T>>,
}

impl<T> BinaryTreeNodeInternal<T> {
    pub fn new(value: T) -> Self {
        Self { value, left: None, right: None }
    }
    
    fn print_helper(root: Option<&BinaryTreeNode<T>>, indent: &mut String, last: bool, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        where T: Display {
        if let Some(root) = root {
            f.write_str(format!("{}", indent).as_str())?;
            if last {
                f.write_str("R----")?;
                indent.push_str("   ");
            } else {
                f.write_str("L----")?;
                indent.push_str("|  ");
            }
            let node = root;
            let value = format!("{}", root.value());
            f.write_str(format!("{}\n", value).as_str())?;

            let mut str1 = indent.clone();
            let mut str2 = indent.clone();
            Self::print_helper(node.left(), &mut str1, false, f)?;
            Self::print_helper(node.right(), &mut str2, true, f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct BinaryTreeNode<T> {
    root: Rc<UnsafeCell<BinaryTreeNodeInternal<T>>>,
}

impl<T: Display> Display for BinaryTreeNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut indent = String::new();
        Self::print_helper(Some(self), &mut indent, true, f)
    }
}
impl<T> BinaryTreeNode<T> {
    pub fn new(value: T) -> Self {
        Self { root: Rc::new(UnsafeCell::new(BinaryTreeNodeInternal::new(value))) }
    }
    fn root(&self) -> &mut BinaryTreeNodeInternal<T> {
        unsafe { self.root.get().as_mut().unwrap() }
    }
    pub fn value(&self) -> &T {
        &self.root().value
    }
    pub fn set_left(&self, value: Option<BinaryTreeNode<T>>) {
        self.root().left = value;
    }
    pub fn set_right(&self, value: Option<BinaryTreeNode<T>>) {
        self.root().right = value;
    }
    pub fn left(&self) -> Option<&BinaryTreeNode<T>> {
        self.root().left.as_ref()
    }
    pub fn right(&self) -> Option<&BinaryTreeNode<T>> {
        self.root().right.as_ref()
    }
    pub fn left_mut(&self) -> Option<&mut BinaryTreeNode<T>> {
        self.root().left.as_mut()
    }
    pub fn right_mut(&self) -> Option<&mut BinaryTreeNode<T>> {
        self.root().right.as_mut()
    }
    pub fn rotate_left(&mut self) 
        where T: Clone {
        let prev_root = self.clone();
        let right = if let Some(right) = prev_root.right() {
            right
        } else { // If there is no right node, there can be no rotation
            return;
        };
        let right_left = right.root().left.clone();
        right.set_left(Some(prev_root.clone()));
        self.root = right.root.clone();
        prev_root.set_right(right_left);
    }
    pub fn rotate_right(&mut self) 
        where T: Clone {
        let prev_root = self.clone();
        let left = if let Some(left) = prev_root.left() {
            left
        } else { // If there is no right node, there can be no rotation
            return;
        };
        let left_right = left.root().right.clone();
        left.set_right(Some(prev_root.clone()));
        self.root = left.root.clone();
        prev_root.set_left(left_right);
    }
    
    fn print_helper(root: Option<&BinaryTreeNode<T>>, indent: &mut String, last: bool, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        where T: Display {
        if let Some(root) = root {
            f.write_str(format!("{}", indent).as_str())?;
            if last {
                f.write_str("R----")?;
                indent.push_str("   ");
            } else {
                f.write_str("L----")?;
                indent.push_str("|  ");
            }
            let node = root;
            let value = format!("{}", root.value());
            f.write_str(format!("{}\n", value).as_str())?;

            let mut str1 = indent.clone();
            let mut str2 = indent.clone();
            Self::print_helper(node.left(), &mut str1, false, f)?;
            Self::print_helper(node.right(), &mut str2, true, f)?;
        }
        Ok(())
    }
}