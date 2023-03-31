//! Configuration of a *jobber* database.

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration of a *jobber* database.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Configuration {
    /// Configuration used when no tag related configuration fit
    pub base: Properties,
    /// Configuration by tag
    pub tags: HashMap<String, Properties>,
}

impl Configuration {
    /// Partially overwrite properties of configurations which match the given tags.
    pub fn set(&mut self, tags: &Option<TagSet>, update: &Properties) -> bool {
        let mut modified = false;
        if let Some(tags) = tags {
            for tag in tags.iter() {
                if let Some(tag_configuration) = self.tags.get_mut(tag) {
                    if tag_configuration.update(update.clone()) {
                        modified = true;
                    }
                } else {
                    self.tags.insert(tag.clone(), update.clone());
                    modified = true;
                }
            }
        } else {
            if self.base.update(update.clone()) {
                modified = true;
            }
        }
        modified
    }
    /// get properties for the given tags and additionally return which tag was relevant
    pub fn get_and_why(&self, tags: &TagSet) -> (Option<String>, &Properties) {
        for tag in &tags.0 {
            if let Some(properties) = self.tags.get(tag) {
                return (Some(tag.clone()), properties);
            }
        }
        (None, &self.base)
    }
    /// get properties for the given tags
    pub fn get(&self, tags: &TagSet) -> &Properties {
        match &self.get_checked(tags) {
            Ok(properties) => properties,
            _ => panic!("unexpected tag collision"),
        }
    }
    /// get properties for the given tags and also check tag configuration consistency
    pub fn get_checked(&self, tags: &TagSet) -> Result<&Properties, Error> {
        let mut found = TagSet::new();
        let mut properties = None;
        for tag in &tags.0 {
            if let Some(p) = self.tags.get(tag) {
                found.insert(tag);
                properties = Some(p);
            }
        }
        match found.len() {
            0 => Ok(&self.base),
            1 => Ok(properties.unwrap()),
            _ => Err(Error::TagCollision(found)),
        }
    }
}

/// Properties within the database configuration.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Properties {
    /// Time resolution in fractional hours
    pub resolution: Option<f64>,
    /// Rate for an hour
    pub rate: Option<f64>,
    /// Maximum work hours per day
    pub max_hours: Option<u32>,
}

impl Properties {
    /// Update properties.
    /// # Arguments
    /// - `properties`: Properties to overwrite (empty properties will be ignored)
    /// # Return Value
    /// Returns `true` if any modification was made.
    pub fn update(&mut self, properties: Properties) -> bool {
        let mut modified = false;
        if let Some(resolution) = properties.resolution {
            self.resolution = Some(resolution);
            modified = true;
        }
        if let Some(rate) = properties.rate {
            self.rate = Some(rate);
            modified = true;
        }
        if let Some(max_hours) = properties.max_hours {
            self.max_hours = Some(max_hours);
            modified = true;
        }
        modified
    }
}

impl Default for Properties {
    fn default() -> Self {
        Self {
            resolution: Some(0.25),
            rate: None,
            max_hours: None,
        }
    }
}

impl std::fmt::Display for Properties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(resolution) = self.resolution {
            writeln!(f, "Resolution: {} hours", resolution)?;
        }
        if let Some(rate) = self.rate {
            writeln!(f, "Payment per hour: {}", rate)?
        };
        if let Some(max_hours) = self.max_hours {
            writeln!(f, "Maximum work time: {} hours", max_hours)?
        };
        Ok(())
    }
}
