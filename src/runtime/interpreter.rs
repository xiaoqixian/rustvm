/**********************************************
  > File Name		: runtime/interpreter.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Tue Nov  2 19:59:37 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

#[warn(unused_imports)]
use crate::objects::{object::Object, frame::{Frame, Block, MultiNew}, integer::Integer, function::Function};
use crate::objects::statics::{FALSE, TRUE};
use crate::code::binary_file_parser::CodeObject;
use crate::code::{byte_code, get_op, byte_code::compare};
use crate::{unwrap_option, as_ref, as_mut};

macro_rules! pop {
    ($self:ident$(, $field:ident)*) => {
        match $self$(.$field)*.pop() {
            None => {panic!("empty stack pop");},
            Some(v) => v
        }
    }
}

macro_rules! ptr_eq {
    ($p1:ident, $p2:ident) => {{
        ($p1 as *mut Integer as u64) == ($p2 as *mut Integer as u64)
    }}
}

pub struct Interpreter {
    pub frame: Box<Frame>,
    pub frame_ptr: *mut Frame
}

impl Drop for Interpreter {
    fn drop(&mut self) {
        colour::red_ln!("dropping frame with codes name: {}", as_ref!(self, frame, codes).co_name.get().unwrap());
    }
}

#[warn(unused_assignments)]
impl Interpreter {
    pub fn new(codes: *mut CodeObject) -> Self {
        let new_frame = Frame::new(codes);
        Interpreter {
            frame: unsafe {Box::from_raw(new_frame)},
            frame_ptr: new_frame
        }
    }

    pub fn run(&mut self) {
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
            let has_argument = (op_code & 0xff) >= byte_code::HAVE_ARGUMENT;
            if has_argument {
                op_arg = self.frame.get_oparg();
            }

            println!("op: {:?}, op_code: {:x}, op_arg: {:x}", match get_op(op_code) {
                None => {panic!("invalid op_code: {}", op_code);},
                Some(e) => e
            }, op_code, op_arg);

/*            let mut b1: &dyn Object;*/
            /*let mut b2: &dyn Object;*/
            let mut p1: *mut dyn Object;
            let mut p2: *mut dyn Object;
            match op_code {
                byte_code::POP_TOP => {assert_ne!(self.frame.stack.pop(), None);}

                byte_code::LOAD_CONST => self.frame.stack.push(as_ref!(self, frame, codes).consts[op_arg as usize].clone()),//reference can be directly cloned.
                
                byte_code::LOAD_NAME => {
                    p1 = as_ref!(self, frame, codes).names[op_arg];
                    p2 = match self.frame.locals.get(&p1) {
                        None => {
                            match unsafe {p1.as_ref()} {
                                None => {println!("op_arg {} represents a null pointer", op_arg);},
                                Some(r) => {r.print();}
                            }
                            panic!("Not found in locals");
                        },
                        Some(s) => {
                            //s: &*mut dyn Object
                            *s
                        }
                    };
                    self.frame.stack.push(p2);
                },

                byte_code::STORE_NAME => {
                    p1 = as_ref!(self, frame, codes).names[op_arg as usize];
                    p2 = pop!(self, frame, stack);
                    self.frame.locals.insert(p1, p2);
                },

                byte_code::PRINT_ITEM => {
                    p1 = pop!(self, frame, stack);
                    match unsafe {p1.as_ref()} {
                        None => {panic!("print null item");},
                        Some(r) => {r.print();}
                    }
                },

                byte_code::PRINT_NEWLINE => {
                    println!("\n");
                },

                byte_code::BINARY_ADD | byte_code::INPLACE_ADD => {
                    p2 = pop!(self, frame, stack);
                    p1 = pop!(self, frame, stack);
                    let b1 = as_ref!(p1);
                    p1 = unwrap_option!(b1.add(p2 as *const _));
                    self.frame.stack.push(p1);
                },

                byte_code::BINARY_SUBTRACT => {
                    p2 = pop!(self, frame, stack);
                    p1 = pop!(self, frame, stack);
                    let b1 = as_ref!(p1);
                    p1 = unwrap_option!(b1.sub(p2 as *const _));
                    self.frame.stack.push(p1);
                },

                byte_code::INPLACE_SUBSTRACT => {
                    p2 = pop!(self, frame, stack);
                    p1 = pop!(self, frame, stack);
                    let b1 = as_mut!(p1);
                    b1.inplace_sub(p2 as *const _);
                    self.frame.stack.push(p1);
                },

                byte_code::BINARY_MULTIPLY | byte_code::INPLACE_MULTIPLY => {
                    p2 = pop!(self, frame, stack);
                    p1 = pop!(self, frame, stack);
                    let b1 = as_ref!(p1);
                    p1 = unwrap_option!(b1.mul(p2 as *const _));
                    self.frame.stack.push(p1);
                },

                byte_code::BINARY_DIVIDE | byte_code::INPLACE_DIVIDE => {
                    p2 = pop!(self, frame, stack);
                    p1 = pop!(self, frame, stack);
                    let b1 = as_ref!(p1);
                    p1 = unwrap_option!(b1.div(p2 as *const _));
                    self.frame.stack.push(p1);
                },

                byte_code::COMPARE_OP => {
                    p2 = pop!(self, frame, stack);
                    p1 = pop!(self, frame, stack);
                    let b1 = as_ref!(p1);
                    
                    let res = match op_arg as u8 {
                        compare::LESS => b1.less(p2 as *const _),
                        compare::LESS_EQUAL => b1.le(p2 as *const _),
                        compare::EQUAL => b1.equal(p2 as *const _),
                        compare::NOT_EQUAL => b1.ne(p2 as *const _),
                        compare::GREATER => b1.greater(p2 as *const _),
                        compare::GREATER_EQUAL => b1.ge(p2 as *const _),

                        _ => {panic!("invalid compare op_arg: {}", op_arg)}
                    };

                    match res {
                        None => {
                            print!("operators: {{");
                            b1.print();
                            unsafe {
                                p2.as_ref().unwrap().print();
                            }
                            println!("}}");
                            panic!("invalid operations");
                        },
                        Some(p) => self.frame.stack.push(p),
                    }
                },

                byte_code::POP_JUMP_IF_TRUE => {
                    p1 = pop!(self, frame, stack);
                    if ptr_eq!(p1, TRUE) {
                        self.frame.pc = op_arg;
                    }
                },

                byte_code::POP_JUMP_IF_FALSE => {
                    p1 = pop!(self, frame, stack);
                    if ptr_eq!(p1, FALSE) {
                        self.frame.pc = op_arg;
                    }
                },

                byte_code::JUMP_ABSOLUTE => {
                    self.frame.pc = op_arg;
                }

                byte_code::JUMP_FORWARD => {
                    self.frame.pc += op_arg;
                },

                byte_code::SETUP_LOOP => {//op_arg: target address after loop
                    let block = Block::new(op_code, self.frame.pc + op_arg, self.frame.stack.len());
                    self.frame.loop_stack.push(block);
                },

                byte_code::POP_BLOCK => {
                    let block = pop!(self, frame, loop_stack);
                    if self.frame.stack.len() < block.level {
                        panic!("stack invaded");
                    }
                    while self.frame.stack.len() > block.level {
                        self.frame.stack.pop();
                    }
                },

                byte_code::BREAK_LOOP => {
                    let block = pop!(self, frame, loop_stack);
                    if self.frame.stack.len() < block.level {
                        panic!("stack invaded");
                    }
                    while self.frame.stack.len() > block.level {
                        self.frame.stack.pop();
                    }
                    self.frame.pc = block.target;
                },

                byte_code::MAKE_FUNCTION => {
                    p1 = pop!(self, frame, stack);//bytecodes of the function
                    self.frame.stack.push(Function::new(p1 as *mut CodeObject));
                }

                byte_code::CALL_FUNCTION => {
                    p1 = pop!(self, frame, stack);
                    self.build_frame(p1);
                }

                byte_code::RETURN_VALUE => {
                    p1 = pop!(self, frame, stack);//frame return value
                    let sender = self.frame.sender;
                    //unsafe {std::mem::drop(*(self.frame_ptr));}
                    self.frame_ptr = sender;
                    self.frame = unsafe {Box::from_raw(sender)};
                },

                _ => {
                    panic!("Unkown op_code {}", op_code);
                }
            }
        }
    }

    fn build_frame(&mut self, callable: *mut dyn Object) {
        let new_frame = Frame::new(callable as *mut Function);
        unsafe {(*new_frame).sender = self.frame_ptr;}
        self.frame_ptr = new_frame;
        self.frame = unsafe {Box::from_raw(new_frame)};
    }
}
