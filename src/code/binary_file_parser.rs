/**********************************************
  > File Name		: binary_file_parser.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Mon 01 Nov 2021 04:16:51 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use crate::errors::Errors::{Self, ParserErrors};

pub struct CodeObject<T> {
    argcount: i32,
    nlocals: i32,
    stacksize: i32,
    flag: i32,
    bytecodes: Vec<u8>,
    consts: Vec<T>,
    names: Vec<T>,
    varnames: Vec<T>,
    freevars: Vec<T>,
    cellvars: Vec<T>,
    file_name: String,
    co_name: String,
    line_number: i32,
    notable: String
}

pub struct BinaryFileParser {
    cur: i32,
    string_table: Vec<String>,//to save the strings so we can unread
    bis: BufferedInputStream
}

impl BinaryFileParser {
    pub fn new(bis) -> Self {
        BinaryFileParser {
            cur: 0,
            string_table: vec![],
            bis
        }
    }

    pub fn parse(&mut self) -> Result<CodeObject, ()> {
        let magic_number = self.bis.read_int();
        println!("magic number: {:#x}", magic_number);
        let moddate = self.bis.read_int();
        println!("moddate: {:#x}", moddate);

        let object_type = self.bis.read_char();
        if object_type == 'c' {
            self.get_codeobject()
        } else {
            Err(())
        }
    }

    pub fn get_codeobject(&mut self) -> Result<CodeObject, ()> {
        let argcount = self.bis.read_int();
        let nlocals = self.bis.read_int();
        let stacksize = self.bis.read_int();
        let flgs = self.bis.read_int();
        
        let bytecodes = self.get_bytecodes();
    }

    pub fn get_bytecodes(&mut self) -> String {
        assert_eq!(self.bis.read_char(), 's');
        
        self.get_string()
    }

    pub fn get_string(&mut self) -> String {
        let mut length = self.bis.read_int();
        let mut res = String::from("");
        
        while length > 0 {
            res.push(self.bis.read_char());
        }
        res
    }

    pub fn get_name(&mut self) -> Result<String, Errors> {
        let c = self.bis.read_char();
        
        if c == 's' {
            return self.get_string();
        }
        else if c == 't' {
            let s = self.get_string();
            self.string_table.push(s);
            return s;
        }
        else if c == 'R' {
            return self.string_table[self.bis.read_int() as usize];
        }
        Err(ParserErrors::GetNameError(String::from("fuck")))
    }
}
