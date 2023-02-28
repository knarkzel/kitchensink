use crate::*;

pub fn Index(cx: Scope) -> Element {
    // Form
    let email = use_state(cx, || String::new());
    let password = use_state(cx, || String::new());

    // Signup
    let signing_up = use_state(cx, || false);
    let signup = move |_| {
        cx.spawn({
            // Setup state
            let email = email.to_owned();
            let password = password.to_owned();
            let signing_up = signing_up.to_owned();
            let router = use_router(cx).clone();
            let set_user = use_set(cx, USER).clone();

            async move {
                signing_up.set(true);
                let response = http::Client::new().signup(&email, &password).await;
                signing_up.set(false);

                match response {
                    Ok(data) => match data.json::<SupabaseUser>().await {
                        Ok(user) => {
                            let _ = LocalStorage::set("user", &user);
                            set_user(Some(user));
                            router.navigate_to("/");
                        }
                        Err(error) => log::error!("{error:?}"),
                    },
                    Err(error) => log::error!("{error:?}"),
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
                oninput: move |event| email.set(event.value.clone()),
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
                oninput: move |event| password.set(event.value.clone()),
            },
        },
        button {
            class: if **signing_up {
                "button is-loading"
            } else {
                "button"
            },
            onclick: signup,
            "Login",
        },
    })
}
