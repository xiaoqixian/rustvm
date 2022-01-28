/**********************************************
  > File Name		: klass.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Thu 27 Jan 2022 04:43:21 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::any::Any;
use std::fmt;
use std::rc::Rc;

use super::object::{Object, ObjRef};
use crate::cast;
use crate::errors::Errors;

pub type KlassRef = &'static dyn Klass;
//type ObjRef = &dyn Object;

pub trait Klass {
    fn as_any(&self) -> &dyn Any;

    fn print(&self, x: &dyn Object) {
        print!("{}", x);
    }

    fn add(&self, this: &dyn Object, other: &dyn Object) -> Result<ObjRef, Errors> {
        //this method is implemented by sub klasses.
        Err(Errors::Null)
    }
}
