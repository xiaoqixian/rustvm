/**********************************************
  > File Name		: type.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Wed 26 Jan 2022 08:05:47 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use super::klass::Klass;

#[derive(Clone)]
pub struct TypeObject {
    own_klass: Klass,
    klass: Klass
}

impl TypeObject {
    pub fn new(own_klass: Klass) -> Self {
        Self {
            own_klass,
            klass: Klass::TypeKlass
        }
    }

    pub fn print(&self) {
        print!("<type ");
    }
}
