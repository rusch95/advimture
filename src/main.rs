extern crate zepto;

use std::error;

fn main() -> Result<(), Box<error::Error>> {
    let editor = zepto::launch()?;
    Ok(())
}
