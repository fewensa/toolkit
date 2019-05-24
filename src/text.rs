/// Uppercase first char
///
/// # Examples
///
/// ```rust
/// assert_eq!("Abc", toolkit::text::uppercase_first_char("abc"));
/// ```
pub fn uppercase_first_char<S: AsRef<str>>(s: S) -> String {
  let mut c = s.as_ref().chars();
  match c.next() {
    None => String::new(),
    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
  }
}

/// Lowercase first char
///
/// # Examples
///
/// ```rust
/// assert_eq!("aBC", toolkit::text::lowercase_first_char("ABC"));
/// ```
pub fn lowercase_first_char<S: AsRef<str>>(s: S) -> String {
  let mut c = s.as_ref().chars();
  match c.next() {
    None => String::new(),
    Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
  }
}
