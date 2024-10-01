use dioxus::prelude::*;

#[component]
pub fn LoginPage() -> Element {
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
                    "Login"
                },
                h2 {
                    style: "
                        color: #4F4F4F;
                        font-size: 1.5rem;
                        font-weight: 400;
                    ",
                    "Authenticate to view and send messages"
                    }
            }
            form {
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
                placeholder: "Email"
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
                }
                button {
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
                    "Login",
                }
            }
        }
    }
}
