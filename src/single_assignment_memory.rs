use crate::variable::Variable;
use std::collections::HashMap;

#[derive(Debug)]
pub struct SingleAssignmentMemory(HashMap<Variable, Value>);

impl SingleAssignmentMemory {
    pub fn new() -> SingleAssignmentMemory {
        SingleAssignmentMemory(HashMap::new())
    }

    pub fn read(&self, variable: &Variable) -> Option<&Value> {
        self.0.get(variable)
    }

    pub fn allocate(&mut self, variable: Variable) {
        // Only allow allocating unallocated variables
        if self.0.contains_key(&variable) {
            panic!(
                "Attempted to allocate already allocated variable {:?}",
                variable
            );
        }
        self.0.insert(variable, Value::Unbound);
    }

    pub fn bind(&mut self, variable: &Variable, value: Value) {
        // Only allow writing to unbound variables
        if let Some(existing_value) = self.0.get(variable) {
            if existing_value.is_bound() {
                panic!("Attempted to write to bound variable {:?}", variable);
            }
            // If the variable is in the memory and unbound, write to it
            self.0.insert(variable.clone(), value);
        } else {
            // If the variable is not in the memory, panic
            panic!("Attempted to write to unallocated variable {:?}", variable);
        }
    }

    pub fn show_memory(&self) {
        for (variable, value) in self.0.iter() {
            println!("{:?} -> {:?}", variable, value);
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Value {
    Unbound,
    Int(i32),
}

impl Value {
    pub fn is_bound(&self) -> bool {
        match self {
            Value::Unbound => false,
            _ => true,
        }
    }
}
