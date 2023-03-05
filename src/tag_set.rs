use serde::{Deserialize, Serialize};

use crate::tags;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TagSet(pub Vec<String>);

impl TagSet {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn from_one(tag: &String) -> Self {
        Self(vec![tag.clone()])
    }
    pub fn from_option_vec(tags: &Option<Vec<String>>) -> Self {
        if let Some(tags) = tags {
            Self(tags.clone())
        } else {
            Self(Vec::new())
        }
    }
    pub fn filter<P>(&self, pred: P) -> Self
    where
        P: Fn(&String) -> bool,
    {
        let mut result = TagSet::new();
        for tag in &self.0 {
            if !pred(tag) {
                result.0.push(tag.clone());
            }
        }
        result
    }
    pub fn contains(&self, tag: &String) -> bool {
        self.0.contains(tag)
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl TagSet {
    pub fn insert(&mut self, tag: &str) -> bool {
        if self.0.contains(&tag.to_string()) {
            false
        } else {
            self.0.push(tag.to_string());
            true
        }
    }
    pub fn insert_many(&mut self, tags: Vec<String>) {
        for tag in tags {
            self.insert(&tag);
        }
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
