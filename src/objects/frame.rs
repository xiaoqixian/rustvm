/**********************************************
  > File Name		: objects/frame.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Sat Nov  6 22:00:50 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use super::object::Object;
use std::collections::HashMap;
use crate::code::binary_file_parser::CodeObject;
use crate::{as_ref};

pub trait MultiNew<T> {
    type Output;
    fn new(codes: T) -> Self::Output;
}


pub struct Block {
    pub _type: u8,
    pub target: usize,
    pub level: usize
}

impl Block {
    pub fn new(_type:u8, target:usize, level:usize) -> Self {
        Self {
            _type,
            target,
            level
        }
    }
}


pub struct Frame {
    pub pc: usize,
    pub stack: Vec<*mut dyn Object>,
    pub loop_stack: Vec<Block>,
    pub locals: HashMap<*mut dyn Object, *mut dyn Object>,
    pub codes: Box<CodeObject>,
    pub sender: Option<Box<Self>>,
}

impl Frame {
    pub fn has_more_codes(&self) -> bool {
        self.pc < self.codes.bytecodes.len()
    }
    
    pub fn get_opcode(&mut self) -> u8 {
        let res = self.codes.bytecodes[self.pc];
        self.pc += 1;
        res
    }

    pub fn get_oparg(&mut self) -> usize {
        let b1 = (self.codes.bytecodes[self.pc] & 0xff) as usize;
        self.pc += 1;
        let b2 = (self.codes.bytecodes[self.pc] & 0xff) as usize;
        self.pc += 1;
        b2 << 8 | b1
    }
}

//this associated functions is used for modules only.
impl MultiNew<*mut CodeObject> for Frame {
    type Output = Box<Frame>;
    fn new(codes: *mut CodeObject) -> Self::Output {
        Box::new(Frame {
            pc: 0,
            stack: Vec::new(),
            loop_stack: Vec::new(),
            locals: HashMap::new(),
            codes: unsafe {Box::from_raw(codes)},
            sender: None
        })
    }
}

//impl MultiNew<*const FunctionObject

impl Object for Frame {}
