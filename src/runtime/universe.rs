/**********************************************
  > File Name		: runtime/universe.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Fri Nov  5 19:20:12 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

pub struct Universe {

}

impl Universe {
    pub fn genesis() {
        use crate::objects;
        objects::Statics::py_none = objects::object::PyNone::get_instance();
    }
}
