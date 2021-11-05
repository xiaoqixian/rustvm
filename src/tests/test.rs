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
}

impl Object for Integer {
    fn sub(&self, rhs: *mut dyn Object) -> Option<*mut dyn Object> {
        None
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


fn main() {
    let a = 0 as *mut Integer;
    let b = 0 as *mut Int;
    let c = a as *mut dyn Object;
    let d = b as *mut dyn Object;
    assert_ne!(c, d);
}
