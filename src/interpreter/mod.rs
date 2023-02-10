mod error;
pub use error::Error;
mod context;
use context::Context;
mod structs;
use structs::Value;
mod binop;
mod runner;
pub use runner::run;