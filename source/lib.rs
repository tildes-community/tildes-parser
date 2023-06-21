//! # Tildes Parser
//!
//! > **Consolidated parsing for Tildes.net HTML.**

#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::missing_docs_in_private_items)]

pub mod regexes;
pub mod selectors;
pub mod utilities;

pub(crate) mod error;
pub(crate) mod from_str;
pub(crate) mod group;
pub(crate) mod group_list;
pub(crate) mod topic;

pub use {
  error::*,
  group::{Group, GroupWikiLink},
  group_list::{GroupList, GroupListSummary},
  topic::*,
};
