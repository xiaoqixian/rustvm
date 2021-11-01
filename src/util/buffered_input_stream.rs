/**********************************************
  > File Name		: buffered_input_stream.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Mon 01 Nov 2021 06:55:38 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::fs::File;
use std::io;
use std::os::unix::prelude::FileExt;
use ascii;
use ascii::AsciiChar;

pub struct BufferedInputStream {
    fp: File,
    buffer: [u8; 256],
    index: usize,
    offset: u64
}

impl BufferedInputStream {
    pub fn new(filename: &String) -> std::io::Result<Self> {
        let mut f = File::open(filename)?;
        let mut bytes: usize;
        Ok(BufferedInputStream {
            fp: f.try_clone()?,
            buffer: {
                let mut temp = [0 as u8; 256];
                bytes = f.read_at(&mut temp, 0 as u64)?;
                temp
            },
            index: 0 as usize,
            offset: (bytes as u64)
        })
    }

    pub fn read(&mut self) -> std::io::Result<u8> {
        if self.index < 256 {
            self.index += 1;
            Ok(self.buffer[self.index-1])
        } else {
            self.index = 1;
            let bytes = self.fp.read_at(&mut self.buffer, self.offset)?;
            self.offset += (bytes as u64);
            Ok(self.buffer[0])
        }
    }

    pub fn read_char(&mut self) -> char {
        let temp = self.read().unwrap("BufferedInputStream read error");
        temp as char
    }

    pub fn read_int(&mut self) -> i32 {
        let a = self.read()? as i32;
        let b = self.read()? as i32;
        let c = self.read()? as i32;
        let d = self.read()? as i32;

        let i = d<<24 | c<<16 | b<<8 | a;
        i
    }

    pub fn unread(&mut self) {
        if (self.index >= 1) {
            self.index -= 1;
        }
    }
}
