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
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use warp::Filter;

type SharedFile = Arc<Mutex<(PathBuf, String)>>;
type Users = Arc<Mutex<HashMap<usize, tokio::sync::mpsc::UnboundedSender<()>>>>;

static NEXT_USER_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);

use anyhow::{Context, Result};
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
async fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let path = if let Some(arg) = args.next() {
        arg
    } else {
        "./target/html/index.html".to_string()
    };
    let path = std::path::Path::new(&path);

    // Ensure the html file exists
    if !path.exists() {
        match path.parent() {
            Some(path) => std::fs::create_dir_all(path).context("Can't create parent directory")?,
            None => {}
        };
        std::fs::write(path, format!("Waiting for {:?}", path)).context("Can't create file")?;
    }

    let html_path = path.to_path_buf();
    let rs_path = "src/";

    // A shared String for the index.html file.
    let html_content: SharedFile = Arc::new(Mutex::new((html_path.clone(), String::new())));
    // A list of web client to notify on file change.
    let users: Users = Arc::new(Mutex::new(std::collections::HashMap::new()));

    // Helper function to read the file
    let load_file = |shared: SharedFile| async move {
        let mut content = shared.lock().expect("Can't acquire the shared file lock");
        content.1.clear();
        match std::fs::read_to_string(&content.0) {
            Err(e) => content.1.push_str(&format!("Can't read {}", e)),
            Ok(new_content) => content.1.push_str(&new_content),
        }
    };

    load_file(html_content.clone()).await;

    // The / route
    let index = warp::path::end().map({
        let html_content = html_content.clone();
        move || {
            let html_content = html_content.clone();
            warp::http::Response::builder()
                .header("content-type", "text/html; charset=utf-8")
                .body(format!("{}{}", html_content.lock().unwrap().1, JS))
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
            .map_or_else(
                |e| println!("Failed to run `xdg-open http://localhost:3030` {}", e),
                |_| (),
            );
    });

    // Watch file changes
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = notify::watcher(tx, std::time::Duration::from_secs(0))
        .context("Failed to create notify watcher")?;
    watcher
        .watch(&html_path, RecursiveMode::NonRecursive)
        .context("Failed to watch demo html")?;
    watcher
        .watch(rs_path, RecursiveMode::Recursive)
        .context("Failed to watch html.rs module")?;

    println!("Watching {:?} and {:?}...", &html_path, rs_path);
    loop {
        let ev = rx.recv();
        match ev {
            Ok(notify::DebouncedEvent::NoticeWrite(path)) => {
                let path = path.as_path().to_str().unwrap_or("");
                println!("Reloading {:?}...", path);
                if path.ends_with(".rs") {
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
