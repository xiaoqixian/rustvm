/**********************************************
  > File Name		: binary_file_parser.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Mon 01 Nov 2021 04:16:51 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use crate::errors::Errors;
use crate::util::buffered_input_stream::BufferedInputStream;
use crate::object::object::Object;

pub struct CodeObject<T: Clone + Object> {
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
    pub fn new(bis: BufferedInputStream) -> Self {
        BinaryFileParser {
            cur: 0,
            string_table: Vec::new(),
            bis
        }
    }

    pub fn get_bytecodes(&mut self) -> Result<String, Errors> {
        assert_eq!(self.bis.read_char()?, 's');
        
        self.get_string()
    }

    pub fn get_string(&mut self) -> Result<String, Errors> {
        let mut length = self.bis.read_int()?;
        let mut res = String::from("");
        
        while length > 0 {
            res.push(self.bis.read_char());
        }
        Ok(res)
    }

    pub fn get_name(&mut self) -> Result<String, Errors> {
        let c = self.bis.read_char()?;
        
        if c == 's' {
            return self.get_string();
        }
        else if c == 't' {
            let s = self.get_string()?;
            self.string_table.push(s);
            return Ok(s);
        }
        else if c == 'R' {
            return Ok(self.string_table[self.bis.read_int() as usize].clone());
        }
        Err(Errors::UnkownCharError(String::from(c)))
    }

    pub fn get_filename(&mut self) -> Result<String, Errors> {
        self.get_name()
    }

    pub fn get_no_table(&mut self) -> Result<String, Errors> {
        let c = self.bis.read_char()?;
        if c != 's' && c != 't' {
            return Err(Errors::Null);
        }

        self.get_string()
    }

    pub fn parse<T: Object + Clone>(&mut self) -> Result<CodeObject<T>, Errors> {
        let magic_number = self.bis.read_int()?;
        println!("magic number: {:#x}", magic_number);
        let moddate = self.bis.read_int()?;
        println!("moddate: {:#x}", moddate);

        let object_type = self.bis.read_char()?;
        if object_type == 'c' {
            self.get_codeobject()
        } else {
            Err(Errors::UnkownCharError(String::from(object_type)))
        }
    }

    pub fn get_tuple<T: Object + Clone>(&mut self) -> Result<Vec<T>, Errors> {
        let length = self.bis.read_int()?;
        let mut list: Vec<T> = Vec::new();
        
        for i in 0..length {
            let c = self.bis.read_char()?;
            
            match c {
                'c' => list.push(self.get_codeobject()?),
                'i' => list.push(self.bis.read_int()?),
                _ => {
                    return Err(Errors::UnkownCharError(String::from(c)));
                }
            }
        }
        Ok(list)
    }

    pub fn get_codeobject<T: Object + Clone>(&mut self) -> Result<CodeObject<T>, Errors> {
        let argcount = self.bis.read_int()?;
        let nlocals = self.bis.read_int()?;
        let stacksize = self.bis.read_int()?;
        let flgs = self.bis.read_int()?;
        
        let bytecodes = self.get_bytecodes()?;
        Err(Errors::Null)
    }
}

    
