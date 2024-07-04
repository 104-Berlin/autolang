use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
    iter::Peekable,
};

use utf8_chars::{BufReadCharsExt, Chars, CharsRaw};

pub trait InputStream {
    type Output;

    fn next(&mut self) -> Option<Self::Output>;
    fn peek(&mut self) -> Option<Self::Output>;
    fn advance(&mut self);
}

impl<I> InputStream for Peekable<I>
where
    I: Iterator<Item = char>,
{
    type Output = char;

    fn next(&mut self) -> Option<Self::Output> {
        Iterator::next(self)
    }

    fn peek(&mut self) -> Option<Self::Output> {
        Peekable::peek(self).cloned()
    }

    fn advance(&mut self) {
        Iterator::next(self);
    }
}

impl<'a> InputStream for &'a str {
    type Output = char;

    fn next(&mut self) -> Option<Self::Output> {
        if !self.is_empty() {
            let c = self.chars().next();
            self.advance();
            c
        } else {
            None
        }
    }

    fn advance(&mut self) {
        if !self.is_empty() {
            *self = &self[1..];
        }
    }

    fn peek(&mut self) -> Option<Self::Output> {
        self.chars().next()
    }
}

pub struct FileInputStream {
    reader: BufReader<File>,
}

impl FileInputStream {
    pub fn new(file: File) -> Self {
        Self {
            reader: BufReader::new(file),
        }
    }
}

impl InputStream for FileInputStream {
    type Output = char;

    fn next(&mut self) -> Option<Self::Output> {
        self.reader.read_char().unwrap()
    }

    fn peek(&mut self) -> Option<Self::Output> {
        let r = self.next()?;
        self.reader.seek(std::io::SeekFrom::Current(-1)).unwrap();
        Some(r)
    }

    fn advance(&mut self) {
        let _ = self.reader.read_char();
    }
}
