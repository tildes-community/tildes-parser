//! All regular expressions used in parsing.

use {lazy_static::lazy_static, regex::Regex};

lazy_static! {
  /// Regular expression for replacing duplicate whitespace.
  ///
  /// ```rust
  /// use tildes_parser::regexes::DUPLICATE_WHITESPACE_RE;
  ///
  /// let text = DUPLICATE_WHITESPACE_RE.replace_all("example\n  text", " ");
  /// assert_eq!("example text", text);
  /// ```
  pub static ref DUPLICATE_WHITESPACE_RE: Regex = Regex::new(r"\s\s+").unwrap();

  /// Regular expression for extracting group subscriber count.
  pub static ref GROUP_SUBSCRIBERS_RE: Regex =
    Regex::new(r"(?P<count>\d+) subscribers").unwrap();

  /// Regular expression for extracting group list activity text.
  pub static ref GROUP_LIST_ACTIVITY_RE: Regex = {
    Regex::new(concat!(
      r"activity: ",
      r"(?P<topic>\d+) topics?, ",
      r"(?P<comment>\d+) comments?",
    )).unwrap()
  };
}
