use collect::{Collect, Context, SpanHandle};
use span::Description;

pub struct LogTrace(());

impl LogTrace {
    pub fn new() -> LogTrace {
        LogTrace(())
    }
}

use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
static CNT: AtomicUsize = ATOMIC_USIZE_INIT;

impl Collect for LogTrace {
    fn new_span(&self, description: &Description, context: &Context) -> Option<SpanHandle> {
        let n = CNT.fetch_add(1, Ordering::Relaxed);
        info!("new_span; description={:?}; context={:?}", description, context);
        Some(SpanHandle::new(::std::sync::Arc::new(LogTrace::new()), n))
    }

    fn close_span(&self, id: usize) {
        info!("close_span; id={}", id);
    }

    fn tag_span_usize(&self, id: usize, key: &'static str, val: usize) {
        info!("tag_span_usize; id={}; key={}; val={}", id, key, val);
    }
}
