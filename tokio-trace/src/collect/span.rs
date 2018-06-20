use collect::{self, Collect};

use std::sync::Arc;

/// Handle to a a span.
pub struct SpanHandle {
    collect: Arc<Collect>,
    id: usize,
}

impl SpanHandle {
    /// Create a new `SpanHandle`
    pub fn new(collect: Arc<Collect>, id: usize) -> SpanHandle {
        SpanHandle {
            collect,
            id,
        }
    }

    pub fn enter<F, R>(&mut self, f: F) -> R
    where F: FnOnce() -> R,
    {
        collect::enter(&*self.collect, self.id, f)
    }

    pub fn close(&self) {
        self.collect.close_span(self.id);
    }

    pub fn tag_usize(&self, key: &'static str, val: usize) {
        self.collect.tag_span_usize(self.id, key, val);
    }
}
