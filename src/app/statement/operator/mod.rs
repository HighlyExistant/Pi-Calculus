use std::{cell::UnsafeCell, fmt::Debug, rc::Rc};

use gelato_parser::tokens::{Punct, Token, Tokens};

use crate::app::{error::{PiResult, Result}, statement::{operator::{binary::BinOp, unary::UniOp}, value::{Value, Values}}};

pub mod unary;
pub mod binary;
#[derive(Clone)]
pub struct OperatorNode {
    value: Rc<UnsafeCell<Operator>>,
}

impl Debug for OperatorNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:#?}", self.value()))
    }
}

impl OperatorNode {
    pub fn new(op: Operator) -> Self {
        Self { value: Rc::new(UnsafeCell::new(op)) }
    }
    /// After a series of values, you will encounter an operator. These operators can only
    /// be continued with this function
    fn parse_next_with_prev(tokens: &mut Tokens, values: Values, op: Punct, in_group: bool) -> Result<Self> {
        let binop = match op.punct() {
            '|' => {
                BinOp::Concurrency
            }
            '+' => {
                BinOp::Choice
            }
            '.' => {
                BinOp::Sequential
            }
            ')' => {
                if in_group {
                    return Ok(OperatorNode::new(Operator::Value { op: values }));
                } else {
                    return Err(PiResult::UnexpectedToken("'|', '+' or '.'"));
                }
            }
            _ => panic!("??? Not supposed to happen")
        };
        Ok(Self::new(Operator::Binary { 
            ty: binop, 
            lhs: OperatorNode::new(Operator::Value { op: values }), 
            rhs: OperatorNode::parse_next(tokens, in_group)? 
        }))
    }
    pub fn parse_next(tokens: &mut Tokens, in_group: bool) -> Result<Self> {
        let first = if let Some(token) = tokens.next() {
            token
        } else {
            return Ok(Self::nil());
        };
        
        match first {
            Token::Punct(punct) => Self::match_punct(tokens, &punct, in_group),
            _ => { // if no other value, then it must be some kind of value
                let (values, op) = Self::match_values(tokens, first, in_group)?;
                if let Some(operator) = op {
                    Self::parse_next_with_prev(tokens, values, operator, in_group)
                } else {
                    Ok(Self::new(Operator::Value { op: values }))
                }
            }
        }
    }
    pub fn parse_next_prev(tokens: &mut Tokens, first: Token, in_group: bool) -> Result<Self> {
        match first {
            Token::Punct(punct) => Self::match_punct(tokens, &punct, in_group),
            _ => { // if no other value, then it must be some kind of value
                let (values, op) = Self::match_values(tokens, first, in_group)?;
                if let Some(operator) = op {
                    Self::parse_next_with_prev(tokens, values, operator, in_group)
                } else {
                    Ok(Self::new(Operator::Value { op: values }))
                }
            }
        }
    }
    fn match_punct(tokens: &mut Tokens, punct: &Punct, in_group: bool) -> Result<Self> {
        match punct.punct() {
            '!' => { // Only operator to look out for is replication
                let next = OperatorNode::parse_next(tokens, in_group)?;
                return Ok(OperatorNode::new(Operator::Unary { ty: UniOp::Replication, val: next }));
            }
            _ => { // if no other value, then it must be some kind of value
                let (values, op) = Self::match_values(tokens, Token::Punct(punct.clone()), in_group)?;
                if let Some(operator) = op {
                    Self::parse_next_with_prev(tokens, values, operator, in_group)
                } else {
                    Ok(Self::new(Operator::Value { op: values }))
                }
            }
        }
    }
    fn is_binop(token: &Token) -> bool {
        if let Token::Punct(p) = token {
            p.punct() == '|' ||
            p.punct() == '+' ||
            p.punct() == '.'
        } else {
            false
        }
    }
    fn match_values(tokens: &mut Tokens, first_element: Token, in_group: bool) -> Result<(Values, Option<Punct>)> {
        let mut values = Values::new();
        values.push_value(Value::parse_next_start(tokens, first_element)?);
        while let Some(token) = tokens.next() {
            if in_group {
                if let Some(punct) = token.get_punct() {
                    if punct.punct() == ')' {
                        return Ok((values, Some(token.get_punct().unwrap())));
                    }
                }
            }
            if Self::is_binop(&token) {
                return Ok((values, Some(token.get_punct().unwrap())));
            } else {
                values.push_value(Value::parse_next_start(tokens, token)?);
            }
        }
        Ok((values, None))
    }
    pub fn nil() -> Self {
        Self::new(Operator::nil())
    }
    pub fn debug(debug: usize) -> Self {
        Self::new(Operator::Value { op: Values { values: vec![Value::Debug(debug)] } })
    }
    pub fn precedence(&self) -> usize {
        self.value().precedence()
    }
    pub fn value(&self) -> &mut Operator {
        unsafe { self.value.get().as_mut().unwrap() }
    }
    pub fn is_leaf(&self) -> bool {
        if let Operator::Value { op } = self.value() {
            true
        } else {
            false
        }
    }
    pub fn is_unary(&self) -> bool {
        if let Operator::Unary { ty, val } = self.value() {
            true
        } else {
            false
        }
    }
    pub fn is_binary(&self) -> bool {
        if let Operator::Binary { ty, lhs, rhs } = self.value() {
            true
        } else {
            false
        }
    }
    pub fn right(&self) -> Option<&OperatorNode> {
        match self.value() {
            Operator::Binary { ty, lhs, rhs } => {
                Some(rhs)
            }
            Operator::Unary { ty, val } => {
                Some(val)
            }
            _ => None
        }
    }
    pub fn left(&self) -> Option<&OperatorNode> {
        match self.value() {
            Operator::Binary { ty, lhs, rhs } => {
                Some(lhs)
            }
            Operator::Unary { ty, val } => {
                Some(val)
            }
            _ => None
        }
    }
    
    pub fn right_mut(&self) -> Option<&mut OperatorNode> {
        match self.value() {
            Operator::Binary { ty, lhs, rhs } => {
                Some(rhs)
            }
            Operator::Unary { ty, val } => {
                Some(val)
            }
            _ => None
        }
    }
    pub fn left_mut(&self) -> Option<&mut OperatorNode> {
        match self.value() {
            Operator::Binary { ty, lhs, rhs } => {
                Some(lhs)
            }
            Operator::Unary { ty, val } => {
                Some(val)
            }
            _ => None
        }
    }
    pub fn set(&self, op: OperatorNode) {
        match self.value() {
            Operator::Unary { ty, val } => {
                *val = op.clone();
            }
            _ => {}
        }
    }
    pub fn rotate_left(&mut self) {
        let root = self.value.clone(); // save value
        match self.value().clone() {
            Operator::Binary { ty, lhs, rhs } => {
                if !rhs.is_binary() {
                    return; // Nothing to be done
                }
                let t1 = rhs.left().unwrap().clone();
                self.value = rhs.value.clone();
                self.left_mut().unwrap().value = root.clone();
                self.left_mut().unwrap().right_mut().unwrap().value = t1.value.clone();
            }
            Operator::Unary { ty, val } => {
                if !val.is_binary() {
                    return; // Nothing to be done
                }
                let lhs = val.left().unwrap().clone();
                self.set(lhs.clone());
                self.value = val.value.clone(); // make root
                self.left_mut().unwrap().value = root; // make prev root = left child

            }
            _ => {}
        }
    }
    // returns whether it bubbled
    pub fn bubble(&mut self) -> bool {
        let right = if let Some(r) = self.right() {
            r
        } else {
            return false;
        };
        let precedence = self.precedence();
        self.right_mut().unwrap().bubble();
        
        if precedence < right.precedence() {
            self.rotate_left();
            return true;
        }
        false
    }
}

#[derive(Debug, Clone)]
pub enum Operator {
    Unary {
        ty: UniOp,
        val: OperatorNode,
    },
    Binary {
        ty: BinOp,
        lhs: OperatorNode,
        rhs: OperatorNode
    },
    Value {
        op: Values
    }
}

impl Operator {
    pub fn nil() -> Self {
        Self::Value { op: Values { values: vec![Value::Nil] } }
    }
    pub fn precedence(&self) -> usize {
        match self {
            Self::Binary { ty, lhs, rhs } => {
                ty.precedence()
            }
            Operator::Unary { ty, val } => {
                ty.precedence()
            }
            Operator::Value { op } => {
                0
            }
        }
    }
}