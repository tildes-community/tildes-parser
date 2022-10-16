//! Parsing for [`/groups`](https://tildes.net/groups).

use std::str::FromStr;

use {
  color_eyre::{eyre::Error, Result},
  scraper::Html,
};

use crate::{
  regexes::{DUPLICATE_WHITESPACE_RE, GROUP_LIST_ACTIVITY_RE},
  selectors::{GROUP_LINK, GROUP_LIST_ACTIVITY, GROUP_LIST_DESCRIPTION},
  utilities::{parse_regex_match, select_first_element_text, selector},
};

/// The group list from the [`/groups`](https://tildes.net/groups) page.
#[derive(Debug)]
pub struct GroupList {
  /// All group summaries found.
  pub summaries: Vec<GroupListSummary>,
}

/// The representation of a group on the [`/groups`](https://tildes.net/groups)
/// page.
#[derive(Debug)]
pub struct GroupListSummary {
  /// The approximate daily comment activity.
  pub comment_activity: Option<i32>,

  /// The group description.
  pub description: Option<String>,

  /// The group name, including leading tilde.
  pub name: String,

  /// The approximate daily topic activity.
  pub topic_activity: Option<i32>,
}

impl FromStr for GroupList {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let html = Html::parse_document(s);
    Self::from_html(&html)
  }
}

impl GroupList {
  /// Parses a [`GroupList`] from a [`scraper::Html`] tree.
  pub fn from_html(html: &Html) -> Result<Self> {
    let summaries = html
      .select(&selector(".group-list li"))
      .map(|parent| {
        let activity_counts = {
          let activity_text =
            select_first_element_text(parent, &GROUP_LIST_ACTIVITY)
              .unwrap_or_default();

          GROUP_LIST_ACTIVITY_RE
            .captures(&DUPLICATE_WHITESPACE_RE.replace_all(&activity_text, " "))
            .map(|captures| {
              (
                parse_regex_match(captures.name("comment")),
                parse_regex_match(captures.name("topic")),
              )
            })
            .unwrap_or_default()
        };

        GroupListSummary {
          comment_activity: activity_counts.0,
          description: select_first_element_text(
            parent,
            &GROUP_LIST_DESCRIPTION,
          ),
          name: select_first_element_text(parent, &GROUP_LINK).unwrap(),
          topic_activity: activity_counts.1,
        }
      })
      .collect();

    Ok(Self { summaries })
  }
}
