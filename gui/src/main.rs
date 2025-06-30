mod app;
mod pages;
mod auth;

use yew::Renderer;

pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    Renderer::<app::App>::new().render();
}