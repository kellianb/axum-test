use crate::components::message::message_display::MessageDisplay;
use dioxus::prelude::*;
use models::message_models::Message;

async fn fetch_messages() -> Result<Vec<Message>, reqwest::Error> {
    let response = reqwest::get("http://127.0.0.1:3000/messages").await?;
    let messages: Vec<Message> = response.json().await?;
    println!("{:?}", messages);
    Ok(messages)
}

#[component]
pub fn MessageListDisplay() -> Element {
    let messages = use_resource(move || fetch_messages());

    match &*messages.read_unchecked() {
        Some(Ok(msglist)) => {
            rsx!(
                div {
                    style: "
                        display: flex;
                        gap: 1.5rem;
                        flex-direction: column;
                        width: fit-content;
                    ",
                    for msg in msglist {
                        MessageDisplay {message: msg.clone()}
                    }
                }
            )
        }
        Some(Err(_)) => {
            rsx! {
                 div{
                    style: "
                        display: flex;
                        gap: 1.5rem;
                        flex-direction: column;
                        width: fit-content;
                    ",
                    "An error occurred while fetching messages"
                },

            }
        }
        None => {
            rsx! {
                 div{
                    style: "
                        display: flex;
                        gap: 1.5rem;
                        flex-direction: column;
                        width: fit-content;
                    ",
                    "Loading items"
                },
            }
        }
    }
}
