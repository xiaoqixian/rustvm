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

    fn add(&self, _rhs: &dyn Object) -> Option<Box<dyn Object>> {
        None
    }

    fn sub(&self, _rhs: &dyn Object) -> Option<Box<dyn Object>> {
        None
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
