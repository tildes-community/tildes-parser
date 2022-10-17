//! Implements [`FromStr`] for parser structs.

use std::str::FromStr;

use {color_eyre::eyre::Error, duplicate::duplicate_item, scraper::Html};

use crate::{Group, GroupList};

#[duplicate_item(
  _Struct;
  [Group];
  [GroupList];
)]
impl FromStr for _Struct {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let html = Html::parse_document(s);
    _Struct::from_html(&html)
  }
}
