pub fn flatten<T>(origin: Option<Option<T>>) -> Option<T> {
  match origin {
    Some(v) => v,
    None => None,
  }
}

