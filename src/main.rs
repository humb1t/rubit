fn main() {
    println!("Hello, quantum world!");
}

enum StandardBasisVector {
    Ground,
    Exited,
}

struct Qubit {
    state: StandardBasisVector,
    /// Energy relaxation time when qubits loss his excited state and decays towards ground state.
    t1: u32,
    /// Dephasing time - time of dephasing and energy relaxation.
    t2: u32,
}

impl Qubit {
    fn is_active(&self) -> bool {
        match self.state {
            StandardBasisVector::Exited => true,
            _ => false,
        }
    }

    fn activate(&mut self) {
        self.state = StandardBasisVector::Exited
    }

    fn deactivate(&mut self) {
        self.state = StandardBasisVector::Ground
    }
}

trait Gate {
    fn apply(&mut self);
}

struct CNOT {
    control: Qubit,
    target: Qubit,
}

impl Gate for CNOT {
    fn apply(&mut self) {
        if self.control.is_active() {
            self.target.activate()
        }
    }
}

/// Universal matrix
/// ( 0 1 )
/// ( 1 0 )
struct X {
    qubit: Qubit,
}

impl Gate for X {
    fn apply(&mut self) {
        if self.qubit.is_active() {
            self.qubit.deactivate()
        } else {
            self.qubit.activate()
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
                state: StandardBasisVector::Ground,
                t1: 0,
                t2: 0,
            }
        };
        x.apply()
    }
}
