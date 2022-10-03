//! Miscellaneous parsing utilities.

use std::str::FromStr;

use {
  regex::Match,
  scraper::{ElementRef, Selector},
};

/// Shorthand to parse a [`regex::Match`] with [`std::str::FromStr`].
pub fn parse_regex_match<T: FromStr>(regex_match: Option<Match>) -> Option<T> {
  regex_match.and_then(|regex_match| regex_match.as_str().parse::<T>().ok())
}

/// Returns the text of the first found element inside the given `parent`
/// element.
pub fn select_first_element_text(
  parent: ElementRef,
  selector: &Selector,
) -> Option<String> {
  parent
    .select(selector)
    .next()
    .map(|element| element.text().collect::<String>())
    .map(|text| text.trim().to_string())
}

/// Shorthand for creating a [`scraper::Selector`].
pub fn selector(selector: &str) -> Selector {
  Selector::parse(selector).unwrap()
}
