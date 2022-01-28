/**********************************************
  > File Name		: object/integer.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Tue Nov  2 20:48:42 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::rc::Rc;
use std::any::Any;
use std::fmt;

use super::object::{Object, ObjRef};
use crate::cast;
use crate::errors::Errors;
use super::klass::{Klass, KlassRef};

pub static INTEGER_KLASS_INSTANCE: IntegerKlass = IntegerKlass {mod_str: "int"};

#[derive(Debug, Copy, Clone)]
pub struct Integer {
    inner: i32,
    klass: &'static IntegerKlass
}

#[derive(Debug, Clone, Copy)]
pub struct IntegerKlass {
    mod_str: &'static str
}

impl Klass for IntegerKlass {
    fn as_any(&self) -> &dyn Any {self}

    fn add(&self, this: &dyn Object, other: &dyn Object) -> Result<ObjRef, Errors> {
        Ok(Integer::new(
            cast!(this, Integer).into() + cast!(other, Integer).into()
        ))
    }
}

impl Integer {
    pub fn into(&self) -> i32 {
        self.inner
    }

    pub fn new(inner: i32) -> ObjRef {
        Rc::new(Self {
            inner,
            klass: &INTEGER_KLASS_INSTANCE
        })
    }

    pub fn new_raw(inner: i32) -> Self {
        Self {
            inner,
            klass: &INTEGER_KLASS_INSTANCE
        }
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl Object for Integer {
    fn klass(&self) -> KlassRef {
        self.klass
    }

    fn as_any(&self) -> &dyn Any {self}
}

/*impl Object for Integer {*/
    /*fn print(&self) {*/
        /*colour::blue_ln!("{}", self.inner);*/
    /*}*/

    /*fn into<i32>(&self) -> Result<i32, Errors> {*/
        /*Ok(self.inner)*/
    /*}*/

    /*fn add(&self, _rhs: Rc<dyn Object>) -> Result<Rc<dyn Object>, Errors> {*/
        /*Ok(Self::new(self.inner + _rhs.into()))*/
    /*}*/

    /*fn sub(&self, _rhs: Rc<dyn Object>) -> Result<Rc<dyn Object>, Errors> {*/
        /*let _rhs_ref:&Self = unsafe {*/
            /*match _rhs.cast::<Self>().as_ref() {*/
                /*None => {return None;},*/
                /*Ok(r) => r*/
            /*}*/
        /*};*/
        /*Ok(Self::new_ptr(self.inner - _rhs_ref.into()))*/
    /*}*/

    /*fn inplace_sub(&mut self, _rhs: Rc<dyn Object>) {*/
        /*let _rhs_ref = ptr_to_ref_no_ret!(_rhs);*/
        /*self.inner -= _rhs_ref.into();*/
    /*}*/

    /*fn mul(&self, _rhs: Rc<dyn Object>) -> Result<Rc<dyn Object>, Errors> {*/
        /*let _rhs_ref:&Self = unsafe {*/
            /*match _rhs.cast::<Self>().as_ref() {*/
                /*None => {return None;},*/
                /*Ok(r) => r*/
            /*}*/
        /*};*/
        /*Ok(Self::new_ptr(self.inner * _rhs_ref.into()))*/
    /*}*/

    /*fn div(&self, _rhs: Rc<dyn Object>) -> Result<Rc<dyn Object>, Errors> {*/
        /*let _rhs_ref:&Self = unsafe {*/
            /*match _rhs.cast::<Self>().as_ref() {*/
                /*None => {return None;},*/
                /*Ok(r) => r*/
            /*}*/
        /*};*/
        /*if _rhs_ref.into() == 0 {*/
            /*panic!("divide by zero");*/
        /*}*/
        /*Ok(Self::new_ptr(self.inner / _rhs_ref.into()))*/
    /*}*/

    /*fn greater(&self, _rhs: Rc<dyn Object>) -> Result<Rc<dyn Object>, Errors> {*/
        /*let _rhs_ref:&Self = unsafe {*/
            /*match _rhs.cast::<Self>().as_ref() {*/
                /*None => {return None;},*/
                /*Ok(r) => r*/
            /*}*/
        /*};*/
        /*if self.inner > _rhs_ref.into() {*/
            /*Ok(statics::TRUE)*/
        /*} else {*/
            /*Ok(statics::FALSE)*/
        /*}*/
    /*}*/

    /*fn less(&self, _rhs: Rc<dyn Object>) -> Result<Rc<dyn Object>, Errors> {*/
        /*let _rhs_ref = ptr_to_ref!(_rhs);*/
        /*if self.inner < _rhs_ref.into() {*/
            /*Ok(statics::TRUE)*/
        /*} else {*/
            /*Ok(statics::FALSE)*/
        /*}*/
    /*}*/

    /*fn equal(&self, _rhs: Rc<dyn Object>) -> Result<Rc<dyn Object>, Errors> {*/
        /*let _rhs_ref = ptr_to_ref!(_rhs);*/
        /*if self.inner == _rhs_ref.into() {*/
            /*Ok(statics::TRUE)*/
        /*} else {*/
            /*Ok(statics::FALSE)*/
        /*}*/
    /*}*/

    /*fn ne(&self, _rhs: Rc<dyn Object>) -> Result<Rc<dyn Object>, Errors> {*/
        /*let _rhs_ref = ptr_to_ref!(_rhs);*/
        /*if self.inner != _rhs_ref.into() {*/
            /*Ok(statics::TRUE)*/
        /*} else {*/
            /*Ok(statics::FALSE)*/
        /*}*/
    /*}*/

    /*fn le(&self, _rhs: Rc<dyn Object>) -> Result<Rc<dyn Object>, Errors> {*/
        /*let _rhs_ref = ptr_to_ref!(_rhs);*/
        /*if self.inner <= _rhs_ref.into() {*/
            /*Ok(statics::TRUE)*/
        /*} else {*/
            /*Ok(statics::FALSE)*/
        /*}*/
    /*}*/

    /*fn ge(&self, _rhs: Rc<dyn Object>) -> Result<Rc<dyn Object>, Errors> {*/
        /*let _rhs_ref = ptr_to_ref!(_rhs);*/
        /*if self.inner >= _rhs_ref.into() {*/
            /*Ok(statics::TRUE)*/
        /*} else {*/
            /*Ok(statics::FALSE)*/
        /*}*/
    /*}*/
/*}*/

