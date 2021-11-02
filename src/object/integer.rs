/**********************************************
  > File Name		: object/integer.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Tue Nov  2 20:48:42 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use super::object::Object;

#[derive(Debug, Copy, Clone)]
pub struct Integer {
    val: i32
}

impl Integer {
    pub fn get(&self) -> i32 {
        self.val
    }

    pub fn new(val: i32) -> Self {
        Integer {
            val
        }
    }
}

impl Object for Integer {
    pub fn add(&self, x: Integer) -> Integer {
        Integer {
            val: self.val + x.get()
        }
    }

    pub fn sub(&self, x: Integer) -> Integer {
        Integer {
            val: self.val - x.get()
        }
    }

    pub fn mul(&self, x: Integer) -> Integer {
        Integer {
            val: self.val * x.get()
        }
    }

    pub fn div(&self, x: Integer) -> Integer {
        if x.get() == 0 {
            panic!("divide by zero");
        }
        Integer {
            val: self.val / x.get()
        }
    }
}
