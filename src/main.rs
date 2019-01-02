fn main() {
    println!("Hello, quantum world!");
}

enum StandardBasisVector{
    Ground, Exited, 
}

struct Qubit{
    state: StandardBasisVector,
}

impl Qubit {
    fn is_active(&self)->bool{
        match self.state {
            StandardBasisVector::Exited => true,
            _ => false,
        }
    }

    fn activate(&mut self){
        self.state = StandardBasisVector::Exited
    }
}

struct CNOT {
    control: Qubit,
    target: Qubit,
}

trait Gate {
    fn apply(&mut self); 
}

impl Gate for CNOT {
    fn apply(&mut self) {
       if self.control.is_active() { 
           self.target.activate()
       }
    }
}
