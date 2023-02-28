use std::io::Cursor;

use crate::*;
use feed_rs::parser;
use futures::future;

pub fn Index(cx: Scope) -> Element {
    // State
    let settings = use_read(cx, SETTINGS);

    // Fetch all feeds and parse them
    let parsed_feeds = use_future(cx, (), |_| {
        let feeds = settings.feeds.to_owned();

        async move {
            let client = http::Client::new();
            let xmls = future::try_join_all(
                feeds
                    .split("\n")
                    .map(str::trim)
                    .filter(|it| it.len() > 0)
                    .map(|url| client.fetch(url)),
            )
            .await;
            match xmls {
                Ok(xmls) => Ok(xmls
                    .iter()
                    .flat_map(|xml| {
                        let buffer = Cursor::new(xml);
                        parser::parse(buffer)
                    })
                    .collect::<Vec<_>>()),
                Err(error) => Err(error),
            }
        }
    });

    cx.render(rsx! {
        h1 {
            class: "mt-0",
            "Feeds",
        },
        match parsed_feeds.value() {
            Some(Ok(feeds)) => rsx! {
                if feeds.is_empty() {
                    rsx! { "Your feed is empty." }
                } else {
                    rsx! {
                        feeds.iter().map(|feed| rsx! {
                            h1 {
                                "{feed.title:?}"
                            }
                        })
                    }
                }
            },
            Some(Err(error)) => rsx! { pre { "{error:#?}" } },
            None => rsx! { "Loading feeds..." },
        }
    })
}
