/**********************************************
  > File Name		: runtime/interpreter.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Tue Nov  2 19:59:37 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::rc::Rc;

use crate::objects::{object::Object, frame::{Frame, Block, MultiNew}, function::Function};
use crate::code::binary_file_parser::CodeObject;
use crate::code::{byte_code, get_op, byte_code::compare};
use crate::errors::Errors;

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
    pub frame: Rc<Frame>
}

impl Drop for Interpreter {
    fn drop(&mut self) {
        colour::red_ln!("dropping frame with codes name: {}", self.frame.codes.co_name);
    }
}

impl Interpreter {
    pub fn new(codes: CodeObject) -> Self {
        Self {
            frame: Rc::new(Frame::new(codes))
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
            op_code = Rc::get_mut(&mut self.frame).unwrap().get_opcode();
            let has_argument = (op_code & 0xff) >= byte_code::HAVE_ARGUMENT;
            if has_argument {
                op_arg = Rc::get_mut(&mut self.frame).unwrap().get_oparg();
            }

            println!("op: {:?}, op_code: {:x}, op_arg: {:x}", match get_op(op_code) {
                None => {panic!("invalid op_code: {}", op_code);},
                Some(e) => e
            }, op_code, op_arg);

            match op_code {
                byte_code::POP_TOP => assert!(self.frame.stack.borrow_mut().pop().is_some()),

                byte_code::LOAD_CONST => self.frame.stack.borrow_mut().push(self.frame.codes.consts[op_arg as usize].clone()),
                
                byte_code::LOAD_NAME => {
                    self.frame.stack.borrow_mut().push(
                        match &self.frame.codes.names[op_arg as usize] {
                            &Object::Str(ref s) => {
                                match self.frame.locals.borrow().get(s) {
                                    None => panic!("op_arg {} as name {} gets nothing", op_arg, s),
                                    Some(o) => (*o).clone()
                                }
                            },
                            _ => panic!("Invalid arg"),
                        }
                    );
                },

                byte_code::STORE_NAME => {
                    self.frame.locals.borrow_mut().insert(
                        match &self.frame.codes.names[op_arg] {
                            &Object::Str(ref s) => (*s).clone(),
                            x => return Err(Errors::InvalidObject(format!("{:?}", x)))
                        }, self.frame.stack.borrow_mut().pop().unwrap());
                },

                byte_code::PRINT_ITEM => {
                    self.frame.stack.borrow_mut().pop().unwrap().print();
                },

                byte_code::PRINT_NEWLINE => {
                    println!("\n");
                },

                byte_code::BINARY_ADD | byte_code::INPLACE_ADD => {
                    let p2 = self.frame.stack.borrow_mut().pop().unwrap();
                    let p1 = self.frame.stack.borrow_mut().pop().unwrap();
                    self.frame.stack.borrow_mut().push(p1.add(&p2)?);
                },

/*                byte_code::BINARY_SUBTRACT => {*/
                    /*p2 = pop!(self, frame, stack);*/
                    /*p1 = pop!(self, frame, stack);*/
                    /*let b1 = as_ref!(p1);*/
                    /*p1 = unwrap_option!(b1.sub(p2 as *const _));*/
                    /*self.frame.stack.borrow_mut().push(p1);*/
                /*},*/

                /*byte_code::INPLACE_SUBSTRACT => {*/
                    /*p2 = pop!(self, frame, stack);*/
                    /*p1 = pop!(self, frame, stack);*/
                    /*let b1 = as_mut!(p1);*/
                    /*b1.inplace_sub(p2 as *const _);*/
                    /*self.frame.stack.borrow_mut().push(p1);*/
                /*},*/

                /*byte_code::BINARY_MULTIPLY | byte_code::INPLACE_MULTIPLY => {*/
                    /*p2 = pop!(self, frame, stack);*/
                    /*p1 = pop!(self, frame, stack);*/
                    /*let b1 = as_ref!(p1);*/
                    /*p1 = unwrap_option!(b1.mul(p2 as *const _));*/
                    /*self.frame.stack.borrow_mut().push(p1);*/
                /*},*/

                /*byte_code::BINARY_DIVIDE | byte_code::INPLACE_DIVIDE => {*/
                    /*p2 = pop!(self, frame, stack);*/
                    /*p1 = pop!(self, frame, stack);*/
                    /*let b1 = as_ref!(p1);*/
                    /*p1 = unwrap_option!(b1.div(p2 as *const _));*/
                    /*self.frame.stack.borrow_mut().push(p1);*/
                /*},*/

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
                    match self.frame.stack.borrow_mut().pop().unwrap() {
                        Object::CodeObject(code_obj) => {
                            self.frame.stack.borrow_mut().push(Object::Function(Function::new(code_obj)));
                        },
                        _ => panic!("Invalid arg"),
                    }
                }

                byte_code::CALL_FUNCTION => {
                    let func_wrap = self.frame.stack.borrow_mut().pop().unwrap();
                    match func_wrap {
                        Object::Function(func) => {
                            self.build_frame(func);
                        },
                        _ => panic!("Invalid arg")
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

    fn build_frame(&mut self, callable: Function) {
        let new_frame = Rc::new(Frame::new_with_sender(callable, self.frame.clone()));
        self.frame = new_frame.clone();
    }
}
