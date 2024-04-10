mod environment;
mod identifier;
mod single_assignment_memory;
mod variable;

use std::collections::BinaryHeap;

use environment::Environment;
use identifier::Identifier;
use single_assignment_memory::{SingleAssignmentMemory, Value};
use variable::Variable;

#[derive(Debug, Clone, Eq, PartialEq)]
struct SemanticInstruction(Vec<VmInstruction>, Environment);
#[derive(Debug, Clone, Eq, PartialEq)]
struct SemanticStack(Vec<SemanticInstruction>);

impl SemanticStack {
    fn pop(&mut self) -> Option<SemanticInstruction> {
        self.0.pop()
    }

    fn push(&mut self, instruction: SemanticInstruction) {
        self.0.push(instruction);
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

// #[derive(Debug)]
// struct ExecutionState(SemanticStack, SingleAssignmentMemory);

#[derive(Debug, Clone, Eq, PartialEq)]
enum VmInstruction {
    Thread(Vec<VmInstruction>),
    Local(Vec<Identifier>, Vec<VmInstruction>),
    Assign(Identifier, Value),
    AssignAdd(Identifier, Identifier, Identifier),
    Print(Identifier),
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum ThreadState {
    // Running,
    Blocked,
    Ready,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Thread {
    id: u32,
    priority: u32, // Lower is higher priority
    state: ThreadState,
    semantic_stack: SemanticStack,
}

static mut THREAD_ID: u32 = 0;

impl Thread {
    fn new(priority: u32, semantic_stack: SemanticStack) -> Thread {
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

struct Vm {
    memory: SingleAssignmentMemory,
    threads: BinaryHeap<Thread>,
}

struct ExecutionResult {
    state: ThreadState,
    semantic_stack: Option<SemanticStack>,
}

impl Vm {
    fn new() -> Vm {
        Vm {
            memory: SingleAssignmentMemory::new(),
            threads: BinaryHeap::new(),
        }
    }

    fn show_memory(&self) {
        self.memory.show_memory();
    }

    fn create_thread(&mut self, priority: u32, semantic_stack: SemanticStack) {
        self.threads.push(Thread::new(priority, semantic_stack));
    }

    fn run(&mut self) {
        while let Some(mut thread) = self.threads.pop() {
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
            println!("\x1b[0;31mINSTRUCTION: Sequence Split\x1b[0m");
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
                println!("\x1b[0;31mINSTRUCTION: Thread\x1b[0m");
                let new_environment = environment.clone();
                let new_semantic_instruction = SemanticInstruction(instructions, new_environment);
                let new_semantic_stack = SemanticStack(vec![new_semantic_instruction]);
                self.threads.push(Thread::new(0, new_semantic_stack));
            }
            VmInstruction::Local(identifiers, instructions) => {
                println!("\x1b[0;31mINSTRUCTION: Local\x1b[0m");
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
                println!("\x1b[0;31mINSTRUCTION: Assign\x1b[0m");
                let variable = environment.lookup(&identifier).unwrap();
                self.memory.bind(variable, value);
            }
            VmInstruction::AssignAdd(identifier, lhs, rhs) => {
                println!("\x1b[0;31mINSTRUCTION: AssignAdd\x1b[0m");
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
                println!("\x1b[0;31mINSTRUCTION: Print\x1b[0m");
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

//MARK: A label

fn main() {
    let x = Identifier::new("X".to_string());
    let y = Identifier::new("Y".to_string());
    let z = Identifier::new("Z".to_string());

    let semantic_instruction = SemanticInstruction(
        vec![VmInstruction::Local(
            vec![z.clone()],
            vec![
                VmInstruction::Local(
                    vec![x.clone(), y.clone()],
                    vec![
                        VmInstruction::Assign(x.clone(), Value::Int(10)),
                        VmInstruction::Print(x.clone()),
                        VmInstruction::Thread(vec![
                            VmInstruction::AssignAdd(z.clone(), x.clone(), y.clone()),
                            VmInstruction::Print(z.clone()),
                        ]),
                        VmInstruction::Assign(y.clone(), Value::Int(20)),
                        VmInstruction::Print(y.clone()),
                        VmInstruction::Print(z.clone()),
                    ],
                ),
                VmInstruction::Print(z),
            ],
        )],
        Environment::new(),
    );

    let semantic_stack = SemanticStack(vec![semantic_instruction]);

    let mut vm = Vm::new();
    vm.create_thread(0, semantic_stack);
    vm.run();
    vm.show_memory();
}
