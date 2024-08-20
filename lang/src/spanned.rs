use std::ops::Deref;

use source_span::Span;

#[derive(Clone, Debug)]
pub struct Spanned<T> {
    pub span: Span,
    pub value: T,
}

impl<T> Spanned<T> {
    pub fn new(value: T, span: Span) -> Spanned<T> {
        Spanned { span, value }
    }

    pub fn map_value<U>(self, f: impl FnOnce(T) -> U) -> Spanned<U> {
        Spanned {
            span: self.span,
            value: f(self.value),
        }
    }

    pub fn map_span(&self, f: impl FnOnce(Span) -> Span) -> Spanned<T>
    where
        T: Clone,
    {
        Spanned {
            span: f(self.span),
            value: self.value.clone(),
        }
    }
}

impl<T> From<T> for Spanned<T> {
    fn from(value: T) -> Self {
        Spanned {
            span: Span::default(),
            value,
        }
    }
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
