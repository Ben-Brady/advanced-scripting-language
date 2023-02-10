mod error;
pub use error::Error;
mod context;
use context::Context;
mod structs;
use structs::Value;
mod binop;
mod interpreter;
pub use interpreter::run;