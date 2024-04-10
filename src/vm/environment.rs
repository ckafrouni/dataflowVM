use super::Identifier;
use super::Variable;
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Environment(HashMap<Identifier, Variable>);

impl Environment {
    pub fn new() -> Environment {
        Environment(HashMap::new())
    }

    pub fn lookup(&self, identifier: &Identifier) -> Option<&Variable> {
        self.0.get(identifier)
    }

    pub fn adjoint(&self, identifier: &Identifier, variable: Variable) -> Environment {
        let mut new_environment = self.clone();
        new_environment.0.insert(identifier.clone(), variable);
        new_environment
    }

    pub fn restrict(&self, identifiers: Vec<Identifier>) -> Environment {
        let mut new_environment = Environment::new();
        for identifier in identifiers {
            if let Some(variable) = self.lookup(&identifier) {
                new_environment = new_environment.adjoint(&identifier, variable.clone());
            }
        }
        new_environment
        
    }
}
