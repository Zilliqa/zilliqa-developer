use crate::highlevel_ir::Instruction;
use crate::highlevel_ir::{
    Operation, ConcreteFunction, ConcreteType, FunctionKind, HighlevelIr, IrLowering, Variant,
};
// TODO: Rewrite as a pass and consider making it a printer.
pub struct HighlevelIrStringGenerator {
    script: String,
}

impl HighlevelIrStringGenerator {
    pub fn new() -> Self {
        HighlevelIrStringGenerator {
            script: "".to_string(),
        }
    }

    pub fn value(&self) -> String {
        self.script.clone()
    }

    fn lower_operation(&mut self, operation: &Operation) {
        match operation {
            Operation::Jump(identifier) => {
                self.script.push_str("jmp %");
                self.script.push_str(&identifier.qualified_name().expect("qualified_name"));
            }
            Operation::ConditionalJump {
                expression,
                on_success,
                on_failure,
            } => {
                self.script.push_str("jmp_if %");
                self.script.push_str(&expression.qualified_name().expect("qualified_name"));
                self.script.push_str(" %");
                self.script.push_str(&on_success.qualified_name().expect("qualified_name"));
                self.script.push_str(" %");
                self.script.push_str(&on_failure.qualified_name().expect("qualified_name"));

            }
            Operation::MemLoad => self.script.push_str("load [TODO]"),
            Operation::MemStore => self.script.push_str("store [TODO]"),
            Operation::IsEqual { left, right } => {
                self.script.push_str("eq %");
                self.script.push_str(&left.qualified_name().expect("qualified_name"));
                self.script.push_str(" %");
                self.script.push_str(&right.qualified_name().expect("qualified_name"));
            },
            Operation::CallFunction { name, arguments } |
            Operation::CallExternalFunction { name, arguments } => {
                self.script.push_str("call ext @");
                self.script.push_str(&name.qualified_name().expect("qualified_name"));
                self.script.push_str("(");
                for (i,arg) in arguments.iter().enumerate() {
                    if i > 0 {
                        self.script.push_str(", ");
                    }
                    self.script.push_str("%");
                    self.script.push_str(&arg.qualified_name().expect("qualified_name"));                    
                }

                self.script.push_str(")")

            },
            Operation::CallStaticFunction {
                name,
                owner,
                arguments,
            } => self.script.push_str("call stat [TODO]"),
            Operation::CallMemberFunction {
                name,
                owner,
                arguments,
            } => unimplemented!(),
            Operation::ResolveSymbol { symbol } => unimplemented!(),
            Operation::Literal { data, typename } => {
                self.script.push_str(&typename.qualified_name().expect("qualified_name"));
                self.script.push_str(" ");
                self.script.push_str(&data);

            }
            Operation::AcceptTransfer => self.script.push_str("accept"),
            Operation::PhiNode(arguments) => {
                self.script.push_str("phi [");
                for (i,arg) in arguments.iter().enumerate() {
                    if i > 0 {
                        self.script.push_str(", ");
                    }
                    self.script.push_str("%");
                    self.script.push_str(&arg.qualified_name().expect("qualified_name"));                    
                }
                self.script.push_str("]");

            }
        }
    }

    fn lower_instruction(&mut self, instr: &Instruction) {
        if let Some(name) = &instr.ssa_name {
            self.script.push_str("%");
            self.script
                .push_str(&name.qualified_name().expect("qualified name"));
            self.script.push_str(" = ");
        }

        if let Some(rettype) = &instr.result_type {
            self.script
                .push_str(&rettype.qualified_name().expect("qualified name"));
            self.script.push_str(" ");
        }

        self.lower_operation(&instr.operation);
    }
}

impl IrLowering for HighlevelIrStringGenerator {
    // Lower a single concrete type from HighlevelIr to LLVM IR.
    fn lower_concrete_type(&mut self, con_type: &ConcreteType) {
        match con_type {
            ConcreteType::Tuple { name, data_layout } => {
                // provide functionality to handle tuple type
                self.script.push_str("tuple %");
                self.script
                    .push_str(&name.qualified_name().expect("qualified name"));
                self.script.push_str(" = (");
                for (i,arg) in data_layout.fields.iter().enumerate() {
                    if i > 0 {
                        self.script.push_str(", ");
                    }
                    self.script.push_str(&arg.qualified_name().expect("qualified_name"));                    
                }
                self.script.push_str(")\n");
            }
            ConcreteType::Variant { name, data_layout } => {
                self.script.push_str("tagged_union %");
                self.script
                    .push_str(&name.qualified_name().expect("qualified name"));
                self.script.push_str(" = {\n");
                for (i,arg) in data_layout.fields.iter().enumerate() {
                    if i > 0 {
                        self.script.push_str(",\n");
                    }

                    self.script.push_str("    ");
                    self.script.push_str(&arg.name.qualified_name().expect("qualified_name"));                    
                    if let Some(data) = &arg.data {
                        self.script.push_str(": ");                        
                        self.script.push_str(&data.qualified_name().expect("qualified_name"));                                            
                    }
                }
                self.script.push_str("\n}\n");
            }
        }
    }
    // Lower a single concrete function from HighlevelIr to LLVM IR.
    fn lower_concrete_function(&mut self, con_function: &ConcreteFunction) {
        let func_name = &con_function
            .name
            .qualified_name()
            .expect("qualified function name");

        match con_function.function_kind {
            FunctionKind::Procedure => {
                self.script.push_str("procedure @");
            }
            FunctionKind::Transition => {
                self.script.push_str("transition @");
            }
            FunctionKind::Function => {
                self.script.push_str("function @");
            }
        }
        self.script.push_str(func_name);
        self.script.push_str("(");

        self.script.push_str(") {\n");
        for block in &con_function.body.blocks {
            self.script
                .push_str(&block.name.qualified_name().expect("qualified name"));
            self.script.push_str(":\n");

            for instr in &block.instructions {
                self.script.push_str("    ");
                self.lower_instruction(&instr);
                self.script.push_str("\n");
            }
            self.script.push_str("\n");

        }
        self.script.push_str("}\n");
    }

    // Lower the entire HighLevelIr to LLVM IR.
    fn lower(&mut self, highlevel_ir: &HighlevelIr) {
        for con_type in &highlevel_ir.type_definitions {
            self.lower_concrete_type(con_type);
        }

        self.script.push_str("\n\n");

        for con_function in &highlevel_ir.function_definitions {
            self.lower_concrete_function(con_function);
        }
    }
}
