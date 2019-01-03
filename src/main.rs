extern crate rand;

use rand::prelude::*;

fn main() {
    println!("Hello, quantum world!");
}

/// Not simply {|0⟩,|1⟩,|+⟩,|−⟩}, but  |ψ⟩=α|0⟩+β|1⟩, 
/// where α and β are complex numbers with the probability constraint |α|2+|β|2=1.
/// Furthermore, the global phase of a quantum state is not detectable; therefore, |ψ⟩ is the same as (e^(iγ))|ψ⟩
/// An alternative representation of a single-qubit state that incorporates both of these constraints can be written as
/// |ψ⟩=√p|0⟩+(e^(iϕ))√(1−p)|1⟩,
/// where 0≤p≤1 is the probability of the bit being in the 0 state, and 0≤ϕ<2π is the quantum phase.
struct QuantumState {
    probability: usize,
    phase: usize,
}

///The computational (or standard) basis corresponds to the two levels |0⟩ and |1⟩
#[derive(Copy, Clone)]
enum StandardBasisVector {
    ///|0>
    ///
    ///(1)
    ///(0)
    Ground,
    ///|1>
    ///
    ///(0)
    ///(1)
    Exited,
}

///The qubit does not always have to be in either |0⟩ or |1⟩;
///it can be in an arbitrary quantum state, denoted |ψ⟩,
///which can be any superposition |ψ⟩=α|0⟩+β|1⟩ of the basis vectors.
///The superposition quantities α and β are complex numbers;
///together they obey |α|2+|β|2=1.
struct Superposition;

///the superposition basis,
///defined by the set {|+⟩,|−⟩}:
///|+⟩=(|0⟩+|1⟩)/√2
///|−⟩=(|0⟩−|1⟩)/√2 
enum SuperpositionBasis {

}

///A qubit is a quantum system consisting of two levels,
///labeled |0⟩ and |1⟩ (here we are using Dirac’s bra-ket notation).
///It is represented by a two-dimensional vector space over the complex numbers C2.
///This means that a qubit takes two complex numbers to fully describe it.
#[derive(Copy, Clone)]
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

    //fn measure(%self) -> StandardBasisVector
    //https://en.wikipedia.org/wiki/Born_rule
}

struct SuperpositionedQubit; 

impl SuperpositionedQubit {
    /// The act of measurement in the computational basis forces the system to be in either the |0⟩ state
    /// or the |1⟩ state with an equal probability.
    fn measure(&self) -> StandardBasisVector {
        let return_active: bool = random();
        if return_active {
            return StandardBasisVector::Exited;
        }
            return StandardBasisVector::Ground;
    }
}

trait Gate {
    fn apply(&mut self) -> Qubit;
}

///A single-qubit quantum gate is a 2×2 unitary matrix 
///(since quantum gates must be reversible and preserve probability amplitudes, the matrices must be unitary).
///The quantum state |ψ′⟩ after the action of the gate is found by multiplying the original quantum state by the gate |ψ′⟩=U|ψ⟩.
///Here U represents the gate.
struct SingleQubitGate {
    matrix: UnitaryMatrix,
}

///In mathematics, a complex square matrix U is unitary if its conjugate transpose U∗ is also its inverse.
struct UnitaryMatrix;

struct CNOT {
    control: Qubit,
    target: Qubit,
}

impl Gate for CNOT {
    fn apply(&mut self) -> Qubit {
        if self.control.is_active() {
            self.target.activate()
        }
        self.target
    }
}

/// The simplest quantum gate is the bit-flip gate, which we denote by X.
/// It takes |0⟩→X|0⟩=|1⟩; in other words, it flips the zero to a one, or vice versa. 
/// This is similar to a classical NOT gate and has a matrix representation of
/// ( 0 1 )
/// ( 1 0 )
struct X {
    qubit: Qubit,
}

impl Gate for X {
    fn apply(&mut self) -> Qubit {
        if self.qubit.is_active() {
            self.qubit.deactivate()
        } else {
            self.qubit.activate()
        }
        self.qubit
    }
}

///via the H gate, we make a uniform superposition of the ground and excited state,
///the H gate transforms the computational basis into a new basis, the superposition basis,
///defined by the set {|+⟩,|−⟩}:
///|+⟩=(|0⟩+|1⟩)/√2
///|−⟩=(|0⟩−|1⟩)/√2 
///The matrix the represents the H gate is
///    1 ( 1 1  )
///H = _ (      )
///   √2 ( 1 -1 )
struct H;

//TODO: impl Gate with Generic Qubit Type as return type if any
impl H {
    fn apply(&mut self) -> SuperpositionedQubit {
        SuperpositionedQubit
    }
}

///The T gate applies a phase of π/4, and has a matrix representation of
///( 1 0        )
///( 0 e^(iπ/4) )

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
            },
        };
        assert!(x.apply().is_active())
    }

    #[test]
    fn x_gate_should_deactivate_qubit_in_exited_state() {
        let mut x = X {
            qubit: Qubit {
                state: StandardBasisVector::Exited,
                t1: 0,
                t2: 0,
            },
        };
        assert!(!x.apply().is_active())
    }

    #[test]
    fn h_gate_should_turn_qubit_state_to_superposition() {
        let mut h = H; 
        //assert!(h.apply().is_in_superposition())
    }

    #[test]
    fn qubit_in_superposition_state_should_return_different_results() {
        let mut q = SuperpositionedQubit; 
        let mut ground_state_detected: bool = false;
        let mut active_state_detected: bool = false;
        for i in 1..20 {
            match q.measure() {
                StandardBasisVector::Exited => active_state_detected = true,
                StandardBasisVector::Ground => ground_state_detected = true
            }
        }
        assert!(ground_state_detected);
        assert!(active_state_detected);
    }
}
