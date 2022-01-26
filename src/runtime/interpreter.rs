/**********************************************
  > File Name		: runtime/interpreter.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Tue Nov  2 19:59:37 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::rc::Rc;
use std::collections::{VecDeque, BTreeMap};

use crate::objects::{object::{Object}, frame::{Frame, Block}, list::List};
use crate::objects::function::{Function, NativeFunction, NativeFuncPointer, MethodFuncPointer, len};
use crate::objects::string::{Str, STR_ATTR, upper};
use crate::code::binary_file_parser::CodeObject;
use crate::code::{byte_code, get_op, byte_code::compare};
use crate::errors::Errors;
use crate::{info, debug, error, unwrap_obj};

/*macro_rules! pop {*/
    /*($self:ident$(, $field:ident)*) => {*/
        /*match $self$(.$field)*.pop() {*/
            /*None => {panic!("empty stack pop");},*/
            /*Some(v) => v*/
        /*}*/
    /*}*/
/*}*/

/*macro_rules! ptr_eq {*/
    /*($p1:ident, $p2:ident) => {{*/
        /*($p1 as *mut Integer as u64) == ($p2 as *mut Integer as u64)*/
    /*}}*/
/*}*/

pub struct Interpreter {
    frame: Rc<Frame>,
    builtin_funcs: BTreeMap<Str, &'static NativeFuncPointer>
}

//impl Drop for Interpreter {
    //fn drop(&mut self) {
        //colour::red_ln!("dropping frame with codes name: {}", self.frame.codes.as_ref().co_name);
    //}
//}

impl Interpreter {
    pub fn new(codes: CodeObject) -> Self {
        //add Str attributes
        unsafe {
            let mut bmap = BTreeMap::<Str, &'static MethodFuncPointer>::new();
            bmap.insert(Str::raw_from("upper"), &upper);
  
            STR_ATTR = Some(bmap);
        }

        let mut builtin_funcs = BTreeMap::<Str, &'static NativeFuncPointer>::new();
        builtin_funcs.insert(Str::raw_from("len"), &len);

        Self {
            frame: Frame::new(Rc::new(Object::CodeObject(codes)), None, None),
            builtin_funcs
        }
    }

    pub fn run(&mut self) -> Result<(), Errors> {
        let mut op_code: u8 = 0;
        let mut op_arg: usize = usize::MAX;

        //let mut pc: usize = 0;//program counter
        //let code_length = codes.bytecodes.len();
/*        let codes_ref = as_ref!(self, frame, bytecodes);*/
        /*let names:&Vec<*mut dyn Object> = &codes_ref.names;*/
        /*let consts = &codes_ref.consts;*/
        /*let locals = &mut frame.locals;//K:locals names, V: locals values.*/

        while self.frame.has_more_codes() {
            op_code = self.frame.get_opcode();
            op_arg = 0;
            let have_argument = (op_code & 0xff) >= byte_code::HAVE_ARGUMENT;
            if have_argument {
                op_arg = self.frame.get_oparg();
            }

            info!("op: {:?}, op_code: {}, op_arg: {}", match get_op(op_code) {
                None => {panic!("invalid op_code: {}", op_code);},
                Some(e) => e
            }, op_code, op_arg);

            match op_code {
                byte_code::POP_TOP => {self.frame.stack.borrow_mut().pop();},

                //byte_code::LOAD_CONST => {self.frame.stack.borrow_mut().push(self.frame.codes.consts[op_arg].clone()); debug!("load const {:?}", self.frame.codes.consts[op_arg]);},
                byte_code::LOAD_CONST => {
                    self.frame.stack.borrow_mut().push(unwrap_obj!(self.frame.codes, CodeObject).consts[op_arg].clone());
                }
                
                byte_code::LOAD_NAME => {
                    match unwrap_obj!(self.frame.codes, CodeObject).names[op_arg].as_ref() {
                        &Object::Str(ref s) => {
                            //LGTB principle
                            //first search locals
                            if let Some(o) = self.frame.locals.borrow().get(s) {
                                self.frame.stack.borrow_mut().push(o.clone());
                                continue;
                            }
                            
                            //then search globals

                            //then search builtin functions.
                            if let Some(nfp) = self.builtin_funcs.get(s) {
                                self.frame.stack.borrow_mut().push(Rc::new(Object::NativeFunction(NativeFunction::new(*nfp, s))));
                                continue;
                            }
                        },
                        v => panic!("Invalid arg {:?}", v)
                    }
                },

                byte_code::LOAD_FAST => {
                    self.frame.stack.borrow_mut().push((*&self.frame.fast_locals.as_ref().unwrap().borrow()[op_arg]).clone());
                },

                byte_code::STORE_FAST => {
                    let v = self.frame.stack.borrow_mut().pop().unwrap();
                    *&mut self.frame.fast_locals.as_ref().unwrap().borrow_mut()[op_arg] = v;
                }

                byte_code::STORE_NAME => {
                    self.frame.locals.borrow_mut().insert({
                        match unwrap_obj!(self.frame.codes, CodeObject).names[op_arg].as_ref() {
                            &Object::Str(ref s) => (*s).clone(),
                            x => return Err(Errors::InvalidObject(format!("{:?}", x)))
                        }}, self.frame.stack.borrow_mut().pop().unwrap());
                },

                byte_code::PRINT_ITEM => {
                    self.frame.stack.borrow_mut().pop().unwrap().print()?;
                },

                byte_code::PRINT_NEWLINE => {
                    println!("\n");
                },

                byte_code::BINARY_ADD | byte_code::BINARY_SUBTRACT |
                byte_code::BINARY_MULTIPLY | byte_code::BINARY_DIVIDE |
                byte_code::BINARY_MOD | byte_code::BINARY_SUBSCR => {
                    let p2 = self.frame.stack.borrow_mut().pop().unwrap();
                    let p1 = self.frame.stack.borrow_mut().pop().unwrap();
                    match op_code {
                        byte_code::BINARY_ADD => self.frame.stack.borrow_mut().push(Rc::new(p1.add(&p2)?)),
                        byte_code::BINARY_SUBTRACT => self.frame.stack.borrow_mut().push(Rc::new(p1.sub(&p2)?)),
                        byte_code::BINARY_MULTIPLY => self.frame.stack.borrow_mut().push(Rc::new(p1.mul(&p2)?)),
                        byte_code::BINARY_DIVIDE => self.frame.stack.borrow_mut().push(Rc::new(p1.div(&p2)?)),
                        byte_code::BINARY_MOD => self.frame.stack.borrow_mut().push(Rc::new(p1.r#mod(&p2)?)),
                        byte_code::BINARY_SUBSCR => self.frame.stack.borrow_mut().push(Rc::new(p1.subscr(&p2)?)),
                        _ => {}

                    }
                },

                byte_code::STORE_SUBSCR => {
                }

                byte_code::LOAD_ATTR => {
                    //first get owner
                    let owner = self.frame.stack.borrow_mut().pop().unwrap();
                    //op_arg is the index of the attribute in names
                    let attr = match unwrap_obj!(self.frame.codes, CodeObject).names[op_arg].as_ref() {
                        &Object::Str(ref s) => s,
                        e => panic!("Invalid arg {:?}", e)
                    };
                    
                    self.frame.stack.borrow_mut().push(Rc::new(Object::get_attr(owner, attr)));
                }

                /*byte_code::COMPARE_OP => {*/
                    /*p2 = pop!(self, frame, stack);*/
                    /*p1 = pop!(self, frame, stack);*/
                    /*let b1 = as_ref!(p1);*/
                    
                    /*let res = match op_arg as u8 {*/
                        /*compare::LESS => b1.less(p2 as *const _),*/
                        /*compare::LESS_EQUAL => b1.le(p2 as *const _),*/
                        /*compare::EQUAL => b1.equal(p2 as *const _),*/
                        /*compare::NOT_EQUAL => b1.ne(p2 as *const _),*/
                        /*compare::GREATER => b1.greater(p2 as *const _),*/
                        /*compare::GREATER_EQUAL => b1.ge(p2 as *const _),*/

                        /*_ => {panic!("invalid compare op_arg: {}", op_arg)}*/
                    /*};*/

                    /*match res {*/
                        /*None => {*/
                            /*print!("operators: {{");*/
                            /*b1.print();*/
                            /*unsafe {*/
                                /*p2.as_ref().unwrap().print();*/
                            /*}*/
                            /*println!("}}");*/
                            /*panic!("invalid operations");*/
                        /*},*/
                        /*Some(p) => self.frame.stack.borrow_mut().push(p),*/
                    /*}*/
                /*},*/

                byte_code::BUILD_LIST => {
                    let mut vd = VecDeque::<Rc<Object>>::with_capacity(op_arg);
                    while op_arg > 0 {
                        vd.push_front(self.frame.stack.borrow_mut().pop().unwrap());
                        op_arg -= 1;
                    }
                    self.frame.stack.borrow_mut().push(Rc::new(Object::List(List::from(vd))));
                }

                byte_code::POP_JUMP_IF_TRUE => {
                    match self.frame.stack.borrow_mut().pop().unwrap().as_ref() {
                        Object::True => self.frame.set_pc(op_arg),
                        Object::False => {},
                        _ => panic!("Invalid arg")
                    }
                },

                byte_code::POP_JUMP_IF_FALSE => {
                    match self.frame.stack.borrow_mut().pop().unwrap().as_ref() {
                        Object::False => self.frame.set_pc(op_arg),
                        Object::True => {},
                        _ => panic!("Invalid arg")
                    }
                },

                byte_code::JUMP_ABSOLUTE => {
                    self.frame.set_pc(op_arg);
                }

                byte_code::JUMP_FORWARD => {
                    self.frame.add_pc_n(op_arg);
                },

                byte_code::SETUP_LOOP => {//op_arg: target address after loop
                    let block = Block::new(op_code, self.frame.get_pc() + op_arg, self.frame.stack.borrow_mut().len());
                    self.frame.loop_stack.borrow_mut().push(block);
                },

                byte_code::POP_BLOCK => {
                    let block = self.frame.loop_stack.borrow_mut().pop().unwrap();
                    if self.frame.stack.borrow_mut().len() < block.level {
                        panic!("stack invaded");
                    }
                    while self.frame.stack.borrow_mut().len() > block.level {
                        self.frame.stack.borrow_mut().pop();
                    }
                },

                byte_code::BREAK_LOOP => {
                    let block = self.frame.loop_stack.borrow_mut().pop().unwrap();
                    if self.frame.stack.borrow_mut().len() < block.level {
                        panic!("stack invaded");
                    }
                    while self.frame.stack.borrow_mut().len() > block.level {
                        self.frame.stack.borrow_mut().pop();
                    }
                    self.frame.set_pc(block.target);
                },

                byte_code::MAKE_FUNCTION => {
                    let code_wrap = self.frame.stack.borrow_mut().pop().unwrap();
                    let defaults = if op_arg > 0 {
                        let mut defaults = VecDeque::<Rc<Object>>::with_capacity(op_arg);
                        while op_arg > 0 {
                            defaults.push_front(self.frame.stack.borrow_mut().pop().unwrap());
                            op_arg -= 1;
                        }
                        Some(Vec::from(defaults))
                    } else {None};

                    self.frame.stack.borrow_mut().push(Rc::new(Object::Function(Function::new(code_wrap, defaults))));
                }

                byte_code::CALL_FUNCTION => {
                    //receive arguments for the function is called.
                    let args = if op_arg > 0 {
                        let mut args = VecDeque::<Rc<Object>>::with_capacity(op_arg);
                        while op_arg > 0 {
                            args.push_front(self.frame.stack.borrow_mut().pop().unwrap());
                            op_arg -= 1;
                        }
                        Some(Vec::from(args))
                    } else {None};

                    let func_wrap = self.frame.stack.borrow_mut().pop().unwrap();
                    match func_wrap.as_ref() {
                        &Object::Function(_) => {
                            self.build_frame(func_wrap, args);
                        },
                        //native function doesn't has a RETURN_VALUE op, so don't build a frame.
                        &Object::NativeFunction(ref nf) => {
                            self.frame.stack.borrow_mut().push(match nf.call(match args {
                                None => Vec::<Rc<Object>>::new(),
                                Some(v) => v
                            }) {
                                None => Rc::new(Object::r#None),
                                Some(v) => v
                            });
                        },

                        //methods
                        &Object::Method(ref m) => {
                            self.frame.stack.borrow_mut().push(match m.call(match args {
                                None => Vec::<Rc<Object>>::new(),
                                Some(v) => v
                            }) {
                                None => Rc::new(Object::r#None),
                                Some(v) => v
                            });
                        }
                        v => panic!("Invalid function {:?}", v)
                    }
                }

                byte_code::RETURN_VALUE => {
                    self.frame.stack.borrow_mut().pop();
                    if let Some(f) = &self.frame.sender {
                        self.frame = f.clone();
                    }
                },

                _ => {
                    panic!("Unkown op_code {}", op_code);
                }
            }
        }
        Ok(())
    }

    #[inline]
    fn build_frame(&mut self, callable: Rc<Object>, args: Option<Vec<Rc<Object>>>) {
        self.frame = Frame::new(callable, args, Some(self.frame.clone()));
    }
}
