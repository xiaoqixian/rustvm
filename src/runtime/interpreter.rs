/**********************************************
  > File Name		: runtime/interpreter.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Tue Nov  2 19:59:37 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

#[warn(unused_imports)]
use crate::objects::{object::Object, integer::Integer, string::Str};
use crate::objects;
use crate::code::binary_file_parser::CodeObject;
use std::collections::HashMap;
use crate::code::{byte_code, get_op};

pub struct Interpreter {
    _stack: Vec<*mut dyn Object>,
    _loop_stack: Vec<*mut dyn Object>
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            _stack: Vec::new(),
            _loop_stack: Vec::new()
        }
    }

    pub fn run(&mut self, codes: Box<CodeObject>) {
        let mut pc: usize = 0;//program counter
        let code_length = codes.bytecodes.len();
        let names:&Vec<*mut dyn Object> = &codes.names;
        let consts = &codes.consts;
        let mut locals: HashMap<*mut dyn Object, *mut dyn Object> = HashMap::new();//K:locals names, V: locals values.
        let mut op_code: u8;
        let mut op_arg = -1;

        //let mut b: Box<Block>;
        print!("bytecodes: ");
        codes.bytecodes.print_vec_hex();
        
        while pc < code_length {
            op_code = codes.bytecodes[pc];
            pc += 1;
            let has_argument = (op_code & 0xff) >= byte_code::HAVE_ARGUMENT;

            op_arg = -1;//op_arg of this op_code
            if has_argument {
                let byte1:i32 = (codes.bytecodes[pc] & 0xff) as i32;
                pc += 1;
                op_arg = (((codes.bytecodes[pc] & 0xff) as i32) << 8) | byte1;
                pc += 1;
            }
            println!("op: {:?}, op_code: {:x}, op_arg: {:x}", get_op(op_code).unwrap(), op_code, op_arg);

/*            let v: &*mut dyn Object;*/
            /*let w: &*mut dyn Object;*/
            /*let u: &*mut dyn Object;*/
            /*let attr: &*mut dyn Object;*/
            let b1: &dyn Object;
            let b2: &dyn Object;
            let p1: *mut dyn Object;
            let p2: *mut dyn Object;
            match op_code {
                byte_code::LOAD_CONST => self._stack.push(consts[op_arg as usize].clone()),//reference can be directly cloned.
                
                byte_code::LOAD_NAME => {
                    p1 = names[op_arg as usize];
                    p2 = match locals.get(&p1) {
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
                    self._stack.push(p2);
                }

                byte_code::STORE_NAME => {
                    p1 = names[op_arg as usize];
                    p2 = match self._stack.pop() {
                        None => {
                            panic!("empty stack");
                        },
                        Some(s) => s
                    };
                    locals.insert(p1, p2);
                }

                byte_code::PRINT_ITEM => {
                    p1 = match self._stack.pop() {
                        None => {panic!("empty stack");},
                        Some(s) => s 
                    };
                    match unsafe {p1.as_ref()} {
                        None => {panic!("print null item");},
                        Some(r) => {r.print();}
                    }
                }

                byte_code::PRINT_NEWLINE => {
                    println!("\n");
                }

                byte_code::BINARY_ADD => {
                    p1 = match self._stack.pop() {
                        None => {panic!("empty stack");},
                        Some(s) => s 
                    };
                    p2 = match self._stack.pop() {
                        None => {panic!("empty stack");},
                        Some(s) => s 
                    };
                    b1 = match unsafe {p1.as_ref()} {
                        None => {panic!("p1 null pointer");},
                        Some(r) => r
                    };
                    self._stack.push(match b1.add(p2 as *const _) {
                        None => {
                            panic!("type doesn't impl add");
                        },
                        Some(v) => v
                    });
                }

                byte_code::RETURN_VALUE => {
                    self._stack.pop();
                }

                _ => {
                    panic!("Unkown op_code {}", op_code);
                }
            }
        }
    }
}
