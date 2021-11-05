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
/*    fn mul<T: Object+Clone>(&mut self, x: &T) -> Box<T>*/
    /*fn div<T: Object+Clone>(&mut self, x: &T) -> Box<T>*/
    /*fn module<T: Object+Clone>(&mut self, x: &T) -> Box<T>*/

    /*fn greater<T: Object+Clone>(&self, x: &T) -> Box<T>*/
    /*fn less<T: Object+Clone>(&self, x: &T) -> Box<T>*/
    /*fn equal<T: Object+Clone>(&self, x: &T) -> Box<T>*/
    /*fn not_equal<T: Object+Clone>(&self, x: &T) -> Box<T>*/
    /*fn le<T: Object+Clone>(&self, x: &T) -> Box<T>*/
    /*fn ge<T: Object+Clone>(&self, x: &T) -> Box<T>*/
/*    fn subscr<T: Object+Clone>(&self, x: &T) -> Box<T>*/
    /*fn contains<T: Object+Clone>(&self, x: &T) -> Box<T>*/

    /*fn len<T: Object+Clone>(&self) -> Box<T>*/
}

//PyNone implements single instance pattern.
pub struct PyNone {}

impl PyNone {
    pub fn new() -> *mut Self {
        Box::into_raw(Box::new(PyNone {}))
    }

    //pub fn get_instance() -> *mut dyn Object {
        //if super::Statics::py_none.is_null() {
            //super::Statics::py_none = Self::new();
        //} 
        //super::Statics::py_none
    //}
}

impl Object for PyNone {
    fn print(&self) {
        println!("None");
    } 
}
