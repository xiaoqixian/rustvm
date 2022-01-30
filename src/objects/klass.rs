/**********************************************
  > File Name		: objects/klass.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Sun Jan 30 16:56:26 2022
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

#[derive(Debug, Clone, Copy)]
pub enum Klass {
    IntegerKlass,
    StringKlass,
    CodeKlass,
    BuiltinKlass,
    ListKlass,
    DictKlass
}
