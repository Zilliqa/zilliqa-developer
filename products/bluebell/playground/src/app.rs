use gloo_console as console;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(Tabs)]
pub fn tabs(props: &Props) -> Html {
    let Props {
        selected_tab,
        on_tab_selected,
        tabs,
    } = props;

    html! {
        <div class="border-b border-gray-200">
            <nav class="-mb-px flex">
                { for tabs.iter().enumerate().map(|(index, tab)| {
                    let is_selected = *selected_tab == index;
                    let button_class = if is_selected {
                        "border-indigo-500 text-indigo-600"
                    } else {
                        "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300"
                    };

                    html! {
                        <button
                            class={"inline-flex items-center px-4 py-2 border-b-2 font-medium".to_owned() + button_class}
                            onclick={on_tab_selected.reform(move |_| index)}
                        >
                            { tab }
                        </button>
                    }
                }) }
            </nav>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub selected_tab: usize,
    pub on_tab_selected: Callback<usize>,
    pub tabs: Vec<String>,
}

#[function_component(App)]
pub fn app() -> Html {
    let source_code = use_state(|| String::new());
    let bytecode: Vec<String> = vec!["PUSH1 0x60".into(), "PUSH1 0x40".into()];
    let stack: Vec<String> = vec![];
    let instruction: String = "PUSH1 0x60".into();
    let memory: Vec<String> = vec![];

    let selected_tab_state = use_state(|| 0); // Default to the first tab
    let selected_tab = *selected_tab_state;

    let compile_button_click = Callback::from(move |_| {
        console::log!("Compile Button clicked");
    });

    let step_button_click = Callback::from(move |_| {
        console::log!("Step Button clicked");
    });

    let run_button_click = Callback::from(move |_| {
        console::log!("Run Button clicked");
    });

    let handle_tab_selected = {
        let selected_tab_state = selected_tab_state.clone();
        Callback::from(move |index: usize| {
            selected_tab_state.set(index);
        })
    };

    let target_input_value = |e: &Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        input.value()
    };

    let handle_source_code_change = {
        let source_code = source_code.clone();
        Callback::from(move |e: InputEvent| {
            let value = target_input_value(&e);
            console::log!(value.clone());
            source_code.set(value);
        })
    };

    let source_code: String = source_code.to_string();
    html! {
        <div class="container mx-auto mt-12 p-6 bg-gray-50 rounded-lg shadow-md">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-8">

                <div class="space-y-6">
                    <h1 class="text-2xl font-bold text-gray-800">{"Scilla Compiler"}</h1>

                    <textarea
                        id="source-code"
                        class="w-full px-3 py-2 border rounded-md focus:ring focus:ring-indigo-200 transition-shadow duration-150"
                        rows="20"
                        value={source_code}
                        oninput={handle_source_code_change}
                        placeholder="Enter Scilla source code here..."
                    />

                    <div>
                        <button class="bg-indigo-600 text-white font-medium px-6 py-2 rounded-lg hover:bg-indigo-700" onclick={compile_button_click.clone()}>
                            {"Compile"}
                        </button>
                    </div>
                </div>

                <div class="space-y-6">
                    <Tabs
                        selected_tab={selected_tab.clone()}
                        on_tab_selected={handle_tab_selected}
                        tabs={vec!["Execute".to_string(), "Intermediate Representation".to_string()]}
                    />

                    { if selected_tab == 0 {
                        html! {
                            <div class="space-y-6">
                                <div class="flex justify-between">
                                    <div class="space-x-4">
                                        <button class="bg-blue-500 text-white font-medium px-5 py-2 rounded-lg hover:bg-blue-600" onclick={step_button_click.clone()}>
                                            {"Step"}
                                        </button>
                                        <button class="bg-green-500 text-white font-medium px-5 py-2 rounded-lg hover:bg-green-600" onclick={run_button_click.clone()}>
                                            {"Run"}
                                        </button>
                                    </div>
                                    <label class="text-gray-700 font-semibold" for="source-code">
                                        {"Source Code"}
                                    </label>
                                </div>
                                <div>
                                    <h2 class="text-lg font-semibold mb-3">{"EVM Bytecode"}</h2>
                                    <div class="p-4 bg-white border rounded-md shadow-sm space-y-2">
                                        { for bytecode.iter().map(|instr| html! {
                                            <div class="flex items-center space-x-4">
                                                <input type="checkbox" class="form-checkbox h-5 w-5 text-blue-600" />
                                                <div>{instr}</div>
                                            </div>
                                        }) }
                                    </div>
                                </div>
                                <div>
                                    <h2 class="text-lg font-semibold mb-3">{"Stack"}</h2>
                                    <div class="p-4 bg-white border rounded-md shadow-sm space-y-2 overflow-y-auto h-48">
                                        { for stack.iter().map(|s| html! { <div>{s}</div> }) }
                                    </div>
                                </div>
                                <div>
                                    <h2 class="text-lg font-semibold mb-3">{"Current Instruction"}</h2>
                                    <div class="p-4 bg-white border rounded-md shadow-sm">
                                        { &instruction }
                                    </div>
                                </div>
                                <div>
                                    <h2 class="text-lg font-semibold mb-3">{"Memory"}</h2>
                                    <div class="p-4 bg-white border rounded-md shadow-sm space-y-2">
                                        { for memory.iter().map(|mem| html! { <div>{mem}</div> }) }
                                    </div>
                                </div>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="space-y-6">
                                // "Intermediate Representation" tab content
                            </div>
                        }
                    } }
                </div>
            </div>
        </div>
    }
}
