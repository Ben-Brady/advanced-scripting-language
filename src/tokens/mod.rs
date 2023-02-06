mod scanner;
use scanner::Scanner;

mod structs;
pub use structs::Token;

mod parser;
pub use parser::parse_file;
