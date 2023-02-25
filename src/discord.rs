use crate::*;

pub fn Index(cx: Scope) -> Element {
    cx.render(rsx! {
        p {
            "You haven't stored a token yet, ",
            a {
                target: "_blank",
                href: "https://discord.com/developers/applications",
                "create a new application and enter it below",
            }
            ":",
        },
        div {
            class: "is-flex",
            input {
                class: "input",
                "type": "password",
                placeholder: "OAuth2 token",
            },
            button {
                class: "button is-link ml-2",
                "Store token",
            },            
        },
    })
}
