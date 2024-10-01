use crate::components::message::message_list::MessageListDisplay;
use dioxus::prelude::*;

#[component]
pub fn ChatPage() -> Element {
    rsx! {
        div {
            style: "
                width: fit-content;
            ",
            h1 {
                style: "
                        font-weight: 700;
                        font-size: 2rem;
                        margin: 1rem 0rem 1rem 0rem;
                        width: fit-content;
                    ",
                "Message list"
            },
            MessageListDisplay {}
        }
    }
}
