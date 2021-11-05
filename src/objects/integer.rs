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

    pub fn new(val: i32) -> Box<Self> {
        Box::new(Integer {val})
    }

    pub fn new_ptr(val: i32) -> *mut Self {
        Box::into_raw(Self::new(val))
    }
}

impl Object for Integer {
    fn print(&self) {
        colour::red_ln!("{}", self.val);
    }

    fn add(&self, _rhs: *const dyn Object) -> Option<*mut dyn Object> {
        let _rhs_ref:&Self = unsafe {
            match _rhs.cast::<Self>().as_ref() {
                None => {return None;},
                Some(r) => r
            }
        };
        Some(Self::new_ptr(self.val + _rhs_ref.get()))
    }

    fn sub(&self, _rhs: *const dyn Object) -> Option<*mut dyn Object> {
        let _rhs_ref:&Self = unsafe {
            match _rhs.cast::<Self>().as_ref() {
                None => {return None;},
                Some(r) => r
            }
        };
        Some(Self::new_ptr(self.val - _rhs_ref.get()))
    }
}

