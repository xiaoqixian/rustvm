/**********************************************
  > File Name		: objects/klass.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Sun Jan 30 16:56:26 2022
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
**********************************************/

//use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Klass {
    IntegerKlass,
    StringKlass,
    CodeKlass,
    BuiltinKlass,
    ListKlass,
    DictKlass
}

//impl PartialEq for Klass {
    //fn eq(&self, other: &Self) -> bool {
        //match self 
    //}
//}
