use crate::layout::{Layout, render_with_html_wrapper};
use clorinde::queries::users::User;
use dioxus::prelude::*;

#[component]
fn UserTable(users: Vec<User>) -> Element {
    rsx! {
        Layout {
            title: "Users Table",
            table {
                thead {
                    tr {
                        th { "ID" }
                        th { "Email" }
                    }
                }
                tbody {
                    for user in users {
                        tr {
                            td { "{user.id}" }
                            td { "{user.email}" }
                        }
                    }
                }
            },

            form {
                action: "/new_user",
                method: "POST",
                label { r#for: "user_email", "Email:" }
                input { id: "user_email", name: "email", r#type: "email", required: "true" }
                button { "Submit" }
            }            
        }
    }
}

pub fn index(users: Vec<User>) -> String {
    let body_content = dioxus_ssr::render_element(rsx! { UserTable { users } });
    render_with_html_wrapper("Users Table", &body_content)
}