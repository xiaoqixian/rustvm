/**********************************************
  > File Name		: test.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Wed 03 Nov 2021 10:02:23 AM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

pub trait Object {
    fn sub(&self, x: &dyn Object) -> Option<Box<dyn Object>> {
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
}

impl Object for Integer {
    fn sub(&self, rhs: &dyn Object) -> Option<Box<dyn Object>> {
        let x = unsafe {
            let p = rhs as *const _ as *const Integer;
            &(*p)
        };
        Some(Int::new(self.get() - x.get()))
    }
}

impl Eat for Integer {}

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
    fn sub(&self, rhs: &dyn Object) -> Option<Box<dyn Object>> {
        let x = unsafe {
            let p = rhs as *const _ as *const Int;
            &(*p)
        };
        Some(Int::new(self.get() - x.get()))
    }
}

#[derive(Debug)]
pub struct Nums {
    num: Box<Integer>
}

impl Nums {
    pub fn new(num: Box<Integer>) -> Box<Self> {
        Box::new(Self {num})
    }
}

fn func(p: *mut dyn Object) {
    unsafe {
        let b = Box::from_raw(p);
    }
}

fn main() {
    let a = Integer::new(1);
    let pa1 = Box::into_raw(a);
    let pa2 = pa1;
    unsafe {
        println!("{:?}", (*pa2).get());
    }
    func(pa1);
    unsafe {
        println!("{:?}", (*pa2).get());
    }
}
