/**********************************************
  > File Name		: object/string.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Wed Nov  3 15:48:57 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use crate::errors::Errors;
use std::ops::Index;

#[derive(Clone, Hash)]
pub struct Str {
    inner: Vec<u8>
}

impl std::cmp::PartialEq for Str {
    fn eq(&self, other: &Self) -> bool {
        if self.inner.len() != other.len() {
            return false;
        }
        for i in 0..self.inner.len() {
            if self.inner[i] != other[i] {
                return false;
            }
        }
        true
    }
}

impl std::cmp::Eq for Str {}

impl Str {
    pub fn from(s: &str) -> Self {
        Self {
            inner: {
                let mut v = vec![0 as u8; s.len()];
                v.clone_from_slice(s.as_bytes());
                v
            }
        }
    }

    pub fn new() -> Self {
        Self {inner: Vec::new()}
    }

    pub fn push(&mut self, c: char) {
        match c as u32 {
            0..=255 => self.inner.push(c as u8),
            _ => {panic!("Doesn't support char out of ASCII: <{}>", c);}
        }
    }

    pub fn into<'a>(&'a self) -> Result<&'a str, Errors> {
        match std::str::from_utf8(self.inner.as_slice()) {
            Ok(v) => Ok(v),
            Err(_) => Err(Errors::Utf8Error(format!("{:?}", self.inner)))
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn print_vec_hex(&self) {
        print!("[");
        for v in self.inner.iter() {
            print!("{:x} ", v);
        }
        println!("]");
    }
}

impl Index<usize> for Str {
    type Output = u8;
    fn index(&self, idx: usize) -> &Self::Output {
        if idx >= self.inner.len() {
            panic!("index out of string bounds");
        }
        &self.inner[idx]
    }
}

impl std::fmt::Debug for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Str")
            .field("inner", &self.into().unwrap())
            .finish()
    }
}

impl std::fmt::Display for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into().unwrap())
    }
}
