/**********************************************
  > File Name		: object/object.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Tue Nov  2 15:48:30 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::any::Any;
use std::fmt::{Debug, Display};
use std::rc::Rc;

use super::klass::Klass;
use super::integer::Integer;
use crate::{cast, cast_match};

pub trait Object: Debug + Display {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn print(&self) {
        println!("{}", self);
    }

    fn klass(&self) -> Klass;

    /**
     * define two dynamic object instances by comparing their 
     * raw pointer values.
     */
    fn equal(&self, other: &Rc<dyn Object>) -> bool {
        if self.klass() != other.klass() {
            false
        } else {
            cast_match!(self) == cast_match!(other)
        }
    }
}
