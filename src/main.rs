mod components;
mod helpers;

fn main() {
    yew::Renderer::<components::app::App>::new().render();
}
