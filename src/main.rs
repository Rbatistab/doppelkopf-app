use yew::Renderer;
use app::App;

mod app;
mod components;
mod pages;
mod services;

fn main() {
    Renderer::<App>::new().render();
}
