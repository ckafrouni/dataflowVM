mod environment;
mod identifier;
mod single_assignment_memory;
mod variable;

use environment::Environment;
use identifier::Identifier;
use single_assignment_memory::{SingleAssignmentMemory, Value};
use variable::Variable;

#[derive(Debug)]
struct SemanticInstruction(Vec<VmInstruction>, Environment);
#[derive(Debug)]
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

#[derive(Debug, Clone)]
enum VmInstruction {
    Local(Vec<Identifier>, Vec<VmInstruction>),
    Assign(Identifier, Value),
    AssignAdd(Identifier, Identifier, Identifier),
    Print(Identifier),
}

struct Vm {
    memory: SingleAssignmentMemory,
}

impl Vm {
    fn new() -> Vm {
        Vm {
            memory: SingleAssignmentMemory::new(),
        }
    }

    fn show_memory(&self) {
        self.memory.show_memory();
    }

    fn run(&mut self, semantic_stack: &mut SemanticStack) {
        while !semantic_stack.is_empty() {
            let semantic_instruction = semantic_stack.pop().unwrap();
            if semantic_instruction.0.len() > 1 {
                semantic_stack.push(SemanticInstruction(
                    semantic_instruction.0[1..].to_vec(),
                    semantic_instruction.1.clone(),
                ));
                semantic_stack.push(SemanticInstruction(
                    vec![semantic_instruction.0[0].clone()],
                    semantic_instruction.1.clone(),
                ));
            } else {
                self.execute(semantic_instruction.0[0].clone(), &semantic_instruction.1);
            }
        }
    }

    fn execute(&mut self, instruction: VmInstruction, environment: &Environment) {
        match instruction {
            VmInstruction::Local(identifiers, instructions) => {
                let new_environment =
                    identifiers
                        .iter()
                        .fold(environment.clone(), |acc, identifier| {
                            let variable = Variable::new();
                            self.memory.allocate(variable.clone());
                            acc.adjoint(identifier, variable)
                        });

                for instruction in instructions {
                    self.execute(instruction, &new_environment);
                }
            }
            VmInstruction::Assign(identifier, value) => {
                let variable = environment.lookup(&identifier).unwrap();
                self.memory.bind(variable, value);
            }
            VmInstruction::AssignAdd(identifier, lhs, rhs) => {
                let lhs_var = environment.lookup(&lhs).unwrap();
                let rhs_var = environment.lookup(&rhs).unwrap();
                let variable = environment.lookup(&identifier).unwrap();

                let lhs_value = self.memory.read(&lhs_var).unwrap();
                let rhs_value = self.memory.read(&rhs_var).unwrap();

                if !lhs_value.is_bound() || !rhs_value.is_bound() {
                    panic!("Unbound variable");
                }

                let result = match (lhs_value, rhs_value) {
                    (Value::Int(lhs), Value::Int(rhs)) => lhs + rhs,
                    _ => panic!("Type error"),
                };

                self.memory.bind(variable, Value::Int(result));
            }
            VmInstruction::Print(identifier) => {
                println!("Printing {:?}", identifier);
                let variable = environment.lookup(&identifier).unwrap();
                let value = self.memory.read(variable).unwrap();
                match value {
                    Value::Unbound => println!("Unbound"),
                    Value::Int(value) => println!("{}", value),
                }
            }
        }
    }
}

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
                        VmInstruction::Assign(y.clone(), Value::Int(20)),
                        VmInstruction::Print(x.clone()),
                        VmInstruction::Print(y.clone()),
                        VmInstruction::AssignAdd(z.clone(), x.clone(), y.clone()),
                        VmInstruction::Print(z.clone()),
                    ],
                ),
                VmInstruction::Print(z),
            ],
        )],
        Environment::new(),
    );

    let mut semantic_stack = SemanticStack(vec![semantic_instruction]);

    let mut vm = Vm::new();
    vm.run(&mut semantic_stack);
    vm.show_memory();
}
