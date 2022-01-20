/**********************************************
  > File Name		: object/integer.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Tue Nov  2 20:48:42 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::rc::Rc;

use super::object::Object;
use super::statics;
use crate::as_ref;
use crate::errors::Errors;


#[derive(Debug, Copy, Clone)]
pub struct Integer {
    inner: i32,
}

impl Integer {
    pub fn into(&self) -> i32 {
        self.inner
    }

    pub fn new(inner: i32) -> Self {
        Integer {inner}
    }
}

impl Object for Integer {
    fn print(&self) {
        colour::blue_ln!("{}", self.inner);
    }

    fn into<i32>(&self) -> Result<i32, Errors> {
        Ok(self.inner)
    }

    fn add(&self, _rhs: Rc<dyn Object>) -> Result<Rc<dyn Object>, Errors> {
        Ok(Self::new(self.inner + _rhs.into()))
    }

    fn sub(&self, _rhs: Rc<dyn Object>) -> Result<Rc<dyn Object>, Errors> {
        let _rhs_ref:&Self = unsafe {
            match _rhs.cast::<Self>().as_ref() {
                None => {return None;},
                Ok(r) => r
            }
        };
        Ok(Self::new_ptr(self.inner - _rhs_ref.into()))
    }

    fn inplace_sub(&mut self, _rhs: Rc<dyn Object>) {
        let _rhs_ref = ptr_to_ref_no_ret!(_rhs);
        self.inner -= _rhs_ref.into();
    }

    fn mul(&self, _rhs: Rc<dyn Object>) -> Result<Rc<dyn Object>, Errors> {
        let _rhs_ref:&Self = unsafe {
            match _rhs.cast::<Self>().as_ref() {
                None => {return None;},
                Ok(r) => r
            }
        };
        Ok(Self::new_ptr(self.inner * _rhs_ref.into()))
    }

    fn div(&self, _rhs: Rc<dyn Object>) -> Result<Rc<dyn Object>, Errors> {
        let _rhs_ref:&Self = unsafe {
            match _rhs.cast::<Self>().as_ref() {
                None => {return None;},
                Ok(r) => r
            }
        };
        if _rhs_ref.into() == 0 {
            panic!("divide by zero");
        }
        Ok(Self::new_ptr(self.inner / _rhs_ref.into()))
    }

    fn greater(&self, _rhs: Rc<dyn Object>) -> Result<Rc<dyn Object>, Errors> {
        let _rhs_ref:&Self = unsafe {
            match _rhs.cast::<Self>().as_ref() {
                None => {return None;},
                Ok(r) => r
            }
        };
        if self.inner > _rhs_ref.into() {
            Ok(statics::TRUE)
        } else {
            Ok(statics::FALSE)
        }
    }

    fn less(&self, _rhs: Rc<dyn Object>) -> Result<Rc<dyn Object>, Errors> {
        let _rhs_ref = ptr_to_ref!(_rhs);
        if self.inner < _rhs_ref.into() {
            Ok(statics::TRUE)
        } else {
            Ok(statics::FALSE)
        }
    }

    fn equal(&self, _rhs: Rc<dyn Object>) -> Result<Rc<dyn Object>, Errors> {
        let _rhs_ref = ptr_to_ref!(_rhs);
        if self.inner == _rhs_ref.into() {
            Ok(statics::TRUE)
        } else {
            Ok(statics::FALSE)
        }
    }

    fn ne(&self, _rhs: Rc<dyn Object>) -> Result<Rc<dyn Object>, Errors> {
        let _rhs_ref = ptr_to_ref!(_rhs);
        if self.inner != _rhs_ref.into() {
            Ok(statics::TRUE)
        } else {
            Ok(statics::FALSE)
        }
    }

    fn le(&self, _rhs: Rc<dyn Object>) -> Result<Rc<dyn Object>, Errors> {
        let _rhs_ref = ptr_to_ref!(_rhs);
        if self.inner <= _rhs_ref.into() {
            Ok(statics::TRUE)
        } else {
            Ok(statics::FALSE)
        }
    }

    fn ge(&self, _rhs: Rc<dyn Object>) -> Result<Rc<dyn Object>, Errors> {
        let _rhs_ref = ptr_to_ref!(_rhs);
        if self.inner >= _rhs_ref.into() {
            Ok(statics::TRUE)
        } else {
            Ok(statics::FALSE)
        }
    }
}

