use crate::*;

pub fn Index(cx: Scope) -> Element {
    // Input
    let email = use_state(cx, || String::new());
    let password = use_state(cx, || String::new());
    let create_account = use_state(cx, || false);
    
    // Authenticate
    let loading = use_state(cx, || false);
    let login = move |_| {
        cx.spawn({
            // Setup state
            let email = email.to_owned();
            let password = password.to_owned();
            let loading = loading.to_owned();
            let create_account = create_account.to_owned();
            let router = use_router(cx).clone();
            let set_user = use_set(cx, USER).clone();
            let set_settings = use_set(cx, SETTINGS).clone();
            
            async move {
                loading.set(true);
                let response = if *create_account {
                    http::Client::new().signup(&email, &password).await
                } else {
                    http::Client::new().login(&email, &password).await
                };

                match response {
                    Ok(data) => match data.json::<SupabaseUser>().await {
                        Ok(user) => {
                            // Fetch settings
                            if !*create_account {
                                match supabase::settings(&user.user.id).await {
                                    Ok(settings) => {
                                        let _ = LocalStorage::set("settings", &settings);
                                        set_settings(settings);   
                                    },
                                    Err(error) => log::error!("{error:?}"),
                                }
                            }
                            loading.set(false);
                            router.navigate_to("/");
                            let _ = LocalStorage::set("user", &user);
                            set_user(Some(user));
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
            "Account",
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
        label {
            class: "checkbox is-block mb-2",
            input {
                class: "mr-1",
                onclick: |_| create_account.set(!**create_account),
                value: "{create_account}",
                "type": "checkbox",
            },
            "Create account?",
        }
        button {
            class: if **loading {
                "button is-loading"
            } else {
                "button"
            },
            onclick: login,
            if **create_account {
                "Sign up"
            } else {
                "Login"
            }
        },
    })
}
