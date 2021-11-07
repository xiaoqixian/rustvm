/**********************************************
  > File Name		: macros.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Sat Nov  6 15:02:50 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

#[macro_export]
macro_rules! unwrap_option {
    ($op: expr) => {{
        match $op {
            None => {panic!("invalid operation");},
            Some(v) => v
        }
    }};
    ($op: expr, $error_op: stmt) => {{
        match $op {
            None => {$error_op},
            Some(v) => v
        }
    }};
}

#[macro_export]
macro_rules! as_ref {
/*    ($ptr:ident) => {{*/
        /*match unsafe {$ptr.as_ref()} {*/
            /*None => {panic!("null pointer: {:?}", stringify!($ptr));},*/
            /*Some(r) => r*/
        /*}*/
    /*}};*/
    ($self:ident$(, $field:ident)*) => {
        match unsafe {$self$(.$field)*.as_ref()} {
            None => {panic!("null pointer: {:?}", stringify!($self$(.$field)*));},
            Some(r) => r
        }
    }
}

#[macro_export]
macro_rules! as_mut {
/*    ($ptr:ident) => {{*/
        /*match unsafe {$ptr.as_mut()} {*/
            /*None => {panic!("null pointer: {:?}", stringify!($ptr));},*/
            /*Some(r) => r*/
        /*}*/
    /*}};*/
    ($self:ident$(, $field:ident)*) => {
        match unsafe {$self$(.$field)*.as_mut()} {
            None => {panic!("null pointer: {:?}", stringify!($self$(.$field)*));},
            Some(r) => r
        }
    }
}
