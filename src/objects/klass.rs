/**********************************************
  > File Name		: objects/klass.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Sun Jan 30 16:56:26 2022
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
**********************************************/

use std::rc::Rc;
//use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use super::{map::Dict, string::{Str, self}, function::{Function, Method}, Object};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Klass {
    IntegerKlass,
    StringKlass,
    CodeKlass,
    BuiltinKlass,
    ListKlass,
    DictKlass,
    FunctionKlass,
    BuiltinMethodKlass,
    MethodKlass,
    TypeKlass,
    NewKlass
}

#[derive(Clone)]
pub struct KlassContainer {
    pub attr_dict: Dict,
    pub name: Str
}

impl KlassContainer {
    pub fn new(attrs: &Object, name: &Object) -> Rc<Self> {
        Rc::new(Self {
            attr_dict: crate::cast!(attrs, Dict).clone(),
            name: crate::cast!(name, Str).clone()
        })
    }

    #[inline]
    pub fn get_attr(&self, attr_name: &Object) -> Option<Object> {
        self.attr_dict.get(attr_name)
    }
}

impl std::fmt::Debug for KlassContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<KlassContainer {}>", &self.name)
    }
}

impl std::fmt::Display for KlassContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}

impl Klass {
    //pub fn initialize(k: Self) -> Rc<KlassContainer> {
        //match k {
            //Self::StringKlass => {
                //let mut attr_dict = Dict::raw_new();
                //attr_dict.put(Str::from("upper"), Method::from(&string::upper, None, Str::from("upper")));
                //Rc::new(KlassContainer {
                    //attr_dict,
                    //name: Str::raw_from("str")
                //})
            //},
            //_ => panic!("Invalid klass {:?}", k)
        //}
    //}
}
