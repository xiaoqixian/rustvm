/**********************************************
  > File Name		: objects/code_object.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Fri 28 Jan 2022 08:48:32 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::any::Any;

use crate::objects::object::{Object, ObjRef};
use crate::objects::string::Str;
use crate::objects::klass::{Klass, KlassRef};
use crate::cast;

pub static CODE_KLASS_INSTANCE: CodeKlass = CodeKlass {mod_str: "CodeObject"};

#[derive(Clone)]
pub struct CodeObject {
    pub argcount: usize,
    pub nlocals: usize,
    pub stacksize: usize,
    pub flags: u32,
    pub bytecodes: ObjRef,
    pub consts: Vec<ObjRef>,
    pub names: Vec<ObjRef>,
    pub var_names: Vec<ObjRef>,
    pub free_vars: Vec<ObjRef>,
    pub cell_vars: Vec<ObjRef>,
    pub file_name: ObjRef,
    pub co_name: ObjRef,
    pub line_number: u32,
    pub notable: ObjRef,
    pub klass: &'static CodeKlass
}

#[derive(Debug, Clone, Copy)]
pub struct CodeKlass {
    mod_str: &'static str
}

impl Klass for CodeKlass {
    fn as_any(&self) -> &dyn Any {self}
}

impl CodeObject {
    #[inline]
    pub fn code_length(&self) -> usize {
        cast!(self.bytecodes, Str).len() 
    }

    #[inline]
    pub fn get_opcode(&self, pc: usize) -> u8 {
        cast!(self.bytecodes, Str)[pc]
    }
}

impl Object for CodeObject {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn klass(&self) -> KlassRef {
        self.klass
    }
}

impl std::fmt::Display for CodeObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CodeObject")
    }
}

impl std::fmt::Debug for CodeObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CodeObject")
    }
}
