/**********************************************
  > File Name		: object/object.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Tue Nov  2 15:48:30 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

pub trait Object {
    //fn get_klass(&self) -> Option<*const Klass>;

    fn add(&self, _rhs: *const dyn Object) -> Option<*mut dyn Object> {
        None
    }

    fn sub(&self, _rhs: *const dyn Object) -> Option<*mut dyn Object> {
        None
    }

    fn print(&self) {
        panic!("this type does not impl print method");
    }

    fn mul(&self, _rhs: *const dyn Object) -> Option<*mut dyn Object> {
        None
    }

    fn div(&self, _rhs: *const dyn Object) -> Option<*mut dyn Object> {
        None
    }

    fn module(&self, _rhs: *const dyn Object) -> Option<*mut dyn Object> {
        None
    }

    fn greater(&self, _rhs: *const dyn Object) -> Option<*mut dyn Object> {
        None
    }

    fn less(&self, _rhs: *const dyn Object) -> Option<*mut dyn Object> {
        None
    }

    fn equal(&self, _rhs: *const dyn Object) -> Option<*mut dyn Object> {
        None
    }

    fn ne(&self, _rhs: *const dyn Object) -> Option<*mut dyn Object> {
        None
    }

    fn ge(&self, _rhs: *const dyn Object) -> Option<*mut dyn Object> {
        None
    }

    fn le(&self, _rhs: *const dyn Object) -> Option<*mut dyn Object> {
        None
    }

    fn subscr(&self, _rhs: *const dyn Object) -> Option<*mut dyn Object> {
        None
    }

    fn contains(&self, _rhs: *const dyn Object) -> Option<*mut dyn Object> {
        None
    }
}

//PyNone implements single instance pattern.
pub struct PyNone {}

impl Object for PyNone {
    fn print(&self) {
        colour::blue_ln!("None");
    } 
}

pub struct PyTrue {pub val:i32}

impl Object for PyTrue {
    fn print(&self) {
        colour::blue_ln!("True");
    }
}

pub struct PyFalse {pub val: i32}

impl Object for PyFalse {
    fn print(&self) {
        colour::blue_ln!("False");
    }
}
