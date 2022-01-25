/**********************************************
  > File Name		: objects/function.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Mon Nov  8 09:54:04 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::rc::Rc;

use super::{object::Object, string::Str};

pub type NativeFuncPointer = dyn Fn(Vec<Rc<Object>>) -> Option<Rc<Object>>;
pub type MethodFuncPointer = dyn Fn(Rc<Object>, Vec<Rc<Object>>) -> Option<Rc<Object>>;

#[derive(Clone)]
pub struct Function {
    pub func_codes: Rc<Object>,
    pub func_name: Rc<Object>,
    pub flags: u32,
    pub defaults: Option<Vec<Rc<Object>>>,
}

impl Function {
    pub fn new(codes_wrap: Rc<Object>, defaults: Option<Vec<Rc<Object>>>) -> Self {
        match codes_wrap.as_ref() {
            &Object::CodeObject(ref codes) => {
                Self {
                    func_name: codes.co_name.clone(),
                    func_codes: codes_wrap,
                    flags: 0,
                    defaults
                }
            },
            _ => panic!("Invalid arg {:?}", codes_wrap)
        }
    }
}

#[derive(Clone)]
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

    pub fn call(&self, args: Vec<Rc<Object>>) -> Option<Rc<Object>> {
        let nfp = self.nfp;
        nfp(args)
    }
}

#[derive(Clone)]
pub struct Method {
    pub owner: Rc<Object>,
    pub mfp: &'static MethodFuncPointer
}

impl Method {
    pub fn from(owner: Rc<Object>, mfp: &'static MethodFuncPointer) -> Self {
        Self {
            owner,
            mfp
        }
    }

    pub fn call(&self, args: Vec<Rc<Object>>) -> Option<Rc<Object>> {
        let mfp = self.mfp;
        mfp(self.owner.clone(), args)
    }
}

pub fn len(args: Vec<Rc<Object>>) -> Option<Rc<Object>> {
    Some(Rc::new(Object::Int(args[0].len())))
}
