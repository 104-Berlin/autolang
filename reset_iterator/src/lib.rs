/// This crate provides a `ResetIterator` type that wraps an iterator and allows you to reset it
/// to a set location. This is useful when you want to peek at the next few items in the iterator
/// without consuming them.
///
/// # Example

pub struct ResetIterator<I: Iterator> {
    iter: I,
    pub(self) peeked: Vec<I::Item>,
    /// Empty if we have no peeked
    pointers: Vec<usize>,
}

impl<I: Iterator> ResetIterator<I> {
    /// Peeks at the next item in the iterator without consuming it
    /// The next call to `peek` or `next` will return the same item
    pub fn peek(&mut self) -> Option<&I::Item> {
        match self.pointers.last() {
            Some(index) => self.peeked.get(*index),
            None => {
                let next = self.iter.next()?;
                self.peeked.push(next);

                self.pointers.push(0);
                Some(self.peeked.last().expect("Peeked item was just pushed"))
            }
        }
    }

    /// Consumes the next item in the iterator and stores it in the peeked list
    pub fn consume(&mut self) -> Option<&I::Item> {
        match self.pointers.last_mut() {
            // We are pointing to an element that is already in the peeked list
            Some(index) if *index < self.peeked.len() - 1 => {
                let res = self.peeked.get(*index);
                *index += 1;
                res
            }
            // We are pointing to the last element, so we want to fetch the next one
            Some(index) => {
                if let Some(next) = self.iter.next() {
                    self.peeked.push(next);
                }

                let value = self.peeked.get(*index)?;
                *index += 1;
                Some(value)
            }
            None if self.peeked.is_empty() => {
                self.peeked.push(self.iter.next()?);
                if let Some(next) = self.iter.next() {
                    self.peeked.push(next);
                }
                self.pointers.push(1);
                self.peeked.first()
            }
            // We are not pointing to any element, but we have peeked values.
            None => {
                self.pointers.push(1);
                if self.peeked.len() == 1 {
                    if let Some(next) = self.iter.next() {
                        self.peeked.push(next);
                    }
                }
                self.peeked.first()
            }
        }
    }

    /// Pushes a pointer to the current read item to jump back to.
    pub fn push_end(&mut self) {
        // One pointer is the tracking pointer.
        // We can delete all the peeked values if we have no pointers
        if self.pointers.len() == 1 {
            let current_pointer = self.pointers.first().expect("Checked for 1 pointer");
            self.peeked.drain(..*current_pointer);
            self.pointers[0] = 0;
            self.pointers.push(0);
        } else if !self.pointers.is_empty() {
            let current_pointer = self
                .pointers
                .last()
                .expect("Checked for at least 1 pointer");
            self.pointers.push(*current_pointer);
        } else {
            unreachable!("What does this even mean?");
        }
    }

    /// Resets the iterator to the beginning of the last set_end
    /// If set_end was never called, or we resetted all set_end we will crash.
    pub fn reset(&mut self) {
        assert!(
            !self.pointers.is_empty(),
            "Cant reset iterator when having to ends to jump to. Call push_end() first."
        );

        self.pointers.pop().expect("Not poping empty vec");
    }

    pub fn pop_end(&mut self) {
        if self.pointers.len() >= 2 {
            self.pointers.remove(self.pointers.len() - 2);
        }
    }
}

impl<T, I> From<T> for ResetIterator<I>
where
    I: Iterator,
    T: IntoIterator<IntoIter = I, Item = I::Item>,
{
    fn from(iter: T) -> Self {
        Self {
            iter: iter.into_iter(),
            peeked: vec![],
            pointers: vec![],
        }
    }
}

/*impl<T, I> Iterator for ResetIterator<I>
where
    I: Iterator<Item = T>,
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume().cloned()
    }
}*/

#[cfg(test)]
mod tests {
    use crate::ResetIterator;

    #[test]
    fn try_out() {
        // Vec with 15 ints. Value equals index of the item
        const ITER: &[i32] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

        let mut iter = ResetIterator::from(ITER);

        // Consume checked
        assert_eq!(iter.peek(), Some(&&0i32));
        assert_eq!(iter.peek(), Some(&&0i32));
        assert_eq!(iter.consume(), Some(&&0i32));
        assert_eq!(iter.peek(), Some(&&1i32));
        assert_eq!(iter.consume(), Some(&&1i32));
        assert_eq!(iter.peek(), Some(&&2i32));
        assert_eq!(iter.peek(), Some(&&2i32));
        assert_eq!(iter.consume(), Some(&&2i32));
        assert_eq!(iter.consume(), Some(&&3i32));
        assert_eq!(iter.consume(), Some(&&4i32));
        iter.reset();
        assert_eq!(iter.consume(), Some(&&0i32));
        assert_eq!(iter.peek(), Some(&&1i32));
        assert_eq!(iter.consume(), Some(&&1i32));
        assert_eq!(iter.peek(), Some(&&2i32));
        assert_eq!(iter.peek(), Some(&&2i32));
        assert_eq!(iter.consume(), Some(&&2i32));
        iter.push_end();

        assert_eq!(iter.consume(), Some(&&3i32));
        assert_eq!(iter.consume(), Some(&&4i32));
        iter.reset();
        assert_eq!(iter.consume(), Some(&&3i32));
        assert_eq!(iter.consume(), Some(&&4i32));
    }

    #[test]
    fn test_set_end() {
        const ITER: &[i32] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

        let mut iter = ResetIterator::from(ITER);

        assert_eq!(iter.consume(), Some(&&0));
        assert_eq!(iter.consume(), Some(&&1));
        iter.push_end();
        assert_eq!(iter.consume(), Some(&&2));
    }

    #[test]
    fn test_reset() {
        const ITER: &[i32] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

        let mut iter = ResetIterator::from(ITER);

        assert_eq!(iter.consume(), Some(&&0));
        assert_eq!(iter.consume(), Some(&&1));
        iter.push_end();
        iter.reset();
        assert_eq!(iter.consume(), Some(&&2));
        assert_eq!(iter.consume(), Some(&&3));
    }

    #[test]
    fn controll_peeked() {
        const ITER: &[i32] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

        let mut iter = ResetIterator::from(ITER);

        assert_eq!(iter.consume(), Some(&&0));
        assert_eq!(iter.consume(), Some(&&1));
        iter.push_end();
        assert_eq!(iter.peeked.len(), 1);
        assert_eq!(iter.consume(), Some(&&2));
        assert_eq!(iter.peeked.len(), 2);
        iter.reset();
        assert_eq!(iter.peeked.len(), 2);
        assert_eq!(iter.consume(), Some(&&2));
        assert_eq!(iter.peeked.len(), 2);
        assert_eq!(iter.consume(), Some(&&3));
        assert_eq!(iter.peeked.len(), 3);
        assert_eq!(iter.consume(), Some(&&4));
        iter.push_end();
        assert_eq!(iter.peeked.len(), 1);
        assert_eq!(iter.consume(), Some(&&5));
        assert_eq!(iter.peeked.len(), 2);
        iter.push_end();
        assert_eq!(iter.peeked.len(), 2);
        assert_eq!(iter.consume(), Some(&&6));
        assert_eq!(iter.peeked.len(), 3);
        iter.reset();
        assert_eq!(iter.peeked.len(), 3);
        assert_eq!(iter.consume(), Some(&&6));
        assert_eq!(iter.peeked.len(), 3);
        iter.reset();
        assert_eq!(iter.peeked.len(), 3);
        assert_eq!(iter.consume(), Some(&&5));
    }
}
