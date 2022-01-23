/**********************************************
  > File Name		: objects/function.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Mon Nov  8 09:54:04 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use crate::code::binary_file_parser::CodeObject;
use super::{object::Object, string::Str};

pub type native_func = dyn Fn(Vec<Object>) -> Option<Object>;

#[derive(Clone)]
pub struct Function {
    pub func_codes: CodeObject,
    pub func_name: Str,
    pub flags: u32,
    pub defaults: Option<Vec<Object>>,
}

impl Function {
    pub fn new(codes: CodeObject, defaults: Option<Vec<Object>>) -> Self {
        Self {
            func_name: codes.co_name.clone(),
            func_codes: codes,
            flags: 0,
            defaults,
        }
    }
}

#[derive(Clone)]
pub struct NativeFunction {
    pub nfp: &'static native_func,
    pub func_name: Str
}

impl NativeFunction {
    pub fn new(nfp: &'static native_func, func_name: &Str) -> Self {
        Self {
            nfp,
            func_name: func_name.clone()
        }
    }

    pub fn call(&self, args: Vec<Object>) -> Option<Object> {
        let nfp = self.nfp;
        nfp(args)
    }
}

pub fn len(args: Vec<Object>) -> Option<Object> {
    Some(Object::Int(args[0].len()))
}
