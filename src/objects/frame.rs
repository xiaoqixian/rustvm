/**********************************************
  > File Name		: objects/frame.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Sat Nov  6 22:00:50 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use super::{object::Object, function::Function, string::Str};
use crate::code::binary_file_parser::CodeObject;

pub trait MultiNew<T> {
    type Output;
    fn new(codes: T) -> Self::Output;
    fn new_with_sender(codes: T, sender: Rc<Self::Output>) -> Self::Output;
}

#[derive(Clone, Copy, Debug)]
pub struct Block {
    pub _type: u8,
    pub target: usize,
    pub level: usize
}

impl Block {
    pub fn new(_type: u8, target: usize, level: usize) -> Self {
        Self {
            _type,
            target,
            level
        }
    }
}

#[derive(Clone)]
pub struct Frame {
    pub pc: RefCell<usize>,
    pub stack: RefCell<Vec<Object>>,
    pub loop_stack: RefCell<Vec<Block>>,
    pub locals: RefCell<HashMap<Str, Object>>,
    //pub codes: Box<CodeObject>,
    pub codes: CodeObject,
    //pub sender: Option<Box<Self>>,
    pub sender: Option<Rc<Self>>
}

impl Frame {
    #[inline]
    pub fn get_pc(&self) -> usize {
        *self.pc.borrow()
    }

    #[inline]
    pub fn set_pc(&self, pc: usize) {
        *self.pc.borrow_mut() = pc;
    } 

    #[inline]
    pub fn add_pc(&self) {
        *self.pc.borrow_mut() += 1;
    }

    #[inline]
    pub fn add_pc_n(&self, n: usize) {
        *self.pc.borrow_mut() += n;
    }

    pub fn has_more_codes(&self) -> bool {
        self.get_pc() < self.codes.code_length()
    }
    
    pub fn get_opcode(&self) -> u8 {
        let res = self.codes.bytecodes[self.get_pc()];
        self.add_pc();
        res
    }

    pub fn get_oparg(&mut self) -> usize {
        let b1 = (self.codes.bytecodes[self.get_pc()] & 0xff) as usize;
        self.add_pc();
        let b2 = (self.codes.bytecodes[self.get_pc()] & 0xff) as usize;
        self.add_pc();
        b2 << 8 | b1
    }
}

//this associated functions is used for modules only.
impl MultiNew<CodeObject> for Frame {
    type Output = Frame;
    fn new(codes: CodeObject) -> Self::Output {
        Frame {
            pc: RefCell::new(0),
            stack: RefCell::new(Vec::new()),
            loop_stack: RefCell::new(Vec::new()),
            locals: RefCell::new(HashMap::new()),
            codes,
            sender: None
        }
    }

    fn new_with_sender(codes: CodeObject, sender: Rc<Self::Output>) -> Self::Output {
        Frame {
            pc: RefCell::new(0),
            stack: RefCell::new(Vec::new()),
            loop_stack: RefCell::new(Vec::new()),
            locals: RefCell::new(HashMap::new()),
            codes,
            sender: Some(sender) 
        }
    }
}

impl MultiNew<Function> for Frame {
    type Output = Frame;
    fn new(func: Function) -> Self::Output {
        Frame {
            pc: RefCell::new(0),
            stack: RefCell::new(Vec::new()),
            loop_stack: RefCell::new(Vec::new()),
            locals: RefCell::new(HashMap::new()),
            codes: func.func_codes,
            sender: None
        }
    }

    fn new_with_sender(func: Function, sender: Rc<Self::Output>) -> Self::Output {
        Frame {
            pc: RefCell::new(0),
            stack: RefCell::new(Vec::new()),
            loop_stack: RefCell::new(Vec::new()),
            locals: RefCell::new(HashMap::new()),
            codes: func.func_codes,
            sender: Some(sender) 
        }
    }
}

