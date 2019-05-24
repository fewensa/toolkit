use std::ffi::OsStr;
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

/// Check string is a number
/// # Example
///
/// ```rust
/// assert_eq!(true, toolkit::number::is_number("0"));
/// assert_eq!(false, toolkit::number::is_number("-1u32"));
/// assert_eq!(true, toolkit::number::is_number("2usize"));
/// assert_eq!(true, toolkit::number::is_number("3.5f32"));
/// assert_eq!(false, toolkit::number::is_number("0.2.1f32"));
/// ```
pub fn is_number<S>(text: S) -> bool where S: AsRef<OsStr> {
  let chars = self::to_chars(text);
  let mut has_dot = false;
  let mut has_negative = false;
  let mut tpmode = false;
  let mut tpsymbol = None;
  let mut tpvec = Vec::with_capacity(5);
  for (ix, &ch) in chars.iter().enumerate() {
    if tpmode {
      tpvec.push(ch);
      continue;
    }
    if ch == '-' && ix == 0 {
      has_negative = true;
      continue;
    }
    if ch == '.' {
      if has_dot { return false; }
      has_dot = true;
      continue;
    }
    if ch == 'f' || ch == 'u' || ch == 'i' {
      tpsymbol = Some(ch);
      tpvec.push(ch);
      tpmode = true;
      continue;
    }
    if self::char_is_digit(ch) { continue; }
    return false;
  }

  if !tpvec.is_empty() {
    let tvs: String = tpvec.into_iter().collect();
    let tpvstr = &tvs[..];
    if !(tpvstr == "i8" || tpvstr == "i16" || tpvstr == "i32" || tpvstr == "i64" || tpvstr == "i128" || tpvstr == "isize" ||
      tpvstr == "u8" || tpvstr == "u16" || tpvstr == "u32" || tpvstr == "u64" || tpvstr == "u128" || tpvstr == "usize" ||
      tpvstr == "f32" || tpvstr == "f64") {
      return false;
    }
  }

  match tpsymbol {
    Some('f') => true,
    Some('i') => !chars.contains(&'.'),
    Some('u') => !chars.contains(&'.') && !chars.contains(&'-'),
    Some(_) => false,
    None => true
  }
}

/// Check string is usize digit
///
/// # Example
///
/// ```rust
/// assert_eq!(true, toolkit::number::is_udigit("2"));
/// ```
pub fn is_udigit<S>(text: S) -> bool where S: AsRef<OsStr> {
  self::is_digit(text, false)
}

/// Check string is isize digit
///
/// # Example
///
/// ```rust
/// assert_eq!(true, toolkit::number::is_idigit("-2"));
/// ```
pub fn is_idigit<S>(text: S) -> bool where S: AsRef<OsStr> {
  self::is_digit(text, true)
}

/// Check string is digit
///
/// # Example
///
/// ```rust
/// assert_eq!(true, toolkit::number::is_digit("2", false));
/// assert_eq!(false, toolkit::number::is_digit("-2", false));
/// assert_eq!(false, toolkit::number::is_digit("0.2", false));
/// ```
pub fn is_digit<S>(text: S, allow_negative: bool) -> bool where S: AsRef<OsStr> {
  let chars = self::to_chars(text);
  for (ix, &ch) in chars.iter().enumerate() {
    if allow_negative && ch == '-' && ix == 0 { continue; }
    if self::char_is_digit(ch) { continue; }
    return false;
  }
  true
}

/// covert string to isize
///
/// # Example
///
/// ```rust
/// assert_eq!(1 as isize, toolkit::number::as_isize("1").unwrap());
/// ```
pub fn as_isize<S>(text: S) -> Result<isize, ParseIntError> where S: AsRef<OsStr> {
  self::to_str(text).parse::<isize>()
}

/// covert string to iseize with default, will check covert error, if error use default value.
///
/// # Example
///
/// ```rust
/// assert_eq!(-1 as isize, toolkit::number::as_isized("a", -1 as isize));
/// ```
pub fn as_isized<S>(text: S, def: isize) -> isize where S: AsRef<OsStr> {
  let tsr = self::to_str(text);
  if tsr.is_empty() {
    return def;
  }
  match tsr.parse::<isize>() {
    Ok(n) => n,
    Err(_) => def
  }
}

/// covert string to i8
///
/// # Example
///
/// ```rust
/// assert_eq!(1 as i8, toolkit::number::as_i8("1").unwrap());
/// ```
pub fn as_i8<S>(text: S) -> Result<i8, ParseIntError> where S: AsRef<OsStr> {
  self::to_str(text).parse::<i8>()
}

/// covert string to i8 with default, will check covert error, if error use default value.
///
/// # Example
///
/// ```rust
/// assert_eq!(-1 as i8, toolkit::number::as_i8d("a", -1 as i8));
/// ```
pub fn as_i8d<S>(text: S, def: i8) -> i8 where S: AsRef<OsStr> {
  let tsr = self::to_str(text);
  if tsr.is_empty() {
    return def;
  }
  match tsr.parse::<i8>() {
    Ok(n) => n,
    Err(_) => def
  }
}

/// covert string to i16
///
/// # Example
///
/// ```rust
/// assert_eq!(1 as i16, toolkit::number::as_i16("1").unwrap());
/// ```
pub fn as_i16<S>(text: S) -> Result<i16, ParseIntError> where S: AsRef<OsStr> {
  self::to_str(text).parse::<i16>()
}

/// covert string to i16 with default, will check covert error, if error use default value.
///
/// # Example
///
/// ```rust
/// assert_eq!(-1 as i16, toolkit::number::as_i16d("a", -1 as i16));
/// ```
pub fn as_i16d<S>(text: S, def: i16) -> i16 where S: AsRef<OsStr> {
  let tsr = self::to_str(text);
  if tsr.is_empty() {
    return def;
  }
  match tsr.parse::<i16>() {
    Ok(n) => n,
    Err(_) => def
  }
}

/// covert string to i32
///
/// # Example
///
/// ```rust
/// assert_eq!(1 as i32, toolkit::number::as_i32("1").unwrap());
/// ```
pub fn as_i32<S>(text: S) -> Result<i32, ParseIntError> where S: AsRef<OsStr> {
  self::to_str(text).parse::<i32>()
}

/// covert string to i32 with default, will check covert error, if error use default value.
///
/// # Example
///
/// ```rust
/// assert_eq!(-1 as i32, toolkit::number::as_i32d("a", -1 as i32));
/// ```
pub fn as_i32d<S>(text: S, def: i32) -> i32 where S: AsRef<OsStr> {
  let tsr = self::to_str(text);
  if tsr.is_empty() {
    return def;
  }
  match tsr.parse::<i32>() {
    Ok(n) => n,
    Err(_) => def
  }
}

/// covert string to i64
///
/// # Example
///
/// ```rust
/// assert_eq!(1 as i64, toolkit::number::as_i64("1").unwrap());
/// ```
pub fn as_i64<S>(text: S) -> Result<i64, ParseIntError> where S: AsRef<OsStr> {
  self::to_str(text).parse::<i64>()
}

/// covert string to i64 with default, will check covert error, if error use default value.
///
/// # Example
///
/// ```rust
/// assert_eq!(-1 as i64, toolkit::number::as_i64d("a", -1 as i64));
/// ```
pub fn as_i64d<S>(text: S, def: i64) -> i64 where S: AsRef<OsStr> {
  let tsr = self::to_str(text);
  if tsr.is_empty() {
    return def;
  }
  match tsr.parse::<i64>() {
    Ok(n) => n,
    Err(_) => def
  }
}

/// covert string to i128
///
/// # Example
///
/// ```rust
/// assert_eq!(1 as i128, toolkit::number::as_i128("1").unwrap());
/// ```
pub fn as_i128<S>(text: S) -> Result<i128, ParseIntError> where S: AsRef<OsStr> {
  self::to_str(text).parse::<i128>()
}

/// covert string to i128 with default, will check covert error, if error use default value.
///
/// # Example
///
/// ```rust
/// assert_eq!(-1 as i128, toolkit::number::as_i128d("a", -1 as i128));
/// ```
pub fn as_i128d<S>(text: S, def: i128) -> i128 where S: AsRef<OsStr> {
  self::as_isized(text, def as isize) as i128
}


/// covert string to usize
///
/// # Example
///
/// ```rust
/// assert_eq!(1 as usize, toolkit::number::as_usize("1").unwrap());
/// ```
pub fn as_usize<S>(text: S) -> Result<usize, ParseIntError> where S: AsRef<OsStr> {
  self::to_str(text).parse::<usize>()
}

/// covert string to usize with default, will check covert error, if error use default value.
///
/// # Example
///
/// ```rust
/// assert_eq!(1 as usize, toolkit::number::as_usized("a", 1 as usize));
/// ```
pub fn as_usized<S>(text: S, def: usize) -> usize where S: AsRef<OsStr> {
  let tsr = self::to_str(text);
  if tsr.is_empty() {
    return def;
  }
  match tsr.parse::<usize>() {
    Ok(n) => n,
    Err(_) => def
  }
}

/// covert string to u8
///
/// # Example
///
/// ```rust
/// assert_eq!(1 as u8, toolkit::number::as_u8("1").unwrap());
/// ```
pub fn as_u8<S>(text: S) -> Result<u8, ParseIntError> where S: AsRef<OsStr> {
  self::to_str(text).parse::<u8>()
}

/// covert string to u8 with default, will check covert error, if error use default value.
///
/// # Example
///
/// ```rust
/// assert_eq!(1 as u8, toolkit::number::as_u8d("a", 1 as u8));
/// ```
pub fn as_u8d<S>(text: S, def: u8) -> u8 where S: AsRef<OsStr> {
  let tsr = self::to_str(text);
  if tsr.is_empty() {
    return def;
  }
  match tsr.parse::<u8>() {
    Ok(n) => n,
    Err(_) => def
  }
}

/// covert string to u16
///
/// # Example
///
/// ```rust
/// assert_eq!(1 as u16, toolkit::number::as_u16("1").unwrap());
/// ```
pub fn as_u16<S>(text: S) -> Result<u16, ParseIntError> where S: AsRef<OsStr> {
  self::to_str(text).parse::<u16>()
}

/// covert string to u16 with default, will check covert error, if error use default value.
///
/// # Example
///
/// ```rust
/// assert_eq!(1 as u16, toolkit::number::as_u16d("a", 1 as u16));
/// ```
pub fn as_u16d<S>(text: S, def: u16) -> u16 where S: AsRef<OsStr> {
  let tsr = self::to_str(text);
  if tsr.is_empty() {
    return def;
  }
  match tsr.parse::<u16>() {
    Ok(n) => n,
    Err(_) => def
  }
}

/// covert string to u32
///
/// # Example
///
/// ```rust
/// assert_eq!(1 as u32, toolkit::number::as_u32("1").unwrap());
/// ```
pub fn as_u32<S>(text: S) -> Result<u32, ParseIntError> where S: AsRef<OsStr> {
  self::to_str(text).parse::<u32>()
}

/// covert string to u32 with default, will check covert error, if error use default value.
///
/// # Example
///
/// ```rust
/// assert_eq!(1 as u32, toolkit::number::as_u32d("a", 1 as u32));
/// ```
pub fn as_u32d<S>(text: S, def: u32) -> u32 where S: AsRef<OsStr> {
  let tsr = self::to_str(text);
  if tsr.is_empty() {
    return def;
  }
  match tsr.parse::<u32>() {
    Ok(n) => n,
    Err(_) => def
  }
}

/// covert string to u64
///
/// # Example
///
/// ```rust
/// assert_eq!(1 as u64, toolkit::number::as_u64("1").unwrap());
/// ```
pub fn as_u64<S>(text: S) -> Result<u64, ParseIntError> where S: AsRef<OsStr> {
  self::to_str(text).parse::<u64>()
}

/// covert string to u64 with default, will check covert error, if error use default value.
///
/// # Example
///
/// ```rust
/// assert_eq!(1 as u64, toolkit::number::as_u64d("a", 1 as u64));
/// ```
pub fn as_u64d<S>(text: S, def: u64) -> u64 where S: AsRef<OsStr> {
  let tsr = self::to_str(text);
  if tsr.is_empty() {
    return def;
  }
  match tsr.parse::<u64>() {
    Ok(n) => n,
    Err(_) => def
  }
}


/// covert string to u128
///
/// # Example
///
/// ```rust
/// assert_eq!(1 as u128, toolkit::number::as_u128("1").unwrap());
/// ```
pub fn as_u128<S>(text: S) -> Result<u128, ParseIntError> where S: AsRef<OsStr> {
  self::to_str(text).parse::<u128>()
}

/// covert string to u128 with default, will check covert error, if error use default value.
///
/// # Example
///
/// ```rust
/// assert_eq!(1 as u128, toolkit::number::as_u128d("a", 1 as u128));
/// ```
pub fn as_u128d<S>(text: S, def: u128) -> u128 where S: AsRef<OsStr> {
  self::as_usized(text, def as usize) as u128
}



/// covert string to f32
///
/// # Example
///
/// ```rust
/// assert_eq!(1.2 as f32, toolkit::number::as_f32("1.2").unwrap());
/// ```
pub fn as_f32<S>(text: S) -> Result<f32, ParseFloatError> where S: AsRef<OsStr> {
  self::to_str(text).parse::<f32>()
}

/// covert string to f32 with default, will check covert error, if error use default value.
///
/// # Example
///
/// ```rust
/// assert_eq!(1.2 as f32, toolkit::number::as_f32d("a", 1.2 as f32));
/// ```
pub fn as_f32d<S>(text: S, def: f32) -> f32 where S: AsRef<OsStr> {
  let tsr = self::to_str(text);
  if tsr.is_empty() {
    return def;
  }
  match tsr.parse::<f32>() {
    Ok(n) => n,
    Err(_) => def
  }
}


/// covert string to f64
///
/// # Example
///
/// ```rust
/// assert_eq!(1.2 as f64, toolkit::number::as_f64("1.2").unwrap());
/// ```
pub fn as_f64<S>(text: S) -> Result<f64, ParseFloatError> where S: AsRef<OsStr> {
  self::to_str(text).parse::<f64>()
}

/// covert string to f64 with default, will check covert error, if error use default value.
///
/// # Example
///
/// ```rust
/// assert_eq!(1.2 as f64, toolkit::number::as_f64d("a", 1.2 as f64));
/// ```
pub fn as_f64d<S>(text: S, def: f64) -> f64 where S: AsRef<OsStr> {
  let tsr = self::to_str(text);
  if tsr.is_empty() {
    return def;
  }
  match tsr.parse::<f64>() {
    Ok(n) => n,
    Err(_) => def
  }
}


fn to_str<S>(text: S) -> String where S: AsRef<OsStr> {
  text.as_ref().to_str().unwrap().to_string()
}

fn to_chars<S>(text: S) -> Vec<char> where S: AsRef<OsStr> {
  text.as_ref().to_str().unwrap().chars().collect()
}

fn char_is_digit(ch: char) -> bool {
  ch == '0' ||
    ch == '1' ||
    ch == '2' ||
    ch == '3' ||
    ch == '4' ||
    ch == '5' ||
    ch == '6' ||
    ch == '7' ||
    ch == '8' ||
    ch == '9'
}
