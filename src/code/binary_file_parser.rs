/**********************************************
  > File Name		: binary_file_parser.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Mon 01 Nov 2021 04:16:51 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::rc::Rc;
use std::any::Any;

//use crate::errors::Errors;
use crate::util::buffered_input_stream::BufferedInputStream;
use crate::objects::object::{Object, ObjRef};
use crate::objects::{string::{Str, NONE}, integer::Integer};
use crate::objects::code_object::{CodeObject, CODE_KLASS_INSTANCE};
use crate::cast;
use crate::errors::Errors;

pub struct BinaryFileParser {
    string_table: Vec<ObjRef>,//to save the strings so we can unread
    bis: BufferedInputStream
}

impl BinaryFileParser {
    pub fn new(bis: BufferedInputStream) -> Self {
        BinaryFileParser {
            string_table: Vec::new(),//for traceback.
            bis
        }
    }

    pub fn get_bytecodes(&mut self) -> Result<ObjRef, Errors> {
        assert_eq!(self.bis.read_char()?, 's');
        
        self.get_string()
    }

    pub fn get_string(&mut self) -> Result<ObjRef, Errors> {
        let mut length = self.bis.read_int()?;
        let mut res = Vec::<u8>::new();
        
        while length > 0 {
            res.push(self.bis.read()?);
            length -= 1;
        }
        Ok(Str::from_vec(res))
    }

    //get variable name
    pub fn get_name(&mut self) -> Result<ObjRef, Errors> {
        let c = self.bis.read_char()?;
        
        if c == 's' {
            self.get_string()
        }
        else if c == 't' {
            let s = self.get_string()?;
            self.string_table.push(s.clone());
            Ok(s)
        }
        else if c == 'R' {
            Ok(self.string_table[self.bis.read_usize()?].clone())
        } else {
            Err(Errors::UnknownCharError(String::from(format!("{} at line {}", c, line!()))))
        }
    }

    pub fn get_file_name(&mut self) -> Result<ObjRef, Errors> {
        self.get_name()
    }

    pub fn get_no_table(&mut self) -> Result<ObjRef, Errors> {
        let c = self.bis.read_char()?;
        if c != 's' && c != 't' {
            return Err(Errors::Null);
        }

        self.get_string()
    }

    pub fn parse(&mut self) -> Result<ObjRef, Errors> {
        let magic_number = self.bis.read_int()?;
        println!("magic number: {:#x}", magic_number);
        let moddate = self.bis.read_int()?;
        println!("moddate: {:#x}", moddate);

        let object_type = self.bis.read_char()?;
        if object_type == 'c' {
            println!("get an CodeObject");
            self.get_codeobject()
        } else {
            Err(Errors::UnknownCharError(String::from(format!("{} at line {}", object_type, line!()))))
        }
    }

    pub fn get_tuple(&mut self) -> Result<Vec<ObjRef>, Errors> {
        let length = self.bis.read_int()?;
        let mut list = Vec::<ObjRef>::new();
        
        for _i in 0..length {
            let c = self.bis.read_char()?;

            match c {
                'c' => list.push(self.get_codeobject()?),
                'i' => list.push(Integer::new(self.bis.read_int()?)),
                'N' => list.push(NONE.clone()),//None
                't' => {
                    let s = self.get_string()?;
                    self.string_table.push(s.clone());
                    list.push(s);
                },
                's' => {
                    list.push(self.get_string()?);
                },
                'R' => {
                    list.push((*self.string_table.get(self.bis.read_usize()?).unwrap()).clone());
                },
                _ => {
                    return Err(Errors::UnknownCharError(String::from(format!("{} at line {}", c, line!()))));
                }
            }
        }
        Ok(list)
    }

    pub fn get_consts(&mut self) -> Result<Vec<ObjRef>, Errors> {
        if self.bis.read_char()? == '(' {
            return self.get_tuple();
        }
        self.bis.unread();
        Err(Errors::Null)
    }

    pub fn get_names(&mut self) -> Result<Vec<ObjRef>, Errors> {
        if self.bis.read_char()? == '(' {
            return self.get_tuple();
        }
        self.bis.unread();
        Err(Errors::Null)
    }

    pub fn get_var_names(&mut self) -> Result<Vec<ObjRef>, Errors> {
        if self.bis.read_char()? == '(' {
            return self.get_tuple();
        }
        self.bis.unread();
        Err(Errors::Null)
    }

    pub fn get_free_vars(&mut self) -> Result<Vec<ObjRef>, Errors> {
        if self.bis.read_char()? == '(' {
            return self.get_tuple();
        }
        self.bis.unread();
        Err(Errors::Null)
    }

    pub fn get_cell_vars(&mut self) -> Result<Vec<ObjRef>, Errors> {
        if self.bis.read_char()? == '(' {
            return self.get_tuple();
        }
        self.bis.unread();
        Err(Errors::Null)
    }
    
    pub fn get_codeobject(&mut self) -> Result<ObjRef, Errors> {
        let argcount = self.bis.read_usize()?;
        let nlocals = self.bis.read_usize()?;
        let stacksize = self.bis.read_usize()?;
        let flags = self.bis.read_u32()?;
        
        let bytecodes = self.get_bytecodes()?;
        let consts = self.get_consts()?;
        let names = self.get_names()?;
        let var_names = self.get_var_names()?;
        let free_vars = self.get_free_vars()?;
        let cell_vars = self.get_cell_vars()?;
        
        let file_name = self.get_file_name()?;
        let module_name = self.get_name()?;
        let begin_line_no = self.bis.read_u32()?;
        let line_no_table = self.get_no_table()?;

        Ok(Rc::new(CodeObject {
            argcount,
            nlocals,
            stacksize,
            flags,
            bytecodes,
            consts,
            names,
            var_names,
            free_vars,
            cell_vars,
            file_name,
            co_name: module_name,
            line_number: begin_line_no,
            notable: line_no_table,
            klass: &CODE_KLASS_INSTANCE
        }))
    }
}
