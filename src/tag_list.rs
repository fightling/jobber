use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug)]
pub struct TagList {
    tags: HashSet<String>,
}

impl TagList {
    pub fn new() -> Self {
        Self {
            tags: HashSet::new(),
        }
    }
    pub fn insert(&mut self, tag: &String) -> bool {
        self.tags.insert(tag.clone())
    }
    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }
}
impl std::fmt::Display for TagList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tags.iter().join(","))
    }
}
