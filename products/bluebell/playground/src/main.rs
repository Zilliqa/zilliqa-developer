mod app;
mod bytecode_view;
mod machine_view;
mod state;
mod vm_remote;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
