/**********************************************
  > File Name		: object/integer.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Tue Nov  2 20:48:42 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use super::object::Object;
use super::statics;

macro_rules! ptr_to_ref {
    ($ptr:ident) => {{
        unsafe {
            match $ptr.cast::<Self>().as_ref() {
                None => {return None;},
                Some(r) => r
            }
        }
    }}
}

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

    pub const fn new_stack(val: i32) -> Self {
        Self {val}
    }
}

impl Object for Integer {
    fn print(&self) {
        colour::blue_ln!("{}", self.val);
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

    fn mul(&self, _rhs: *const dyn Object) -> Option<*mut dyn Object> {
        let _rhs_ref:&Self = unsafe {
            match _rhs.cast::<Self>().as_ref() {
                None => {return None;},
                Some(r) => r
            }
        };
        Some(Self::new_ptr(self.val * _rhs_ref.get()))
    }

    fn div(&self, _rhs: *const dyn Object) -> Option<*mut dyn Object> {
        let _rhs_ref:&Self = unsafe {
            match _rhs.cast::<Self>().as_ref() {
                None => {return None;},
                Some(r) => r
            }
        };
        if _rhs_ref.get() == 0 {
            panic!("divide by zero");
        }
        Some(Self::new_ptr(self.val / _rhs_ref.get()))
    }

    fn greater(&self, _rhs: *const dyn Object) -> Option<*mut dyn Object> {
        let _rhs_ref:&Self = unsafe {
            match _rhs.cast::<Self>().as_ref() {
                None => {return None;},
                Some(r) => r
            }
        };
        if self.val > _rhs_ref.get() {
            Some(statics::TRUE)
        } else {
            Some(statics::FALSE)
        }
    }

    fn less(&self, _rhs: *const dyn Object) -> Option<*mut dyn Object> {
        let _rhs_ref = ptr_to_ref!(_rhs);
        if self.val < _rhs_ref.get() {
            Some(statics::TRUE)
        } else {
            Some(statics::FALSE)
        }
    }

    fn equal(&self, _rhs: *const dyn Object) -> Option<*mut dyn Object> {
        let _rhs_ref = ptr_to_ref!(_rhs);
        if self.val == _rhs_ref.get() {
            Some(statics::TRUE)
        } else {
            Some(statics::FALSE)
        }
    }

    fn ne(&self, _rhs: *const dyn Object) -> Option<*mut dyn Object> {
        let _rhs_ref = ptr_to_ref!(_rhs);
        if self.val != _rhs_ref.get() {
            Some(statics::TRUE)
        } else {
            Some(statics::FALSE)
        }
    }

    fn le(&self, _rhs: *const dyn Object) -> Option<*mut dyn Object> {
        let _rhs_ref = ptr_to_ref!(_rhs);
        if self.val <= _rhs_ref.get() {
            Some(statics::TRUE)
        } else {
            Some(statics::FALSE)
        }
    }

    fn ge(&self, _rhs: *const dyn Object) -> Option<*mut dyn Object> {
        let _rhs_ref = ptr_to_ref!(_rhs);
        if self.val >= _rhs_ref.get() {
            Some(statics::TRUE)
        } else {
            Some(statics::FALSE)
        }
    }
}

