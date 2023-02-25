use crate::*;

pub fn Index(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 {
            class: "m-0",
            "Login",
        },
        div {
            class: "field mt-4",
            label {
                class: "label",
                "Email",
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
        label {
            class: "checkbox is-block",
            input {
                "type": "checkbox",
            },
            " Stay logged in",
        },
        button {
            class: "button is-link mt-2",
            "Login",
        },
    })
}
