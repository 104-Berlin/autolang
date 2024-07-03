pub trait InputStream {
    type Output;

    fn next(&mut self) -> Option<Self::Output>;
    fn peek(&mut self) -> Option<Self::Output>;
    fn advance(&mut self);
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
