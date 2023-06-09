//! All error types.

use thiserror::Error;

/// Errors that can happen while parsing.
#[derive(Debug, Error)]
pub enum ParseError {
  /// The error for when HTML text or attributes are missing and they should be
  /// present. This could be user error by having an edge case not covered, or
  /// Tildes could have been updated and the HTML changed in the meantime.
  #[error("Missing expected HTML values")]
  MissingExpectedHtml,
}
