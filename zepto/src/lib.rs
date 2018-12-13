extern crate tui;
extern crate termion;
extern crate rand;
extern crate unicode_width;

pub(crate) mod util;
mod frontend;

use std::io;
use std::error;
use crate::frontend::launch_frontend;

pub fn launch() -> Result<(), Box<error::Error>> {
    launch_frontend()
}
