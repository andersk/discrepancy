use std::cmp;
use super::level::{Level, MIN_LEVEL, MAX_LEVEL};
use super::monoid::Monoid;
use super::reverse::Reverse;
use super::sign::Sign;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiscrepancySubrange {
    pub sum: i32,
    pub short_plus_level: Level,
    pub minus_level: Level,
    pub long_plus_level: Level,
    pub minus_position: u32,
}

impl DiscrepancySubrange {
    pub fn singleton(position: u32, value: &Option<(Sign, Level)>) -> DiscrepancySubrange {
        match *value {
            None => DiscrepancySubrange {
                sum: -1,
                short_plus_level: MIN_LEVEL,
                minus_level: MAX_LEVEL,
                long_plus_level: MIN_LEVEL,
                minus_position: position,
            },
            Some((Sign::Plus, level)) => DiscrepancySubrange {
                sum: 1,
                short_plus_level: level,
                minus_level: MIN_LEVEL,
                long_plus_level: level,
                minus_position: !0,
            },
            Some((Sign::Minus, level)) => DiscrepancySubrange {
                sum: -1,
                short_plus_level: MIN_LEVEL,
                minus_level: level,
                long_plus_level: MIN_LEVEL,
                minus_position: position,
            },
        }
    }

    pub fn join(&self, other: &DiscrepancySubrange) -> DiscrepancySubrange {
        let (minus_level, Reverse(long_plus_level), minus_position) =
            cmp::max(
                (
                    self.minus_level,
                    Reverse(self.long_plus_level),
                    self.minus_position,
                ),
                (
                    other.minus_level,
                    Reverse(other.long_plus_level),
                    other.minus_position,
                ),
            );
        match self.sum.cmp(&other.sum) {
            cmp::Ordering::Less => other.clone(),
            cmp::Ordering::Greater => self.clone(),
            cmp::Ordering::Equal => {
                DiscrepancySubrange {
                    sum: self.sum,
                    short_plus_level: cmp::min(self.short_plus_level, other.short_plus_level),
                    minus_level,
                    long_plus_level,
                    minus_position,
                }
            }
        }
    }
}

impl Monoid for DiscrepancySubrange {
    fn empty() -> DiscrepancySubrange {
        DiscrepancySubrange {
            sum: 0,
            short_plus_level: MIN_LEVEL,
            minus_level: MIN_LEVEL,
            long_plus_level: MIN_LEVEL,
            minus_position: !0,
        }
    }

    fn append(&self, other: &DiscrepancySubrange) -> DiscrepancySubrange {
        let (minus_level, Reverse(long_plus_level), minus_position) =
            cmp::max(
                (
                    self.minus_level,
                    Reverse(cmp::max(self.long_plus_level, other.short_plus_level)),
                    self.minus_position,
                ),
                (
                    other.minus_level,
                    Reverse(cmp::max(self.short_plus_level, other.long_plus_level)),
                    other.minus_position,
                ),
            );
        DiscrepancySubrange {
            sum: self.sum + other.sum,
            short_plus_level: cmp::max(self.short_plus_level, other.short_plus_level),
            minus_level,
            long_plus_level,
            minus_position,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DiscrepancyRange {
    pub all: DiscrepancySubrange,
    pub max_prefix: DiscrepancySubrange,
    pub max_suffix: DiscrepancySubrange,
    pub max_infix: DiscrepancySubrange,
}

impl DiscrepancyRange {
    pub fn singleton(value: DiscrepancySubrange) -> DiscrepancyRange {
        let subrange = DiscrepancySubrange::empty().join(&value);
        DiscrepancyRange {
            all: value,
            max_prefix: subrange.clone(),
            max_suffix: subrange.clone(),
            max_infix: subrange,
        }
    }
}

impl Monoid for DiscrepancyRange {
    fn empty() -> DiscrepancyRange {
        DiscrepancyRange {
            all: DiscrepancySubrange::empty(),
            max_prefix: DiscrepancySubrange::empty(),
            max_suffix: DiscrepancySubrange::empty(),
            max_infix: DiscrepancySubrange::empty(),
        }
    }

    fn append(&self, other: &DiscrepancyRange) -> DiscrepancyRange {
        DiscrepancyRange {
            all: self.all.append(&other.all),
            max_prefix: self.max_prefix.join(&self.all.append(&other.max_prefix)),
            max_suffix: self.max_suffix.append(&other.all).join(&other.max_suffix),
            max_infix: self.max_suffix.append(&other.max_prefix).join(
                &self.max_infix.join(&other.max_infix),
            ),
        }
    }
}
