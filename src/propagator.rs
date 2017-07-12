use super::level::Level;
use super::sign::Sign;

pub trait PropagatorOps: Copy {
    fn signs_len(self) -> u32;
    fn get_sign(self, position: u32) -> Option<(Sign, Level)>;
}

pub trait Propagator {
    fn set<O: PropagatorOps, S: FnMut(u32, Sign, Level)>(
        &mut self,
        o: O,
        position: u32,
        sign: Sign,
        level: Level,
        set: S,
    );
    fn unset<O: PropagatorOps>(&mut self, o: O, position: u32);
    fn grow<O: PropagatorOps, S: FnMut(u32, Sign, Level)>(&mut self, o: O, set: S);
}

impl Propagator for () {
    fn set<O, S>(&mut self, _o: O, _position: u32, _sign: Sign, _level: Level, _set: S) {}

    fn unset<O>(&mut self, _o: O, _position: u32) {}

    fn grow<O, S>(&mut self, _o: O, _set: S) {}
}

macro_rules! propagator_tuple_impl {
    ($($i:tt $Pi:ident)+) => {
        impl<$($Pi: Propagator),*> Propagator for ($($Pi,)*) {
            fn set<O: PropagatorOps, S: FnMut(u32, Sign, Level)>(
                &mut self,
                o: O,
                position: u32,
                sign: Sign,
                level: Level,
                mut set: S
            ) {
                $(
                    self.$i.set(o, position, sign, level, |position, sign, level| {
                        set(position, sign, level)
                    });
                )*
            }

            fn unset<O: PropagatorOps>(&mut self, o: O, position: u32) {
                $(
                    self.$i.unset(o, position);
                )*
            }

            fn grow<O: PropagatorOps, S: FnMut(u32, Sign, Level)>(&mut self, o: O, mut set: S) {
                $(
                    self.$i.grow(
                        o,
                        |position, sign, level| set(position, sign, level),
                    );
                )*
            }
        }
    };
}

macro_rules! propagator_tuple_impl1 {
    ([$($i:tt $Pi:ident)*]) => {
    };
    ([$($i:tt $Pi:ident)*] $j:tt $Pj:ident $($k:tt $Pk:ident)*) => {
        propagator_tuple_impl!($($i $Pi)* $j $Pj);
        propagator_tuple_impl1!([$($i $Pi)* $j $Pj] $($k $Pk)*);
    };
}

propagator_tuple_impl1!([] 0 P0 1 P1 2 P2 3 P3 4 P4 5 P5 6 P6 7 P7 8 P8 9 P9 10 P10 11 P11);
