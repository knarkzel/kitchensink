use crate::*;

pub fn Index(cx: Scope) -> Element {
    // Input
    let feeds = use_state(cx, || String::new());
    let saving = use_state(cx, || false);
    
    // User
    let user = use_read(cx, USER);

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
                value: "{feeds}",
                placeholder: "RSS or Atom feeds",
                oninput: move |event| feeds.set(event.value.clone()),
            },
            button {
                class: if **saving {
                    "button mt-2 is-loading"
                } else {
                    "button mt-2"
                },
                onclick: move |_| {
                    cx.spawn({
                        // Setup query
                        let client = Supabase::new();
                        let query = json!({
                            "user_id": user.as_ref().unwrap().user.id,
                            "feeds": **feeds,
                        });
                        let saving = saving.to_owned();
                        
                        async move {
                            saving.set(true);
                            let _ = client.from("settings").upsert(query.to_string()).on_conflict("user_id").execute().await;
                            saving.set(false);
                        }
                    });
                },
                "Save",
            },
        }
    })
}
