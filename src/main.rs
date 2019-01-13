extern crate rand;
#[macro_use]
extern crate approx;
extern crate nalgebra as na;

use na::Vector2;
use rand::prelude::*;

fn main() {
    println!("Hello, quantum world!");
}

fn ket0() -> Vector2<i8> {
    Vector2::new(1, 0)
}

fn ket1() -> Vector2<i8> {
    Vector2::new(0, 1)
}

#[derive(Copy, Clone)]
struct Qubit {
    state: Vector2<i8>,
    is_in_superposition: bool,
}

impl Qubit {
    fn is_active(&self) -> bool {
        if !self.is_in_superposition {
            if self.state[0] == 0 as i8 {
                true
            } else {
                false
            }
        } else {
            let return_active: bool = random();
            return_active
        }
    }

    fn measure(&self) -> Vector2<i8> {
        if self.is_in_superposition {
            let return_active: bool = random();
            if return_active {
                ket1()
            } else {
                ket0()
            }
        } else {
            *&self.state
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
        if !self.qubit.is_in_superposition {
            Qubit {
                state: matrix * self.qubit.state,
                is_in_superposition: false,
            }
        } else {
            self.qubit
        }
    }
}

struct H {}

impl H {
    fn apply(&self, qubit: Qubit) -> Qubit {
        if !qubit.is_in_superposition {
            let matrix = na::Matrix2::new(1, 1, 1, -1);
            Qubit {
                state: matrix * qubit.state,
                is_in_superposition: true,
            }
        } else {
            qubit
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x_gate_should_activate_qubit_in_ground_state() {
        let mut x = X {
            qubit: Qubit {
                state: ket0(),
                is_in_superposition: false,
            },
        };
        assert!(x.apply().is_active())
    }

    #[test]
    fn x_gate_should_deactivate_qubit_in_exited_state() {
        let mut x = X {
            qubit: Qubit {
                state: ket1(),
                is_in_superposition: false,
            },
        };
        assert!(!x.apply().is_active())
    }

    #[test]
    fn h_gate_should_turn_qubit_state_to_superposition() {
        assert!(
            H {}.apply(Qubit {
                state: ket1(),
                is_in_superposition: false,
            }).is_in_superposition
        )
    }

    #[test]
    fn qubit_in_superposition_state_should_return_different_results() {
        let mut q = H {}.apply(Qubit {
            state: ket1(),
            is_in_superposition: false,
        });
        let mut ground_state_detected: bool = false;
        let mut active_state_detected: bool = false;
        for i in 1..20 {
            let vector = q.measure();
            println!("Standard {},{}", vector[0], vector[1]);
            if vector[1] == 1 {
                active_state_detected = true
            } else {
                ground_state_detected = true
            }
        }
        assert!(ground_state_detected);
        assert!(active_state_detected);
    }
}
