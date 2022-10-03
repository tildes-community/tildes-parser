//! All [`Selector`]s used in parsing.

use {lazy_static::lazy_static, scraper::Selector};

use crate::utilities::selector;

lazy_static! {
  /// Selector for the group description.
  pub static ref GROUP_DESCRIPTION: Selector = selector(".group-short-description");

  /// Selector for links to Tildes groups.
  pub static ref GROUP_LINK: Selector = selector(".link-group");

  /// Selector for the activity section in group list items.
  pub static ref GROUP_LIST_ACTIVITY: Selector = selector(".group-list-activity");

  /// Selector for the description section in group list items.
  pub static ref GROUP_LIST_DESCRIPTION: Selector = selector(".group-list-description");

  /// Selector for the group name.
  pub static ref GROUP_NAME: Selector = selector("#sidebar h3");

  /// Selector for the group subscriber count.
  pub static ref GROUP_SUBSCRIBERS: Selector = selector(".group-subscription-count");

  /// Selector for group wiki links.
  pub static ref GROUP_SUB_GROUP_LINKS: Selector = selector(r#"#sidebar .link-group"#);

  /// Selector for group wiki links.
  pub static ref GROUP_WIKI_LINKS: Selector = selector(r#"#sidebar [href*="/wiki/"]"#);
}
