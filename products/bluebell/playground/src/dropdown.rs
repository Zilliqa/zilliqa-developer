use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct DropdownProps {
    pub items: Vec<String>,
    pub on_item_click: Callback<usize>,
}

#[function_component(Dropdown)]
pub fn dropdown(props: &DropdownProps) -> Html {
    let DropdownProps {
        items,
        on_item_click,
    } = props;

    let is_open_state = use_state(|| false);

    let toggle_dropdown = {
        let is_open_state = is_open_state.clone();
        move |_event| {
            is_open_state.set(!*is_open_state);
        }
    };

    html! {
        <div class="w-full flex items-center justify-center py-2">
            <div class="z-10 flex flex-col ">
                <div class="bg-zinc-600 p-2  w-56  rounded border-gray-200 flex justify-between items-center cursor-pointer" onclick={toggle_dropdown.clone()}>
                    {"Examples"}
                    <svg class="-mr-1 h-5 w-5 text-gray-400" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                        <path fill-rule="evenodd" d="M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z" clip-rule="evenodd" />
                    </svg>
                </div>
                { if *is_open_state {
                    html! {
                        <div class="z-10 mt-2 w-56 max-h-56 rounded-md bg-white" role="menu" aria-orientation="vertical" aria-labelledby="menu-button" tabindex="-1">
                            <div class="py-1 h-24 overflow-y-auto" role="none">
                                { for items.iter().enumerate().map(|(i, item)| {
                                let item_click = {
                                    let orig_item_click = on_item_click.clone();
                                    let toggle_dropdown = toggle_dropdown.clone();
                                    move |e| {
                                        toggle_dropdown(e);
                                        orig_item_click.emit(i);
                                    }
                                };

                                    html! {
                                        <a href="#" class="cursor-pointer text-gray-700 block px-4 py-2 text-sm" role="menuitem" tabindex="-1" onclick={item_click}>
                                            { item }
                                        </a>
                                    }
                                })}
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }}
            </div>
        </div>
    }
}
