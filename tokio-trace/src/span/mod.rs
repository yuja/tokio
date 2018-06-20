mod description;
mod span;

pub use self::description::Description;
pub use self::span::{span, Span, SpanId};

#[macro_export]
macro_rules! span {
    ($name:expr) => {
        $crate::span::span(&{
            let mut description = $crate::span::Description::new($name);
            description.set_module_path(module_path!());
            description
        });
    };
    ($name:expr, $( $k:ident = $v:expr ),+) => {{
        let span = span!($name);
        $(
            span.tag(stringify!($k), $v);
        )+
        span
    }};
}
