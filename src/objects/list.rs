/**********************************************
  > File Name		: objects/list.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Thu 04 Nov 2021 04:37:58 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::rc::Rc;

use super::{Object, object::Object as ObjectTrait, klass::Klass, integer::Integer};
use crate::cast;

#[derive(Clone, Debug)]
pub struct List {
    inner: Vec<Object> 
}

impl List {
    pub fn new() -> Object {
        Rc::new(Self {
            inner: Vec::new()
        })
    }

    pub fn with_capacity(size: usize) -> Object {
        Rc::new(Self {
            inner: Vec::with_capacity(size)
        })
    }

    pub fn from_vec(v: Vec<Object>) -> Object {
        Rc::new(Self {
            inner: Vec::from(v)
        })
    }

    pub fn from_vd(v: std::collections::VecDeque<Object>) -> Object {
        Rc::new(Self {
            inner: Vec::from(v)
        })
    }

    pub fn into(&self) -> &Vec<Object> {
        &self.inner
    }

    //in python, a list supports slice index
    pub fn slice_index(&self, slice: &Object) -> Object {
        match slice.klass() {
            Klass::ListKlass => {
                let s = cast!(slice, Self);
                let mut res = Vec::<Object>::new();
                for i in s.into().iter() {
                    res.push(self[i].clone());
                }
                Self::from_vec(res)
            },
            _ => panic!("Invalid slice index {:?}", slice)
        }
    }

    //a list also supports range index,
    //if the start or end parameter is None,
    //It's from the start of the list or to the end of the list
    //by default.
    pub fn range_index(&self, start: Option<Object>, end: Option<Object>) -> Object {
        let s = match start {
            None => 0,
            Some(v) => cast!(v, Integer).into() as usize
        };
        let e = match end {
            None => self.inner.len(),
            Some(v) => cast!(v, Integer).into() as usize
        };

        assert!(s <= e);
        assert!(e <= self.inner.len());

        let mut res = Vec::<Object>::new();
        for i in s..e {
            res.push(self[i].clone());
        }
        Self::from_vec(res)
    }
}

impl ObjectTrait for List {
    fn as_any(&self) -> &dyn std::any::Any {self}

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn klass(&self) -> super::klass::Klass {
        super::klass::Klass::ListKlass
    }
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("[");
        if self.inner.len() > 0 {
            s.push_str(format!("{}", &self.inner[0]).as_str());
        }

        for i in 1..self.inner.len() {
            s.push_str(format!(", {}", &self.inner[i]).as_str());
        }

        write!(f, "{}]", s)
    }
}

impl std::ops::Index<usize> for List {
    type Output = Object;
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl std::ops::Index<&Object> for List {
    type Output = Object;
    fn index(&self, index: &Object) -> &Self::Output {
        match index.klass() {
            Klass::IntegerKlass => {
                let i = cast!(index, Integer).into() as usize;
                &self.inner[i]
            },
            _ => panic!("Invalid index type {:?}", index)
        }
    }
}
