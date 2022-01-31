/**********************************************
  > File Name		: objects/function.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Mon Nov  8 09:54:04 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::rc::Rc;

use super::{Object, string::Str, klass::Klass, object::Object as ObjectTrait};
use crate::cast;

pub type NativeFuncPointer = dyn Fn(Vec<Object>) -> Option<Object>;
pub type MethodFuncPointer = dyn Fn(Object, Vec<Object>) -> Option<Object>;

#[derive(Clone)]
pub struct Function {
    pub func_codes: Object,
    pub func_name: Object,
    pub flags: u32,
    pub defaults: Option<Vec<Object>>,
}

impl Function {
    pub fn new(codes_wrap: Object, defaults: Option<Vec<Object>>) -> Rc<Self> {
        Rc::new(match codes_wrap.klass() {
            Klass::CodeKlass => {
                let codes = cast!(codes_wrap, crate::code::code_object::CodeObject);
                Self {
                    func_name: codes.co_name.clone(),
                    func_codes: codes_wrap,
                    flags: 0,
                    defaults
                }
            },
            _ => panic!("Invalid codes_wrap {:?}", codes_wrap)
        })
    }
}

impl ObjectTrait for Function {
    fn as_any(&self) -> &dyn std::any::Any {self}

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {self}

    fn klass(&self) -> Klass {
        Klass::FunctionKlass
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.func_name)
    }
}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<func {}>", self.func_name)
    }
}



/*#[derive(Clone)]*/
/*pub struct NativeFunction {*/
    /*pub nfp: &'static NativeFuncPointer,*/
    /*pub func_name: Str*/
/*}*/

/*impl NativeFunction {*/
    /*pub fn new(nfp: &'static NativeFuncPointer, func_name: &Str) -> Self {*/
        /*Self {*/
            /*nfp,*/
            /*func_name: func_name.clone()*/
        /*}*/
    /*}*/

    /*pub fn call(&self, args: Vec<Object>) -> Option<Object> {*/
        /*let nfp = self.nfp;*/
        /*nfp(args)*/
    /*}*/
/*}*/

/*#[derive(Clone)]*/
/*pub struct Method {*/
    /*pub owner: Object,*/
    /*pub mfp: &'static MethodFuncPointer*/
/*}*/

/*impl Method {*/
    /*pub fn from(owner: Object, mfp: &'static MethodFuncPointer) -> Self {*/
        /*Self {*/
            /*owner,*/
            /*mfp*/
        /*}*/
    /*}*/

    /*pub fn call(&self, args: Vec<Object>) -> Option<Object> {*/
        /*let mfp = self.mfp;*/
        /*mfp(self.owner.clone(), args)*/
    /*}*/
/*}*/

/*pub fn len(args: Vec<Object>) -> Option<Object> {*/
    /*Some(Rc::new(Object::Int(args[0].len())))*/
/*}*/
