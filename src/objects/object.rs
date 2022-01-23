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
use std::collections::BTreeMap;

use crate::errors::Errors;
use crate::objects::function::{Function, NativeFunction};
use crate::objects::{string::Str, list::List};
use crate::code::binary_file_parser::CodeObject;

#[derive(Clone)]
pub enum Object {
    r#None,
    True,
    False,
    Int(i32),
    Str(Str),
    List(List),
    Function(Function),
    NativeFunction(NativeFunction),
    CodeObject(CodeObject),
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Self::r#None => write!(f, "None"),
            &Self::True => write!(f, "True"),
            &Self::False => write!(f, "False"),
            &Self::Int(i) => write!(f, "Int({})", i),
            &Self::Str(ref s) => write!(f, "Str({})", s),
            &Self::List(ref l) => write!(f, "List({})", l),
            &Self::Function(ref func) => write!(f, "<func, {}>", func.func_name),
            &Self::NativeFunction(ref native_func) => write!(f, "<native_func, {}>", native_func.func_name),
            &Self::CodeObject(ref code) => write!(f, "CodeObject"),
            _ => panic!("Invalid type")
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Self::r#None => write!(f, "None"),
            &Self::True => write!(f, "True"),
            &Self::False => write!(f, "False"),
            &Self::Int(i) => write!(f, "{}", i),
            &Self::Str(ref s) => write!(f, "{}", s),
            &Self::List(ref l) => write!(f, "{}", l),
            &Self::Function(ref func) => write!(f, "<func, {}>", func.func_name),
            &Self::NativeFunction(ref native_func) => write!(f, "<native_func, {}>", native_func.func_name),
            &Self::CodeObject(ref code) => write!(f, "CodeObject"),
            _ => panic!("Invalid type")
        }
    }
}

impl Object {
    pub fn print(&self) -> Result<(), Errors> {
        match self {
            &Self::r#None => println!("None"),
            &Self::True => println!("True"),
            &Self::False => println!("False"),
            &Self::Int(i) => println!("{}", i),
            &Self::Str(ref v) => println!("{}", v),
            &Self::List(ref l) => println!("{}", l),
            &Self::Function(ref f) => {},
            _ => {}
        }
        Ok(())
    }

    pub fn add(&self, rhs: &Object) -> Result<Self, Errors> {
        match rhs {
            &Self::Int(r) => {
                match self {
                    &Self::Int(i) => Ok(Self::Int(i + r)),
                    _ => Err(Errors::InvalidArg(format!("{:?}", self)))
                }
            },
            _ => Err(Errors::InvalidArg(format!("{:?}", rhs)))
        }
    }

    pub fn sub(&self, rhs: &Object) -> Result<Self, Errors> {
        match rhs {
            &Self::Int(r) => {
                match self {
                    &Self::Int(i) => Ok(Self::Int(i - r)),
                    _ => Err(Errors::InvalidArg(format!("{:?}", self)))
                }
            },
            _ => Err(Errors::InvalidArg(format!("{:?}", rhs)))
        }
    }

    pub fn mul(&self, rhs: &Object) -> Result<Self, Errors> {
        match rhs {
            &Self::Int(r) => {
                match self {
                    &Self::Int(i) => Ok(Self::Int(i * r)),
                    _ => Err(Errors::InvalidArg(format!("{:?}", self)))
                }
            },
            _ => Err(Errors::InvalidArg(format!("{:?}", rhs)))
        }
    }

    pub fn div(&self, rhs: &Object) -> Result<Self, Errors> {
        match rhs {
            &Self::Int(r) => {
                match self {
                    &Self::Int(i) => Ok(Self::Int(i / r)),
                    _ => Err(Errors::InvalidArg(format!("{:?}", self)))
                }
            },
            _ => Err(Errors::InvalidArg(format!("{:?}", rhs)))
        }
    }

    pub fn r#mod(&self, rhs: &Object) -> Result<Self, Errors> {
        match rhs {
            &Self::Int(r) => {
                match self {
                    &Self::Int(i) => Ok(Self::Int(i % r)),
                    _ => Err(Errors::InvalidArg(format!("{:?}", self)))
                }
            },
            _ => Err(Errors::InvalidArg(format!("{:?}", rhs)))
        }
    }

    pub fn subscr(&self, rhs: &Object) -> Result<Self, Errors> {
        match self {
            &Object::List(ref l) => match rhs {
                &Self::Int(i) => if i >= l.inner.len() as i32 || i < 0 {
                    Err(Errors::IndexOutBounds(i))
                } else {
                    Ok((*&l.inner[i as usize]).clone())
                },
                _ => panic!("Invalid arg {:?}", rhs)
            }
            _ => Err(Errors::InvalidArg(format!("{:?}", rhs)))
        }
    }

    pub fn store_subscr(&mut self, i: i32, new_item: Object) -> Result<(), Errors> {
        let index = i as usize;
        match self {
            &mut Object::List(ref mut l) => {
                if index > l.inner.len() {
                    Err(Errors::IndexOutBounds(i))
                } else {
                    *&mut l.inner[index] = new_item;
                    Ok(())
                }
            },
            &mut Object::Str(ref mut s) => {
                if index > s.len() {
                    Err(Errors::IndexOutBounds(i))
                } else {
                    Ok(())
                }
            },
            _ => panic!("Invalid arg {:?}", self)
        }
    }

    pub fn len(&self) -> i32 {
        match self {
            &Object::Str(ref s) => s.len() as i32,
            &Object::List(ref l) => l.inner.len() as i32,
            _ => panic!("Invalid arg {:?}", self)
        }
    }

}


