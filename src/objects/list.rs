/**********************************************
  > File Name		: objects/list.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Thu 04 Nov 2021 04:37:58 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use super::object::Object;

#[derive(Clone, Debug)]
pub struct ArrayList<T: Clone> {
    array: Vec<T>
}

impl Object for ArrayList {}

impl<T> ArrayList<T: Clone> {
    pub fn new() -> Box<Self> {
        Box::new(ArrayList {array: Vec::new()})
    }
}