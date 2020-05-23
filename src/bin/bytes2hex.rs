use std::io::{BufReader, BufWriter, Read, stdin, stdout, Write};

use clap::App;

fn main() -> Result<(), &'static str> {
  let _args = App::new("bytes2hex")
      .about("Quickly convert from binary to hex.")
      .long_about("Quickly convert from binary to hex.\nRead from stdin, output to stdout.")
      .get_matches();

  let reader = BufReader::new(stdin());
  let  input_bytes = reader.bytes();
  let mut writer = BufWriter::new(stdout());

  for byte in input_bytes {
    let byte = byte.map_err(|_| "Failed to read.")?;
    write!(&mut writer, "{:02x}", byte).map_err(|_| "Failed to write.")?;
  }

  Ok(())
}
