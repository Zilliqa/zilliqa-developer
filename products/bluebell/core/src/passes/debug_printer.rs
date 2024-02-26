use scilla_parser::ast::{TraversalResult, TreeTraversalMode};

use crate::intermediate_representation::{
    pass::IrPass,
    pass_executor::PassExecutor,
    primitives::{
        CaseClause, ConcreteFunction, ConcreteType, ContractField, EnumValue, FunctionBlock,
        FunctionBody, FunctionKind, Instruction, IntermediateRepresentation, IrIdentifier,
        IrIndentifierKind, Operation, Tuple, VariableDeclaration, Variant,
    },
    symbol_table::SymbolTable,
};

pub struct DebugPrinter {
    script: String,
}

impl IrPass for DebugPrinter {
    fn initiate(&mut self) {
        self.script = "".to_string();
    }

    fn finalize(&mut self) {}

    fn visit_symbol_kind(
        &mut self,
        _mode: TreeTraversalMode,
        kind: &mut IrIndentifierKind,
        _symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        match kind {
            IrIndentifierKind::TemplateFunctionName => self.script.push_str("!!"),
            IrIndentifierKind::StaticFunctionName => self.script.push_str("@!"),
            IrIndentifierKind::FunctionName => self.script.push_str("@"),
            IrIndentifierKind::TransitionName => self.script.push_str("@"),
            IrIndentifierKind::ProcedureName => self.script.push_str("@"),
            IrIndentifierKind::ExternalFunctionName => self.script.push_str("@"),
            IrIndentifierKind::TypeName => self.script.push_str("%"),
            IrIndentifierKind::ComponentName => self.script.push_str("@"),
            IrIndentifierKind::Event => self.script.push_str("@"),
            IrIndentifierKind::Namespace => self.script.push_str("@"),
            IrIndentifierKind::BlockLabel => self.script.push_str(":"),
            IrIndentifierKind::ContextResource => self.script.push_str("~"),
            IrIndentifierKind::VirtualRegister => self.script.push_str("%"),
            IrIndentifierKind::VirtualRegisterIntermediate => self.script.push_str("%"),
            IrIndentifierKind::Memory => self.script.push_str("%"),
            IrIndentifierKind::State => self.script.push_str("#"),

            IrIndentifierKind::Unknown => self.script.push_str("?"),
        }
        Ok(TraversalResult::SkipChildren)
    }
    fn visit_symbol_name(
        &mut self,
        mode: TreeTraversalMode,
        symbol: &mut IrIdentifier,
        _symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => {
                match symbol.kind {
                    IrIndentifierKind::FunctionName
                    | IrIndentifierKind::TemplateFunctionName
                    | IrIndentifierKind::StaticFunctionName
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
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        self.script.push_str("    ");
        let _ = enum_value.name.visit(self, symbol_table)?;

        if let Some(data) = &mut enum_value.data {
            self.script.push_str(" : ");
            let _ = data.visit(self, symbol_table)?;
        }

        self.script.push_str(",\n");
        Ok(TraversalResult::SkipChildren)
    }

    fn visit_tuple(
        &mut self,
        _mode: TreeTraversalMode,
        tuple: &mut Tuple,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        for field in tuple.fields.iter_mut() {
            self.script.push_str("    ");
            let _ = field.visit(self, symbol_table)?;
            self.script.push_str(",\n");
        }
        Ok(TraversalResult::SkipChildren)
    }

    fn visit_variant(
        &mut self,
        _mode: TreeTraversalMode,
        _variant: &mut Variant,
        _symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        // Pass through deliberate
        Ok(TraversalResult::Continue)
    }

    fn visit_variable_declaration(
        &mut self,
        _mode: TreeTraversalMode,
        var_dec: &mut VariableDeclaration,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        var_dec.name.visit(self, symbol_table)?;
        self.script.push_str(" : ");
        var_dec.typename.visit(self, symbol_table)?;
        Ok(TraversalResult::SkipChildren)
    }

    fn visit_operation(
        &mut self,
        _mode: TreeTraversalMode,
        operation: &mut Operation,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        match operation {
            Operation::TerminatingRef(identifier) => {
                self.script.push_str("; unused variable ");
                identifier.visit(self, symbol_table)?;
            }
            Operation::Noop => {
                self.script.push_str("noop");
            }
            Operation::Jump(identifier) => {
                self.script.push_str("jmp ");
                identifier.visit(self, symbol_table)?;
            }
            Operation::ConditionalJump {
                expression,
                on_success,
                on_failure,
            } => {
                self.script.push_str("jmp_if ");
                expression.visit(self, symbol_table)?;
                self.script.push_str(" ");
                on_success.visit(self, symbol_table)?;
                self.script.push_str(" ");
                on_failure.visit(self, symbol_table)?;
            }
            Operation::MemLoad => self.script.push_str("mload [TODO]"),
            Operation::MemStore => self.script.push_str("mstore [TODO]"),
            Operation::StateLoad { address } => {
                self.script.push_str("sload ");
                match &address.name.resolved {
                    Some(n) => self.script.push_str(&n),
                    None => self.script.push_str("UNRESOLVED"),
                }
            }
            Operation::StateStore {
                address: _,
                value: _,
            } => self.script.push_str("sstore [TODO]"),

            Operation::IsEqual { left, right } => {
                self.script.push_str("eq ");
                left.visit(self, symbol_table)?;
                self.script.push_str(" ");
                right.visit(self, symbol_table)?;
            }
            Operation::CallFunction { name, arguments }
            | Operation::CallExternalFunction { name, arguments } => {
                self.script.push_str("call ");
                name.visit(self, symbol_table)?;
                self.script.push_str("( ");
                for (i, arg) in arguments.iter_mut().enumerate() {
                    if i > 0 {
                        self.script.push_str(", ");
                    }
                    arg.visit(self, symbol_table)?;
                }
                self.script.push_str(" )");
            }

            Operation::CallStaticFunction {
                name,
                owner: _,
                arguments,
            } => {
                // TODO: Support for owner

                self.script.push_str("call ");
                name.visit(self, symbol_table)?;
                self.script.push_str("( ");

                for (i, arg) in arguments.iter_mut().enumerate() {
                    if i > 0 {
                        self.script.push_str(", ");
                    }
                    arg.visit(self, symbol_table)?;
                }
                self.script.push_str(" )");
            }
            Operation::CallMemberFunction {
                name: _,
                owner: _,
                arguments: _,
            } => unimplemented!(),
            Operation::ResolveSymbol { symbol } => {
                symbol.visit(self, symbol_table)?;
            }
            Operation::ResolveContextResource { symbol } => {
                symbol.visit(self, symbol_table)?;
            }

            Operation::Literal { data, typename } => {
                typename.visit(self, symbol_table)?;
                self.script.push_str(" ");
                self.script.push_str(&data);
            }
            Operation::PhiNode(arguments) => {
                self.script.push_str("phi [");
                for (i, arg) in arguments.iter_mut().enumerate() {
                    if i > 0 {
                        self.script.push_str(", ");
                    }
                    arg.visit(self, symbol_table)?;
                }
                self.script.push_str("]");
            }
            Operation::Return(arg) => {
                self.script.push_str("return");
                match arg {
                    Some(r) => {
                        self.script.push_str(" ");
                        r.visit(self, symbol_table)?;
                    }
                    &mut None => {
                        self.script.push_str(" void");
                    }
                };
            }
            Operation::Revert(arg) => {
                self.script.push_str("revert");
                match arg {
                    Some(r) => {
                        self.script.push_str(" ");
                        r.visit(self, symbol_table)?;
                    }
                    &mut None => todo!(),
                };
            }
        }
        Ok(TraversalResult::SkipChildren)
    }

    fn visit_instruction(
        &mut self,
        _mode: TreeTraversalMode,
        instr: &mut Instruction,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        self.script.push_str("    ");
        if let Some(name) = &mut instr.ssa_name {
            name.visit(self, symbol_table)?;
            self.script.push_str(" = ");
        }

        if let Some(rettype) = &mut instr.result_type {
            rettype.visit(self, symbol_table)?;
            self.script.push_str(" ");
        }

        let _ = instr.operation.visit(self, symbol_table)?;

        self.script.push_str("\n");

        Ok(TraversalResult::SkipChildren)
    }

    fn visit_function_block(
        &mut self,
        _mode: TreeTraversalMode,
        block: &mut FunctionBlock,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        block.name.visit(self, symbol_table)?;
        self.script.push_str(":");
        self.script.push_str("\n    ;; arguments:");
        for arg in &block.block_arguments {
            self.script.push_str(" ");
            self.script.push_str(&arg);
            self.script.push_str(",");
        }
        self.script.push_str("\n    ;; enters_from:");
        for arg in &block.enters_from {
            self.script.push_str(" ");
            self.script.push_str(&arg);
            self.script.push_str(",");
        }
        self.script.push_str("\n    ;; exits_to:");
        for arg in &block.exits_to {
            self.script.push_str(" ");
            self.script.push_str(&arg);
            self.script.push_str(",");
        }

        self.script.push_str("\n    ;; jump args:");
        for (name, args) in &block.jump_required_arguments {
            self.script.push_str("\n    ;;   * ");
            self.script.push_str(&name);
            for arg in args {
                self.script.push_str("\n    ;;        - ");
                self.script.push_str(&arg);
            }
        }

        self.script.push_str("\n");
        for instr in block.instructions.iter_mut() {
            instr.visit(self, symbol_table)?;
        }
        self.script.push_str("\n");
        Ok(TraversalResult::SkipChildren)
    }

    fn visit_function_body(
        &mut self,
        _mode: TreeTraversalMode,
        function_body: &mut FunctionBody,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        for block in &mut function_body.blocks {
            block.visit(self, symbol_table)?;
        }
        Ok(TraversalResult::SkipChildren)
    }

    fn visit_concrete_type(
        &mut self,
        _mode: TreeTraversalMode,
        con_type: &mut ConcreteType,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        match con_type {
            ConcreteType::Tuple {
                name,
                data_layout,
                namespace: _,
            } => {
                self.script.push_str("tuple ");
                let _ = name.visit(self, symbol_table)?;
                self.script.push_str(" = (\n");
                let _ = data_layout.visit(self, symbol_table)?;
                self.script.push_str(")\n");
            }
            ConcreteType::Variant {
                name,
                data_layout,
                namespace: _,
            } => {
                self.script.push_str("tagged_union ");
                let _ = name.visit(self, symbol_table)?;
                self.script.push_str(" = {\n");
                let _ = data_layout.visit(self, symbol_table)?;
                self.script.push_str("}\n");
            }
        }
        Ok(TraversalResult::SkipChildren)
    }

    fn visit_contract_field(
        &mut self,
        _mode: TreeTraversalMode,
        _function_kind: &mut ContractField,
        _symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        self.script
            .push_str(";; TODO: field emitter not implemented!\n");
        Ok(TraversalResult::Continue)
    }

    fn visit_function_kind(
        &mut self,
        _mode: TreeTraversalMode,
        function_kind: &mut FunctionKind,
        _symbol_table: &mut SymbolTable,
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
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        let _ = fnc.function_kind.visit(self, symbol_table)?;

        self.script.push_str(" ");
        let _ = fnc.name.visit(self, symbol_table)?;
        self.script.push_str("(");
        for (i, arg) in fnc.arguments.iter_mut().enumerate() {
            if i > 0 {
                self.script.push_str(", ");
            }
            let _ = arg.visit(self, symbol_table)?;
        }
        self.script.push_str(") : ");
        if let Some(rt) = &mut fnc.return_type {
            self.script.push_str("(TODO)");
            self.script.push_str(&rt);
            // rt.visit(self, symbol_table)
        } else {
            self.script.push_str("(untyped)");
        }
        self.script.push_str(" {\n");

        fnc.body.visit(self, symbol_table)?;
        self.script.push_str("}\n");
        Ok(TraversalResult::SkipChildren)
    }

    fn visit_primitives(
        &mut self,
        mode: TreeTraversalMode,
        _primitives: &mut IntermediateRepresentation,
        _symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => {
                // TODO: Emit scilla version etc
                unimplemented!()
            }
            TreeTraversalMode::Exit => {}
        }

        Ok(TraversalResult::Continue)
    }

    fn visit_case_clause(
        &mut self,
        _mode: TreeTraversalMode,
        _con_function: &mut CaseClause,
        _symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        unimplemented!()
    }
}

impl DebugPrinter {
    pub fn new() -> Self {
        DebugPrinter {
            script: "".to_string(),
        }
    }

    pub fn value(&self) -> String {
        self.script.clone()
    }
}
