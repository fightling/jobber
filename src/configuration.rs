use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Configuration {
    /// Configuration used when no tag related configuration fit
    pub base: Properties,
    /// Configuration by tag
    pub tags: HashMap<String, Properties>,
}

impl Configuration {
    pub fn set(
        &mut self,
        tags: &Option<Vec<String>>,
        resolution: Option<f64>,
        pay: Option<f64>,
        max_hours: Option<u32>,
    ) -> Properties {
        let update = Properties {
            resolution,
            pay,
            max_hours,
        };
        if let Some(tags) = tags {
            for tag in tags {
                if let Some(tag_configuration) = self.tags.get_mut(tag) {
                    tag_configuration.update(update.clone());
                } else {
                    self.tags.insert(tag.clone(), update.clone());
                }
            }
        } else {
            self.base.update(update.clone());
        }
        update
    }
    /// provides configurations for display trait implementation
    pub fn get_and_why(&self, tags: &TagSet) -> (Option<String>, &Properties) {
        for tag in &tags.0 {
            if let Some(properties) = self.tags.get(tag) {
                return (Some(tag.clone()), properties);
            }
        }
        (None, &self.base)
    }
    pub fn get(&self, tags: &TagSet) -> &Properties {
        match &self.get_checked(tags) {
            Ok(properties) => properties,
            _ => panic!("unexpected tag collision"),
        }
    }
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Properties {
    /// Time resolution in fractional hours
    pub resolution: Option<f64>,
    /// Pay for an hour
    pub pay: Option<f64>,
    /// Maximum work hours per day
    pub max_hours: Option<u32>,
}

impl Properties {
    pub fn update(&mut self, properties: Properties) {
        if let Some(resolution) = properties.resolution {
            self.resolution = Some(resolution);
        }
        if let Some(pay) = properties.pay {
            self.pay = Some(pay);
        }
        if let Some(max_hours) = properties.max_hours {
            self.max_hours = Some(max_hours);
        }
    }
}

impl Default for Properties {
    fn default() -> Self {
        Self {
            resolution: Some(0.25),
            pay: None,
            max_hours: None,
        }
    }
}

impl std::fmt::Display for Properties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(resolution) = self.resolution {
            writeln!(f, "Resolution: {} hours", resolution)?;
        }
        if let Some(pay) = self.pay {
            writeln!(f, "Payment per hour: {}", pay)?
        };
        if let Some(max_hours) = self.max_hours {
            writeln!(f, "Maximum work time: {} hours", max_hours)?
        };
        Ok(())
    }
}
