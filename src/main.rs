extern crate discrepancy;

use discrepancy::discrepancy_propagator::DiscrepancyPropagator;
use discrepancy::first_unfilled_propagator::FirstUnfilledPropagator;
use discrepancy::level::MIN_LEVEL;
use discrepancy::multiplicative_propagator::MultiplicativePropagator;
use discrepancy::neg_propagator::NegPropagator;
use discrepancy::output_propagator::OutputPropagator;
use discrepancy::propagator::PropagatorOps;
use discrepancy::sign::Sign;
use discrepancy::solver::Solver;

use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    assert_eq!(args.len(), 2, "wrong number of arguments");
    let discrepancy = args[1].parse().expect("discrepancy argument");

    let mut solver = Solver::new();
    let mut propagator = (
        OutputPropagator::new(),
        FirstUnfilledPropagator::new(),
        DiscrepancyPropagator::new(discrepancy),
        NegPropagator::new(DiscrepancyPropagator::new(discrepancy)),
        MultiplicativePropagator::new(),
    );

    while solver.level_ok(MIN_LEVEL) {
        let position = propagator.1.first_unfilled();
        if position == !0 {
            solver.grow(&mut propagator);
        } else {
            let sign = if position % 3 == 0 {
                Sign::Plus
            } else if position % 3 == 1 {
                Sign::Minus
            } else {
                -solver.get_sign(position / 3).unwrap().0
            };
            solver.assume(&mut propagator, position, sign);
        }
    }
    println!("done");
}
