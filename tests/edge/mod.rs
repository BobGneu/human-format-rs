// Consolidated edge-case tests

mod clamp_behavior;
mod large_values;
mod nan_inf;
mod parse_errors;
mod rounding;

pub use clamp_behavior::*;
pub use large_values::*;
pub use nan_inf::*;
pub use parse_errors::*;
pub use rounding::*;
