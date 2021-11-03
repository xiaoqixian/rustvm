/**********************************************
  > File Name		: object/string.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Wed Nov  3 15:48:57 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use super::object::Object;

#[derive(Debug, Clone)]
pub struct Str {
    val: Vec<u8>
}

impl Str {
    pub fn from(s: &str) -> Box<dyn Object> {
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

    pub fn new() -> Box<dyn Object> {
        Box::new(Str {val: Vec::new()})
    }

    pub fn push(&mut self, c: char) {
        match c.len_utf8() {
            1 => self.val.push(c as u8),
            _ => {panic!("Doesn't support char out of ASCII");}
        }
    }

    pub fn get<'a>(&'a self) -> &'a str {
        str::from_utf8(self.val)
    }
}

impl Object for Str {
}
