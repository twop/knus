use std::path::PathBuf;
use std::sync::Arc;

use combine::stream::{StreamOnce};

/// Keeps object's boundary positions in the original file
#[derive(Clone, Debug)]
pub struct Spanned<T, S> {
    pub(crate) span: S,
    pub(crate) value: T,
}

/// Span used for single-file configs
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Span(pub usize, pub usize);

/// Span used for configs that are split across different files
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileSpan(pub Arc<PathBuf>, pub Span);

/// Adds custom span type ot the abstract syntax tree (AST) nodes
pub trait SpanContext<P> {
    type Span;
    fn from_positions(&self, start: P, end: P) -> Self::Span;
}

pub(crate) struct SimpleContext;

pub(crate) struct FileContext {
    file_path: Arc<PathBuf>,
}

impl SpanContext<usize> for SimpleContext {
    type Span = Span;
    fn from_positions(&self, start: usize, end: usize) -> Self::Span {
        Span(start, end)
    }
}

impl SpanContext<usize> for FileContext {
    type Span = FileSpan;
    fn from_positions(&self, start: usize, end: usize) -> Self::Span {
        FileSpan(self.file_path.clone(), Span(start, end))
    }
}

impl<T, S> std::ops::Deref for Spanned<T, S> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T, S> Spanned<T, S> {
    pub fn span(&self) -> &S {
        &self.span
    }
}

impl<S, T: PartialEq<T>> PartialEq for Spanned<T, S> {
    fn eq(&self, other: &Spanned<T, S>) -> bool {
        self.value == other.value
    }
}

impl<S, T: PartialOrd<T>> PartialOrd for Spanned<T, S> {
    fn partial_cmp(&self, other: &Spanned<T, S>)
        -> Option<std::cmp::Ordering>
    {
        self.value.partial_cmp(&other.value)
    }
}

impl<S, T: Ord> Ord for Spanned<T, S> {
    fn cmp(&self, other: &Spanned<T, S>) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl<S, T: Eq> Eq for Spanned<T, S> {}

impl<S, T: std::hash::Hash> std::hash::Hash for Spanned<T, S> {
    fn hash<H>(&self, state: &mut H)
        where H: std::hash::Hasher,
    {
        self.value.hash(state)
    }
}