use std::sync::{Arc, Mutex};

mod web;
mod watcher;
mod websock;

use self::web::*;
use self::watcher::*;
use self::websock::*;

pub fn start() {
    let last_generated = Arc::new(Mutex::new(None));
    let rx = run_watcher(last_generated.clone(), "./test.ares");
    run_websocket(rx);
    run_web(last_generated.clone());
}
