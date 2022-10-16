//! Parsing for `/~<group>`.

use std::str::FromStr;

use {
  color_eyre::{eyre::Error, Result},
  scraper::Html,
};

use crate::{
  regexes::GROUP_SUBSCRIBERS_RE,
  selectors::{
    GROUP_DESCRIPTION, GROUP_NAME, GROUP_SUBSCRIBERS, GROUP_SUB_GROUP_LINKS,
    GROUP_WIKI_LINKS,
  },
  utilities::{
    extract_anchor_values, parse_regex_match, select_first_element_text,
  },
};

/// A group's information.
#[derive(Debug)]
pub struct Group {
  /// The group description.
  pub description: Option<String>,

  /// The group name, including leading tilde.
  pub name: String,

  /// Names of sub-groups.
  pub sub_groups: Vec<String>,

  /// The amount of subscribers.
  pub subscribers: i32,

  /// Links to wiki pages.
  pub wiki_links: Vec<GroupWikiLink>,
}

/// A group's wiki link.
#[derive(Debug)]
pub struct GroupWikiLink {
  /// The name of the wiki page.
  pub name: String,

  /// The URL to the wiki page.
  pub url: String,
}

impl FromStr for Group {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let html = Html::parse_document(s);
    Self::from_html(&html)
  }
}

impl Group {
  /// Parses a [`Group`] from a [`scraper::Html`] tree.
  pub fn from_html(html: &Html) -> Result<Self> {
    let description =
      select_first_element_text(html.root_element(), &GROUP_DESCRIPTION);

    let name =
      select_first_element_text(html.root_element(), &GROUP_NAME).unwrap();

    let subscribers = parse_regex_match(
      GROUP_SUBSCRIBERS_RE
        .captures_iter(
          &select_first_element_text(html.root_element(), &GROUP_SUBSCRIBERS)
            .unwrap(),
        )
        .next()
        .unwrap()
        .name("count"),
    )
    .unwrap();

    let sub_groups = html
      .select(&GROUP_SUB_GROUP_LINKS)
      .map(|element| extract_anchor_values(element).0)
      .collect();

    let wiki_links = html
      .select(&GROUP_WIKI_LINKS)
      .map(|element| {
        let (name, url) = extract_anchor_values(element);
        GroupWikiLink { name, url }
      })
      .collect();

    Ok(Self {
      description,
      name,
      sub_groups,
      subscribers,
      wiki_links,
    })
  }
}
