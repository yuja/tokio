use collect::{Context, SpanHandle};
use span::Description;

/// Manages a trace
pub trait Collect {
    /// Create a new span
    ///
    /// TODO: Should `name` be `&'static str`
    fn new_span(&self, desc: &Description, ctx: &Context) -> Option<SpanHandle>;

    /// Called when the span is closed
    fn close_span(&self, id: usize);

    fn tag_span_usize(&self, id: usize, key: &'static str, val: usize);
}
