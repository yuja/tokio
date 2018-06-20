use collect::SpanHandle;

pub trait Value {
    fn tag_span(&self, span: &SpanHandle, key: &'static str);
}

impl Value for usize {
    fn tag_span(&self, span: &SpanHandle, key: &'static str) {
        span.tag_usize(key, *self);
    }
}
