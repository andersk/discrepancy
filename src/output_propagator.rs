use super::level::Level;
use super::propagator::{PropagatorOps, Propagator};
use super::sign::Sign;

pub struct OutputPropagator {
    printing: bool,
    printed: u32,
}

impl OutputPropagator {
    pub fn new() -> OutputPropagator {
        OutputPropagator {
            printing: false,
            printed: 0,
        }
    }

    fn invalidate<O>(&mut self, _o: O, position: u32) {
        if position < self.printed {
            if self.printing {
                println!("\nlength {}", self.printed);
                self.printing = false;
            }
            self.printed = position;
        }
    }
}

impl Propagator for OutputPropagator {
    fn set<O: PropagatorOps, S>(
        &mut self,
        o: O,
        position: u32,
        sign: Sign,
        _level: Level,
        _set: S,
    ) {
        match o.get_sign(position) {
            Some((sign1, _)) if sign1 != sign => self.invalidate(o, position),
            _ => (),
        };
    }

    fn unset<O: PropagatorOps>(&mut self, o: O, position: u32) {
        self.invalidate(o, position);
    }

    fn grow<O: PropagatorOps, S>(&mut self, o: O, _set: S) {
        let len = o.signs_len() - 1;
        if len > self.printed {
            if !self.printing {
                self.printing = true;
                if len - 1 > self.printed {
                    print!("delete {}\n", len - 1 - self.printed);
                }
                print!("add ");
            }
            print!(
                "{}",
                (self.printed..len)
                    .map(|position| o.get_sign(position).unwrap().0.to_char())
                    .collect::<String>()
            );
            self.printed = len;
        }
    }
}
