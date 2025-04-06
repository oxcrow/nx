pub mod data;
pub mod error;
pub mod lexer;
pub mod parser;

use error::Result;

pub fn dev() -> Result<()> {
    Ok(())
}
