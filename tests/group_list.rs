use std::fs::read_to_string;

use {insta::assert_debug_snapshot, scraper::Html, tildes_parser::GroupList};

#[test]
fn test_group_list_parsing() {
  let html = read_to_string("tests/samples/group_list.html").unwrap();
  let html = Html::parse_document(&html);
  let group_list = GroupList::from_html(&html).unwrap();
  assert_debug_snapshot!(group_list);
}
