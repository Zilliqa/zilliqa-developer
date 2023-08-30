mod app;
mod state;
mod vm_remote;
mod bytecode_view;
mod machine_view;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
