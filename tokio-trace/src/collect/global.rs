use collect::Collect;

use std::cell::Cell;

thread_local!(static CURRENT_TRACE: Cell<Option<Context>> = Cell::new(None));

pub(crate) struct CurrentTrace<'a> {
    collect: &'a Collect,
    span: Option<usize>,
}

/// Stores the trace context in thread-local
#[derive(Copy, Clone)]
struct Context {
    collect: *const Collect,
    span: Option<usize>,
}

impl<'a> CurrentTrace<'a> {
    pub(crate) fn collect(&self) -> &Collect {
        self.collect
    }

    pub(crate) fn span(&self) -> Option<usize> {
        self.span
    }
}

/// Set the trace collector
///
/// # Panics
///
/// This function panics if a trace collector is already set.
pub fn with_default<F, R>(collect: &Collect, f: F) -> R
where F: FnOnce() -> R,
{
    CURRENT_TRACE.with(|cell| {
        assert!(cell.get().is_none(), "default collector already set");

        enter2(cell, collect, None, f)
    })
}

/// Enter a span
pub(crate) fn enter<F, R>(collect: &Collect, span: usize, f: F) -> R
where F: FnOnce() -> R,
{
    CURRENT_TRACE.with(|cell| {
        enter2(cell, collect, Some(span), f)
    })
}

fn enter2<F, R>(
    cell: &Cell<Option<Context>>,
    collect: &Collect,
    span: Option<usize>,
    f: F
) -> R
where F: FnOnce() -> R
{
    // Ensure that the executor is removed from the thread-local context
    // when leaving the scope. This handles cases that involve panicking.
    struct Reset<'a> {
        cell: &'a Cell<Option<Context>>,
        prev: Option<Context>,
    }

    impl<'a> Drop for Reset<'a> {
        fn drop(&mut self) {
            self.cell.set(self.prev.take());
        }
    }

    unsafe fn hide_lt<'a>(p: *const (Collect + 'a)) -> *const (Collect + 'static) {
        use std::mem;
        mem::transmute(p)
    }

    let prev = cell.get();

    let _reset = Reset {
        cell,
        prev,
    };

    // While scary, this is safe. The function takes a
    // `&Collect`, which guarantees that the reference lives for the
    // duration of `with_default`.
    //
    // Because we are always clearing the TLS value at the end of the
    // function, we can cast the reference to 'static which thread-local
    // cells require.
    let collect = unsafe { hide_lt(collect as &_ as *const _) };

    cell.set(Some(Context {
        collect,
        span,
    }));

    f()
}

pub(crate) fn with_current<F, R>(f: F) -> R
where F: FnOnce(Option<&CurrentTrace>) -> R
{
    CURRENT_TRACE.with(|cell| {
        match cell.get() {
            Some(ctx) => {
                // The lifetime is guaranteed as the reference is only passed
                // into a closure.
                let collect = unsafe { &*ctx.collect };
                let current = CurrentTrace {
                    collect,
                    span: ctx.span,
                };

                f(Some(&current))
            }
            None => {
                f(None)
            }
        }
    })
}
