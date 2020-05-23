use clap::{Arg, App};
use std::io::{stdout, Write, stdin, Read};

fn main() -> Result<(), &'static str> {
  let args = App::new("hex2bytes")
      .about("Quickly convert hex to binary.")
      .arg(Arg::with_name("hex")
          .index(1)
          .about("The hex string to parse, or stdin if not specified.")
          .value_name("HEX")
          .required(false))
      .get_matches();
  let mut _readbuf;
  let hex_str = match args.value_of("hex") {
    Some(hex) => hex,
    None => {
      _readbuf = String::new();
      stdin().read_to_string(&mut _readbuf).map_err(|_| "Reading from stdin failed.")?;
      _readbuf.trim_end_matches('\n')
    }
  };
  let out = stdout();
  let mut out = out.lock();
  let mut chars_iter = hex_str.chars();
  let mut buf = String::with_capacity(2);
  loop {
    if let Some(ch1) = chars_iter.next() {
      if ch1 == ' ' {
        continue;
      }
      let ch2 = chars_iter.next().ok_or("Incomplete hex")?;
      if !ch1.is_ascii_alphanumeric() || !ch2.is_alphanumeric() {
        return Err("Invalid character");
      }
      buf.clear();
      buf.push(ch1);
      buf.push(ch2);
      let byte = u8::from_str_radix(&buf, 16).map_err(|_| "Invalid character")?;
      out.write(&[byte]).map_err(|_| "Output failed.")?;
    } else {
      return Ok(());
    }
  }
}
