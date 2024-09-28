use dioxus::prelude::*;
mod message;
mod message_list;
mod navbar;

fn main() {
    launch(app);
}

pub fn app() -> Element {
    rsx! {
        div {
            navbar::Navbar {}
        }

        "story"
    }
}
