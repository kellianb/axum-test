use dioxus::prelude::*;
use models::user_models::User;

async fn fetch_user_data(id: i32) -> Result<User, reqwest::Error> {
    let request_url = format! {"http://127.0.0.1:3000/users/{id}"};
    let response = reqwest::get(request_url).await?;
    let user_info: User = response.json().await?;
    println!("{:?}", user_info);
    Ok(user_info)
}

#[component]
pub fn UserInfoCardDisplay(id: i32) -> Element {
    let user = use_resource(move || fetch_user_data(id));

    rsx! {
        a {
            style: "
            font-weight: 400;
            color: gray;
            text-decoration: none;
            ",
            href: "/u/{id}",
            match &*user.read_unchecked() {
                Some(Ok(user)) => {
                    rsx!(
                        div {
                            style: "
                            display: flex;
                            gap: 0.5rem;
                            align-items: center;
                            ",
                            "{user.username}"
                            if user.role != "user" {
                                p {
                                    style: "
                                    font-weight: 500;
                                    font-size: 0.7rem;
                                    background-color: lightgrey;
                                    color: grey;
                                    padding: 0.3rem 0.5rem 0.3rem 0.5rem;
                                    border-radius: 0.7rem;
                                    ",
                                    "{user.role}"
                                }
                            }
                        }
                    )
                }
                Some(Err(_)) => {
                    rsx! {"Error loading user: {id}"}
                }
                None => {
                    rsx! {"Loading user: {id}"}
                }
            }
        },
    }
}
