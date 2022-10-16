use std::fs::read_to_string;

use {insta::assert_debug_snapshot, tildes_parser::Group};

#[test]
fn test_group_parsing() {
  let html = read_to_string("tests/samples/group.html").unwrap();
  let group = &html.parse::<Group>().unwrap();
  assert_debug_snapshot!(group);
}
