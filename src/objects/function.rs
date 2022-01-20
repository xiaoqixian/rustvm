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

#[derive(Clone)]
pub struct Function {
    pub func_codes: CodeObject,
    pub func_name: Str,
    pub flags: u32
}

impl Function {
    pub fn new(codes: CodeObject) -> Self {
        Self {
            func_name: codes.co_name.clone(),
            func_codes: codes,
            flags: 0
        }
    }
}

