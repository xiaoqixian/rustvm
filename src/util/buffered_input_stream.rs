/**********************************************
  > File Name		: buffered_input_stream.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Mon 01 Nov 2021 06:55:38 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::fs::File;
use std::os::unix::prelude::FileExt;
use crate::errors::Errors;

pub struct BufferedInputStream {
    fp: File,
    buffer: [u8; 256],
    index: usize,
    offset: u64
}

impl BufferedInputStream {
    pub fn new(filename: &String) -> Result<Self, Errors> {
        let f = match File::open(filename) {
            Ok(v) => v,
            Err(e) => {
                return Err(Errors::StdFileError(format!("{:?}", e)));
            }
        };
        let bytes: usize;
        Ok(BufferedInputStream {
            fp: match f.try_clone() {
                Ok(v) => v,
                Err(e) => {
                    return Err(Errors::StdFileError(format!("{:?}", e)));
                }
            },
            buffer: {
                let mut temp = [0 as u8; 256];
                bytes = match f.read_at(&mut temp, 0 as u64) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(Errors::StdIOError(format!("{:?}", e)));
                    }
                };
                temp
            },
            index: 0 as usize,
            offset: bytes as u64
        })
    }

    pub fn read(&mut self) -> Result<u8, Errors> {
        if self.index < 256 {
            self.index += 1;
            Ok(self.buffer[self.index-1])
        } else {
            self.index = 1;
            let bytes = match self.fp.read_at(&mut self.buffer, self.offset) {
                Ok(v) => v,
                Err(e) => {
                    return Err(Errors::StdIOError(format!("{:?}", e)));
                }
            };
            self.offset += bytes as u64;
            Ok(self.buffer[0])
        }
    }

    pub fn read_char(&mut self) -> Result<char, Errors> {
        let temp = self.read()?;
        Ok(temp as char)
    }

    pub fn read_int(&mut self) -> Result<i32, Errors> {
        let a = self.read()? as i32;
        let b = self.read()? as i32;
        let c = self.read()? as i32;
        let d = self.read()? as i32;

        let i = d<<24 | c<<16 | b<<8 | a;
        Ok(i)
    }

    pub fn unread(&mut self) {
        if self.index >= 1 {
            self.index -= 1;
        }
    }
}
