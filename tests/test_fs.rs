

#[test]
fn test_append() {
  let file = &format!("{}/_tmp.txt", toolkit::path::root_dir())[..];
  assert!(toolkit::fs::append(file, "A").is_ok());
  assert!(toolkit::fs::append(file, "B").is_ok());
  assert!(toolkit::fs::append(file, "C").is_ok());
}

