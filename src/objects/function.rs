/**********************************************
  > File Name		: objects/function.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Mon Nov  8 09:54:04 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use crate::code::binary_file_parser::{CodeObject};
use super::{object::Object, string::Str};
use std::collections::HashMap;

pub struct Function {
    pub func_codes: *mut CodeObject,
    pub func_name: Str,
    pub flags: u32
}

impl Function {
    pub fn new(codes: *mut CodeObject) -> *mut dyn Object {
        Box::into_raw(Box::new(Self {
            func_codes: codes,
            func_name: unsafe {(*(*codes).co_name).clone()},
            flags: 0
        }))
    }
}

impl Object for Function {
    fn print(&self) {
        colour::blue_ln!("<function: {}>", self.func_name.get().unwrap());
    }
}
