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
use super::{map::Dict, string::{Str, self}, function::{Function, Method}};

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
    attr_dict: Dict,
    name: Str
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
