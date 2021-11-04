/**********************************************
  > File Name		: runtime/interpreter.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Tue Nov  2 19:59:37 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use crate::objects::{object::Object, integer::Integer, string::Str};
use crate::objects;
use crate::code::binary_file_parser::CodeObject;
use std::collections::HashMap;
use crate::code::ByteCode;

pub struct Interpreter {
    _stack: Vec<Box<dyn Object>>,
    _consts: Vec<Box<dyn Object>>,
    _loop_stack: Vec<Box<dyn Object>>
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            _stack: Vec::new(),
            _consts: Vec::new(),
            _loop_stack: Vec::new()
        }
    }

    pub fn run(&mut self, codes: Box<CodeObject>) {
        let mut pc: usize = 0;//program counter
        let mut code_length = codes.bytecodes.len();
        let names = &mut codes.names;
        let locals: HashMap<*mut dyn Object, *mut dyn Object> = HashMap::new();//K:locals names, V: locals values.

        //let mut b: Box<Block>;
        
        while pc < code_length {
            let op_code:u8 = codes.bytecodes[pc];
            pc += 1;
            let has_argument = (op_code & 0xff) >= ByteCode::HAVE_ARGUMENT;

            let op_arg:i32 = -1;//op_arg of this op_code
            if has_argument {
                let byte1:i32 = (codes.bytecodes[pc] & 0xff) as i32;
                pc += 1;
                op_arg = ((codes.bytecodes[pc] & 0xff) << 8) as i32 | byte1;
            }
            println!("op_arg: {}", op_arg);

            let v: &Box<dyn Object>;
            let w: &Box<dyn Object>;
            let u: &Box<dyn Object>;
            let attr: &Box<dyn Object>;
            let b1: Box<dyn Object>;//for pop
            let b2: Box<dyn Object>;//for pop
            let p1: *mut dyn Object;
            let p2: *mut dyn Object;
            match op_code {
                ByteCode::LOAD_CONST => self._stack.push(self._consts[op_code as usize].clone()),
                
                ByteCode::LOAD_NAME => {
                    p1 = &mut names[op_arg as usize] as *mut _;
                    w = match locals.get(p1) {
                        None => {
                            v.print();
                            panic!("Not found in locals");
                        },
                        Some(s) => objects::as_box::<dyn Object>(s)
                    };
                    self._stack.push(w.clone());
                }

                ByteCode::STORE_NAME => {
                    v = &names[op_arg as usize];
                    p1 = objects::box_clone_ptr::<dyn Object>(v);
                    p2 = match self._stack.pop() {
                        None => {
                            panic!("empty stack");
                        },
                        Some(s) => Box::into_raw(s)
                    };
                    locals.insert(p1, p2);
                }

                ByteCode::PRINT_ITEM => {
                    b1 = match self._stack.pop() {
                        None => {panic!("empty stack");},
                        Some(s) => s 
                    };
                    b1.print();
                }

                ByteCode::PRINT_NEWLINE => {
                    println!("\n");
                }

                ByteCode::BINARY_ADD => {
                    b1 = match self._stack.pop() {
                        None => {panic!("empty stack");},
                        Some(s) => s 
                    };
                    b2 = match self._stack.pop() {
                        None => {panic!("empty stack");},
                        Some(s) => s 
                    };
                    self._stack.push(match b1.add(b2.as_ref()) {
                        None => {
                            panic!("type doesn't impl add");
                        },
                        Some(v) => v
                    });
                }

                ByteCode::RETURN_VALUE => {
                    self._stack.pop();
                }

                _ => {
                    panic!("Unkown op_code {}", op_code);
                }
            }
        }
    }
}
