// Copyright (C) 2022 Bauke <me@bauke.xyz>
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU Affero General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
// details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! # Tildes Parser
//!
//! > **Consolidated parsing for Tildes.net HTML.**

#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::missing_docs_in_private_items)]

pub mod regexes;
pub mod selectors;
pub mod utilities;

pub(crate) mod group;
pub(crate) mod group_list;

pub use {
  group::{Group, GroupWikiLink},
  group_list::{GroupList, GroupListSummary},
  scraper::Html,
};
