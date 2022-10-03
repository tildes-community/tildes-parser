use std::fs::read_to_string;

use {insta::assert_debug_snapshot, scraper::Html, tildes_parser::Group};

#[test]
fn test_group_parsing() {
  let html = read_to_string("tests/samples/group.html").unwrap();
  let html = Html::parse_document(&html);
  let group = Group::from_html(&html).unwrap();
  assert_debug_snapshot!(group);
}
