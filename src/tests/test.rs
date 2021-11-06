/**********************************************
  > File Name		: test.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Wed 03 Nov 2021 10:02:23 AM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

pub trait Object {
    fn sub(&self, x: *mut dyn Object) -> Option<*mut dyn Object> {
        None
    }

    fn print(&self) {
        println!("this item doesn't impl print");
    }
}

pub trait Eat {
    fn eat(&self) {
        println!("I'm fucking eat");
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Integer {
    val: i32
}

impl Integer {
    fn get(&self) -> i32 {
        self.val
    }

    fn new(v: i32) -> Box<Integer> {
        Box::new(Self {val: v})
    }

    fn set(&mut self, v: i32) {
        self.val = v;
    }

    fn new_stack(v: i32) -> Self {
        Self {val: v}
    }
}

impl Object for Integer {
    fn sub(&self, rhs: *mut dyn Object) -> Option<*mut dyn Object> {
        None
    }

    fn print(&self) {
        println!("{}", self.val);
    }
}

/*impl Eat for Integer {}*/

#[derive(Debug)]
pub struct Int {
    val: i32
}

impl Int {
    fn get(&self) -> i32 {
        self.val
    }

    fn new(v: i32) -> Box<dyn Object> {
        Box::new(Self {val: v})
    }
}

impl Object for Int {
    fn sub(&self, rhs: *mut dyn Object) -> Option<*mut dyn Object> {
        None
    }
}

/*#[derive(Debug)]*/
/*pub struct Nums {*/
    /*num: Box<Integer>*/
/*}*/

/*impl Nums {*/
    /*pub fn new(num: Box<Integer>) -> Box<Self> {*/
        /*Box::new(Self {num})*/
    /*}*/
/*}*/

enum Fuck {
    Less = 0,
    Equal,
    Greater
}

const Fuck: Integer = Integer {val: 1};
const FuckP: *mut dyn Object = {
    &Fuck as *const Integer as *mut Integer as *mut dyn Object
};

#[macro_export]
macro_rules! unwrap_option {
    ($op: expr, $error_op: stmt) => {{
        match $op {
            None => {$error_op},
            Some(v) => v
        }
    }};
}

struct PyTrue {}

impl Object for PyTrue {
    fn print(&self) {
        println!("True");
    }
}

const TRUE_INSTANCE: PyTrue = PyTrue {};
const TRUE:*mut dyn Object = &TRUE_INSTANCE as *const PyTrue as *mut PyTrue as *mut dyn Object;

fn greater(a: i32, b: i32) -> Option<*mut dyn Object> {
    if a > b {
        Some(TRUE)
    } else {
        None
    }
}

struct Ref<'a> {
    i: i32,
    I: &'a Integer
}

impl Ref<'a> {
    pub fn new<'a>(ir: &'a Integer) -> Self<'a> {
        Ref {
            i: 0,
            I: ir
        }
    }
}

fn main() {
    let a = Integer::new_stack(2);
    let ra = Ref::new(&a);
    a.print();
}
