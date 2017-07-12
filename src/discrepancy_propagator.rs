use super::discrepancy_range::{DiscrepancySubrange, DiscrepancyRange};
use super::level::{Level, MIN_LEVEL};
use super::monoid::Monoid;
use super::propagator::{PropagatorOps, Propagator};
use super::range_vec::RangeVec;
use super::sign::Sign;

pub struct DiscrepancyPropagator {
    discrepancy: u32,
    low: RangeVec<DiscrepancyRange>,
}

impl DiscrepancyPropagator {
    fn fudge(&self) -> usize {
        (1 + self.discrepancy % 2) as usize
    }

    fn dummy_prefix(&self) -> DiscrepancySubrange {
        DiscrepancySubrange {
            sum: self.discrepancy as i32 - 1,
            short_plus_level: MIN_LEVEL,
            minus_level: MIN_LEVEL,
            long_plus_level: MIN_LEVEL,
            minus_position: !0,
        }
    }

    pub fn new(discrepancy: u32) -> DiscrepancyPropagator {
        let mut p = DiscrepancyPropagator {
            discrepancy,
            low: RangeVec::new(),
        };

        let dummy = if discrepancy % 2 == 1 {
            DiscrepancyRange::singleton(p.dummy_prefix())
        } else {
            DiscrepancyRange::singleton(p.dummy_prefix().append(&DiscrepancySubrange::singleton(
                0,
                &Some((Sign::Minus, MIN_LEVEL)),
            )))
        };
        p.low.push(dummy);

        p
    }

    fn set_sign<O: PropagatorOps>(
        &mut self,
        o: O,
        position: u32,
        value: &Option<(Sign, Level)>,
    ) {
        let index = position as usize + self.fudge();

        let s = if index / 2 == 0 {
            DiscrepancyRange::singleton(self.dummy_prefix().append(&DiscrepancySubrange::singleton(
                position,
                value,
            )))
        } else if index % 2 == 1 {
            DiscrepancyRange::singleton(
                DiscrepancySubrange::singleton(position - 1, &o.get_sign(position - 1))
                    .append(&DiscrepancySubrange::singleton(position, value)),
            )
        } else if position == (o.signs_len() - 1) {
            DiscrepancyRange::singleton(DiscrepancySubrange::singleton(position, value).append(
                &DiscrepancySubrange::singleton(position + 1, &Some((Sign::Minus, MIN_LEVEL))),
            ))
        } else {
            DiscrepancyRange::singleton(DiscrepancySubrange::singleton(position, value).append(
                &DiscrepancySubrange::singleton(position + 1, &o.get_sign(position + 1)),
            ))
        };
        self.low.set(index / 2, s);
    }

    fn propagate<O: PropagatorOps, S: FnMut(u32, Sign, Level)>(&mut self, o: O, mut set: S) {
        let infix = self.low.concat_all().max_infix;
        if infix.sum > 2 * (self.discrepancy as i32 - 1) {
            set(!0, Sign::Minus, infix.short_plus_level);
        } else if infix.sum >= 2 * (self.discrepancy as i32 - 1) &&
                   infix.minus_level > infix.long_plus_level &&
                   infix.minus_position != !0
        {
            debug_assert!(match o.get_sign(infix.minus_position) {
                Some((Sign::Minus, level1)) => infix.long_plus_level < level1,
                _ => true,
            });
            set(infix.minus_position, Sign::Minus, infix.long_plus_level)
        }
    }
}

impl Propagator for DiscrepancyPropagator {
    fn set<O: PropagatorOps, S: FnMut(u32, Sign, Level)>(
        &mut self,
        o: O,
        position: u32,
        sign: Sign,
        level: Level,
        set: S,
    ) {
        self.set_sign(o, position, &Some((sign, level)));
        self.propagate(o, set)
    }

    fn unset<O: PropagatorOps>(&mut self, o: O, position: u32) {
        self.set_sign(o, position, &None);
    }

    fn grow<O: PropagatorOps, S: FnMut(u32, Sign, Level)>(&mut self, o: O, set: S) {
        let position = o.signs_len() - 1;
        if (position as usize + self.fudge()) % 2 == 0 {
            self.low.push(DiscrepancyRange::singleton(
                DiscrepancySubrange::singleton(position, &None).append(
                    &DiscrepancySubrange::singleton(position + 1, &Some((Sign::Minus, MIN_LEVEL))),
                ),
            ));
        } else {
            self.set_sign(o, position, &None);
        }
        self.propagate(o, set)
    }
}
