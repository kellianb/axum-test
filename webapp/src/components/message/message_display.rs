use crate::components::user::user_info_card::UserInfoCardDisplay;
use dioxus::prelude::*;
use models::message_models::Message;

#[component]
pub fn MessageDisplay(message: Message) -> Element {
    rsx! {
        div {
            style: "
            width: fit-content;
            display: flex;
            flex-direction: column;
            gap: 0.2rem;
            ",
            div {
                style: "
                background-color: lightgray;
                border-radius: 0.2rem 1rem 1rem 1rem;
                width: fit-content;
                padding: 1rem;
                display: flex;
                gap: 0.7rem;
                align-items: center;
                ",
                p {
                    // style: "margin: 1rem 0rem 1rem 0rem;",
                    "{message.content}"
                },
                p {
                    style: "
                    font-size: 0.7rem;
                    font-style: italic;
                    color: gray;
                    ",
                    "{message.sent_at}"
                },
            }
            UserInfoCardDisplay {id: message.sender_id},
        }
    }
}
