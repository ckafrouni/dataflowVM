use crate::{Environment, VmInstruction};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SemanticInstruction(pub Vec<VmInstruction>, pub Environment);
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SemanticStack(pub Vec<SemanticInstruction>);

impl SemanticStack {
    pub fn pop(&mut self) -> Option<SemanticInstruction> {
        self.0.pop()
    }

    pub fn push(&mut self, instruction: SemanticInstruction) {
        self.0.push(instruction);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

// #[derive(Debug)]
// struct ExecutionState(SemanticStack, SingleAssignmentMemory);
