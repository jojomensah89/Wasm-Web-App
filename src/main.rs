use yew::Renderer;
use crate::app::App;

mod app;
mod pages;
mod router;

fn main() {
    Renderer::<App>::new().render();
}
