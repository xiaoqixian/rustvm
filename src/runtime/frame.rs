/**********************************************
  > File Name		: objects/frame.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Sat Nov  6 22:00:50 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::collections::BTreeMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::objects::{Object, string::Str, klass::Klass};
use crate::code::code_object::CodeObject;
use crate::cast;

#[derive(Clone, Copy, Debug)]
pub struct Block {
    pub _type: u8,
    pub target: usize,
    pub level: usize
}

impl Block {
    pub fn new(_type: u8, target: usize, level: usize) -> Self {
        Self {
            _type,
            target,
            level
        }
    }
}

#[derive(Clone)]
pub struct Frame {
    pub pc: RefCell<usize>,
    pub stack: RefCell<Vec<Object>>,
    pub loop_stack: RefCell<Vec<Block>>,
    pub locals: RefCell<BTreeMap<Str, Object>>,
    pub globals: BTreeMap<Str, Object>,
    pub fast_locals: Option<RefCell<Vec<Object>>>,
    pub codes: Object,
    pub sender: Option<Rc<Self>>
}

impl Frame {
    #[inline]
    pub fn get_pc(&self) -> usize {
        *self.pc.borrow()
    }

    #[inline]
    pub fn set_pc(&self, pc: usize) {
        *self.pc.borrow_mut() = pc;
    } 

    #[inline]
    pub fn add_pc(&self) {
        *self.pc.borrow_mut() += 1;
    }

    #[inline]
    pub fn add_pc_n(&self, n: usize) {
        *self.pc.borrow_mut() += n;
    }

    #[inline]
    pub fn has_more_codes(&self) -> bool {
        self.get_pc() < cast!(cast!(self.codes, CodeObject).bytecodes, Str).len()
    }
    
    pub fn get_opcode(&self) -> u8 {
        let res = cast!(cast!(self.codes, CodeObject).bytecodes, Str)[self.get_pc()];
        self.add_pc();
        res
    }

    pub fn get_oparg(&self) -> usize {
        let b1 = self.get_opcode() as usize;
        let b2 = self.get_opcode() as usize;
        b2 << 8 | b1
    }
}

impl Frame {
    pub fn new(codes: Object, args: Option<Vec<Object>>, sender: Option<Rc<Self>>) -> Rc<Self> {
        Rc::new(match codes.klass() {
            Klass::CodeKlass => {
                Self {
                    pc: RefCell::new(0),
                    stack: RefCell::new(Vec::new()),
                    loop_stack: RefCell::new(Vec::new()),
                    locals: RefCell::new(BTreeMap::new()),
                    globals: BTreeMap::new(),
                    fast_locals: match args {
                        None => None,
                        Some(v) => Some(RefCell::new(v))
                    },
                    codes,
                    sender
                }
            },
/*            &Object::Function(ref func) => {*/
                /*Self {*/
                    /*pc: RefCell::new(0),*/
                    /*stack: RefCell::new(Vec::new()),*/
                    /*loop_stack: RefCell::new(Vec::new()),*/
                    /*locals: RefCell::new(BTreeMap::new()),*/
                    /*globals: BTreeMap::new(),*/
                    /*fast_locals: {*/
                        /*let mut arg_num = match func.func_codes.as_ref() {*/
                            /*&Object::CodeObject(ref c) => c.argcount,*/
                            /*_ => panic!("Invalid arg {:?}", codes)*/
                        /*};*/
                        /*let mut fast_locals = vec![Rc::new(Object::r#None); arg_num];*/
                        
                        /*if let &Some(ref defaults) = &func.defaults {*/
                            /*let mut dft_num = defaults.len();*/
                            /*assert!(arg_num >= dft_num);*/

                            /*while dft_num > 0 {*/
                                /*dft_num -= 1;*/
                                /*arg_num -= 1;*/
                                /* *&mut fast_locals[arg_num] = (*&defaults[dft_num]).clone();*/
                            /*}*/
                        /*}*/
                        
                        /*if let Some(args_v) = args {*/
                            /*fast_locals.splice(..args_v.len(), args_v);*/
                        /*}*/

                        /*Some(RefCell::new(fast_locals))*/
                    /*},*/
                    /*codes: func.func_codes.clone(),*/
                    /*sender*/
                /*}*/
            /*},*/
            _ => panic!("Invalid frame codes {:?}", codes)
        })
    }

    #[inline]
    pub fn get_const(&self, index: usize) -> Object {
        self.codes.as_any().downcast_ref::<CodeObject>().unwrap().consts[index].clone()
    }

    #[inline]
    pub fn get_name(&self, index: usize) -> Object {
        self.codes.as_any().downcast_ref::<CodeObject>().unwrap().names[index].clone()
    }

    #[inline]
    pub fn get_local(&self, name: Object) -> Object {
        match self.locals.borrow().get(name.as_any().downcast_ref::<Str>().unwrap()) {
            None => panic!("{:?} not found in locals", name),
            Some(v) => v.clone()
        }
    }

    #[inline]
    pub fn store_name(&self, index: usize, item: Object) {
        crate::debug!("store name {:?} for item {:?}", self.get_name(index), item);
        self.locals.borrow_mut().insert(
            self.get_name(index).as_any().downcast_ref::<Str>().unwrap().clone(),
            item
        );
    }
}

