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
use crate::runtime::interpreter::{Block};

pub struct Frame {
    pc: usize,
    _stack: Vec<*mut dyn Object>,
    _loop_stack: Vec<Block>,
    pub _consts: &'static Vec<*mut dyn Object>,
    _locals: HashMap<*mut dyn Object, *mut dyn Object>,
    _names: Vec<*mut dyn Object>,
}
