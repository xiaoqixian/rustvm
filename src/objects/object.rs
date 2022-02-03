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
use super::function::{Method, Function};
use super::string::{self, Str};
use crate::{cast, cast_match};

pub trait Object: Debug + Display {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn print(&self) {
        println!("{}", self);
    }

    fn klass(&self) -> Klass {
        Klass::NewKlass
    }

    fn get_attr(&self, owner: Rc<dyn Object>, attr_name: &Rc<dyn Object>) -> Option<Rc<dyn Object>> {
        match owner.klass() {
            Klass::StringKlass => {
                match cast!(attr_name, super::string::Str).into().unwrap() {
                    "upper" => Some(Method::new(owner, Function::from_nfp(&string::upper, Str::raw_from("upper")))),
                    _ => panic!("Invalid str attribute")
                }
            },
            Klass::NewKlass => {
                let ins = cast!(owner, NewObject);
                //first search instance attributes.
                //then search in static attributes.
                let attr = match ins.ins_attrs.get(attr_name) {
                    Some(res) => Some(res),
                    None => ins.kc.get_attr(attr_name)
                };

                match attr {
                    None => None,
                    Some(attr) => match attr.klass() {
                        Klass::FunctionKlass => Some(Method::new(owner, attr)),
                        _ => Some(attr)
                    }
                }
            },
            v => panic!("Invalid klass {:?}", v)
        }
    }

    fn set_attr(&self, owner: Rc<dyn Object>, attr_name: Rc<dyn Object>, item: Rc<dyn Object>) {
        crate::debug!("strong_count of {:?} is {}", owner, Rc::strong_count(&owner));
        let obj_dict = unsafe {&mut *(cast!(owner, NewObject) as *const NewObject as *mut NewObject)};
        obj_dict.ins_attrs.put(attr_name, item);
    }

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

/**
 * define a struct to represent a user-defined class instance.
 * An instance contains class static attributes (including all 
 * static members and methods) and instance's own attributes.
 */
#[derive(Clone)]
pub struct NewObject {
    ins_attrs: super::map::Dict, //no need to share, using Dict is fine.
    kc: Rc<super::klass::KlassContainer>,
}

impl NewObject {
    pub fn new(kc: Rc<super::klass::KlassContainer>) -> Rc<dyn Object> {
        Rc::new(Self {
            ins_attrs: super::map::Dict::raw_new(),
            kc
        })
    }
}

impl std::fmt::Debug for NewObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<type {} at {:#x}>", self.kc, self as *const _ as usize)
    }
}

impl std::fmt::Display for NewObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<type {} at {:#x}>", self.kc, self as *const _ as usize)
    }
}

impl Object for NewObject {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn klass(&self) -> Klass {
        Klass::NewKlass
    }
}
