/**********************************************
  > File Name		: object/string.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Wed Nov  3 15:48:57 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::collections::BTreeMap;
use std::rc::Rc;
use std::cmp::{PartialOrd, Ord, Ordering, PartialEq, Eq};
use std::ops::Index;
use std::any::Any;

use crate::errors::Errors;
use crate::objects::object::{Object, ObjRef};
use crate::objects::klass::{Klass, KlassRef};
use crate::objects::function::MethodFuncPointer;
use crate::cast;

//pub static mut STR_ATTR: Option<BTreeMap<Str, &'static MethodFuncPointer>> = None;
pub static STRING_KLASS_INSTANCE: StringKlass = StringKlass {mod_str: "str"};
pub static NONE: Rc<Str> = Rc::new(Str::raw_from("None"));
pub static TRUE: Rc<Str> = Rc::new(Str::raw_from("True"));
pub static FALSE: Rc<Str> = Rc::new(Str::raw_from("False"));

#[derive(Clone)]
pub struct Str {
    inner: Vec<u8>,
    klass: &'static StringKlass
}

pub struct StringKlass {
    mod_str: &'static str
}

impl Klass for StringKlass {
    fn as_any(&self) -> &dyn Any {self}
}

impl Object for Str {
    fn as_any(&self) -> &dyn Any {self}

    fn klass(&self) -> KlassRef {
        self.klass
    }
}

impl PartialEq for Str {
    fn eq(&self, other: &Self) -> bool {
        if self.inner.len() != other.inner.len() {
            return false;
        }

        for i in 0..self.inner.len() {
            if self.inner[i] != other.inner[i] {
                return false;
            }
        }
        true
    }
}

impl Eq for Str {}

impl Ord for Str {
    fn cmp(&self, other: &Self) -> Ordering {
        let l1 = self.inner.len();
        let l2 = other.inner.len();
        let len = if l1 < l2 { l1 } else { l2 };

        for i in 0..len {
            if self.inner[i] < other.inner[i] {
                return Ordering::Less;
            } else if self.inner[i] > other.inner[i] {
                return Ordering::Greater;
            }
        }

        l1.cmp(&l2)
    }
}

impl PartialOrd for Str {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Str {
    pub fn new_raw() -> Self {
        Self {
            inner: Vec::new(),
            klass: &STRING_KLASS_INSTANCE
        }
    }

    pub fn raw_from(s: &str) -> Self {
        Self {
            inner: {
                let mut v = vec![0 as u8; s.len()];
                v.clone_from_slice(s.as_bytes());
                v
            },
            klass: &STRING_KLASS_INSTANCE
        }
    }

    pub fn new() -> ObjRef {
        Rc::new(Self {
            inner: Vec::new(),
            klass: &STRING_KLASS_INSTANCE
        })
    }

    pub fn from(s: &str) -> ObjRef {
        Rc::new(Self::raw_from(s))
    }

    pub fn from_vec(v: Vec<u8>) -> ObjRef {
        Rc::new(Self {
            inner: v,
            klass: &STRING_KLASS_INSTANCE
        })
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

    pub fn inner_ref<'a>(&'a self) -> &'a Vec<u8> {
        &self.inner
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

/*
 * define some python str methods
 * store them in a static map
 */
//pub fn upper(owner: ObjRef, args: Vec<ObjRef>) -> Option<ObjRef> {
    //let mut v = Vec::<u8>::new();
    //let s: &Str = match owner.as_ref() {
        //&Object::Str(ref s) => s,
        //_ => panic!("Invalid owner {:?}", owner)
    //};
    //for i in s.inner.iter() {
        //let c = *i as char;
        //if c <= 'z' && c >= 'a' {
            //v.push(*i - 32);
        //} else {
            //v.push(*i);
        //}
    //}
    //Some(Str::from_vec(v))
//}


