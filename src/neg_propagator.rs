use super::level::Level;
use super::propagator::{PropagatorOps, Propagator};
use super::sign::Sign;

#[derive(Clone, Copy)]
struct NegPropagatorOps<O: PropagatorOps> {
    o: O,
}

impl<O: PropagatorOps> PropagatorOps for NegPropagatorOps<O> {
    fn signs_len(self) -> u32 {
        self.o.signs_len()
    }

    fn get_sign(self, position: u32) -> Option<(Sign, Level)> {
        self.o.get_sign(position).map(
            |(sign, level)| (-sign, level),
        )
    }
}

pub struct NegPropagator<P: Propagator> {
    propagator: P,
}

impl<P: Propagator> NegPropagator<P> {
    pub fn new(propagator: P) -> NegPropagator<P> {
        NegPropagator { propagator }
    }
}

impl<P: Propagator> Propagator for NegPropagator<P> {
    fn set<O: PropagatorOps, S: FnMut(u32, Sign, Level)>(
        &mut self,
        o: O,
        position: u32,
        sign: Sign,
        level: Level,
        mut set: S,
    ) {
        self.propagator.set(
            NegPropagatorOps { o },
            position,
            -sign,
            level,
            |position, sign, level| {
                set(position, -sign, level)
            },
        );
    }

    fn unset<O: PropagatorOps>(&mut self, o: O, position: u32) {
        self.propagator.unset(NegPropagatorOps { o }, position);
    }

    fn grow<O: PropagatorOps, S: FnMut(u32, Sign, Level)>(&mut self, o: O, mut set: S) {
        self.propagator.grow(NegPropagatorOps { o }, |position,
         sign,
         level| {
            set(position, -sign, level)
        });
    }
}
