//! Temporal context of a jobber run.
//!
//! Having this is necessary for testing.

use super::prelude::*;

/// Temporal context of a jobber run.
#[derive(PartialEq, Clone, Debug)]
pub struct Context(DateTime);

impl Context {
    /// Create new context with current time.
    pub fn new() -> Self {
        Self(DateTime::now())
    }
    /// Create new context with given time.
    /// Only use in tests!
    pub fn new_test(local: &str) -> Self {
        Self(local.into())
    }
    /// Return time in context
    pub fn time(&self) -> DateTime {
        DateTime::from(self.0)
    }
    /// Return time in context
    pub fn date(&self) -> Date {
        DateTime::from(self.0).date()
    }
}
