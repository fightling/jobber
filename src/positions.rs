//! Positions within the database.

use itertools::Itertools;

/// Range between two Positions (including both ends).
pub struct PositionalRanges(Vec<(usize, usize)>);

impl PositionalRanges {
    /// Create empty positional range.
    pub const fn new() -> Self {
        Self(Vec::new())
    }
    // Return `true` if list is emtpy.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    /// Add new position to list.
    pub fn push(&mut self, item: (usize, usize)) {
        self.0.push(item)
    }
    /// Return last position in list.
    pub fn last(&self) -> Option<&(usize, usize)> {
        self.0.last()
    }
    /// Return last position in list as mutable.
    pub fn last_mut(&mut self) -> Option<&mut (usize, usize)> {
        self.0.last_mut()
    }
}

impl std::fmt::Display for PositionalRanges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|(f, t)| {
                    if f == t {
                        format!("{f}", f = f + 1)
                    } else {
                        format!("{f}-{t}", f = f + 1, t = t + 1)
                    }
                })
                .join(",")
        )
    }
}

/// List of positions (indexes) within the database.
#[derive(Clone, Debug)]
pub struct Positions(Vec<usize>);

impl Positions {
    pub fn into_ranges(&self) -> PositionalRanges {
        let mut ranges = PositionalRanges::new();
        for pos in self {
            if ranges.is_empty() {
                ranges.push((pos, pos));
            } else if ranges.last().unwrap().1 == (pos - 1) {
                ranges.last_mut().unwrap().1 = pos;
            } else {
                ranges.push((pos, pos));
            }
        }
        ranges
    }
    /// Return read-only iterator.
    pub fn iter(&self) -> core::slice::Iter<'_, usize> {
        self.0.iter()
    }
    /// Return `true` if list contains position
    pub fn contains(&self, position: &usize) -> bool {
        self.0.contains(position)
    }
}

/// Iterator over positions.
pub struct PositionsIterator<'a> {
    positions: &'a Positions,
    index: usize,
}

impl<'a> Iterator for PositionsIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(position) = self.positions.0.get(self.index) {
            self.index += 1;
            Some(*position)
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a Positions {
    type Item = usize;
    type IntoIter = PositionsIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PositionsIterator {
            positions: self,
            index: 0,
        }
    }
}

impl FromIterator<usize> for Positions {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut positions = Positions(Vec::new());
        for i in iter {
            positions.0.push(i)
        }
        positions
    }
}

impl std::fmt::Display for Positions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_ranges())
    }
}
