use crate::*;
use dioxus_router::use_router;

pub fn Index(cx: Scope) -> Element {
    // Form
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
            let set_user = use_set(cx, USER).clone();
            let router = use_router(cx).clone();
            
            async move {
                output.set(String::new());
                signing_up.set(true);
                let response = Client::new().signup(&email, &password).await;
                signing_up.set(false);
                
                match response {
                    Ok(data) => match data.json::<SupabaseUser>().await {
                        Ok(user) => {
                            router.navigate_to("/");
                            set_user(Some(user));                            
                        },
                        Err(error) => output.set(format!("{error:#?}")),
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
                autocomplete: "email",
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
                },
            },
        },
        if **email_valid && **password_valid {
            if **signing_up {
                rsx! {
                    button {
                        class: "button is-loading",
                        "Sign up",
                    },
                }
            } else {
                rsx! {
                    button {
                        onclick: signup,
                        class: "button",
                        "Sign up",
                    },
                }
            }
        } else {
            rsx! {
                button {
                    disabled: "true",
                    class: "button",
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
