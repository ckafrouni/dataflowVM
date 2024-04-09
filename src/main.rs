use std::collections::HashMap;

#[derive(Debug)]
enum Value {
    Unbound,
    Int(i32),
}

impl Value {
    fn is_bound(&self) -> bool {
        match self {
            Value::Unbound => false,
            _ => true,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Identifier(String);
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Variable(String);

#[derive(Debug, Clone)]
struct Environment(HashMap<Identifier, Variable>);
#[derive(Debug)]
struct SingleAssignmentMemory(HashMap<Variable, Value>);

impl SingleAssignmentMemory {
    fn new() -> SingleAssignmentMemory {
        SingleAssignmentMemory(HashMap::new())
    }

    fn read(&self, variable: &Identifier) -> Option<&Value> {
        self.0.get(&Variable(variable.0.clone()))
    }

    fn write(&mut self, variable: &Identifier, value: Value) {
        if let Some(existing_value) = self.0.get(&Variable(variable.0.clone())) {
            if existing_value.is_bound() {
                panic!("Variable already bound");
            }
        }

        self.0.insert(Variable(variable.0.clone()), value);
    }
}

enum VmInstruction {
    Declare(Identifier),
    Assign(Identifier, i32),
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

    fn run(&mut self, instructions: Vec<VmInstruction>) {
        for instruction in instructions {
            self.execute(instruction);
        }
    }

    fn execute(&mut self, instruction: VmInstruction) {
        match instruction {
            VmInstruction::Declare(identifier) => {
                self.memory.write(&identifier, Value::Unbound);
            }
            VmInstruction::Assign(identifier, value) => {
                self.memory.write(&identifier, Value::Int(value));
            }
            VmInstruction::AssignAdd(identifier, lhs, rhs) => {
                let lhs_value = self.memory.read(&lhs).unwrap();
                let rhs_value = self.memory.read(&rhs).unwrap();

                if !lhs_value.is_bound() || !rhs_value.is_bound() {
                    panic!("Unbound variable");
                }

                let result = match (lhs_value, rhs_value) {
                    (Value::Int(lhs), Value::Int(rhs)) => lhs + rhs,
                    _ => panic!("Type error"),
                };

                self.memory.write(&identifier, Value::Int(result));
            }
            VmInstruction::Print(identifier) => {
                let value = self.memory.read(&identifier).unwrap();
                match value {
                    Value::Unbound => println!("Unbound"),
                    Value::Int(value) => println!("{}", value),
                }
            }
        }
    }
}

fn main() {
    let mut vm = Vm::new();

    let instructions = vec![
        VmInstruction::Declare(Identifier("x".to_string())),
        VmInstruction::Assign(Identifier("x".to_string()), 10),
        VmInstruction::Print(Identifier("x".to_string())),
        VmInstruction::Declare(Identifier("y".to_string())),
        VmInstruction::Assign(Identifier("y".to_string()), 20),
        VmInstruction::Print(Identifier("y".to_string())),
        VmInstruction::Declare(Identifier("z".to_string())),
        VmInstruction::AssignAdd(
            Identifier("z".to_string()),
            Identifier("x".to_string()),
            Identifier("y".to_string()),
        ),
        VmInstruction::Print(Identifier("z".to_string())),
    ];

    vm.run(instructions);
}
