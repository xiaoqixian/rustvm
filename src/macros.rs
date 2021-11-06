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
