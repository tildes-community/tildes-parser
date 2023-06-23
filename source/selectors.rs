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

  /// Selector for sub groups.
  pub static ref GROUP_SUB_GROUP_LINKS: Selector = selector(r#"#sidebar .link-group"#);

  /// Selector for group wiki links.
  pub static ref GROUP_WIKI_LINKS: Selector = selector(r#"#sidebar [href*="/wiki/"]"#);

  /// Selector for the site header context.
  pub static ref SITE_HEADER_CONTEXT: Selector = selector(".site-header-context");

  /// Selector for `<time>` elements that have a `datetime` attribute.
  pub static ref TIME_WITH_DATETIME: Selector = selector("time[datetime]");

  /// Selector for the topic comment count.
  pub static ref TOPIC_COMMENT_COUNT: Selector = selector(".topic-comments-header h2");

  /// Selector for the topic full byline.
  pub static ref TOPIC_FULL_BYLINE: Selector = selector(".topic-full-byline");

  /// Selector for a link topic's content.
  pub static ref TOPIC_FULL_LINK: Selector = selector(".topic-full-link a");

  /// Selector for the topic tag elements.
  pub static ref TOPIC_FULL_TAGS: Selector = selector(".topic-full-tags a");

  /// Selector for a text topic's content.
  pub static ref TOPIC_FULL_TEXT: Selector = selector(".topic-full-text");

  /// Selector for the main topic `<article>`.
  pub static ref TOPIC_MAIN_ARTICLE: Selector = selector("main > .topic-full");

  /// Selector for a topic toast warning.
  pub static ref TOPIC_TOAST_WARNING: Selector = selector(".toast.toast-warning");

  /// Selector for the topic vote count.
  pub static ref TOPIC_VOTE_COUNT: Selector = selector(".topic-voting-votes");
}
