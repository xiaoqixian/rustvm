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
use crate::objects::object::Object;
use crate::objects::integer::Integer;
use crate::objects::string::Str;
use crate::objects;

pub struct CodeObject {
    pub argcount: i32,
    pub nlocals: i32,
    pub stacksize: i32,
    pub flags: i32,
    pub bytecodes: Box<Str>,
    pub consts: Vec<*mut dyn Object>,
    pub names: Vec<*mut dyn Object>,
    pub var_names: Vec<*mut dyn Object>,
    pub free_vars: Vec<*mut dyn Object>,
    pub cell_vars: Vec<*mut dyn Object>,
    pub file_name: Box<Str>,
    pub co_name: Box<Str>,
    pub line_number: i32,
    pub notable: Box<Str>
}

impl Object for CodeObject {
    fn print(&self) {
        println!("CodeObject {{\nargcount: {}", self.argcount);
        println!("nlocals: {}", self.nlocals);
    }
}

pub struct BinaryFileParser {
    cur: i32,
    string_table: Vec<*mut Str>,//to save the strings so we can unread
    bis: BufferedInputStream
}

impl BinaryFileParser {
    pub fn new(bis: BufferedInputStream) -> Self {
        BinaryFileParser {
            cur: 0,
            string_table: Vec::new(),//for traceback.
            bis
        }
    }

    pub fn get_bytecodes(&mut self) -> Result<Box<Str>, Errors> {
        assert_eq!(self.bis.read_char()?, 's');
        
        self.get_string()
    }

    pub fn get_string(&mut self) -> Result<Box<Str>, Errors> {
        let mut length = self.bis.read_int()?;
        let mut res = Str::new();
        
        while length > 0 {
            res.push(self.bis.read_char()?);
            length -= 1;
        }
        Ok(res)
    }

    //get variable name
    pub fn get_name(&mut self) -> Result<Box<Str>, Errors> {
        let c = self.bis.read_char()?;
        
        if c == 's' {
            return self.get_string();
        }
        else if c == 't' {
            let s = self.get_string()?;
            let cp:Box<Str> = s.clone();
            self.string_table.push(Box::into_raw(cp));
            return Ok(s);
        }
        else if c == 'R' {
            let sp: *mut Str = self.string_table[self.bis.read_int()? as usize];
            let cp: Box<Str> = unsafe {
                Box::new((*sp).clone())
            };
            return Ok(cp)
        }
        Err(Errors::UnknownCharError(String::from(format!("{} at line {}", c, line!()))))
    }

    pub fn get_file_name(&mut self) -> Result<Box<Str>, Errors> {
        self.get_name()
    }

    pub fn get_no_table(&mut self) -> Result<Box<Str>, Errors> {
        let c = self.bis.read_char()?;
        if c != 's' && c != 't' {
            return Err(Errors::Null);
        }

        self.get_string()
    }

    pub fn parse(&mut self) -> Result<*mut CodeObject, Errors> {
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

    pub fn get_tuple(&mut self) -> Result<Vec<*mut dyn Object>, Errors> {
        let length = self.bis.read_int()?;
        let mut list: Vec<*mut dyn Object> = Vec::new();
        
        for _i in 0..length {
            let c = self.bis.read_char()?;
            
            match c {
                'c' => list.push(self.get_codeobject()?),
                'i' => list.push(Integer::new_ptr(self.bis.read_int()?)),
                'N' => list.push(objects::statics::PY_NONE),//None
                't' => {
                    let s = self.get_string()?;
                    let scp:Box<Str> = s.clone();
                    list.push(Box::into_raw(s));
                    self.string_table.push(Box::into_raw(scp));
                },
                _ => {
                    return Err(Errors::UnknownCharError(String::from(format!("{} at line {}", c, line!()))));
                }
            }
        }
        Ok(list)
    }

    pub fn get_consts(&mut self) -> Result<Vec<*mut dyn Object>, Errors> {
        if self.bis.read_char()? == '(' {
            return self.get_tuple()
        }
        self.bis.unread();
        Err(Errors::Null)
    }

    pub fn get_names(&mut self) -> Result<Vec<*mut dyn Object>, Errors> {
        if self.bis.read_char()? == '(' {
            return self.get_tuple();
        }
        self.bis.unread();
        Err(Errors::Null)
    }

    pub fn get_var_names(&mut self) -> Result<Vec<*mut dyn Object>, Errors> {
        if self.bis.read_char()? == '(' {
            return self.get_tuple();
        }
        self.bis.unread();
        Err(Errors::Null)
    }

    pub fn get_free_vars(&mut self) -> Result<Vec<*mut dyn Object>, Errors> {
        if self.bis.read_char()? == '(' {
            return self.get_tuple();
        }
        self.bis.unread();
        Err(Errors::Null)
    }

    pub fn get_cell_vars(&mut self) -> Result<Vec<*mut dyn Object>, Errors> {
        if self.bis.read_char()? == '(' {
            return self.get_tuple();
        }
        self.bis.unread();
        Err(Errors::Null)
    }
    
    pub fn get_codeobject(&mut self) -> Result<*mut CodeObject, Errors> {
        let argcount = self.bis.read_int()?;
        let nlocals = self.bis.read_int()?;
        let stacksize = self.bis.read_int()?;
        let flags = self.bis.read_int()?;
        
        let bytecodes = self.get_bytecodes()?;
        let consts = self.get_consts()?;
        let names = self.get_names()?;
        let var_names = self.get_var_names()?;
        let free_vars = self.get_free_vars()?;
        let cell_vars = self.get_cell_vars()?;
        
        let file_name = self.get_file_name()?;
        let module_name = self.get_name()?;
        let begin_line_no = self.bis.read_int()?;
        let line_no_table = self.get_no_table()?;

        Ok(Box::into_raw(Box::new(CodeObject {
            argcount: argcount,
            nlocals: nlocals,
            stacksize: stacksize,
            flags: flags,
            bytecodes: bytecodes,
            consts: consts,
            names: names,
            var_names: var_names,
            free_vars: free_vars,
            cell_vars: cell_vars,
            file_name: file_name,
            co_name: module_name,
            line_number: begin_line_no,
            notable: line_no_table
        })))
    }
}

    
