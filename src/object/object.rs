/**********************************************
  > File Name		: object/object.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Tue Nov  2 15:48:30 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

pub trait Object {
    fn add<T: Object>(&mut self, x: &T) -> T;
    fn sub<T: Object>(&mut self, x: &T) -> T;
    fn mul<T: Object>(&mut self, x: &T) -> T;
    fn div<T: Object>(&mut self, x: &T) -> T;
    fn module<T: Object>(&mut self, x: &T) -> T;

    fn greater<T: Object>(&self, x: &T) -> T;
    fn less<T: Object>(&self, x: &T) -> T;
    fn equal<T: Object>(&self, x: &T) -> T;
    fn not_equal<T: Object>(&self, x: &T) -> T;
    fn le<T: Object>(&self, x: &T) -> T;
    fn ge<T: Object>(&self, x: &T) -> T;
    fn subscr<T: Object>(&self, x: &T) -> T;
    fn contains<T: Object>(&self, x: &T) -> T;

    fn len<T: Object>(&self) -> T;
}
