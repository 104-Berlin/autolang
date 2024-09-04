use std::ops::Deref;

use miette::{SourceOffset, SourceSpan};

#[derive(Clone, Debug)]
pub struct Spanned<T> {
    pub span: SourceSpan,
    pub value: T,
}

impl<T> Spanned<T> {
    pub fn new(value: T, span: SourceSpan) -> Spanned<T> {
        Spanned { span, value }
    }

    pub fn map_value<U>(self, f: impl FnOnce(T) -> U) -> Spanned<U> {
        Spanned {
            span: self.span,
            value: f(self.value),
        }
    }

    pub fn map_span(&self, f: impl FnOnce(SourceSpan) -> SourceSpan) -> Spanned<T>
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
            span: SourceSpan::new(SourceOffset::from(0), 0),
            value,
        }
    }
}

pub trait SpanExt {
    fn union(&self, other: &Self) -> Self;
    fn next(&self) -> Self;
}

impl SpanExt for SourceSpan {
    fn union(&self, other: &Self) -> Self {
        let start = self.offset().min(other.offset());
        let end = (self.offset() + self.len()).max(other.offset() + other.len());
        SourceSpan::new(start.into(), end - start)
    }

    fn next(&self) -> Self {
        SourceSpan::new(SourceOffset::from(self.offset() + self.len()), 0)
    }
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

pub trait WithSpan {
    fn with_span(self, span: SourceSpan) -> Spanned<Self>
    where
        Self: Sized;
}

impl<T> WithSpan for T {
    fn with_span(self, span: SourceSpan) -> Spanned<Self> {
        Spanned { span, value: self }
    }
}
