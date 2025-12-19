// Consolidated edge-case tests

mod parse_errors;
mod nan_inf;
mod rounding;
mod large_values;
mod clamp_behavior;

pub use parse_errors::*;
pub use nan_inf::*;
pub use rounding::*;
pub use large_values::*;
pub use clamp_behavior::*;
