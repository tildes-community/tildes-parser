use std::fs::read_to_string;

use {insta::assert_debug_snapshot, tildes_parser::Topic};

#[test]
fn test_topic_parsing() {
  let samples = ["link", "text", "deleted", "scheduled"];
  for sample in samples {
    let html =
      read_to_string(format!("tests/samples/topic_{sample}.html")).unwrap();
    let topic = &html.parse::<Topic>().unwrap();
    assert_debug_snapshot!(sample, topic);
  }
}
