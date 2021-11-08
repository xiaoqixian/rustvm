/**********************************************
  > File Name		: object/string.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Wed Nov  3 15:48:57 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use super::object::Object;
use crate::errors::Errors;
use std::ops::Index;

#[derive(Debug, Clone)]
pub struct Str {
    val: Vec<u8>
}

impl Str {
    pub fn from(s: &str) -> Box<Str> {
        Box::new(Str {
            val: {
                let mut v = Vec::new();
                for b in s.bytes() {
                    v.push(b);
                }
                v
            }
        })
    }

    pub fn new() -> Box<Str> {
        Box::new(Str {val: Vec::new()})
    }

    pub fn push(&mut self, c: char) {
        match c as u32 {
            0..=255 => self.val.push(c as u8),
            _ => {panic!("Doesn't support char out of ASCII: <{}>", c);}
        }
    }

    pub fn get<'a>(&'a self) -> Result<&'a str, Errors> {
        match std::str::from_utf8(self.val.as_slice()) {
            Ok(v) => Ok(v),
            Err(_) => Err(Errors::Utf8Error(format!("{:?}", self.val)))
        }
    }

    pub fn len(&self) -> usize {
        self.val.len()
    }

    pub fn print_vec_hex(&self) {
        print!("[");
        for v in self.val.iter() {
            print!("{:x} ", v);
        }
        println!("]");
    }
}

impl Object for Str {
    fn print(&self) {
        match self.get() {
            Ok(v) => {
                colour::blue_ln!("{}", v);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}

impl Index<usize> for Str {
    type Output = u8;
    fn index(&self, idx: usize) -> &Self::Output {
        if idx > self.val.len() {
            panic!("index out of string bounds");
        }
        &self.val[idx]
    }
}
