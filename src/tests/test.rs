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

trait Build<T> {
    fn new(para1: T) -> Option<*mut dyn Object> {
        None
    }
}

pub trait Eat {
    fn eat(&self) {
        println!("I'm fucking eat");
    }
}

#[derive(Debug, Clone)]
pub struct Integer {
    pub val: i32
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

impl Build<Integer> for Integer {
    fn new(para: Integer) -> Option<*mut dyn Object> {
        Some(Box::into_raw(Box::new(Integer {val:para.get()})))
    }
}

impl Build<Int> for Integer {
    fn new(para: Int) -> Option<*mut dyn Object> {
        Some(Box::into_raw(Box::new(Integer {val: para.get()})))
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
    I: &'a mut Integer
}

impl<'a> Ref<'a> {
    pub fn new(ir: *mut Integer) -> Self {
        let irr = as_ref!(ir);
        Ref {
            i: 0,
            I: irr
        }
    }

    pub fn run(&mut self) {
        println!("{}", self.I.get());
        //let a = Box::into_raw(Integer::new(2));
        let mut a = Integer {val: 2};
        let pa = &mut a as *mut Integer;
        self.I = as_ref!(pa);
        println!("{}", self.I.get());
    }

    pub fn run_again(&self) {
        println!("{}", self.I.get());
    }
}

struct Container {
    pub v: Vec<*mut dyn Object>
}

struct Conta {
    pub v: Vec<*mut dyn Object>,
    pub vv: Vec<*mut dyn Object>
}

#[macro_export]
macro_rules! as_ref {
    ($ptr:ident) => {{
        match unsafe {$ptr.as_mut()} {
            None => {panic!("null pointer: {:?}", stringify!($ptr));},
            Some(r) => r
        }
    }};
    ($self:ident, $field:ident) => {{
        match unsafe {$self.$ptr.as_mut()} {
            None => {panic!("null pointer: {:?}", stringify!($self.$field));},
            Some(r) => r
        }
    }}
}

fn main() {
    let mut a = Integer {val: 10};
    let mut r = Ref::new(&mut a as *mut Integer);
    std::mem::drop(a);
    r.run();
    r.run_again();
}
