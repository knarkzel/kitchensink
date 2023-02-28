use crate::*;

pub fn Index(cx: Scope) -> Element {
    // State
    let saving = use_state(cx, || false);
    let settings = use_read(cx, SETTINGS);
    let set_settings = use_set(cx, SETTINGS);
    
    cx.render(rsx! {
        h1 {
            class: "mt-0",
            "Settings",
        },
        div {
            class: "box p-4",
            h2 {
                id: "feeds mt-0",
                "Feeds",
            },
            textarea {
                class: "textarea",
                value: "{settings.feeds}",
                placeholder: "RSS or Atom feeds",
                oninput: move |event| {
                    let settings = Settings {
                        feeds: event.value.clone(),
                        ..*settings
                    };
                    set_settings(settings);
                },
            },
            button {
                class: if **saving {
                    "button mt-2 is-loading"
                } else {
                    "button mt-2"
                },
                onclick: move |_| {
                    cx.spawn({
                        // Setup state
                        let saving = saving.to_owned();
                        let user = use_read(cx, USER).to_owned();
                        let query = json!({
                            "user_id": user.as_ref().unwrap().user.id,
                            "feeds": settings.feeds,
                        });
                        let _ = LocalStorage::set("settings", settings);
                        
                        async move {
                            saving.set(true);
                            if let Err(error) = supabase::save(query.to_string()).await {
                                log::error!("{error:?}");
                            } else {
                            }
                            saving.set(false);
                        }
                    });
                },
                "Save",
            },
        }
    })
}
