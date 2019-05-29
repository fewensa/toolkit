use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::fs::File;

pub fn append<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> io::Result<()> {
  if !path.as_ref().exists() {
    File::create(&path)?;
  }
  let mut file = fs::OpenOptions::new()
    .write(true)
    .append(true)
    .open(path)
    .unwrap();
  Ok(writeln!(file, "{}", String::from_utf8_lossy(contents.as_ref()).into_owned())?)
}

