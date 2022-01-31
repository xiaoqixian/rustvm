/**********************************************
  > File Name		: code/code_object.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Sun Jan 30 14:56:23 2022
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::fmt::{Debug, Display};

use crate::objects::{Object, klass::Klass};
use crate::objects::object::Object as ObjectTrait;

type CodeBuffer = crate::objects::string::Str; //use Str to store a piece of code.

#[derive(Clone)]
pub struct CodeObject {
    pub argcount: usize,
    pub nlocals: usize,
    pub stacksize: usize,
    pub flags: u32,
    pub bytecodes: Object,
    pub consts: Vec<Object>,
    pub names: Vec<Object>,
    pub var_names: Vec<Object>,
    pub free_vars: Vec<Object>,
    pub cell_vars: Vec<Object>,
    pub file_name: Object,
    pub co_name: Object,
    pub line_number: u32,
    pub notable: Object,
}

impl ObjectTrait for CodeObject {
    fn as_any(&self) -> &dyn std::any::Any {self}

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn klass(&self) -> Klass {
        Klass::CodeKlass
    }
}

impl CodeObject {
    #[inline]
    pub fn code_length(&self) -> usize {
        self.bytecodes.as_any().downcast_ref::<CodeBuffer>().unwrap().len() as usize
    }

    #[inline]
    pub fn get_opcode(&self, pc: usize) -> u8 {
        self.bytecodes.as_any().downcast_ref::<CodeBuffer>().unwrap()[pc]
    }
}

impl Debug for CodeObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<code_object {}>", self.co_name)
    }
}

impl Display for CodeObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.co_name)
    }
}
