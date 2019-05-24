

use toolkit;

#[test]
fn test_uppercase_first_char() {
  assert_eq!("Abc", toolkit::text::uppercase_first_char("abc"));
}

#[test]
fn lowercase_first_char() {
  assert_eq!("aBC", toolkit::text::lowercase_first_char("ABC"));
}
