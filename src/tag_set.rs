//! Set of job tags.

use super::prelude::*;
use serde::{Deserialize, Serialize};

/// Set of job tags.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TagSet(pub Vec<String>);

impl TagSet {
    /// Create from scratch.
    pub const fn new() -> Self {
        Self(Vec::new())
    }
    /// Filter tags.
    pub fn filter<P>(&self, pred: P) -> Self
    where
        P: Fn(&&String) -> bool,
    {
        TagSet(self.0.iter().filter(pred).map(|t| t.to_string()).collect())
    }
    /// Return read-only iterator
    pub fn iter(&self) -> core::slice::Iter<'_, String> {
        self.0.iter()
    }
    /// Check if tag is contained.
    pub fn contains(&self, tag: &String) -> bool {
        self.0.contains(tag)
    }
    /// Check if this set is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    /// Return number of contained tags.
    pub fn len(&self) -> usize {
        self.0.len()
    }
    /// Modify tags like described in the manual.
    pub fn modify(&self, modification: &TagSet) -> TagSet {
        let mut modify = false;
        let mut tags = TagSet::new();
        for tag in modification.iter() {
            if tag.starts_with('+') || tag.ends_with('+') {
                tags.insert(tag[1..].into());
                modify = true;
            } else if tag.starts_with('-') || tag.ends_with('-') {
                tags.remove(tag[1..].into());
                modify = true;
            }
        }
        if modify {
            tags
        } else {
            modification.clone()
        }
    }
}

impl TagSet {
    /// Insert net tag.
    pub fn insert(&mut self, tag: &str) -> bool {
        if self.0.contains(&tag.to_string()) {
            false
        } else {
            self.0.push(tag.to_string());
            true
        }
    }
    /// Insert a lot of tags.
    pub fn insert_many(&mut self, tags: TagSet) {
        for tag in tags.iter() {
            self.insert(&tag);
        }
    }
    /// Remove a tag from the set.
    pub fn remove(&mut self, tag: &str) {
        self.0 = self
            .0
            .iter()
            .filter(|t| *t != tag)
            .map(|t| t.to_string())
            .collect();
    }
}
impl std::fmt::Display for TagSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (n, tag) in self.0.iter().enumerate() {
            tags::format(f, tag)?;
            if n + 1 < self.0.len() {
                write!(f, ", ")?;
            }
        }
        Ok(())
    }
}

impl From<Option<TagSet>> for TagSet {
    /// Flatten optional [TagSet] (empty if option was `None`).
    fn from(tags: Option<TagSet>) -> Self {
        if let Some(tags) = tags {
            tags.clone()
        } else {
            TagSet::new()
        }
    }
}

impl From<&Option<String>> for TagSet {
    /// Convert from optional comma separated string of tag names without spaces.
    fn from(tag: &Option<String>) -> Self {
        if let Some(tag) = tag {
            Self(tag.split(',').map(|t| t.to_string()).collect())
        } else {
            Self(Vec::new())
        }
    }
}
impl From<&str> for TagSet {
    /// convert from comma separated string of tag names without spaces
    fn from(tag: &str) -> Self {
        assert!(!tag.contains(" "));
        Self(tag.split(',').map(|t| t.to_string()).collect())
    }
}
