use std::io::{self, Write};

use anyhow::Result;

pub fn write_to_stdout(rendered: &str) -> Result<()> {
    write!(io::stdout().lock(), "{}", rendered)?;
    Ok(())
}
