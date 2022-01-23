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

use crate::objects::{object::Object, frame::{Frame, Block, MultiNew}, string::Str, list::List};
use crate::objects::function::{Function, NativeFunction, native_func, len};
use crate::code::binary_file_parser::CodeObject;
use crate::code::{byte_code, get_op, byte_code::compare};
use crate::errors::Errors;
use crate::{info, debug, error};

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
    builtin_funcs: BTreeMap<Str, &'static native_func>
}

impl Drop for Interpreter {
    fn drop(&mut self) {
        colour::red_ln!("dropping frame with codes name: {}", self.frame.codes.co_name);
    }
}

impl Interpreter {
    pub fn new(codes: CodeObject) -> Self {
        let mut builtin_funcs = BTreeMap::<Str, &'static native_func>::new();
        builtin_funcs.insert(Str::from("len"), &len);

        Self {
            frame: Rc::new(Frame::new(codes, None, None)),
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

                byte_code::LOAD_CONST => {self.frame.stack.borrow_mut().push(self.frame.codes.consts[op_arg].clone()); debug!("load const {:?}", self.frame.codes.consts[op_arg]);},
                
                byte_code::LOAD_NAME => {
                    //LGTB principle
                    //first search locals
                    match &self.frame.codes.names[op_arg] {
                        &Object::Str(ref s) => {
                            //LGTB principle
                            //first search locals
                            if let Some(o) = self.frame.locals.borrow().get(s) {
                                self.frame.stack.borrow_mut().push((*o).clone());
                                continue;
                            }
                            
                            //then search globals

                            //then search builtins
                            if let Some(nfp) = self.builtin_funcs.get(s) {
                                self.frame.stack.borrow_mut().push(Object::NativeFunction(NativeFunction::new(&len, s)));
                                continue;
                            }
                        },
                        _ => panic!("Invalid arg {:?}", &self.frame.codes.names[op_arg])
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
                    self.frame.locals.borrow_mut().insert(
                        match &self.frame.codes.names[op_arg] {
                            &Object::Str(ref s) => (*s).clone(),
                            x => return Err(Errors::InvalidObject(format!("{:?}", x)))
                        }, self.frame.stack.borrow_mut().pop().unwrap());
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
                        byte_code::BINARY_ADD => self.frame.stack.borrow_mut().push(p1.add(&p2)?),
                        byte_code::BINARY_SUBTRACT => self.frame.stack.borrow_mut().push(p1.sub(&p2)?),
                        byte_code::BINARY_MULTIPLY => self.frame.stack.borrow_mut().push(p1.mul(&p2)?),
                        byte_code::BINARY_DIVIDE => self.frame.stack.borrow_mut().push(p1.div(&p2)?),
                        byte_code::BINARY_MOD => self.frame.stack.borrow_mut().push(p1.r#mod(&p2)?),
                        byte_code::BINARY_SUBSCR => self.frame.stack.borrow_mut().push(p1.subscr(&p2)?),
                        _ => {}

                    }
                },

                byte_code::STORE_SUBSCR => {
                    
                }

                byte_code::LOAD_ATTR => {
                    
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
                    let mut vd = VecDeque::<Object>::with_capacity(op_arg);
                    while op_arg > 0 {
                        vd.push_front(self.frame.stack.borrow_mut().pop().unwrap());
                        op_arg -= 1;
                    }
                    self.frame.stack.borrow_mut().push(Object::List(List::from(vd)));
                }

                byte_code::POP_JUMP_IF_TRUE => {
                    match self.frame.stack.borrow_mut().pop().unwrap() {
                        Object::True => self.frame.set_pc(op_arg),
                        Object::False => {},
                        _ => panic!("Invalid arg")
                    }
                },

                byte_code::POP_JUMP_IF_FALSE => {
                    match self.frame.stack.borrow_mut().pop().unwrap() {
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
                        let mut defaults = VecDeque::<Object>::with_capacity(op_arg);
                        while op_arg > 0 {
                            defaults.push_front(self.frame.stack.borrow_mut().pop().unwrap());
                            op_arg -= 1;
                        }
                        Some(Vec::from(defaults))
                    } else {None};

                    match code_wrap {
                        Object::CodeObject(code_obj) => {
                            self.frame.stack.borrow_mut().push(Object::Function(Function::new(code_obj, defaults)));
                        },
                        _ => panic!("Invalid code_obj {:?}", code_wrap),
                    }
                }

                byte_code::CALL_FUNCTION => {
                    //receive arguments passed in when the function is called.
                    let args = if op_arg > 0 {
                        let mut args = VecDeque::<Object>::with_capacity(op_arg);
                        while op_arg > 0 {
                            args.push_front(self.frame.stack.borrow_mut().pop().unwrap());
                            op_arg -= 1;
                        }
                        Some(Vec::from(args))
                    } else {None};

                    let func_wrap = self.frame.stack.borrow_mut().pop().unwrap();
                    match func_wrap {
                        Object::Function(func) => {
                            self.build_frame(func, args);
                        },
                        //native function doesn't has a RETURN_VALUE op, so don't build a frame.
                        Object::NativeFunction(nf) => {
                            self.frame.stack.borrow_mut().push(match nf.call(match args {
                                None => Vec::<Object>::new(),
                                Some(v) => v
                            }) {
                                None => Object::r#None,
                                Some(v) => v
                            });
                        }
                        _ => panic!("Invalid function {:?}", func_wrap)
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

    fn build_frame(&mut self, callable: Function, args: Option<Vec<Object>>) {
        let new_frame = Rc::new(Frame::new(callable, args, Some(self.frame.clone())));
        self.frame = new_frame.clone();
    }
}
