//! All [`Selector`]s used in parsing.

use {lazy_static::lazy_static, scraper::Selector};

use crate::utilities::selector;

lazy_static! {
  /// Selector for links to Tildes groups.
  pub static ref GROUP_LINK: Selector = selector(".link-group");

  /// Selector for the activity section in group list items.
  pub static ref GROUP_LIST_ACTIVITY: Selector = selector(".group-list-activity");

  /// Selector for the description section in group list items.
  pub static ref GROUP_LIST_DESCRIPTION: Selector = selector(".group-list-description");
}
