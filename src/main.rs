extern crate rand;

use rand::prelude::*;

fn main() {
    println!("Hello, quantum world!");
}

/// Not simply {|0⟩,|1⟩,|+⟩,|−⟩}, but  |ψ⟩=α|0⟩+β|1⟩, 
/// where α and β are complex numbers with the probability constraint (amplitudes) |α|^2+|β|^2=1.
/// Furthermore, the global phase of a quantum state is not detectable; therefore, |ψ⟩ is the same as (e^(iγ))|ψ⟩
/// An alternative representation of a single-qubit state that incorporates both of these constraints can be written as
/// |ψ⟩=√p|0⟩+(e^(iϕ))√(1−p)|1⟩,
/// where 0≤p≤1 is the probability of the bit being in the 0 state, and 0≤ϕ<2π is the quantum phase.
struct QuantumPureState {
    probability: usize,
    phase: usize,
}

///A pure state is one fully specified by a single ket
///a coherent superposition as described above.
///Coherence is essential for a qubit to be in a superposition state.
///With interactions and decoherence, it is possible to put the qubit in a mixed state,
///a statistical combination or incoherent mixture of different pure states.
///Mixed states can be represented by points inside the Bloch sphere (or in the Bloch ball).
///A mixed qubit state has three degrees of freedom: the angles p and ϕ,
///as well as the length r of the vector that represents the mixed state. 
struct QuantumMixedState {
    probability: usize,
    phase: usize,
    r: usize,
}

enum QubitStatesOperations {
    ///Quantum logic gates, building blocks for a quantum circuit in a quantum computer, operate on
    ///one, two, or three qubits: mathematically, the qubits undergo a (reversible) unitary
    ///transformation under the quantum gate. For a single qubit, unitary transformations
    ///correspond to rotations of the qubit (unit) vector on the Bloch sphere to specific
    ///superpositions. For two qubits, the Controlled NOT gate can be used to entangle or
    ///disentangle them.
    Gate,
    ///Standard basis measurement is an irreversible operation in which information is gained about
    ///the state of a single qubit (and coherence is lost). The result of the measurement will be
    ///either: ket 0 with probability alfa^2 or ket 1 with probability betta^2.
    ///Measurement of the state of the qubit alters the magnitudes of α and β. 
    ///For instance, if the result of the measurement is ket 1, α is changed to 0 and β is changed to the phase factor
    ///e^iϕ no longer experimentally accessible.
    ///When a qubit is measured, the superposition state collapses to a basis state (up to a phase)
    ///and the relative phase is rendered inaccessible (i.e., coherence is lost). Note that a
    ///measurement of a qubit state that is entangled with another quantum system transforms the
    ///qubit state, a pure state, into a mixed state (an incoherent mixture of pure states) as the
    ///relative phase of the qubit state is rendered inaccessible.
    Measure,
}

///The computational (or standard) basis corresponds to the two levels |0⟩ and |1⟩
///They are written in the conventional Dirac—or "bra–ket"—notation;|0⟩ and |1⟩
///are pronounced "ket 0" and "ket 1", respectively. These two orthonormal basis states,
///{|0⟩ and |1⟩} together called the computational basis, are said to span the two-dimensional linear vector (Hilbert) space of the qubit. 
#[derive(Copy, Clone)]
enum StandardBasisVector {
    ///|0> or ket 0
    ///
    ///(1)
    ///(0)
    Ground,
    ///|1> or ket 1
    ///
    ///(0)
    ///(1)
    Exited,
}

///ubit basis states can also be combined to form product basis states. For example, two qubits
///could be represented in a four-dimensional linear vector space spanned by the following product
///basis states:
///        [1]       [0]
/// |00> = [0], |01>=[1]
///        [0]       [0]
///        [0]       [0]
struct ProductBasisState{
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


/// Third type of basis - circular (or Y) basis:
/// |↻⟩=1√2(|0⟩+i|1⟩), |↺⟩=1√2(|0⟩−i|1⟩).
/// to make |↻⟩ from the |0> state - use H followed by S. 
enum CircularBasis{}

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
    //TODO: To measure in this basis, we must rotate the computation basis (Z) to the circular basis (Y). To do this, use an S† followed by H before your measurement.
    //implement z measure
    fn measure(&self) -> StandardBasisVector {
        let return_active: bool = random();
        if return_active {
            return StandardBasisVector::Exited;
        }
            return StandardBasisVector::Ground;
    }

    fn is_in_superposition(&self) -> bool {
        true
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
struct H{
    row_1: [i32, 2],
    row_2: [i32, 2],
}

//TODO: impl Gate with Generic Qubit Type as return type if any
impl H {
    fn new() -> H {
        H {
            row_1: [1,1],
            row_2: [1, -1]
        }
    }

    fn apply(qubit: Qubit) -> SuperpositionedQubit {
        SuperpositionedQubit
    }
}

fn h() {
    let ket_1 = [1,0];
    let ket_0 = [0,1];
    let hadamard_matrix = H::new();
}

///The T gate applies a phase of π/4, and has a matrix representation of
///( 1 0        )
///( 0 e^(iπ/4) )
struct T {
    qubit: QuantumPureState,
}

impl T {
    fn apply(&mut self) {
    }
}


/// One of Clifford group - S
/// ( 1 0 )
/// ( 0 i )
/// It also could be described as T^2 gate
struct S;

/// One from Pauli group - Z
/// ( 1 0 )
/// ( 0 -1)
/// It also could be described as T^4 gate
struct Z;

//TODO: S with sword and T with sword

/// Y Gate
/// ( 0 -i )
/// ( i  0 ) 
/// It also could be described as XZ
struct Y;

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
        assert!(
            H::apply(
                Qubit {
                 state: StandardBasisVector::Exited,
                 t1: 0,
                 t2: 0,
                }
            ).is_in_superposition())
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

    #[test]
    fn qubit_in_ground_state_after_one_t_gate_should_have_probability_of_ground_state_08_of_active_state_14(){
        let mut t = T {
            qubit: Qubit {
                state: StandardBasisVector::Exited,
                t1: 0,
                t2: 0,
            },
        };
        //T.apply()
        //result.measure() 0->0.85, 1->0.14, Xlength->0.7
        //T.apply() pi/2
        //result.measure() 0->0.5, 1->0.5, Xlength->0
        //3pi/4
        //result.measure() 0->0.14, 1->0.85, Xlength->-0.7
        //pi
        //result.measure() 0->0, 1->1, Xlength->-1
        //5pi/4
        //result.measure() 0->0.85, 1->0.14, Xlength->0.7
        //3pi/2
        //result.measure() 0->0.5, 1->0.5, Xlength->0
        //7pi/4
        //result.measure() 0->0.14, 1->0.85, Xlength->-0.7
    }

    //TODO: circular basis measurements: https://quantumexperience.ng.bluemix.net/qx/tutorial?sectionId=full-user-guide&page=002-The_Weird_and_Wonderful_World_of_the_Qubit~2F003-Introducing_qubit_phase
}
