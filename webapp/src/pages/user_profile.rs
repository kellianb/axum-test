use dioxus::prelude::*;
use models::user_models::User;

async fn fetch_user_data(id: i32) -> Result<User, reqwest::Error> {
    let request_url = format! {"http://127.0.0.1:3000/users/{id}"};
    let response = reqwest::get(request_url).await?;
    let messages: User = response.json().await?;
    println!("{:?}", messages);
    Ok(messages)
}

#[component]
pub fn UserProfilePage(id: i32) -> Element {
    let user = use_resource(move || fetch_user_data(id));

    match &*user.read_unchecked() {
        Some(Ok(user)) => {
            rsx! {
                div {
                    style: "
                    display: flex;
                    gap: 1rem;
                    align-items: center;
                    ",
                    h1 {
                        style: "
                        font-weight: 700;
                        font-size: 2rem;
                        margin: 1rem 0rem 1rem 0rem;
                        ",
                        "{user.username}"
                    },
                    p {
                        style: "
                        font-weight: 500;
                        font-size: 1rem;
                        background-color: lightgrey;
                        color: grey;
                        padding: 0.5rem;
                        border-radius: 0.7rem;
                        ",
                        "{user.role}"
                    }
                }
                p {"joined at {user.created_at}"}
            }
        }
        Some(Err(_)) => {
            rsx! {"Error loading user: {id}"}
        }
        None => {
            rsx! {"Loading user: {id}"}
        }
    }
}
