use crate::*;

pub fn Index(cx: Scope) -> Element {
    // Hooks
    let email = use_state(cx, || String::new());
    let password = use_state(cx, || String::new());
    let email_valid = use_state(cx, || false);
    let password_valid = use_state(cx, || false);
    
    cx.render(rsx! {
        h1 {
            class: "m-0",
            "Sign up",
        },
        div {
            class: "field mt-4",
            label {
                class: "label",
                "Email",
            },
            input {
                class: "input",
                "type": "email",
                value: "{email}",
                oninput: move |event| {
                    email_valid.set(event.value.contains("@"));
                    email.set(event.value.clone());
                },
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
                value: "{password}",
                oninput: move |event| {
                    password_valid.set(event.value.len() >= 8);
                    password.set(event.value.clone());   
                }
            },
        },
        if **email_valid && **password_valid {
            rsx! {
                button {
                    class: "button is-link",
                    "Sign up",
                },
            }
        } else {
            rsx! {
                button {
                    disabled: "true",
                    class: "button is-link",
                    "Sign up",
                },                
            }
        }
    })
}
