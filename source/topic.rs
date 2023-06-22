//! Parsing for `/~<group>/<topic-id>`.

use scraper::Html;

use crate::{
  regexes::DUPLICATE_WHITESPACE_RE,
  selectors::{
    SITE_HEADER_CONTEXT, TOPIC_COMMENT_COUNT, TOPIC_FULL_BYLINE,
    TOPIC_FULL_LINK, TOPIC_FULL_TAGS, TOPIC_FULL_TEXT, TOPIC_MAIN_ARTICLE,
    TOPIC_TOAST_WARNING, TOPIC_VOTE_COUNT,
  },
  utilities::select_first_element_text,
  ParseError,
};

/// A Tildes topic.
#[derive(Debug)]
pub struct Topic {
  /// The name of the author.
  pub author: TopicAuthor,

  /// The amount of comments the topic has. Comments themselves have to be
  /// parsed separately.
  pub comment_total: i32,

  /// The content of the topic.
  pub content: TopicContent,

  /// The group the topic was posted in, with a leading tilde character.
  pub group: String,

  /// The unique ID of the topic.
  pub id: String,

  /// Whether the topic is locked.
  pub is_locked: bool,

  /// Whether the topic is official (not yet implemented, is always false).
  ///
  /// TODO: Add is_official. This isn't possible right now because topics don't
  /// have any indicator of being marked as official. The only place it's shown
  /// is in the topic listing. See #787 in the Tildes issue tracker.
  pub is_official: bool,

  /// All tags applied to the topic.
  pub tags: Vec<String>,

  /// The amount of votes the topic has received.
  pub vote_count: i32,
}

/// All the different ways a topic author can be represented.
#[derive(Debug)]
pub enum TopicAuthor {
  /// The normal case, where the topic author is available.
  Name(String),

  /// The topic was posted by Tildes itself.
  ///
  /// Technically the user for this is [Tildes](https://tildes.net/user/tildes)
  /// but in the topic it says "Automatically posted <date>" where the username
  /// normally goes, so may as well special-case it here too.
  Scheduled,

  /// The user was banned, deleted their account or disassociated the topic from
  /// their account.
  Unknown,
}

/// The different types of content a topic can have.
#[derive(Debug)]
pub enum TopicContent {
  /// The topic is a link topic pointing to an external site.
  Link(String),

  /// The topic is a text topic with a HTML body.
  Text(String),

  /// The topic's content is no longer available.
  Unknown,
}

impl Topic {
  /// Parses a [`Topic`] from a [`scraper::Html`] tree.
  pub fn from_html(html: &Html) -> Result<Self, ParseError> {
    let topic_article_element = html
      .select(&TOPIC_MAIN_ARTICLE)
      .next()
      .ok_or(ParseError::MissingExpectedHtml)?;

    let topic_byline =
      select_first_element_text(topic_article_element, &TOPIC_FULL_BYLINE)
        .map(|byline| {
          DUPLICATE_WHITESPACE_RE
            .replace_all(&byline, " ")
            .to_string()
        })
        .ok_or(ParseError::MissingExpectedHtml)?;

    let author = if topic_byline.starts_with("Automatically posted") {
      TopicAuthor::Scheduled
    } else if topic_byline.ends_with("unknown user") {
      TopicAuthor::Unknown
    } else {
      TopicAuthor::Name(
        topic_byline
          .split(" ")
          .last()
          .ok_or(ParseError::MissingExpectedHtml)?
          .to_string(),
      )
    };

    let comment_total = if let Some(comment_total) =
      select_first_element_text(topic_article_element, &TOPIC_COMMENT_COUNT)
    {
      comment_total
        .split(" ")
        .next()
        .map(|count| count.parse::<i32>())
        .ok_or(ParseError::MissingExpectedHtml)?
        .map_err(|_| ParseError::MissingExpectedHtml)?
    } else {
      0
    };

    let content = if let Some(link_content) =
      topic_article_element.select(&TOPIC_FULL_LINK).next()
    {
      TopicContent::Link(link_content.text().collect::<String>())
    } else if let Some(text_content) =
      topic_article_element.select(&TOPIC_FULL_TEXT).next()
    {
      TopicContent::Text(text_content.inner_html().trim().to_string())
    } else {
      TopicContent::Unknown
    };

    let group = DUPLICATE_WHITESPACE_RE
      .replace_all(
        html
          .select(&SITE_HEADER_CONTEXT)
          .next()
          .ok_or(ParseError::MissingExpectedHtml)?
          .text()
          .collect::<String>()
          .trim(),
        "",
      )
      .to_string();
    assert!(group.starts_with("~"));

    let id = topic_article_element
      .value()
      .id()
      .ok_or(ParseError::MissingExpectedHtml)?[6..]
      .to_string();

    let is_locked =
      select_first_element_text(topic_article_element, &TOPIC_TOAST_WARNING)
        .map(|toast| toast.contains("This topic is locked."))
        .unwrap_or_default();

    let tags = topic_article_element
      .select(&TOPIC_FULL_TAGS)
      .map(|tag| tag.text().collect::<String>())
      .collect::<Vec<_>>();

    let vote_count =
      select_first_element_text(topic_article_element, &TOPIC_VOTE_COUNT)
        .map(|vote_count| vote_count.parse::<i32>())
        .ok_or(ParseError::MissingExpectedHtml)?
        .map_err(|_| ParseError::MissingExpectedHtml)?;

    let topic = Topic {
      author,
      comment_total,
      content,
      group,
      id,
      is_locked,
      is_official: false, // TODO: Implement this once it can be done.
      tags,
      vote_count,
    };

    Ok(topic)
  }
}
