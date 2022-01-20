/**********************************************
  > File Name		: object/object.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Tue Nov  2 15:48:30 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::fmt;

use crate::errors::Errors;
use crate::objects::function::Function;
use crate::objects::string::Str;
use crate::code::binary_file_parser::CodeObject;

#[derive(Clone)]
pub enum Object {
    NONE,
    True,
    False,
    Int(i32),
    Str(Str),
    Function(Function),
    CodeObject(CodeObject),
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Self::Int(i) => write!(f, "Int({})", i),
            &Self::Str(ref s) => write!(f, "Str({})", s),
            &Self::Function(ref func) => write!(f, "<func, {:?}>", func.func_name),
            &Self::CodeObject(ref code) => write!(f, "CodeObject"),
            _ => panic!("Invalid type")
        }
    }
}

impl Object {
    pub fn print(&self) -> Result<(), Errors> {
        match self {
            &Self::NONE => print!("None"),
            &Self::True => print!("True"),
            &Self::False => print!("False"),
            &Self::Int(i) => print!("{}", i),
            &Self::Str(ref v) => print!("{}", v),
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
}


