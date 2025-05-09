#![no_std]

extern crate display;
extern crate errors;
extern crate macros;
extern crate result;
extern crate str_ext;
extern crate vec;

use core::str::from_utf8_unchecked;
use display::Fmt;
use result::Result;
use vec::Vec;

pub struct Formatter {
    buffer: Vec<u8>,
}

impl Fmt for Formatter {
    fn append(&mut self, s: &str) -> Result<()> {
        self.buffer.extend_from_slice(s.as_bytes())
    }

    fn to_str(&self) -> &str {
        match self.buffer.len() {
            0 => "",
            len => unsafe { from_utf8_unchecked(&self.buffer[0..len]) },
        }
    }
}

impl Formatter {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn with_capacity(size: usize) -> Result<Self> {
        Ok(Self {
            buffer: Vec::with_capacity(size)?,
        })
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

extern crate error;
use display::Display;
use errors::*;
use macros::*;
use str_ext::StrExt;

errors!(Abc);

fn test_err() -> Result<()> {
    err!(Abc)
}

pub fn real_main(_argc: i32, _argv: *const *const i8) -> i32 {
    let e = test_err();
    let _ = println!("e='{}'", e.unwrap_err());
    0
}
