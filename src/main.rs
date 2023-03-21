mod state;

use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

use termion::{color, is_tty, style};

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();
    if !is_tty(&stdout) || !is_tty(&stdin()) {
        eprintln!("hydr requires an interactive terminal");
        exit(1);
    }
    write!(
        stdout,
        "Hello, {}Rust{}!\n",
        color::Fg(color::Rgb(0xff, 0x80, 0x00)),
        style::Reset,
    )?;

    Ok(())
}
