mod app;
mod bytecode_view;
mod machine_view;
mod state;
mod vm_remote;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
