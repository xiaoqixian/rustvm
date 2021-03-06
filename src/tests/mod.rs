use crate::util::buffered_input_stream::BufferedInputStream;
use crate::code::binary_file_parser::BinaryFileParser;
use crate::runtime::interpreter::Interpreter;

/*fn BufferedInputStream_test() {*/
    /*let mut bis = BufferedInputStream::new(&String::from("/home/lunar/pros/rustvm/__pycache__/hello.cpython-39.pyc")).unwrap();*/
    /*let a = bis.read_int().unwrap();*/
    /*println!("magic number is {:#x}", a);*/
/*}*/

/*#[derive(Debug)]*/
/*struct S {*/
    /*i: i32,*/
    /*b: String*/
/*}*/

/*fn test() {*/
    /*let mut s = S {*/
        /*i: 1,*/
        /*b: String::from("fuck")*/
    /*};*/
    /*let mut b = s;*/
    /*println!("{:?}", b);*/
/*}*/

/*
 * add and sub operations of numbers
 */
#[test]
fn test1() {
    let path = String::from("/home/lunar/pros/rustvm/src/tests/test.pyc");
    let mut bis = BufferedInputStream::new(&path).unwrap();
    let mut parser = BinaryFileParser::new(bis);
    let main_codes = parser.parse().unwrap();
    let mut itp = Interpreter::new(main_codes);
/*    use crate::runtime::universe::Universe;*/
    /*Universe::genesis();*/
    itp.run();
}
