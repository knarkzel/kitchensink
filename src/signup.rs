use crate::*;

pub fn Index(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 {
            class: "m-0",
            "Sign up",
        },
        div {
            class: "field mt-4",
            label {
                class: "label",
                "Username",
            },
            input {
                class: "input",
                "type": "text",
            },            
        },
        div {
            class: "field",
            label {
                class: "label",
                "Password",
            },
            input {
                class: "input",
                "type": "password",
            },            
        },
        button {
            class: "button is-link",
            "Sign up",
        },
    })
}
