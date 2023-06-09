//! Implements [`FromStr`] for parser structs.

use std::str::FromStr;

use {duplicate::duplicate_item, scraper::Html};

use crate::{Group, GroupList, ParseError};

#[duplicate_item(
  _Struct;
  [Group];
  [GroupList];
)]
impl FromStr for _Struct {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let html = Html::parse_document(s);
    Self::from_html(&html)
  }
}
