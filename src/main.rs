extern crate rand;
#[macro_use]
extern crate approx;
extern crate nalgebra as na;

use na::Vector2;
use rand::prelude::*;

fn main() {
    println!("Hello, quantum world!");
}

#[derive(Copy, Clone)]
enum Basis {
    Standard { vector: Vector2<i8> },
    Superposition { vector: Vector2<i8> },
    Circular {},
}

fn ket0() -> Basis {
    Basis::Standard {
        vector: Vector2::new(1, 0),
    }
}

fn ket1() -> Basis {
    Basis::Standard {
        vector: Vector2::new(0, 1),
    }
}

#[derive(Copy, Clone)]
struct Qubit {
    state: Basis,
}

impl Qubit {
    fn is_active(&self) -> bool {
        match &self.state {
            Basis::Standard { vector } => {
                if vector[0] == 0 as i8 {
                    true
                } else {
                    false
                }
            }
            Basis::Superposition { vector } => {
                let return_active: bool = random();
                return_active
            }
            _ => false,
        }
    }

    fn measure(&self) -> Basis {
        match &self.state {
            Basis::Superposition { vector } => {
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
            Basis::Superposition { vector } => true,
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
        let matrix = na::Matrix2::new(0, 1, 1, 0);
        match self.qubit.state {
            Basis::Standard { vector } => Qubit {
                state: Basis::Standard {
                    vector: matrix * vector,
                },
            },
            _ => self.qubit,
        }
    }
}

struct H {}

impl H {
    fn apply(&self, qubit: Qubit) -> Qubit {
        match qubit.state {
            Basis::Standard { vector } => {
                let matrix = na::Matrix2::new(1, 1, 1, -1);
                Qubit {
                    state: Basis::Superposition {
                        vector: matrix * vector,
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
        assert!(H {}.apply(Qubit { state: ket1() }).is_in_superposition())
    }

    #[test]
    fn qubit_in_superposition_state_should_return_different_results() {
        let mut q = H {}.apply(Qubit { state: ket1() });
        let mut ground_state_detected: bool = false;
        let mut active_state_detected: bool = false;
        for i in 1..20 {
            match q.measure() {
                Basis::Standard { vector } => {
                    println!("Standard {},{}", vector[0], vector[1]);
                    if vector[1] == 1 {
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
