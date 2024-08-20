use std::{
    sync::mpsc::channel,
    path::Path,
    time::{
        Instant,
        Duration
    }
};

use notify::{
    Config,
    RecommendedWatcher,
    Watcher,
    RecursiveMode,
};

fn handle_change(last_event_time: &mut Instant, debounce_time: Duration) {
    let now = Instant::now();

    if now.duration_since(*last_event_time) > debounce_time {
        println!("Change!");

        *last_event_time = now;
    }
}

fn main() {
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();
    watcher.watch(Path::new("."), RecursiveMode::Recursive).unwrap();

    let debounce_time = Duration::from_millis(250);
    let mut last_event_time = Instant::now();

    for res in rx {
        match res {
            Ok(_) => handle_change(&mut last_event_time, debounce_time),
            Err(e) => println!("Error: {e:?}",)
        }
    }
}

