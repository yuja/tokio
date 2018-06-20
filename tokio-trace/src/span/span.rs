use collect;
use span::Description;
use tag;

/// Event handle
pub struct Span {
    id: SpanId,
}

/// A span identifier
pub struct SpanId {
    handle: Option<collect::SpanHandle>,
}

pub fn span(description: &Description) -> Span {
    let handle = collect::with_current(|current| {
        current.and_then(|current| {
            let context = collect::Context::new(current.span());
            current.collect().new_span(description, &context)
        })
    });

    // Create the span identifier
    let id = SpanId { handle };

    // Create the handle
    Span { id }
}

impl Span {
    /// Enter the context of the span
    pub fn enter<F, R>(&mut self, f: F) -> R
    where F: FnOnce() -> R,
    {
        match self.id.handle {
            Some(ref mut handle) => handle.enter(f),
            None => f(),
        }
    }

    pub fn follows_from(&self, other: &SpanId) {
        unimplemented!();
    }

    pub fn tag<T: tag::Value>(&self, key: &'static str, value: T) {
        if let Some(ref handle) = self.id.handle {
            value.tag_span(handle, key);
        }
    }

    pub fn log(&self) {
        unimplemented!();
    }
}

impl Drop for Span {
    fn drop(&mut self) {
        if let Some(ref handle) = self.id.handle {
            handle.close();
        }
    }
}
