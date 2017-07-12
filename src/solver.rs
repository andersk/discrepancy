use super::level::{Level, MIN_LEVEL};
use super::propagator::{PropagatorOps, Propagator};
use super::sign::Sign;

use std::cmp;

pub struct Assumption {
    position: u32,
    sign: Sign,
    log: Vec<u32>,
}

pub struct Solver {
    signs: Vec<Option<(Sign, Level)>>,
    done: bool,
    assumptions: Vec<Assumption>,
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            signs: vec![],
            done: false,
            assumptions: vec![],
        }
    }

    pub fn assume<P: Propagator>(&mut self, propagator: &mut P, position: u32, sign: Sign) {
        debug_assert!(self.signs[position as usize].is_none());
        self.assumptions.push(Assumption {
            position,
            sign,
            log: Vec::new(),
        });
        let level = self.current_level();
        self.set_signs(propagator, vec![(position, sign, level)]);
    }

    fn get_assumption(&self, level: Level) -> (u32, Sign) {
        let assumption = &self.assumptions[level.0 as usize - 1];
        (assumption.position, assumption.sign)
    }

    fn undo_level<P: Propagator>(&mut self, propagator: &mut P, level: Level) {
        debug_assert!(self.level_ok(level));
        while self.assumptions.len() >= level.0 as usize {
            if let Some(assumption) = self.assumptions.pop() {
                for position in assumption.log.into_iter().rev() {
                    if let Some((_, level1)) = self.signs[position as usize] {
                        if level1 >= level {
                            propagator.unset(&*self, position);
                            self.signs[position as usize] = None;
                        }
                    }
                }
            } else {
                self.done = true;
                break;
            }
        }
    }

    pub fn current_level(&mut self) -> Level {
        Level(self.assumptions.len() as u32)
    }

    pub fn level_ok(&mut self, level: Level) -> bool {
        !self.done && level.0 <= self.assumptions.len() as u32
    }

    pub fn set_signs<P: Propagator>(
        &mut self,
        propagator: &mut P,
        mut queue: Vec<(u32, Sign, Level)>,
    ) {
        while let Some((position, sign, level)) = queue.pop() {
            if !self.level_ok(level) {
                continue;
            }

            if position == !0 {
                if level != MIN_LEVEL {
                    let (position2, sign2) = self.get_assumption(level);
                    queue.push((position2, -sign2, Level(level.0 - 1)));
                }
                self.undo_level(propagator, level);
                continue;
            } else if let Some((sign1, level1)) = self.signs[position as usize] {
                if sign != sign1 {
                    let level2 = cmp::max(level, level1);
                    if level2 != MIN_LEVEL {
                        let (position2, sign2) = self.get_assumption(level2);
                        queue.push((position2, -sign2, Level(level2.0 - 1)));
                    }
                    self.undo_level(propagator, level2);
                    if !self.level_ok(level) {
                        continue;
                    }
                } else if level >= level1 {
                    continue;
                }
            };

            propagator.set(&*self, position, sign, level, |position1, sign1, level1| {
                queue.push((position1, sign1, level1))
            });
            self.signs[position as usize] = Some((sign, level));
            if level != MIN_LEVEL {
                self.assumptions[level.0 as usize - 1].log.push(position);
            }
        }
    }

    pub fn grow<P: Propagator>(&mut self, propagator: &mut P) {
        self.signs.push(None);
        let mut queue = Vec::new();
        propagator.grow(&*self, |position1, sign1, level1| {
            queue.push((position1, sign1, level1))
        });
        self.set_signs(propagator, queue);
    }
}

impl<'a> PropagatorOps for &'a Solver {
    fn signs_len(self) -> u32 {
        self.signs.len() as u32
    }

    fn get_sign(self, position: u32) -> Option<(Sign, Level)> {
        self.signs[position as usize]
    }
}
