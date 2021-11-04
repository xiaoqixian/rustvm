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
    val: i32,
}

impl Integer {
    pub fn get(&self) -> i32 {
        self.val
    }

    pub fn new(val: i32) -> Box<dyn Object> {
        Box::new(Integer {val})
    }
}

impl Object for Integer {
    fn print(&self) {
        println!("{}", self.val);
    }

    fn add(&self, _rhs: &dyn Object) -> Option<Box<dyn Object>> {
        let _rhs_ref = super::as_ref::<Self>(_rhs);
        Some(Self::new(self.val + _rhs_ref.get()))
    }

    fn sub(&self, _rhs: &dyn Object) -> Option<Box<dyn Object>> {
        let _rhs_ref = super::as_ref::<Self>(_rhs);
        Some(Self::new(self.val - _rhs_ref.get()))
    }
}

