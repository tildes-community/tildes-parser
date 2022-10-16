use std::fs::read_to_string;

use {insta::assert_debug_snapshot, tildes_parser::GroupList};

#[test]
fn test_group_list_parsing() {
  let html = read_to_string("tests/samples/group_list.html").unwrap();
  let group_list = html.parse::<GroupList>().unwrap();
  assert_debug_snapshot!(group_list);
}
