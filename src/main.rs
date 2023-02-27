mod files;
mod process;
mod utils;

use crate::process::process;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    process(&mut stdin, &mut stdout)?;

    Ok(())
}
