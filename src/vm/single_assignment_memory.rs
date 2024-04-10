use std::collections::HashMap;

use super::{Environment, Identifier, Variable, VmInstruction};

#[derive(Debug)]
pub struct SingleAssignmentMemory(HashMap<Variable, Value>);

impl SingleAssignmentMemory {
    pub fn new() -> SingleAssignmentMemory {
        SingleAssignmentMemory(HashMap::new())
    }

    pub fn read(&self, variable: &Variable) -> Option<&Value> {
        self.0.get(variable)
    }

    pub fn allocate(&self, variable: Variable) -> SingleAssignmentMemory {
        // Only allow allocating unallocated variables
        if self.0.contains_key(&variable) {
            panic!(
                "Attempted to allocate already allocated variable {:?}",
                variable
            );
        }
        let mut new_memory = self.0.clone();
        new_memory.insert(variable, Value::Unbound);
        SingleAssignmentMemory(new_memory)
    }

    pub fn bind(&self, variable: &Variable, value: Value) -> SingleAssignmentMemory {
        // Only allow writing to unbound variables
        if let Some(existing_value) = self.0.get(variable) {
            if existing_value.is_bound() {
                panic!("Attempted to write to bound variable {:?}", variable);
            }
            let mut new_memory = self.0.clone();
            new_memory.insert(variable.clone(), value);
            SingleAssignmentMemory(new_memory)
        } else {
            // If the variable is not in the memory, panic
            panic!("Attempted to write to unallocated variable {:?}", variable);
        }
    }

    pub fn show_memory(&self) {
        println!("|----------------|---------|");
        println!("|    Variable    |  Value  |");
        println!("|----------------|---------|");
        for (variable, value) in self.0.iter() {
            println!("| {:?} | {:?} |", variable, value);
        }
        println!("|----------------|---------|");
    }
}

pub type Atom = String;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Value {
    Unbound,
    Int(i32),
    Proc(Vec<Identifier>, Vec<VmInstruction>, Environment),
    Atom(Atom),
    Record(HashMap<Atom, Value>),
}

impl Value {
    pub fn is_bound(&self) -> bool {
        match self {
            Value::Unbound => false,
            _ => true,
        }
    }
}
