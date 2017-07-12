use std::u32;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Level(pub u32);

pub const MIN_LEVEL: Level = Level(u32::MIN);
pub const MAX_LEVEL: Level = Level(u32::MAX);
