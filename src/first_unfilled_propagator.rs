use super::level::Level;
use super::monoid::Monoid;
use super::propagator::{PropagatorOps, Propagator};
use super::queue_range::QueueRange;
use super::range_vec::RangeVec;
use super::sign::Sign;

pub struct FirstUnfilledPropagator {
    unfilled: RangeVec<QueueRange>,
}

impl FirstUnfilledPropagator {
    pub fn new() -> FirstUnfilledPropagator {
        FirstUnfilledPropagator { unfilled: RangeVec::new() }
    }

    pub fn first_unfilled(&self) -> u32 {
        self.unfilled.concat_all().next
    }
}

impl Propagator for FirstUnfilledPropagator {
    fn set<O, S>(&mut self, _o: O, position: u32, _sign: Sign, _level: Level, _set: S) {
        self.unfilled.set(position as usize, QueueRange::empty());
    }

    fn unset<O>(&mut self, _o: O, position: u32) {
        self.unfilled.set(
            position as usize,
            QueueRange::present(position),
        );
    }

    fn grow<O: PropagatorOps, S>(&mut self, o: O, _set: S) {
        self.unfilled.push(QueueRange::present(o.signs_len() - 1));
    }
}
