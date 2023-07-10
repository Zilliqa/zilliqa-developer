use inkwell::context::Context;
use inkwell::module::Module;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;


struct LlvmExecutable<'ctx> {
    context: &'ctx mut Context,
    module:  Module<'ctx>,
}

impl<'ctx> LlvmExecutable<'ctx> {

    pub fn new(name: &str, context: &'ctx mut Context) -> Self {
        let module = context.create_module(name);
       
        Self {
            context,
            module 
        }
    }

}
