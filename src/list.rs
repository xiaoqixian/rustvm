/**********************************************
  > File Name		: list.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Sat 22 Jan 2022 03:54:59 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

/**
 * A python list
 */

use super::object::Object;

pub struct List {
    pub inner: Vec<Object>
}

impl List {
    pub fn new() -> Self {
        Self {
            inner: Vec::new()
        }
    }

    pub fn with_capacity(size: usize) -> Self {
        Self {
            inner: Vec::with_capacity(size)
        }
    }
}
