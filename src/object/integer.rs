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

    fn get_ref<'a>(_rhs: &'a dyn Object) -> &'a Self {
        let _rhs_p = _rhs as *const _ as *const Self;
        unsafe {
            &(*_rhs_p)
        }
    }
}

impl Object for Integer {
    fn add(&self, _rhs: &dyn Object) -> Option<Box<dyn Object>> {
        let _rhs_ref = Self::get_ref(_rhs);
        Some(Self::new(self.val + _rhs_ref.get()))
    }

    fn sub(&self, _rhs: &dyn Object) -> Option<Box<dyn Object>> {
        let _rhs_ref = Self::get_ref(_rhs);
        Some(Self::new(self.val - _rhs_ref.get()))
    }
}

