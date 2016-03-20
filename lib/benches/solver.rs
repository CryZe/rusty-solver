#![feature(test)]

extern crate lib_rusty_solver as solver;
extern crate test;

use test::Bencher;
use solver::prelude::*;
use solver::differential_equation::PhaseFieldEquation;
use solver::precondition::ConstantPreCondition;
use solver::boundary_condition::Dirichlet;

#[bench]
fn bench_solving(b: &mut Bencher) {
    let ref precondition = ConstantPreCondition(0.0);
    let equation = PhaseFieldEquation {
        t: 0.9,
        gamma: 1.0,
        tau: 2.0,
        epsilon: 5.0,
        tm: 1.0,
        la: 0.5,
    };

    let boundary_conditions = BoundaryConditions::new(Dirichlet(0.0),
                                                      Dirichlet(0.0),
                                                      Dirichlet(0.0),
                                                      Dirichlet(0.0));

    let mut solver = Solver::new(equation, boundary_conditions, (5000, 5000), 0.1, 1.0);

    solver.execute_precondition(precondition);

    b.iter(|| solver.solve_next_frame());
}
