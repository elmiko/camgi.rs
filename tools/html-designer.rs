// Copyright (C) 2022 Red Hat
// SPDX-License-Identifier: GPL-3.0-or-later

/// This script can be used to develop the html output:
/// - Spawn a cargo watch command to rebuild the example.
/// - Spawn an http service to serve the example.
/// - Watch for file change, and notify web client to reload on change.
/// - Spawn xdg-open warp url.
///
/// The code is mostly adapted from <https://github.com/seanmonstar/warp/blob/master/examples/sse_chat.rs>
use notify::{RecursiveMode, Watcher};
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use warp::Filter;

type SharedFile = Arc<Mutex<String>>;
type Users = Arc<Mutex<HashMap<usize, tokio::sync::mpsc::UnboundedSender<()>>>>;

static NEXT_USER_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
const PATH: &str = "./target/html/index.html";

use warp::sse::Event;
use warp::Stream;

fn event_stream(users: Users) -> impl Stream<Item = Result<Event, Infallible>> + Send + 'static {
    use futures_util::StreamExt;
    // Create a channel to receive reload event
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    let rx = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);

    // Register the channel sender
    let my_id = NEXT_USER_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    users.lock().unwrap().insert(my_id, tx);

    // Map notify events to sse event
    rx.map(|()| Ok(Event::default().event("reload").data("data")))
}

#[tokio::main]
async fn main() {
    // A shared String for the index.html file.
    let html_content: SharedFile = Arc::new(Mutex::new(String::new()));
    // A list of web client to notify on file change.
    let users: Users = Arc::new(Mutex::new(std::collections::HashMap::new()));

    // Helper function to read the file
    let load_file = |shared: SharedFile| async move {
        let mut content = shared.lock().unwrap();
        content.clear();
        content.push_str(&std::fs::read_to_string(PATH).expect("oops"));
    };

    load_file(html_content.clone()).await;

    // The / route
    let index = warp::path::end().map({
        let html_content = html_content.clone();
        move || {
            let html_content = html_content.clone();
            warp::http::Response::builder()
                .header("content-type", "text/html; charset=utf-8")
                .body(format!("{}{}", html_content.lock().unwrap(), JS))
        }
    });

    // The /events route
    let events = {
        let users = Arc::clone(&users);
        let users = warp::any().map(move || users.clone());
        warp::path("events")
            .and(warp::get())
            .and(users)
            .map(|users: Users| {
                let stream = event_stream(users);
                warp::sse::reply(stream)
            })
    };

    // Spawn the warp service
    println!("Spawning warp service at http://0.0.0.0:3030");
    tokio::spawn(async {
        let routes = index.or(events);
        warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
    });

    tokio::spawn(async {
        tokio::process::Command::new("xdg-open")
            .arg("http://localhost:3030")
            .spawn()
            .expect("Failed to run xdg-open http://localhost:3030");
    });

    // Watch file changes
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = notify::watcher(tx, std::time::Duration::from_secs(0)).unwrap();
    watcher.watch(PATH, RecursiveMode::NonRecursive).unwrap();
    watcher
        .watch("src/html.rs", RecursiveMode::NonRecursive)
        .unwrap();

    println!("Watching files...");
    loop {
        let ev = rx.recv();
        match ev {
            Ok(notify::DebouncedEvent::NoticeWrite(path)) => {
                let path = path.as_path().to_str().unwrap_or("");
                println!("Reloading {:?}...", path);
                if path.ends_with("html.rs") {
                    tokio::spawn(async {
                        let mut child = tokio::process::Command::new("cargo")
                            .arg("run")
                            .arg("--")
                            .arg("demo")
                            .spawn()?;

                        // Await until the command completes
                        let status = child.wait().await?;
                        println!("Build status: {}", status);
                        let result: Result<(), std::io::Error> = Ok(());
                        result
                    });
                } else {
                    let shared = Arc::clone(&html_content);
                    let users = Arc::clone(&users);
                    tokio::spawn({
                        async move {
                            load_file(shared.clone()).await;
                            users.lock().unwrap().retain(|uid, tx| {
                                println!("Sending event to {}", uid);
                                tx.send(()).is_ok()
                            });
                        }
                    });
                }
            }
            Ok(_) => {}
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

static JS: &str = r#"
<script>
  const events = new EventSource("/events")
  events.addEventListener("reload", e => {
    window.location.reload()
  })
</script>
"#;