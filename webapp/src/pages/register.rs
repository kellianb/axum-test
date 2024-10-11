use dioxus::prelude::*;
use models::user_models::{CreateUser, NewUser};
use serde_json::json;

pub async fn send_create_user_data(
    create_user_data: CreateUser,
) -> Result<NewUser, reqwest::Error> {
    let request_url = format! {"http://127.0.0.1:3000/users"};

    let client = reqwest::Client::new();
    client
        .post(request_url)
        .json(&create_user_data)
        .send()
        .await?
        .json()
        .await
}

#[component]
pub fn RegisterPage() -> Element {
    rsx! {
        div {
            style: "
                width: 100%;
                min-height: 100vh;
                display: flex;
                justify-content: center;
                align-items: center;
                gap: 2rem;
            ",
            div {
                style: "
                    display: flex;
                    flex-direction: column;
                    gap: 1rem;
                ",
                h1 {
                    style: "
                        font-size: 4rem;
                        font-weight: 600;
                    ",
                    "Register"
                },
                h2 {
                    style: "
                        color: #4F4F4F;
                        font-size: 1.5rem;
                        font-weight: 400;
                    ",
                    "Register to gain access to chats"
                    }
            }
            form {
                onsubmit: move |event| {
                    event.stop_propagation();
                    if let Ok(data) = event.data().parsed_values::<CreateUser>() {
                        use_resource(move || send_create_user_data(data.clone()));
                    }
                },
                style: "
                    display: flex;
                    flex-direction: column;
                    gap: 1rem;
                    min-width: 20rem;
                ",
                input {
                style: "
                    min-height: 1rem;
                    background-color: white;
                    box-sizing: border-box;
                    width: 100%;
                    min-height: 3rem;
                    border-radius: 9px;
                    border-style: solid;
                    border-color: #D9D9D9;
                    padding: 0 5% 0 5%;
                    font-size: clamp(0.8rem, 5vw, 1rem);
                    color: #777777;
                    border-width: 2px;
                ",
                placeholder: "Username"
                },
                input {
                style: "
                    background-color: white;
                    box-sizing: border-box;
                    width: 100%;
                    min-height: 3rem;
                    border-radius: 9px;
                    border-style: solid;
                    border-color: #D9D9D9;
                    padding: 0 5% 0 5%;
                    font-size: clamp(0.8rem, 5vw, 1rem);
                    color: #777777;
                    border-width: 2px;
                ",
                placeholder: "Password"
                },
                input {
                style: "
                    background-color: white;
                    box-sizing: border-box;
                    width: 100%;
                    min-height: 3rem;
                    border-radius: 9px;
                    border-style: solid;
                    border-color: #D9D9D9;
                    padding: 0 5% 0 5%;
                    font-size: clamp(0.8rem, 5vw, 1rem);
                    color: #777777;
                    border-width: 2px;
                ",
                placeholder: "Role"
                },
                button {
                    r#type: "submit",
                    value: "Submit",
                    style: "
                        background-color: white;
                        box-sizing: border-box;
                        width: 100%;
                        min-height: 3rem;
                        border-radius: 9px;
                        border-style: solid;
                        padding: 0 5% 0 5%;
                        margin-top: 0.9rem;
                        background-color: black;
                        color: white;
                        font-weight: bold;
                        font-size: 1.56rem;
                        min-height: 3rem;
                    ",
                    "Register",
                }
            }
        }
    }
}
