use crate::*;

pub fn Index(cx: Scope) -> Element {
    // Input
    let email = use_state(cx, || String::new());
    let password = use_state(cx, || String::new());

    // Signup
    let logging_in = use_state(cx, || false);
    let login = move |_| {
        cx.spawn({
            // Setup state
            let email = email.to_owned();
            let password = password.to_owned();
            let logging_in = logging_in.to_owned();
            let router = use_router(cx).clone();
            let set_user = use_set(cx, USER).clone();
            let set_settings = use_set(cx, SETTINGS).clone();
            
            async move {
                logging_in.set(true);
                let response = http::Client::new().login(&email, &password).await;
                logging_in.set(false);

                match response {
                    Ok(data) => match data.json::<SupabaseUser>().await {
                        Ok(user) => {
                            // Fetch settings
                            match supabase::settings(&user.user.id).await {
                                Ok(settings) => {
                                    let _ = LocalStorage::set("settings", &settings);
                                    set_settings(settings);   
                                },
                                Err(error) => log::error!("{error:?}"),
                            }

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
            "Login",
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
            class: if **logging_in {
                "button is-loading"
            } else {
                "button"
            },
            onclick: login,
            "Login",
        },
    })
}
