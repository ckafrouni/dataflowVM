mod environment;
mod identifier;
mod semantics;
mod single_assignment_memory;
mod variable;

use std::collections::BinaryHeap;

pub use environment::Environment;
pub use identifier::Identifier;
pub use semantics::{SemanticInstruction, SemanticStack};
pub use single_assignment_memory::{SingleAssignmentMemory, Value};
pub use variable::Variable;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VmInstruction {
    Thread(Vec<VmInstruction>),
    Local(Vec<Identifier>, Vec<VmInstruction>),
    Assign(Identifier, Value),
    AssignAdd(Identifier, Identifier, Identifier),
    Print(Identifier),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ThreadState {
    // Running,
    Blocked,
    Ready,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread {
    id: u32,
    priority: u32, // Lower is higher priority
    state: ThreadState,
    semantic_stack: SemanticStack,
}

static mut THREAD_ID: u32 = 0;

impl Thread {
    pub fn new(priority: u32, semantic_stack: SemanticStack) -> Thread {
        unsafe {
            THREAD_ID += 1;
            Thread {
                id: THREAD_ID,
                priority,
                state: ThreadState::Ready,
                semantic_stack,
            }
        }
    }
}

impl PartialOrd for Thread {
    fn partial_cmp(&self, other: &Thread) -> Option<std::cmp::Ordering> {
        Some(self.priority.cmp(&other.priority))
    }
}

impl Ord for Thread {
    fn cmp(&self, other: &Thread) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

pub struct Vm {
    memory: SingleAssignmentMemory,
    threads: BinaryHeap<Thread>,
}

pub struct ExecutionResult {
    state: ThreadState,
    semantic_stack: Option<SemanticStack>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            memory: SingleAssignmentMemory::new(),
            threads: BinaryHeap::new(),
        }
    }

    pub fn show_memory(&self) {
        self.memory.show_memory();
    }

    pub fn create_thread(&mut self, priority: u32, semantic_stack: SemanticStack) {
        self.threads.push(Thread::new(priority, semantic_stack));
    }

    pub fn run(&mut self) {
        while let Some(mut thread) = self.threads.pop() {
            println!("Threads {:?}", self.threads);
            println!(
                "Running thread {:?} with priority {:?} and state {:?}",
                thread.id, thread.priority, thread.state
            );
            if thread.state == ThreadState::Blocked {
                self.threads.push(thread);
                continue;
            }
            thread.priority += 1;
            if !thread.semantic_stack.is_empty() {
                let res = self.execute(thread.semantic_stack.clone());

                thread.state = res.state;
                if let Some(semantic_stack) = res.semantic_stack {
                    thread.semantic_stack = semantic_stack;
                }

                self.threads.push(thread);
            }
        }
    }

    fn execute(&mut self, mut semantic_stack: SemanticStack) -> ExecutionResult {
        let semantic_instruction = semantic_stack.pop().unwrap();
        let SemanticInstruction(instructions, environment) = semantic_instruction;
        if instructions.len() > 1 {
            // println!("\x1b[0;31mINSTRUCTION: Sequence Split\x1b[0m");
            let head_semantic_instruction =
                SemanticInstruction(instructions[0..1].to_vec(), environment.clone());
            let tail_semantic_instruction =
                SemanticInstruction(instructions[1..].to_vec(), environment.clone());
            semantic_stack.push(tail_semantic_instruction);
            semantic_stack.push(head_semantic_instruction);
            return ExecutionResult {
                state: ThreadState::Ready,
                semantic_stack: Some(semantic_stack),
            };
        }
        let instruction = instructions[0].clone();
        match instruction {
            VmInstruction::Thread(instructions) => {
                // println!("\x1b[0;31mINSTRUCTION: Thread\x1b[0m");
                let new_environment = environment.clone();
                let new_semantic_instruction = SemanticInstruction(instructions, new_environment);
                let new_semantic_stack = SemanticStack(vec![new_semantic_instruction]);
                // println!("Creating thread");
                self.threads.push(Thread::new(0, new_semantic_stack));
            }
            VmInstruction::Local(identifiers, instructions) => {
                // println!("\x1b[0;31mINSTRUCTION: Local\x1b[0m");
                let new_environment =
                    identifiers
                        .iter()
                        .fold(environment.clone(), |acc, identifier| {
                            let variable = Variable::new();
                            self.memory.allocate(variable.clone());
                            acc.adjoint(identifier, variable)
                        });

                let new_semantic_instruction = SemanticInstruction(instructions, new_environment);
                semantic_stack.push(new_semantic_instruction);
                return ExecutionResult {
                    state: ThreadState::Ready,
                    semantic_stack: Some(semantic_stack),
                };
            }
            VmInstruction::Assign(identifier, value) => {
                // println!("\x1b[0;31mINSTRUCTION: Assign\x1b[0m");
                let variable = environment.lookup(&identifier).unwrap();
                self.memory.bind(variable, value);
            }
            VmInstruction::AssignAdd(identifier, lhs, rhs) => {
                // println!("\x1b[0;31mINSTRUCTION: AssignAdd\x1b[0m");
                let lhs_var = environment.lookup(&lhs).unwrap();
                let rhs_var = environment.lookup(&rhs).unwrap();
                let variable = environment.lookup(&identifier).unwrap();

                let lhs_value = self.memory.read(&lhs_var).unwrap();
                let rhs_value = self.memory.read(&rhs_var).unwrap();

                if !lhs_value.is_bound() || !rhs_value.is_bound() {
                    return ExecutionResult {
                        state: ThreadState::Blocked,
                        semantic_stack: Some(semantic_stack),
                    };
                }

                let result = match (lhs_value, rhs_value) {
                    (Value::Int(lhs), Value::Int(rhs)) => lhs + rhs,
                    _ => panic!("Type error"),
                };

                self.memory.bind(variable, Value::Int(result));
            }
            VmInstruction::Print(identifier) => {
                // println!("\x1b[0;31mINSTRUCTION: Print\x1b[0m");
                let variable = environment.lookup(&identifier).unwrap();
                let value = self.memory.read(variable).unwrap();
                match value {
                    Value::Unbound => println!("\x1b[0;32m{:?} <- Unbound\x1b[0m", identifier),
                    Value::Int(value) => {
                        println!("\x1b[0;32m{:?} <- {:?}\x1b[0m", identifier, value)
                    }
                }
            }
        }
        ExecutionResult {
            state: ThreadState::Ready,
            semantic_stack: Some(semantic_stack),
        }
    }
}
