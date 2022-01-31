/**********************************************
  > File Name		: object/mod.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Tue Nov  2 15:48:20 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

pub mod object;
pub mod klass;
pub mod string;
pub mod integer;
//pub mod frame;
//pub mod function;
pub mod list;
pub mod map;

use std::rc::Rc;
pub type Object = Rc<dyn object::Object>;

#[macro_export]
macro_rules! cast {
    ($expr: expr, $type: ty) => {
        match $expr.as_any().downcast_ref::<$type>() {
            None => panic!("Invalid {} {:?}", stringify!($expr), $expr),
            Some(v) => v
        }
    };
}

#[macro_export]
macro_rules! cast_mut {
    ($expr: expr, $type: ty) => {
        //expr is expected to have only one strong count.
        match Rc::get_mut($expr) {
            None => panic!("call get_mut on {:?} with {} strong count", $expr, Rc::strong_count($expr)),
            Some(o) => {
                match o.as_any_mut().downcast_mut::<$type>() {
                    None => panic!("Invalid {} {:?}", stringify!($expr), $expr),
                    Some(v) => v
                }
            }
        }
    };
}

#[macro_export]
macro_rules! cast_match {
    ($expr: expr) => {
        match $expr.klass() {
            Klass::IntegerKlass => cast!($expr, crate::objects::integer::Integer) as *const _ as usize,
            Klass::StringKlass => cast!($expr, crate::objects::string::Str) as *const _ as usize,
            Klass::ListKlass => cast!($expr, crate::objects::list::List) as *const _ as usize,
            Klass::DictKlass => cast!($expr, crate::objects::map::Dict) as *const _ as usize,
            Klass::CodeKlass => cast!($expr, crate::code::code_object::CodeObject) as *const _ as usize,
            v => panic!("Invalid klass {:?}", v)
        }
    }
}
/**
 * define some builtin constant values 
 * like None, True, False
 */
#[derive(Debug, Clone, Copy)]
pub enum BuiltinValue {
    r#None,
    True,
    False
}

impl BuiltinValue {
    pub fn new(name: &str) -> Object {
        Rc::new(match name {
            "None" => Self::r#None,
            "True" => Self::True,
            "False" => Self::False,
            _ => panic!("Invalid BuiltinValue {}", name)
        })
    }
}

impl object::Object for BuiltinValue {
    fn as_any(&self) -> &dyn std::any::Any {self}

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn print(&self) {
        print!("{}", self);
    }

    fn klass(&self) -> klass::Klass {
        klass::Klass::BuiltinKlass
    }
}

impl std::fmt::Display for BuiltinValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            &BuiltinValue::r#None => "None",
            &BuiltinValue::True => "True",
            &BuiltinValue::False => "False"
        })
    }
}
