use crate::constants::{TraversalResult, TreeTraversalMode};
use crate::highlevel_ir::Instruction;
use crate::highlevel_ir::{
    ConcreteFunction, ConcreteType, EnumValue, FunctionBlock, FunctionBody, FunctionKind,
    HighlevelIr, IrIdentifier, IrIndentifierKind, Operation, Tuple, VariableDeclaration, Variant,
};
use crate::highlevel_ir_pass::HighlevelIrPass;
use crate::highlevel_ir_pass_executor::HighlevelIrPassExecutor;

pub struct HighlevelIrDebugPrinter {
    script: String,
}

impl HighlevelIrPass for HighlevelIrDebugPrinter {
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
            IrIndentifierKind::Memory => self.script.push_str("%"),

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
                self.script.push_str(&symbol.qualified_name()?);
            }
        }
        Ok(TraversalResult::Continue)
    }

    fn visit_enum_value(
        &mut self,
        _mode: TreeTraversalMode,
        enum_value: &mut EnumValue,
    ) -> Result<TraversalResult, String> {
        self.script.push_str("    ");
        let _ = enum_value.name.visit(self)?;

        if let Some(data) = &mut enum_value.data {
            self.script.push_str(" : ");
            let _ = data.visit(self)?;
        }

        self.script.push_str(",\n");
        Ok(TraversalResult::SkipChildren)
    }

    fn visit_tuple(
        &mut self,
        _mode: TreeTraversalMode,
        tuple: &mut Tuple,
    ) -> Result<TraversalResult, String> {
        for field in tuple.fields.iter_mut() {
            self.script.push_str("    ");
            let _ = field.visit(self)?;
            self.script.push_str(",\n");
        }
        Ok(TraversalResult::SkipChildren)
    }

    fn visit_variant(
        &mut self,
        _mode: TreeTraversalMode,
        _variant: &mut Variant,
    ) -> Result<TraversalResult, String> {
        // Pass through deliberate
        Ok(TraversalResult::Continue)
    }

    fn visit_variable_declaration(
        &mut self,
        _mode: TreeTraversalMode,
        var_dec: &mut VariableDeclaration,
    ) -> Result<TraversalResult, String> {
        var_dec.name.visit(self)?;
        self.script.push_str(" : ");
        var_dec.typename.visit(self)?;
        Ok(TraversalResult::SkipChildren)
    }

    fn visit_operation(
        &mut self,
        _mode: TreeTraversalMode,
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
                name: _,
                owner: _,
                arguments: _,
            } => self.script.push_str("call stat [TODO]"),
            Operation::CallMemberFunction {
                name: _,
                owner: _,
                arguments: _,
            } => unimplemented!(),
            Operation::ResolveSymbol { symbol: _ } => unimplemented!(),
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
        _mode: TreeTraversalMode,
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

        let _ = instr.operation.visit(self)?;

        self.script.push_str("\n");

        Ok(TraversalResult::SkipChildren)
    }

    fn visit_function_block(
        &mut self,
        _mode: TreeTraversalMode,
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
        _mode: TreeTraversalMode,
        con_type: &mut ConcreteType,
    ) -> Result<TraversalResult, String> {
        match con_type {
            ConcreteType::Tuple {
                name,
                data_layout,
                namespace: _,
            } => {
                self.script.push_str("tuple ");
                let _ = name.visit(self)?;
                self.script.push_str(" = (\n");
                let _ = data_layout.visit(self)?;
                self.script.push_str(")\n");
            }
            ConcreteType::Variant {
                name,
                data_layout,
                namespace: _,
            } => {
                self.script.push_str("tagged_union ");
                let _ = name.visit(self)?;
                self.script.push_str(" = {\n");
                let _ = data_layout.visit(self)?;
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
        _mode: TreeTraversalMode,
        fnc: &mut ConcreteFunction,
    ) -> Result<TraversalResult, String> {
        let _ = fnc.function_kind.visit(self)?;

        self.script.push_str(" ");
        let _ = fnc.name.visit(self)?;
        self.script.push_str("(");
        for (i, arg) in fnc.arguments.iter_mut().enumerate() {
            if i > 0 {
                self.script.push_str(", ");
            }
            let _ = arg.visit(self)?;
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
        _highlevel_ir: &mut HighlevelIr,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => {
                // TODO: Emit scilla version etc
                // unimplemented!()
            }
            TreeTraversalMode::Exit => {
                println!("{}", self.script);
            }
        }
        Ok(TraversalResult::Continue)
    }
}

impl HighlevelIrDebugPrinter {
    pub fn new() -> Self {
        HighlevelIrDebugPrinter {
            script: "".to_string(),
        }
    }

    pub fn value(&self) -> String {
        self.script.clone()
    }
}
