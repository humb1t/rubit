extern crate rand;
#[macro_use]
extern crate approx;
extern crate nalgebra as na;

use rand::prelude::*;
use na::{Vector2};

fn main() {
    println!("Hello, quantum world!");
}

#[derive(Copy, Clone)]
enum Basis {
    Standard {vector: Vector2<i8> },
    Superposition { top: i8, bottom: i8 },
    Circular {},
}

fn ket0() -> Basis {
    Basis::Standard { top: 1, bottom: 0 }
}

fn ket1() -> Basis {
    Basis::Standard { top: 0, bottom: 1 }
}

#[derive(Copy, Clone)]
struct Qubit {
    state: Basis,
}

impl Qubit {
    fn is_active(&self) -> bool {
        match &self.state {
            Basis::Standard { top, bottom } => {
                if *top == 0 as i8 {
                    true
                } else {
                    false
                }
            }
            Basis::Superposition { top, bottom } => {
                let return_active: bool = random();
                return_active
            }
            _ => false,
        }
    }

    fn measure(&self) -> Basis {
        match &self.state {
            Basis::Superposition { top, bottom } => {
                let return_active: bool = random();
                if return_active {
                    ket1()
                } else {
                    ket0()
                }
            }
            _ => *&self.state,
        }
    }

    fn is_in_superposition(&self) -> bool {
        match &self.state {
            Basis::Superposition { top, bottom } => true,
            _ => false,
        }
    }

    fn activate(&mut self) {
        self.state = ket1()
    }

    fn deactivate(&mut self) {
        self.state = ket0()
    }
}

struct CNOT {
    control: Qubit,
    target: Qubit,
}

impl CNOT {
    fn apply(&mut self) -> Qubit {
        if self.control.is_active() {
            self.target.activate()
        }
        self.target
    }
}

struct X {
    qubit: Qubit,
}

impl X {
    fn apply(&mut self) -> Qubit {
        let a11 = 0 as i8;
        let a12 = 1 as i8;
        let a21 = 1 as i8;
        let a22 = 0 as i8;
        match self.qubit.state {
            Basis::Standard { top, bottom } => {
                let new_top = a11 * top + a12 * bottom;
                let new_bottom = a21 * top + a22 * bottom;
                Qubit {
                    state: Basis::Standard {
                        top: new_top,
                        bottom: new_bottom,
                    },
                }
            }
            _ => self.qubit,
        }
    }
}

struct H {
    row_1: [i8; 2],
    row_2: [i8; 2],
}

impl H {
    fn new() -> H {
        H {
            row_1: [1, 1],
            row_2: [1, -1],
        }
    }

    fn apply(&self, qubit: Qubit) -> Qubit {
        match qubit.state {
            Basis::Standard { top, bottom } => {
                let new_top = *&self.row_1[0] * top + *&self.row_1[1] * bottom;
                let new_bottom = *&self.row_2[0] * top + *&self.row_2[1] * bottom;
                Qubit {
                    state: Basis::Superposition {
                        top: new_top,
                        bottom: new_bottom,
                    },
                }
            }
            _ => qubit,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x_gate_should_activate_qubit_in_ground_state() {
        let mut x = X {
            qubit: Qubit { state: ket0() },
        };
        assert!(x.apply().is_active())
    }

    #[test]
    fn x_gate_should_deactivate_qubit_in_exited_state() {
        let mut x = X {
            qubit: Qubit { state: ket1() },
        };
        assert!(!x.apply().is_active())
    }

    #[test]
    fn h_gate_should_turn_qubit_state_to_superposition() {
        assert!(H::new()
            .apply(Qubit { state: ket1() })
            .is_in_superposition())
    }

    #[test]
    fn qubit_in_superposition_state_should_return_different_results() {
        let mut q = H::new().apply(Qubit { state: ket1() });
        let mut ground_state_detected: bool = false;
        let mut active_state_detected: bool = false;
        for i in 1..20 {
            match q.measure() {
                Basis::Standard { top, bottom } => {
                    println!("Standard {},{}", top, bottom);
                    if bottom == 1 {
                        active_state_detected = true
                    } else {
                        ground_state_detected = true
                    }
                }
                _ => (),
            }
        }
        assert!(ground_state_detected);
        assert!(active_state_detected);
    }
}
