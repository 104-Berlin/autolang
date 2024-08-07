use std::{
    fs::File,
    io::{BufReader, Seek},
    iter::Peekable,
};

use utf8_chars::BufReadCharsExt;

pub trait InputStream {
    type Output;

    fn next(&mut self) -> Option<Self::Output>;
    fn peek(&mut self) -> Option<Self::Output>;
    fn advance(&mut self);

    fn is_next(&mut self, expected: Self::Output) -> bool
    where
        Self::Output: PartialEq,
    {
        self.peek() == Some(expected)
    }

    /// Returns the Expected output if it is the next
    /// Returns None if the expected does not match
    fn expect(&mut self, expected: Self::Output) -> Option<Self::Output>
    where
        Self::Output: PartialEq,
    {
        let output = self.peek()?;

        if output == expected {
            Some(output)
        } else {
            None
        }
    }

    /// Returns Some(..) if the next input is the expected output
    /// Returns None if the input could not be matched
    fn consume_checked(&mut self, expected: Self::Output) -> Option<Self::Output>
    where
        Self::Output: PartialEq,
    {
        self.expect(expected).inspect(|_| self.advance())
    }

    fn consume_checked_or<E>(&mut self, expected: Self::Output, error: E) -> Result<Self::Output, E>
    where
        Self: Sized,
        Self::Output: PartialEq,
    {
        self.consume_checked(expected).ok_or(error)
    }
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
