use std::{env, fs, io};
use std::path::Path;

pub fn canonicalize_path<S: AsRef<Path>>(path: S) -> io::Result<String> {
  let buf = fs::canonicalize(path)?;
  Ok(buf.to_str().ok_or(io::ErrorKind::NotFound)?.to_string())
}

pub fn root_dir() -> String {
  canonicalize_path("./").expect("Can not get root dir")
}

pub fn out_dir() -> String {
  env::var("OUT_DIR").expect("Can not get out dir")
}



