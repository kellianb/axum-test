use dioxus::prelude::*;
mod navbar;

fn main() {
    launch(App);
}

pub fn App() -> Element {
    rsx! {
        div {
            navbar::Navbar {}
        }
        "story"
    }
}
