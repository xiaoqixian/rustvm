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
