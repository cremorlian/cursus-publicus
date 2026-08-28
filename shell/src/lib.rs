#![no_std]

use embedded_io::Write;
use cp_core::opening_text;

pub fn run<W: Write>(writer: &mut W) -> Result<(), W::Error> {
    writer.write_all(opening_text().as_bytes())?;
    writer.flush()?;
    Ok(())
}
