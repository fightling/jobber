//! Check a job before insertion into job database.

use super::prelude::*;
use std::collections::HashSet;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// Selectable checks
#[derive(Hash, Eq, PartialEq, EnumIter, Debug)]
pub enum Check {
    /// Emit `Warning::Overlaps` if job would overlap another in time.
    Overlaps,
    /// Emit `Warning::UnknownTags` if any tags of the new job are unknown within the database.
    UnknownTags,
    /// Emit `Warning::ConfirmDeletion` if a job is about to be deleted.
    /// This check is done outside of `Checks`.
    ConfirmDeletion,
}

/// A set of selectable checks.
#[derive(Debug)]
pub struct Checks(HashSet<Check>);

impl Checks {
    /// Select all checks.
    pub fn all() -> Self {
        Self {
            0: HashSet::from_iter(Check::iter()),
        }
    }
    /// Select all checks but the given one.
    pub fn all_but(check: Check) -> Self {
        Self {
            0: HashSet::from_iter(Check::iter().filter(|c| *c != check)),
        }
    }
    /// Omit all checks.
    pub fn omit() -> Self {
        Self { 0: HashSet::new() }
    }
    pub fn no_confirm() -> Self {
        Self {
            0: HashSet::from([Check::Overlaps]),
        }
    }
    /// Return `true` if the given check is included.
    pub fn has(&self, check: Check) -> bool {
        self.0.contains(&check)
    }
    /// Check a job.
    /// # Arguments
    /// - `jobs`: Reference to the database.
    /// - `pos`: Position at which the job will be stored. This is important for modifying existing jobs.
    /// - `job`: The job to add or modify.
    /// - `context`: Temporal context to use.
    pub fn check(
        &self,
        jobs: &Jobs,
        pos: Option<usize>,
        job: &Job,
        context: &Context,
    ) -> Result<(), Error> {
        let mut warnings = Vec::new();

        // check for temporal plausibility
        if let Some(end) = job.end {
            if job.start >= end {
                return Err(Error::EndBeforeStart(job.start, end));
            }
        }

        // check for overlapping
        if self.has(Check::Overlaps) {
            let mut overlapping = JobList::new_from(jobs);
            for (n, j) in jobs.iter().enumerate() {
                if let Some(pos) = pos {
                    if n != pos && job.overlaps(&j, context) {
                        overlapping.push(n, j);
                    }
                } else if job.overlaps(&j, context) {
                    overlapping.push(n, j);
                }
            }
            if !overlapping.is_empty() {
                warnings.push(Warning::Overlaps {
                    new: job.clone(),
                    existing: overlapping.into(),
                });
            }
        }

        // check for unknown tag
        if self.has(Check::UnknownTags) {
            let tags = jobs.tags();
            let unknown_tags: TagSet = job.tags.filter(|tag| !tags.contains(tag));
            if !unknown_tags.0.is_empty() {
                warnings.push(Warning::UnknownTags(unknown_tags));
            }
        }

        // check for colliding tags
        jobs.configuration.get_checked(&job.tags)?;

        // react if any warnings
        if !warnings.is_empty() {
            return Err(Error::Warnings(warnings));
        }
        Ok(())
    }
}
