/**********************************************
  > File Name		: errors.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Mon Nov  1 22:28:30 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

#[derive(Debug)]
pub enum Errors {
    //common errors
    StdIOError(String),
    StdFileError(String),
    Utf8Error(String),
    Null, //Just to avoid Option

    //parser errors
    GetNameError(String),
    UnknownCharError(String),
    ObjectMethodNotImplemented(&'static str),

    InvalidArg(String),
    InvalidObject(String)
}
