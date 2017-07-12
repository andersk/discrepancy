use super::factor::Factor;
use super::level::{Level, MIN_LEVEL};
use super::propagator::{PropagatorOps, Propagator};
use super::sign::Sign;

use std::cmp;

pub struct MultiplicativePropagator {
    factor: Factor,
}

impl MultiplicativePropagator {
    pub fn new() -> MultiplicativePropagator {
        MultiplicativePropagator { factor: Factor::new() }
    }
}

impl Propagator for MultiplicativePropagator {
    fn set<O: PropagatorOps, S: FnMut(u32, Sign, Level)>(
        &mut self,
        o: O,
        position: u32,
        sign: Sign,
        level: Level,
        mut set: S,
    ) {
        for k in self.factor[position + 1].clone() {
            if let Some((sign1, level1)) = o.get_sign(k - 1) {
                set(
                    (position + 1) / k - 1,
                    sign * sign1,
                    cmp::max(level, level1),
                );
            } else if let Some((sign1, level1)) = o.get_sign((position + 1) / k - 1) {
                set(k - 1, sign * sign1, cmp::max(level, level1));
            }
        }

        for k in 1..(o.signs_len() - 1) / (position + 1) {
            if let Some((sign1, level1)) = o.get_sign(k) {
                set(
                    (position + 1) * (k + 1) - 1,
                    sign * sign1,
                    cmp::max(level, level1),
                );
            }
        }
    }

    fn unset<O>(&mut self, _o: O, _position: u32) {}

    fn grow<O: PropagatorOps, S: FnMut(u32, Sign, Level)>(&mut self, o: O, mut set: S) {
        let position = o.signs_len() - 1;
        self.factor.compute(position + 1);

        if position == 0 {
            set(0, Sign::Plus, MIN_LEVEL);
        } else {
            for k in self.factor[position + 1].clone() {
                if let Some((sign1, level1)) = o.get_sign(k - 1) {
                    if let Some((sign2, level2)) = o.get_sign((position + 1) / k - 1) {
                        set(position, sign1 * sign2, cmp::max(level1, level2));
                    }
                }
            }
        }
    }
}
