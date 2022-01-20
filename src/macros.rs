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

#[macro_export]
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }}
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! info {
    ($string: expr) => {
        //println!("{}[INFO {}:{}] {}", termion::color::Fg(termion::color::Blue), file!(), line!(), $string);
        colour::blue_ln!("[INFO {}:{}] {}", file!(), line!(), $string);
    };
    ($string: expr, $($formats: tt)*) => {
        let s = format!($string, $($formats)*);
        colour::blue_ln!("[INFO {}:{}] {}", file!(), line!(), s);
    }
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! info {
    ($string: expr) => {};
    ($string: expr, $($formats: expr)*) => {}
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
    ($string: expr) => {
        colour::yellow_ln!("[DEBUG {}:{}:{}] {}", file!(), crate::function!(), line!(), $string);
    };
    ($string: expr, $($formats: tt)*) => {
        let s = format!($string, $($formats)*);
        colour::yellow_ln!("[DEBUG {}:{}:{}] {}", file!(), crate::function!(), line!(), s);
    }
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug {
    ($string: expr) => {};
    ($string: expr, $($formats: expr)*) => {}
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! error {
    ($string: expr) => {
        colour::red_ln!("[ERROR {}:{}:{}] {}", file!(), crate::function!(), line!(), $string);
    };
    ($string: expr, $($formats: tt)*) => {
        let s = format!($string, $($formats)*);
        colour::red_ln!("[ERROR {}:{}:{}] {}", file!(), crate::function!(), line!(), s);
    }
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! error {
    ($string: expr) => {};
    ($string: expr, $($formats: expr)*) => {}
}
