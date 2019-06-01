

#[test]
fn test_append() {
  let file = toolkit::path::root_dir().join("_tmp.txt");
  println!("{:?}", file);
  assert!(toolkit::fs::append(&file, "A").is_ok());
  assert!(toolkit::fs::append(&file, "B").is_ok());
  assert!(toolkit::fs::append(&file, "C").is_ok());
}

