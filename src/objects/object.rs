/**********************************************
  > File Name		: object/object.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Tue Nov  2 15:48:30 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::fmt;
use std::rc::Rc;
use std::any::Any;

use crate::errors::Errors;
use super::klass::{Klass, KlassRef};
use super::string::{Str, NONE, TRUE, FALSE};

/**
 * All Object trait object have to be 
 * represented in ObjRef type.
 */
pub type ObjRef = Rc<dyn Object>;

#[macro_export]
macro_rules! cast {
    ($expr: expr, $type: ty) => {
        $expr.as_any().downcast_ref::<$type>().unwrap()
    };
}

pub trait Object: fmt::Debug + fmt::Display {
    fn klass(&self) -> KlassRef;

    fn as_any(&self) -> &dyn Any;

    fn print(&self) where Self: Sized {
        self.klass().print(self);
    }

    fn is_none(&self) -> bool where Self: Sized {
        self as *const dyn Object == NONE.as_ref() as *const dyn Object
    }

    fn is_true(&self) -> bool where Self: Sized {
        self as *const dyn Object == TRUE.as_ref() as *const dyn Object
    }

    fn is_false(&self) -> bool where Self: Sized {
        self as *const dyn Object == FALSE.as_ref() as *const dyn Object
    }

    fn add(&self, other: ObjRef) -> ObjRef where Self: Sized {
        self.klass().add(self, other.as_ref()).unwrap()
    }
}



/*impl Object {*/
    /*pub fn get_attr(owner: Rc<Self>, attr: &Str) -> Self {*/
        /*match owner.as_ref() {*/
            /*&Self::Str(_) => {*/
                /*//first search methods*/
                /*if let Some(m) = unsafe {STR_ATTR.as_ref().unwrap().get(attr)} {*/
                    /*return Object::Method(Method::from(owner, m));*/
                /*}*/
                /*panic!("Invalid attribute {}", attr)*/
            /*},*/
            /*_ => panic!("Invalid arg {:?}", owner)*/
        /*}*/
    /*}*/

    /*pub fn print(&self) -> Result<(), Errors> {*/
        /*match self {*/
            /*&Self::r#None => println!("None"),*/
            /*&Self::True => println!("True"),*/
            /*&Self::False => println!("False"),*/
            /*&Self::Int(i) => println!("{}", i),*/
            /*&Self::Str(ref v) => println!("{}", v),*/
            /*&Self::List(ref l) => println!("{}", l),*/
            /*&Self::Function(ref f) => {},*/
            /*_ => {}*/
        /*}*/
        /*Ok(())*/
    /*}*/

    /*pub fn add(&self, rhs: &Object) -> Result<Self, Errors> {*/
        /*match rhs {*/
            /*&Self::Int(r) => {*/
                /*match self {*/
                    /*&Self::Int(i) => Ok(Self::Int(i + r)),*/
                    /*_ => Err(Errors::InvalidArg(format!("{:?}", self)))*/
                /*}*/
            /*},*/
            /*_ => Err(Errors::InvalidArg(format!("{:?}", rhs)))*/
        /*}*/
    /*}*/

    /*pub fn sub(&self, rhs: &Object) -> Result<Self, Errors> {*/
        /*match rhs {*/
            /*&Self::Int(r) => {*/
                /*match self {*/
                    /*&Self::Int(i) => Ok(Self::Int(i - r)),*/
                    /*_ => Err(Errors::InvalidArg(format!("{:?}", self)))*/
                /*}*/
            /*},*/
            /*_ => Err(Errors::InvalidArg(format!("{:?}", rhs)))*/
        /*}*/
    /*}*/

    /*pub fn mul(&self, rhs: &Object) -> Result<Self, Errors> {*/
        /*match rhs {*/
            /*&Self::Int(r) => {*/
                /*match self {*/
                    /*&Self::Int(i) => Ok(Self::Int(i * r)),*/
                    /*_ => Err(Errors::InvalidArg(format!("{:?}", self)))*/
                /*}*/
            /*},*/
            /*_ => Err(Errors::InvalidArg(format!("{:?}", rhs)))*/
        /*}*/
    /*}*/

    /*pub fn div(&self, rhs: &Object) -> Result<Self, Errors> {*/
        /*match rhs {*/
            /*&Self::Int(r) => {*/
                /*match self {*/
                    /*&Self::Int(i) => Ok(Self::Int(i / r)),*/
                    /*_ => Err(Errors::InvalidArg(format!("{:?}", self)))*/
                /*}*/
            /*},*/
            /*_ => Err(Errors::InvalidArg(format!("{:?}", rhs)))*/
        /*}*/
    /*}*/

    /*pub fn r#mod(&self, rhs: &Object) -> Result<Self, Errors> {*/
        /*match rhs {*/
            /*&Self::Int(r) => {*/
                /*match self {*/
                    /*&Self::Int(i) => Ok(Self::Int(i % r)),*/
                    /*_ => Err(Errors::InvalidArg(format!("{:?}", self)))*/
                /*}*/
            /*},*/
            /*_ => Err(Errors::InvalidArg(format!("{:?}", rhs)))*/
        /*}*/
    /*}*/

    /*pub fn subscr(&self, rhs: &Object) -> Result<Self, Errors> {*/
        /*match self {*/
            /*&Object::List(ref l) => match rhs {*/
                /*&Self::Int(i) => if i >= l.inner.len() as i32 || i < 0 {*/
                    /*Err(Errors::IndexOutBounds(i))*/
                /*} else {*/
                    /*Ok((*l.inner[i as usize].as_ref()).clone())*/
                /*},*/
                /*_ => panic!("Invalid arg {:?}", rhs)*/
            /*}*/
            /*_ => Err(Errors::InvalidArg(format!("{:?}", rhs)))*/
        /*}*/
    /*}*/

    /*pub fn store_subscr(&mut self, i: i32, new_item: Object) -> Result<(), Errors> {*/
        /*let index = i as usize;*/
        /*match self {*/
            /*&mut Object::List(ref mut l) => {*/
                /*if index > l.inner.len() {*/
                    /*Err(Errors::IndexOutBounds(i))*/
                /*} else {*/
                    /* *&mut l.inner[index] = new_item;*/
                    /*Ok(())*/
                /*}*/
            /*},*/
            /*&mut Object::Str(ref mut s) => {*/
                /*if index > s.len() {*/
                    /*Err(Errors::IndexOutBounds(i))*/
                /*} else {*/
                    /*Ok(())*/
                /*}*/
            /*},*/
            /*_ => panic!("Invalid arg {:?}", self)*/
        /*}*/
    /*}*/

    /*pub fn len(&self) -> i32 {*/
        /*match self {*/
            /*&Object::Str(ref s) => s.len() as i32,*/
            /*&Object::List(ref l) => l.inner.len() as i32,*/
            /*_ => panic!("Invalid arg {:?}", self)*/
        /*}*/
    /*}*/

/*}*/
