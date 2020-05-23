use std::io::{BufReader, BufWriter, stdin, stdout, Read, Write};

use clap::{App, Arg};

fn main() -> Result<(), &'static str> {
  let args = App::new("bytes2hex")
      .about("Quickly convert from binary to hex.")
      .long_about("Quickly convert from binary to hex.\nRead from stdin, output to stdout.")
      .get_matches();

  let reader = BufReader::new(stdin());
  let mut input_bytes = reader.bytes();
  let mut writer = BufWriter::new(stdout());

  while let Some(byte) = input_bytes.next() {
    let byte = byte.map_err(|_| "Failed to read.")?;
    write!(&mut writer, "{:02x}", byte).map_err(|_| "Failed to write.")?;
  }

  Ok(())
}
