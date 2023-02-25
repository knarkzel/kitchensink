use crate::*;

pub fn Index(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 {
            class: "mt-0",
            "Discord",
        },
        p {
            "You haven't stored a token yet, ",
            a {
                target: "_blank",
                href: "https://discord.com/developers/applications",
                "create a new application and insert token below",
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
