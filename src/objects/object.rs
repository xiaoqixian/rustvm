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

use super::klass::Klass;

pub trait Object: Debug + Display {
    fn as_any(&self) -> &dyn Any;

    fn print(&self) {
        println!("{}", self);
    }

    fn klass(&self) -> Klass;
}
