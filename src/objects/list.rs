/**********************************************
  > File Name		: objects/list.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Thu 04 Nov 2021 04:37:58 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::rc::Rc;

use super::object::Object;

#[derive(Clone, Debug)]
pub struct List {
    pub inner: Vec<Rc<Object>> //make inner public is a lazy way to avoid implementing all methods of Vec<Object>
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

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("[");
        if self.inner.len() > 0 {
            s.push_str(format!("{}", &self.inner[0]).as_str());
        }

        for i in 1..self.inner.len() {
            s.push_str(format!(", {}", &self.inner[i]).as_str());
        }

        write!(f, "{}]", s)
    }
}

impl std::convert::From<Vec<Rc<Object>>> for List {
    fn from(inner: Vec<Rc<Object>>) -> Self {
        Self {
            inner
        }
    }
}

impl std::convert::From<std::collections::VecDeque<Rc<Object>>> for List {
    fn from(v: std::collections::VecDeque<Rc<Object>>) -> Self {
        Self {
            inner: Vec::from(v)
        }
    }
}
