use super::prelude::*;
use std::collections::HashSet;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Hash, Eq, PartialEq, EnumIter, Debug)]
pub enum Check {
    Overlaps,
    UnknownTags,
    ConfirmDeletion,
    EndBeforeStart,
    CollidingTags,
}

#[derive(Debug)]
pub struct Checks(HashSet<Check>);

impl Checks {
    pub fn all() -> Self {
        Self {
            0: HashSet::from_iter(Check::iter()),
        }
    }
    pub fn all_but(check: Check) -> Self {
        Self {
            0: HashSet::from_iter(Check::iter().filter(|c| *c != check)),
        }
    }
    pub fn omit() -> Self {
        Self { 0: HashSet::new() }
    }
    pub fn no_confirm() -> Self {
        Self {
            0: HashSet::from([Check::Overlaps]),
        }
    }
    pub fn has(&self, check: Check) -> bool {
        self.0.contains(&check)
    }
    pub fn check(
        &self,
        jobs: &Jobs,
        pos: Option<usize>,
        job: &Job,
        context: &Context,
    ) -> Result<(), Error> {
        let mut warnings = Vec::new();

        // check for temporal plausibility
        if self.has(Check::EndBeforeStart) {
            if let Some(end) = job.end {
                if job.start >= end {
                    return Err(Error::EndBeforeStart(job.start, end));
                }
            }
        }

        // check for overlapping
        if self.has(Check::Overlaps) {
            let mut overlapping = JobList::new_from(jobs);
            for (n, j) in jobs.jobs.iter().enumerate() {
                if job.overlaps(j, context) {
                    if let Some(pos) = pos {
                        if n != pos {
                            overlapping.push(n, j.clone());
                        }
                    } else {
                        overlapping.push(n, j.clone());
                    }
                }
            }
            if !overlapping.is_empty() {
                warnings.push(Warning::Overlaps {
                    new: job.clone(),
                    existing: overlapping,
                });
            }
        }

        // check for unknown tag
        if self.has(Check::UnknownTags) {
            let tags = jobs.tags();
            let unknown_tags: TagSet = job.tags.filter(|tag| tags.contains(tag));
            if !unknown_tags.0.is_empty() {
                warnings.push(Warning::UnknownTags(unknown_tags));
            }
        }

        // check for colliding tags
        if self.has(Check::CollidingTags) {
            jobs.get_configuration(&job.tags)?;
        }

        // react if any warnings
        if !warnings.is_empty() {
            return Err(Error::Warnings(warnings));
        }
        Ok(())
    }
}
