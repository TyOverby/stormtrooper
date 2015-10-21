use notify::{PollWatcher, Watcher, Event};
use notify::op::WRITE;
use std::fs::File;
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};
use std::io::Read;
use std::thread;

use super::super::{script, svg, Drawing};

pub fn run_watcher(last_generated: Arc<Mutex<Option<String>>>, file: &str) -> Receiver<()> {
    fn update(file: &str, last_generated: &Arc<Mutex<Option<String>>>) {
        let mut file = File::open(&file[..]).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();

        let mut drawing = Drawing::new();
        if let Err(_) = script::run_script(&mut drawing, &buf) {
            return;
        }

        let mut out_buf = Vec::new();
        svg::write_svg(&drawing, &mut out_buf).unwrap();
        let out_str = String::from_utf8(out_buf).unwrap();
        let mut guard = last_generated.lock().unwrap();
        *guard = Some(out_str);
    }

    // Create a channel to receive the events.
    let (tx, rx) = channel();
    // This one is for the web socket server
    let (notify_sx, notify_rx) = channel();

    let mut watcher = PollWatcher::new_with_delay(tx, 10).unwrap();
    watcher.watch(file).unwrap();
    update(file, &last_generated);

    let file = file.to_owned();
    thread::spawn(move || {
        let notify_sx = notify_sx;
        let _watcher = watcher;
        for change in rx.iter() {
            match change {
                Event { op: Ok(WRITE), .. } => {
                    update(&file, &last_generated);
                    notify_sx.send(()).unwrap()
                }
                other => {
                    println!("something else!: {:?}", other)
                }
            }
        }
    });
    notify_rx
}
