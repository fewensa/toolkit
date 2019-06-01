use std::{env, fs, io};
use std::path::{Path, PathBuf};

//pub fn canonicalize_path<S: AsRef<Path>>(path: S) -> io::Result<String> {
//  let buf = fs::canonicalize(path)?;
//  println!("{:?}", buf);
//  Ok(buf.to_str().ok_or(io::ErrorKind::NotFound)?.to_string())
//}

pub fn root_dir() -> PathBuf {
  Path::new("./").canonicalize().expect("Can not get root dir")
}

pub fn out_dir() -> PathBuf {
  Path::new(&env::var("OUT_DIR").expect("Can not get out dir")[..]).canonicalize().expect("Can not get out dir")
}



