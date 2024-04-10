mod vm;

use vm::{Environment, Identifier, SemanticInstruction, SemanticStack, Value, Vm, VmInstruction};

fn main() {
    let x = Identifier::new("X".to_string());
    let y = Identifier::new("Y".to_string());
    let z = Identifier::new("Z".to_string());
    let foo = Identifier::new("foo".to_string());

    let semantic_instruction = SemanticInstruction(
        vec![
            VmInstruction::Local(
                vec![foo.clone()],
                vec![
                    VmInstruction::ProcDef(
                        foo.clone(),
                        vec![x.clone(), y.clone(), z.clone()], // Parameters
                        vec![],                                // Free variables
                        vec![
                            VmInstruction::Print(x.clone()),
                            VmInstruction::AssignAdd(z.clone(), x.clone(), y.clone()),
                        ], // Body
                    ),
                    VmInstruction::Local(
                        vec![x.clone(), y.clone(), z.clone()],
                        vec![
                            VmInstruction::Assign(x.clone(), Value::Int(5)),
                            VmInstruction::Assign(y.clone(), Value::Int(6)),
                            VmInstruction::Print(x.clone()),
                            VmInstruction::ProcCall(
                                foo.clone(),
                                vec![x.clone(), y.clone(), z.clone()],
                            ),
                            VmInstruction::Print(z.clone()),
                        ],
                    ),
                ],
            ),
            VmInstruction::Local(
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
            ),
        ],
        Environment::new(),
    );

    let semantic_stack = SemanticStack(vec![semantic_instruction]);

    let mut vm = Vm::new();
    vm.create_thread(0, semantic_stack);
    vm.run();
    vm.show_memory();
}
