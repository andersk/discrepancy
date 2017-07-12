use std::cmp;
use super::monoid::Monoid;

#[derive(Clone, Debug)]
pub struct QueueRange {
    pub next: u32,
}

impl QueueRange {
    pub fn present(position: u32) -> QueueRange {
        QueueRange { next: position }
    }
}

impl Monoid for QueueRange {
    fn empty() -> QueueRange {
        QueueRange { next: !0 }
    }

    fn append(&self, other: &QueueRange) -> QueueRange {
        QueueRange { next: cmp::min(self.next, other.next) }
    }
}
