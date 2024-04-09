use crate::identifier::Identifier;
use crate::variable::Variable;
use std::collections::HashMap;

#[derive(Debug, Clone)]
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

    pub fn _restrict(&self, identifier: &Identifier) -> Environment {
        let mut new_environment = self.clone();
        new_environment.0.remove(identifier);
        new_environment
    }
}
