/**********************************************
  > File Name		: object/integer.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Tue Nov  2 20:48:42 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::rc::Rc;
use std::any::Any;
use std::fmt::{Debug, Display};

use super::object::Object as ObjectTrait;
use super::Object;
use super::klass::Klass;
//use crate::errors::Errors;


#[derive(Debug, Copy, Clone)]
pub struct Integer {
    inner: i32,
}

impl Integer {
    pub fn into(&self) -> i32 {
        self.inner
    }

    pub fn new(inner: i32) -> Object {
        Rc::new(Integer {inner})
    }
}

impl ObjectTrait for Integer {
    fn as_any(&self) -> &dyn Any {self}

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn klass(&self) -> Klass {
        Klass::IntegerKlass
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
