/**********************************************
  > File Name		: objects/function.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Mon Nov  8 09:54:04 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::rc::Rc;

use super::{object::{Object, ObjRef}, string::Str};
use super::code_object::CodeObject;
use super::klass::{KlassRef, Klass};
use crate::cast;

pub type NativeFuncPointer = dyn Fn(Vec<ObjRef>) -> Option<ObjRef>;
pub type MethodFuncPointer = dyn Fn(ObjRef, Vec<ObjRef>) -> Option<ObjRef>;
pub static FUNCTION_KLASS_INSTANCE: FunctionKlass = FunctionKlass {mod_str: "FunctionKlass"};

#[derive(Clone, Debug)]
pub struct Function {
    pub func_codes: ObjRef,
    pub func_name: ObjRef,
    pub flags: u32,
    pub defaults: Option<Vec<ObjRef>>,
    pub klass: &'static FunctionKlass
}

#[derive(Clone, Debug)]
pub struct FunctionKlass {
    mod_str: &'static str
}

impl Klass for FunctionKlass {
    fn as_any(&self) -> &dyn std::any::Any {self}
}

impl Object for Function {
    fn as_any(&self) -> &dyn std::any::Any {self}

    fn klass(&self) -> KlassRef {self.klass}
}

impl Function {
    pub fn new(codes_wrap: ObjRef, defaults: Option<Vec<ObjRef>>) -> ObjRef {
        Rc::new(Self {
            func_name: cast!(codes_wrap, CodeObject).co_name.clone(),
            func_codes: codes_wrap,
            flags: 0,
            defaults,
            klass: &FUNCTION_KLASS_INSTANCE
        })
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<func {}>", self.func_name)
    }
}

#[derive(Clone, Debug)]
pub struct NativeFunction {
    pub nfp: &'static NativeFuncPointer,
    pub func_name: Str
}

impl NativeFunction {
    pub fn new(nfp: &'static NativeFuncPointer, func_name: &Str) -> Self {
        Self {
            nfp,
            func_name: func_name.clone()
        }
    }

    pub fn call(&self, args: Vec<ObjRef>) -> Option<ObjRef> {
        let nfp = self.nfp;
        nfp(args)
    }
}

impl std::fmt::Display for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native_func {}>", self.func_name)
    }
}

impl Object for 

#[derive(Clone)]
pub struct Method {
    pub owner: ObjRef,
    pub mfp: &'static MethodFuncPointer
}

impl Method {
    pub fn from(owner: ObjRef, mfp: &'static MethodFuncPointer) -> Self {
        Self {
            owner,
            mfp
        }
    }

    pub fn call(&self, args: Vec<ObjRef>) -> Option<ObjRef> {
        let mfp = self.mfp;
        mfp(self.owner.clone(), args)
    }
}
