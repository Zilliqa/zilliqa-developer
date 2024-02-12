use std::{cell::RefCell, rc::Rc};

use evm_assembly::{executable::EvmExecutable, instruction::RustPosition};
use gloo_timers::callback::Timeout;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::Dispatch;

use crate::state::{State, StateMessage};

pub struct ByteCodeViewInstruction {
    pub label: Option<String>,
    pub bytecode_position: u32,
    pub hex_bytecode_position: String,
    pub text: String,
    pub comment: String,
    pub rust_position: Option<RustPosition>,
}

pub struct ByteCodeView {
    props: ByteCodeViewProps,
    // selected_tab: usize,
    instructions: Vec<ByteCodeViewInstruction>,
    timeout: Option<Timeout>,
}
pub enum ByteCodeViewMessage {
    // SelectTab(usize),
    SetInstructions(Vec<ByteCodeViewInstruction>),
    ScrollToPosition(usize),
}
#[derive(Properties)]
pub struct ByteCodeViewProps {
    pub executable: Rc<RefCell<EvmExecutable>>,
    pub data: String,
    pub program_counter: u32,
}

impl Clone for ByteCodeViewProps {
    fn clone(&self) -> Self {
        Self {
            executable: Rc::clone(&self.executable),
            data: self.data.clone(),
            program_counter: self.program_counter,
        }
    }
}

impl PartialEq for ByteCodeViewProps {
    fn eq(&self, other: &Self) -> bool {
        // Logic to compare two ByteCodeViewProps.
        // If you only need reference equality for the Rc:
        Rc::ptr_eq(&self.executable, &other.executable)
            && self.data == other.data
            && self.program_counter == other.program_counter
    }
}

impl Component for ByteCodeView {
    type Message = ByteCodeViewMessage;
    type Properties = ByteCodeViewProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props().clone();

        let mut ret = Self {
            props: props.clone(),
            // selected_tab: 0,
            instructions: Vec::new(),
            timeout: None,
        };
        Component::changed(&mut ret, ctx, &props);
        ret
    }

    fn changed(&mut self, ctx: &Context<Self>, props: &ByteCodeViewProps) -> bool {
        let functions = (*props.executable).clone().into_inner().ir.functions;

        let mut instructions: Vec<ByteCodeViewInstruction> = Vec::new();
        for function in &functions {
            for block in &function.blocks {
                let mut next_label = Some(block.name.clone());

                for instr in &block.instructions {
                    let bytecode_position = match instr.position {
                        Some(v) => v,
                        None => 0,
                    };

                    let instruction_value = if instr.arguments.len() > 0 {
                        let argument: String = instr
                            .arguments
                            .iter()
                            .map(|byte| format!("{:02x}", byte).to_string())
                            .collect();

                        format!("{} 0x{}", instr.opcode.to_string(), argument).to_string()
                    } else {
                        instr.opcode.to_string()
                    };

                    let next_instr = ByteCodeViewInstruction {
                        label: next_label,
                        bytecode_position,
                        hex_bytecode_position: format!("0x{:02x}", bytecode_position).to_string(),
                        text: instruction_value,
                        comment: instr.comment.clone().unwrap_or("".to_string()),
                        rust_position: instr.rust_position.clone(),
                    };
                    instructions.push(next_instr);
                    next_label = None;
                }
            }
        }
        self.props = ctx.props().clone();
        ctx.link()
            .send_message(ByteCodeViewMessage::SetInstructions(instructions));
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            /*
            ByteCodeViewMessage::SelectTab(index) => {
                self.selected_tab = index;
                true
            }
            */
            ByteCodeViewMessage::SetInstructions(instructions) => {
                self.instructions = instructions;
                true
            }
            ByteCodeViewMessage::ScrollToPosition(index) => {
                // Set up a new timeout to scroll to the element after a slight delay
                let id = format!("instr-{}", index);
                self.timeout.take();
                self.timeout = Some(Timeout::new(100, move || {
                    if let Some(element) = web_sys::window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .get_element_by_id(&id)
                    {
                        element.scroll_into_view();
                    }
                }));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let program_counter = self.props.program_counter;

        html! {
                <>
                    <h2 class="text-2xl ">{"EVM Bytecode"}</h2>
                    <div class="px-2 bg-white overflow-x-auto space-y-1">
                        {
                            for self.instructions.iter().enumerate().map(|(index, instr)| {
                                let id = format!("instr-{}", index+5);
                                if program_counter == instr.bytecode_position {
                                    ctx.link().send_message(ByteCodeViewMessage::ScrollToPosition(index));
                                }

                                let instruction_position =  instr.bytecode_position ;
                                html! {
                                    <>
                                        {
                                            if let Some(label) = &instr.label {
                                                html! {
                                                    <div class="pt-6 font-medium flex items-center space-x-4 text-sm text-gray-700">
                                                    {label}
                                                    </div>
                                                }
                                            } else {
                                                html!{}
                                            }
                                        }


                                        <div class={
                                          if program_counter == instruction_position {
                                              "p-2 rounded flex  items-start justify-start space-x-4 text-sm text-gray-700 bg-gray-300 whitespace-nowrap"
                                          }
                                          else
                                          {
                                             "p-2 rounded  flex  items-start justify-start space-x-4 text-sm text-gray-700 whitespace-nowrap"
                                          }
                                        } id={id}> // Assign the unique id here
                                            <input type="checkbox" class="flex-0 form-checkbox min-h-5 min-w-5 h-5 w-5 text-blue-600 rounded"
                                                   oninput={move |e:InputEvent| {
                                                        let value = e.target_unchecked_into::<HtmlInputElement>().checked();

                                                        let dispatch = Dispatch::<State>::new();
                                                        if value {
                                                            dispatch.apply(StateMessage::AddBreakPoint(instruction_position));
                                                        } else {
                                                            dispatch.apply(StateMessage::RemoveBreakPoint(instruction_position));
                                                        }
                                                    }}

                                             />
                                            <div class="h-5">{instr.hex_bytecode_position.clone()}</div>
                                            <div class="flex flex-col overflow-x-auto">
                                                <div>{instr.text.clone()}</div>
                                            {
                                                if instr.comment.len() > 0 {
                                                    html! {
                                                        <div class="font-mono text-xs text-gray-400 text-sm text-gray-700 whitespace-nowrap">
                                                            {instr.comment.clone()}
                                                        </div>
                                                    }
                                                } else {
                                                    html!{}
                                                }
                                            }{
                                                if let Some(rp) = &instr.rust_position {
                                                    html! {
                                                        <div class="font-mono text-xs text-gray-400 text-sm text-gray-700 whitespace-nowrap">
                                                            {rp.filename.clone()}{":"}{rp.line}
                                                        </div>
                                                    }
                                                } else {
                                                    html!{}
                                                }
                                            }
                                                </div>
                                        </div>
                                    </>
                                }
                            })
                        }
                    </div>
                </>
        }
    }
}
