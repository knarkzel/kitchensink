use crate::*;

pub fn Index(cx: Scope) -> Element {
    // Hooks
    let email = use_state(cx, || String::new());
    let password = use_state(cx, || String::new());
    let email_valid = use_state(cx, || false);
    let password_valid = use_state(cx, || false);

    // Signup
    let signing_up = use_state(cx, || false);
    let output = use_state(cx, || String::new());
    let signup = move |_| {
        cx.spawn({
            let email = email.to_owned();
            let password = password.to_owned();
            let signing_up = signing_up.to_owned();
            let output = output.to_owned();
            
            async move {
                output.set(String::new());
                signing_up.set(true);
                let response = Client::new().signup(&email, &password).await;
                signing_up.set(false);
                
                match response {
                    Ok(data) => if let Ok(body) = data.text().await {
                        output.set(body);
                    }
                    Err(error) => output.set(format!("{error:#?}")),
                }
            }
        });
    };
    
    cx.render(rsx! {
        h1 {
            class: "mt-0",
            "Sign up",
        },
        div {
            class: "field",
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
            if **signing_up {
                rsx! {
                    button {
                        class: "button is-link is-loading",
                        "Sign up",
                    },
                }
            } else {
                rsx! {
                    button {
                        onclick: signup,
                        class: "button is-link",
                        "Sign up",
                    },
                }
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
        if output.len() > 0 {
            rsx! {
                pre {
                    class: "mt-4",
                    "{output}",
                },
            }
        }
    })
}
