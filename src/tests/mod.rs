use crate::util::buffered_input_stream::BufferedInputStream;

fn BufferedInputStream_test() {
    let mut bis = BufferedInputStream::new(&String::from("/home/lunar/pros/rustvm/__pycache__/hello.cpython-39.pyc")).unwrap();
    let a = bis.read_int().unwrap();
    println!("magic number is {:#x}", a);
}

#[derive(Debug)]
struct S {
    i: i32,
    b: String
}

#[test]
fn test() {
    let mut s = S {
        i: 1,
        b: String::from("fuck")
    };
    let mut b = s;
    println!("{:?}", b);
}
