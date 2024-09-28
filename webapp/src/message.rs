use dioxus::prelude::*;
use models::message_models::Message;

#[component]
pub fn message(message: Message) -> Element {
    rsx! {
        div {
            "{message.content}"
        }
    }
}
