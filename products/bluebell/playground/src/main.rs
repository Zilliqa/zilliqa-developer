mod app;
mod state;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
