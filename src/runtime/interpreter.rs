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

use crate::objects::{Object, string::Str, integer::Integer, list::List, map::Dict, function::{Function, Method}, klass::Klass, r#type::TypeObject};
use super::frame::Frame;
use crate::errors::Errors;
use crate::code::{byte_code, get_op, code_object::CodeObject};
use crate::{info, debug, cast, cast_mut};

const OK: u8 = 0;
const RETURN: u8 = 1;

pub struct Interpreter {
    frame: Rc<Frame>,
    ret_value: Option<Object>,
    //builtin_funcs: BTreeMap<Str, &'static NativeFuncPointer>
}

//impl Drop for Interpreter {
    //fn drop(&mut self) {
        //colour::red_ln!("dropping frame with codes name: {}", self.frame.codes.as_ref().co_name);
    //}
//}

impl Interpreter {
    pub fn new(codes: Object) -> Self {
        //add Str attributes
/*        unsafe {*/
            /*let mut bmap = BTreeMap::<Str, &'static MethodFuncPointer>::new();*/
            /*bmap.insert(Str::raw_from("upper"), &upper);*/
  
            /*STR_ATTR = Some(bmap);*/
        /*}*/

        /*let mut builtin_funcs = BTreeMap::<Str, &'static NativeFuncPointer>::new();*/
        /*builtin_funcs.insert(Str::raw_from("len"), &len);*/

        Self {
            frame: Frame::new(codes, None, None),
            ret_value: None,
            //builtin_funcs
        }
    }

    pub fn run(&mut self) -> Result<(), Errors> {
        let mut op_code: u8 = 0;
        let mut op_arg: usize = usize::MAX;
        let mut status: u8 = 0;

        debug!("first frame globals: {:?}", self.frame.globals);

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
                    self.frame.stack.borrow_mut().push(self.frame.get_const(op_arg));
                }
                
                byte_code::LOAD_NAME => {
                    self.frame.stack.borrow_mut().push(match self.frame.get_local(self.frame.get_name(op_arg)) {
                        Some(v) => v,
                        None => {
                            panic!("name {:?} not found", self.frame.get_name(op_arg))
                        }
                    });
                },

                byte_code::LOAD_FAST => {
                    debug!("load fast {:?}", self.frame.fast_locals.as_ref().unwrap().borrow()[op_arg]);
                    self.frame.stack.borrow_mut().push(self.frame.fast_locals.as_ref().unwrap().borrow()[op_arg].clone());
                },

                byte_code::STORE_FAST => {
                    *&mut self.frame.fast_locals.as_ref().unwrap().borrow_mut()[op_arg] = self.frame.stack.borrow_mut().pop().unwrap();
                }

                byte_code::STORE_NAME => {
                    self.frame.store_name(op_arg, self.frame.stack.borrow_mut().pop().unwrap());
                },

                byte_code::LOAD_ATTR => {
                    let owner = self.frame.stack.borrow_mut().pop().unwrap();

                    self.frame.stack.borrow_mut().push(match owner.get_attr(owner.clone(), &self.frame.get_name(op_arg)) {
                        None => panic!("Attr {:?} not found in {:?}", self.frame.get_name(op_arg), owner),
                        Some(v) => v
                    });
                },

                byte_code::STORE_ATTR => {
                    let owner = self.frame.stack.borrow_mut().pop().unwrap();
                    let item = self.frame.stack.borrow_mut().pop().unwrap();
                    let name = self.frame.get_name(op_arg);
                    
                    owner.set_attr(owner.clone(), name, item);
                },

                byte_code::LOAD_LOCALS => {
                    self.frame.stack.borrow_mut().push(self.frame.locals.borrow().clone());
                }

                byte_code::PRINT_ITEM => {
                    self.frame.stack.borrow_mut().pop().unwrap().print();
                },

                byte_code::PRINT_NEWLINE => {
                    println!("\n");
                },

                //byte_code::BINARY_ADD | byte_code::BINARY_SUBTRACT |
                //byte_code::BINARY_MULTIPLY | byte_code::BINARY_DIVIDE |
                //byte_code::BINARY_MOD | byte_code::BINARY_SUBSCR => {
                    //let p2 = self.frame.stack.borrow_mut().pop().unwrap();
                    //let p1 = self.frame.stack.borrow_mut().pop().unwrap();
                    //match op_code {
                        //byte_code::BINARY_ADD => self.frame.stack.borrow_mut().push(Rc::new(p1.add(&p2)?)),
                        //byte_code::BINARY_SUBTRACT => self.frame.stack.borrow_mut().push(Rc::new(p1.sub(&p2)?)),
                        //byte_code::BINARY_MULTIPLY => self.frame.stack.borrow_mut().push(Rc::new(p1.mul(&p2)?)),
                        //byte_code::BINARY_DIVIDE => self.frame.stack.borrow_mut().push(Rc::new(p1.div(&p2)?)),
                        //byte_code::BINARY_MOD => self.frame.stack.borrow_mut().push(Rc::new(p1.r#mod(&p2)?)),
                        //byte_code::BINARY_SUBSCR => self.frame.stack.borrow_mut().push(Rc::new(p1.subscr(&p2)?)),
                        //_ => {}

                    //}
                //},

/*                byte_code::STORE_SUBSCR => {*/
                /*}*/

                /*byte_code::LOAD_ATTR => {*/
                    /*//first get owner*/
                    /*let owner = self.frame.stack.borrow_mut().pop().unwrap();*/
                    /*//op_arg is the index of the attribute in names*/
                    /*let attr = match unwrap_obj!(self.frame.codes, CodeObject).names[op_arg].as_ref() {*/
                        /*&Object::Str(ref s) => s,*/
                        /*e => panic!("Invalid arg {:?}", e)*/
                    /*};*/
                    
                    /*self.frame.stack.borrow_mut().push(Rc::new(Object::get_attr(owner, attr)));*/
                /*}*/

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
                    self.frame.stack.borrow_mut().push(List::from_vd(vd));
                },

                byte_code::SLICE0 => {
                    //slice0 indexes the whole list
                    //as the whole list is already on the top of the stack,
                    //so we don't need to do anything
                },

                byte_code::SLICE1 => {
                    //slice1 indexes from an start index to the end
                    let start = self.frame.stack.borrow_mut().pop().unwrap();
                    let lst = self.frame.stack.borrow_mut().pop().unwrap();

                    self.frame.stack.borrow_mut().push(cast!(lst, List).range_index(Some(start), None));
                },

                byte_code::SLICE2 => {
                    //slice2 indexes from the start to an end index.
                    let end = self.frame.stack.borrow_mut().pop().unwrap();
                    let lst = self.frame.stack.borrow_mut().pop().unwrap();

                    self.frame.stack.borrow_mut().push(cast!(lst, List).range_index(None, Some(end)));
                },

                byte_code::SLICE3 => {
                    //slice3 indexes from a start index to an end index.
                    let end = self.frame.stack.borrow_mut().pop().unwrap();
                    let start = self.frame.stack.borrow_mut().pop().unwrap();
                    let lst = self.frame.stack.borrow_mut().pop().unwrap();

                    self.frame.stack.borrow_mut().push(cast!(lst, List).range_index(Some(start), Some(end)));
                },

                byte_code::BUILD_MAP => {
                    self.frame.stack.borrow_mut().push(Dict::new());
                },

                byte_code::STORE_MAP => {
                    let v = self.frame.stack.borrow_mut().pop().unwrap();
                    let k = self.frame.stack.borrow_mut().pop().unwrap();
                    let mut m = self.frame.stack.borrow_mut().pop().unwrap();
                    //let map = cast_mut!(&mut m, Dict);
                    let map = match Rc::get_mut(&mut m) {
                        None => panic!("call get_mut on {:?} with {} strong count", m, Rc::strong_count(&m)),
                        Some(o) => {
                            match o.as_any_mut().downcast_mut::<Dict>() {
                                None => panic!("Invalid {:?}", m),
                                Some(v) => v
                            }
                        }
                    };

                    map.put(k, v);
                    self.frame.stack.borrow_mut().push(m);
                }

                /*byte_code::POP_JUMP_IF_TRUE => {*/
                    /*match self.frame.stack.borrow_mut().pop().unwrap().as_ref() {*/
                        /*Object::True => self.frame.set_pc(op_arg),*/
                        /*Object::False => {},*/
                        /*_ => panic!("Invalid arg")*/
                    /*}*/
                /*},*/

                /*byte_code::POP_JUMP_IF_FALSE => {*/
                    /*match self.frame.stack.borrow_mut().pop().unwrap().as_ref() {*/
                        /*Object::False => self.frame.set_pc(op_arg),*/
                        /*Object::True => {},*/
                        /*_ => panic!("Invalid arg")*/
                    /*}*/
                /*},*/

                /*byte_code::JUMP_ABSOLUTE => {*/
                    /*self.frame.set_pc(op_arg);*/
                /*}*/

                /*byte_code::JUMP_FORWARD => {*/
                    /*self.frame.add_pc_n(op_arg);*/
                /*},*/

                /*byte_code::SETUP_LOOP => {//op_arg: target address after loop*/
                    /*let block = Block::new(op_code, self.frame.get_pc() + op_arg, self.frame.stack.borrow_mut().len());*/
                    /*self.frame.loop_stack.borrow_mut().push(block);*/
                /*},*/

                /*byte_code::POP_BLOCK => {*/
                    /*let block = self.frame.loop_stack.borrow_mut().pop().unwrap();*/
                    /*if self.frame.stack.borrow_mut().len() < block.level {*/
                        /*panic!("stack invaded");*/
                    /*}*/
                    /*while self.frame.stack.borrow_mut().len() > block.level {*/
                        /*self.frame.stack.borrow_mut().pop();*/
                    /*}*/
                /*},*/

                /*byte_code::BREAK_LOOP => {*/
                    /*let block = self.frame.loop_stack.borrow_mut().pop().unwrap();*/
                    /*if self.frame.stack.borrow_mut().len() < block.level {*/
                        /*panic!("stack invaded");*/
                    /*}*/
                    /*while self.frame.stack.borrow_mut().len() > block.level {*/
                        /*self.frame.stack.borrow_mut().pop();*/
                    /*}*/
                    /*self.frame.set_pc(block.target);*/
                /*},*/

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

                    self.frame.stack.borrow_mut().push(Function::from_code(code_wrap, defaults, Some(self.frame.globals.clone())));
                }

                byte_code::CALL_FUNCTION => {
                    //receive arguments for the function is called.
                    let args = if op_arg > 0 {
                        let mut args = VecDeque::<Object>::with_capacity(op_arg);
                        while op_arg > 0 {
                            args.push_front(self.frame.stack.borrow_mut().pop().unwrap());
                            op_arg -= 1;
                        }
                        Some(Vec::from(args))
                    } else {None};

                    let func_wrap = self.frame.stack.borrow_mut().pop().unwrap();
                    self.build_frame(func_wrap, args);
/*                    match func_wrap.klass() {*/
                        /*Klass::FunctionKlass => {*/
                            /*self.build_frame(func_wrap, args);*/
                        /*},*/

                        /*Klass::MethodKlass => {*/
                            /*let method = cast!(func_wrap, Method);*/
                            /*let m_args = match args {*/
                                /*None => Some(vec![method.owner.clone(); 1]),*/
                                /*Some(v) => {*/
                                    /*let mut vd = VecDeque::from(v);*/
                                    /*vd.push_front(method.owner.clone());*/
                                    /*Some(Vec::from(vd))*/
                                /*}*/
                            /*};*/
                            /*self.build_frame(method.func.clone(), m_args);*/
                        /*}*/
                        /*//native function doesn't has a RETURN_VALUE op, so don't build a frame.*/
                        /*//&Object::NativeFunction(ref nf) => {*/
                            /*//self.frame.stack.borrow_mut().push(match nf.call(match args {*/
                                /*//None => Vec::<Object>::new(),*/
                                /*//Some(v) => v*/
                            /*//}) {*/
                                /*//None => Rc::new(Object::r#None),*/
                                /*//Some(v) => v*/
                            /*//});*/
                        /*//},*/

                        /*////methods*/
                        /*//&Object::Method(ref m) => {*/
                            /*//self.frame.stack.borrow_mut().push(match m.call(match args {*/
                                /*//None => Vec::<Object>::new(),*/
                                /*//Some(v) => v*/
                            /*//}) {*/
                                /*//None => Rc::new(Object::r#None),*/
                                /*//Some(v) => v*/
                            /*//});*/
                        /*//}*/
                        /*v => panic!("Invalid function {:?}", v)*/
                    /*}*/
                },

                byte_code::BUILD_CLASS => {
                    let attrs = self.frame.stack.borrow_mut().pop().unwrap();
                    let supers = self.frame.stack.borrow_mut().pop().unwrap();
                    let name = self.frame.stack.borrow_mut().pop().unwrap();

                    self.frame.stack.borrow_mut().push(TypeObject::new(&attrs, &name));
                },

                byte_code::RETURN_VALUE => {
                    let ret_value = self.frame.stack.borrow_mut().pop().unwrap();
                    self.ret_value = Some(ret_value.clone());
                    if self.frame.entry_frame {
                        if let Some(f) = &self.frame.sender {
                            self.frame = f.clone();
                        }
                        return Ok(());
                    }
                    match &self.frame.sender {
                        Some(f) => {
                            self.frame = f.clone();
                            self.frame.stack.borrow_mut().push(ret_value);
                        },
                        None => return Ok(()),
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
    fn build_frame(&mut self, callable: Object, args: Option<Vec<Object>>) {
        match callable.klass() {
            Klass::FunctionKlass => {
                self.frame = Frame::new(callable, args, Some(self.frame.clone()));
            },
            Klass::MethodKlass => {
                let method = cast!(callable, Method);
                let func = cast!(method.func, Function);

                let m_args = match args {
                    None => vec![method.owner.clone(); 1],
                    Some(v) => {
                        let mut vd = VecDeque::from(v);
                        vd.push_front(method.owner.clone());
                        Vec::from(vd)
                    }
                };
                match func.nfp {
                    Some(nfp) => {
                        self.frame.stack.borrow_mut().push(match nfp(m_args) {
                            None => crate::objects::BuiltinValue::new("None"),
                            Some(v) => v
                        });
                    },
                    None => {
                        self.build_frame(method.func.clone(), Some(m_args));
                    }
                }
            },
            Klass::TypeKlass => {
                let tp = cast!(callable, TypeObject);
                let ins = tp.allocate_instance();
                if let Some(init) = ins.get_attr(ins.clone(), &Str::from("__init__")) {
                    self.frame.stack.borrow_mut().push(ins.clone());

                    let m_args = match args {
                        None => Some(vec![ins; 1]),
                        Some(v) => {
                            let mut vd = VecDeque::from(v);
                            vd.push_front(ins);
                            Some(Vec::from(vd))
                        }
                    };
                    self.frame = Frame::new(cast!(init, Method).func.clone(), m_args, Some(self.frame.clone()));
                    Rc::make_mut(&mut self.frame).entry_frame = true;
                    self.run().unwrap();
                }
            }
            v => panic!("Invalid callable klass {:?}", v)
        }
    }
}
