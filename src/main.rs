pub mod pages;

use pages::index::Index;

fn main() {
    yew::Renderer::<Index>::new().render();
}
