use yew::Renderer;

mod app;
mod pages;
mod auth;
mod srv;

pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    Renderer::<app::App>::new().render();
}