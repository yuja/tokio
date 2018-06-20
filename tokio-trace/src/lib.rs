//! Tokio event system
//!
//! Each span encapsulates the following state:
//!
//! * An operation name
//! * A start timestamp
//! * A finish timestamp
//! * A set of zero or more key:value Span Tags. The keys must be strings. The
//!   values may be strings, bools, or numeric types.
//! * A SpanContext (see below)
//! * References to zero or more causally-related Spans (via the SpanContext of
//!   those related Spans)

#[macro_use]
extern crate log as l;

pub mod collect;
pub mod log;
pub mod span;
pub mod tag;

pub use span::Span;
