use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Span {
    pub(crate) range: Range<usize>,
}

impl Span {
    pub fn new(range: Range<usize>) -> Self {
        Self { range }
    }
    pub fn start(&self) -> usize {
        self.range.start
    }
    pub fn end(&self) -> usize {
        self.range.end
    }
}

pub trait ContainsSpan {
    fn span(&self) -> &Span;
    fn are_adjacent(&self, other: &impl ContainsSpan) -> bool {
        self.span().end() == other.span().start()
    }
}