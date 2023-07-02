use crate::constants::{TraversalResult, TreeTraversalMode};
use crate::highlevel_ir::Instruction;
use crate::highlevel_ir::{
    ConcreteFunction, ConcreteType, EnumValue, FunctionBlock, FunctionBody, FunctionKind,
    HighlevelIr, IrIdentifier, IrIndentifierKind, IrLowering, Operation, Tuple,
    VariableDeclaration, Variant,
};
use crate::highlevel_ir_pass::HighlevelIrPass;
use crate::highlevel_ir_pass_executor::HighlevelIrPassExecutor;

// TODO: Rewrite as a pass and consider making it a printer.
pub struct HighlevelIrStringGenerator {
    script: String,
}

impl HighlevelIrPass for HighlevelIrStringGenerator {
    fn visit_symbol_kind(
        &mut self,
        _mode: TreeTraversalMode,
        kind: &mut IrIndentifierKind,
    ) -> Result<TraversalResult, String> {
        match kind {
            IrIndentifierKind::FunctionName => self.script.push_str("@"),
            IrIndentifierKind::TransitionName => self.script.push_str("@"),
            IrIndentifierKind::ProcedureName => self.script.push_str("@"),
            IrIndentifierKind::ExternalFunctionName => self.script.push_str("@"),
            IrIndentifierKind::TypeName => self.script.push_str("%"),
            IrIndentifierKind::ComponentName => self.script.push_str("@"),
            IrIndentifierKind::Event => self.script.push_str("@"),
            IrIndentifierKind::Namespace => self.script.push_str("@"),
            IrIndentifierKind::BlockLabel => self.script.push_str(":"),
            IrIndentifierKind::VirtualRegister => self.script.push_str("%"),
            IrIndentifierKind::VirtualRegisterIntermediate => self.script.push_str("%"),
            IrIndentifierKind::Unknown => self.script.push_str("?"),
        }
        Ok(TraversalResult::SkipChildren)
    }
    fn visit_symbol_name(
        &mut self,
        mode: TreeTraversalMode,
        symbol: &mut IrIdentifier,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => {
                match symbol.kind {
                    IrIndentifierKind::FunctionName
                    | IrIndentifierKind::TransitionName
                    | IrIndentifierKind::ProcedureName
                    | IrIndentifierKind::ExternalFunctionName
                    | IrIndentifierKind::TypeName
                    | IrIndentifierKind::BlockLabel
                    | IrIndentifierKind::Namespace => {
                        return Ok(TraversalResult::Continue);
                    }
                    _ => (),
                }

                if let Some(name) = &symbol.type_reference {
                    self.script.push_str(&name);
                } else {
                    self.script.push_str("(untyped)");
                }
                self.script.push_str(" ");
            }
            TreeTraversalMode::Exit => {
                if let Some(name) = &symbol.resolved {
                    self.script.push_str(&name);
                } else {
                    self.script.push_str("[");
                    self.script.push_str(&symbol.unresolved);
                    self.script.push_str("]");
                }
            }
        }
        Ok(TraversalResult::Continue)
    }

    fn visit_enum_value(
        &mut self,
        mode: TreeTraversalMode,
        enum_value: &mut EnumValue,
    ) -> Result<TraversalResult, String> {
        self.script.push_str("    ");
        enum_value.name.visit(self);

        if let Some(data) = &mut enum_value.data {
            self.script.push_str(" : ");
            data.visit(self);
        }

        self.script.push_str(",\n");
        Ok(TraversalResult::SkipChildren)
    }

    fn visit_tuple(
        &mut self,
        mode: TreeTraversalMode,
        tuple: &mut Tuple,
    ) -> Result<TraversalResult, String> {
        for field in tuple.fields.iter_mut() {
            self.script.push_str("    ");
            field.visit(self);
            self.script.push_str(",\n");
        }
        Ok(TraversalResult::SkipChildren)
    }

    fn visit_variant(
        &mut self,
        mode: TreeTraversalMode,
        variant: &mut Variant,
    ) -> Result<TraversalResult, String> {
        // Pass through deliberate
        Ok(TraversalResult::Continue)
    }

    fn visit_variable_declaration(
        &mut self,
        _mode: TreeTraversalMode,
        var_dec: &mut VariableDeclaration,
    ) -> Result<TraversalResult, String> {
        self.script.push_str(&var_dec.name);
        self.script.push_str(":");
        var_dec.typename.visit(self)?;
        Ok(TraversalResult::SkipChildren)
    }

    fn visit_operation(
        &mut self,
        mode: TreeTraversalMode,
        operation: &mut Operation,
    ) -> Result<TraversalResult, String> {
        match operation {
            Operation::Jump(identifier) => {
                self.script.push_str("jmp ");
                identifier.visit(self)?;
            }
            Operation::ConditionalJump {
                expression,
                on_success,
                on_failure,
            } => {
                self.script.push_str("jmp_if ");
                expression.visit(self)?;
                self.script.push_str(" ");
                on_success.visit(self)?;
                self.script.push_str(" ");
                on_failure.visit(self)?;
            }
            Operation::MemLoad => self.script.push_str("load [TODO]"),
            Operation::MemStore => self.script.push_str("store [TODO]"),
            Operation::IsEqual { left, right } => {
                self.script.push_str("eq ");
                left.visit(self)?;
                self.script.push_str(" ");
                right.visit(self)?;
            }
            Operation::CallFunction { name, arguments }
            | Operation::CallExternalFunction { name, arguments } => {
                self.script.push_str("call ");
                name.visit(self)?;
                self.script.push_str(" ");
                for (i, arg) in arguments.iter_mut().enumerate() {
                    if i > 0 {
                        self.script.push_str(", ");
                    }
                    arg.visit(self)?;
                }
            }

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
                typename.visit(self)?;
                self.script.push_str(" ");
                self.script.push_str(&data);
            }
            Operation::AcceptTransfer => self.script.push_str("accept"),
            Operation::PhiNode(arguments) => {
                self.script.push_str("phi [");
                for (i, arg) in arguments.iter_mut().enumerate() {
                    if i > 0 {
                        self.script.push_str(", ");
                    }
                    arg.visit(self)?;
                }
                self.script.push_str("]");
            }
        }
        Ok(TraversalResult::SkipChildren)
    }

    fn visit_instruction(
        &mut self,
        mode: TreeTraversalMode,
        instr: &mut Instruction,
    ) -> Result<TraversalResult, String> {
        self.script.push_str("    ");
        if let Some(name) = &mut instr.ssa_name {
            name.visit(self)?;
            self.script.push_str(" = ");
        }

        if let Some(rettype) = &mut instr.result_type {
            rettype.visit(self)?;
            self.script.push_str(" ");
        }

        instr.operation.visit(self);

        self.script.push_str("\n");

        Ok(TraversalResult::SkipChildren)
    }

    fn visit_function_block(
        &mut self,
        mode: TreeTraversalMode,
        block: &mut FunctionBlock,
    ) -> Result<TraversalResult, String> {
        block.name.visit(self)?;
        self.script.push_str(":\n");
        for instr in block.instructions.iter_mut() {
            instr.visit(self)?;
        }
        self.script.push_str("\n");
        Ok(TraversalResult::SkipChildren)
    }

    fn visit_function_body(
        &mut self,
        _mode: TreeTraversalMode,
        function_body: &mut FunctionBody,
    ) -> Result<TraversalResult, String> {
        for block in &mut function_body.blocks {
            block.visit(self)?;
        }
        Ok(TraversalResult::SkipChildren)
    }

    fn visit_concrete_type(
        &mut self,
        mode: TreeTraversalMode,
        con_type: &mut ConcreteType,
    ) -> Result<TraversalResult, String> {
        match con_type {
            ConcreteType::Tuple { name, data_layout } => {
                self.script.push_str("tuple ");
                name.visit(self)?;
                self.script.push_str(" = (\n");
                data_layout.visit(self);
                self.script.push_str(")\n");
            }
            ConcreteType::Variant { name, data_layout } => {
                self.script.push_str("tagged_union ");
                name.visit(self)?;
                self.script.push_str(" = {\n");
                data_layout.visit(self);
                self.script.push_str("}\n");
            }
        }
        Ok(TraversalResult::SkipChildren)
    }

    fn visit_function_kind(
        &mut self,
        _mode: TreeTraversalMode,
        function_kind: &mut FunctionKind,
    ) -> Result<TraversalResult, String> {
        match function_kind {
            FunctionKind::Procedure => {
                self.script.push_str("procedure");
            }
            FunctionKind::Transition => {
                self.script.push_str("transition");
            }
            FunctionKind::Function => {
                self.script.push_str("function");
            }
        }

        Ok(TraversalResult::SkipChildren)
    }

    fn visit_concrete_function(
        &mut self,
        mode: TreeTraversalMode,
        fnc: &mut ConcreteFunction,
    ) -> Result<TraversalResult, String> {
        fnc.function_kind.visit(self);

        self.script.push_str(" ");
        fnc.name.visit(self);
        self.script.push_str("(");
        for (i, arg) in fnc.arguments.iter_mut().enumerate() {
            if i > 0 {
                self.script.push_str(", ");
            }
            arg.visit(self);
        }
        self.script.push_str(") : ");
        if let Some(rt) = &mut fnc.return_type {
            self.script.push_str("(TODO)");
            self.script.push_str(&rt);
            // rt.visit(self)
        } else {
            self.script.push_str("(untyped)");
        }
        self.script.push_str(" {\n");

        fnc.body.visit(self)?;
        self.script.push_str("}\n");
        Ok(TraversalResult::SkipChildren)
    }

    fn visit_highlevel_ir(
        &mut self,
        mode: TreeTraversalMode,
        highlevel_ir: &mut HighlevelIr,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => {
                // TODO: Emit scilla version etc
                // unimplemented!()
            }
            TreeTraversalMode::Exit => {
                // unimplemented!()
            }
        }
        Ok(TraversalResult::Continue)
    }
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
                self.script
                    .push_str(&identifier.qualified_name().expect("qualified_name"));
            }
            Operation::ConditionalJump {
                expression,
                on_success,
                on_failure,
            } => {
                self.script.push_str("jmp_if %");
                self.script
                    .push_str(&expression.qualified_name().expect("qualified_name"));
                self.script.push_str(" %");
                self.script
                    .push_str(&on_success.qualified_name().expect("qualified_name"));
                self.script.push_str(" %");
                self.script
                    .push_str(&on_failure.qualified_name().expect("qualified_name"));
            }
            Operation::MemLoad => self.script.push_str("load [TODO]"),
            Operation::MemStore => self.script.push_str("store [TODO]"),
            Operation::IsEqual { left, right } => {
                self.script.push_str("eq %");
                self.script
                    .push_str(&left.qualified_name().expect("qualified_name"));
                self.script.push_str(" %");
                self.script
                    .push_str(&right.qualified_name().expect("qualified_name"));
            }
            Operation::CallFunction { name, arguments }
            | Operation::CallExternalFunction { name, arguments } => {
                self.script.push_str("call ext @");
                self.script
                    .push_str(&name.qualified_name().expect("qualified_name"));
                self.script.push_str("(");
                for (i, arg) in arguments.iter().enumerate() {
                    if i > 0 {
                        self.script.push_str(", ");
                    }
                    self.script.push_str("%");
                    self.script
                        .push_str(&arg.qualified_name().expect("qualified_name"));
                }

                self.script.push_str(")")
            }
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
                self.script
                    .push_str(&typename.qualified_name().expect("qualified_name"));
                self.script.push_str(" ");
                self.script.push_str(&data);
            }
            Operation::AcceptTransfer => self.script.push_str("accept"),
            Operation::PhiNode(arguments) => {
                self.script.push_str("phi [");
                for (i, arg) in arguments.iter().enumerate() {
                    if i > 0 {
                        self.script.push_str(", ");
                    }
                    self.script.push_str("%");
                    self.script
                        .push_str(&arg.qualified_name().expect("qualified_name"));
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
                for (i, arg) in data_layout.fields.iter().enumerate() {
                    if i > 0 {
                        self.script.push_str(", ");
                    }
                    self.script
                        .push_str(&arg.qualified_name().expect("qualified_name"));
                }
                self.script.push_str(")\n");
            }
            ConcreteType::Variant { name, data_layout } => {
                self.script.push_str("tagged_union %");
                self.script
                    .push_str(&name.qualified_name().expect("qualified name"));
                self.script.push_str(" = {\n");
                for (i, arg) in data_layout.fields.iter().enumerate() {
                    if i > 0 {
                        self.script.push_str(",\n");
                    }

                    self.script.push_str("    ");
                    self.script
                        .push_str(&arg.name.qualified_name().expect("qualified_name"));
                    if let Some(data) = &arg.data {
                        self.script.push_str(": ");
                        self.script
                            .push_str(&data.qualified_name().expect("qualified_name"));
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
