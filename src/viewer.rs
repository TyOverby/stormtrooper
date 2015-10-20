use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;

use notify::{self, PollWatcher};
use iron::{Iron, Response, status, Request, Headers};
use iron::headers::{HeaderFormat, Header};
use router::Router;
use typemap::TypeMap;

use super::{script, svg, Drawing};

fn run_web(last_generated: Arc<Mutex<Option<String>>>) {
    let mut router = Router::new();

    router.get("debug.svg", move |_: &mut Request| {
        let mut headers = Headers::new();
        headers.set_raw("content-type", vec!["image/svg+xml".bytes().collect()]);

        let guard = last_generated.lock().unwrap();
        let mut body = (*guard).as_ref().unwrap().clone();

        Ok(Response {
            status: Some(status::Ok),
            headers: headers,
            extensions: TypeMap::new(),
            body: Some(Box::new(body))
        })
    });

    router.get("release.svg", |_: &mut Request| {
        Ok(Response::with((status::Ok, "release.svg")))
    });

    Iron::new(router).http("localhost:3000").unwrap();
}

fn run_watcher(last_generated: Arc<Mutex<Option<String>>>, file: &str) {
    use notify::{PollWatcher, Error, Watcher, Event};
    use notify::op::WRITE;
    use std::fs::File;
    use std::sync::mpsc::channel;
    use std::io::Read;
    use std::thread;

    // Create a channel to receive the events.
    let (tx, rx) = channel();

    let mut watcher = PollWatcher::new_with_delay(tx, 50).unwrap();
    watcher.watch(file).unwrap();

    let file = file.to_owned();
    thread::spawn(move || {
        let _watcher = watcher;
        for change in rx.iter() {
            match change {
                Event { op: Ok(WRITE), .. } => {
                    println!("written! {:?}", change);
                    let mut file = File::open(&file[..]).unwrap();
                    let mut buf = String::new();
                    file.read_to_string(&mut buf).unwrap();

                    let mut drawing = Drawing::new();
                    script::run_script(&mut drawing, &buf);

                    let mut out_buf = Vec::new();
                    svg::write_svg(&drawing, &mut out_buf).unwrap();
                    let out_str = String::from_utf8(out_buf).unwrap();
                    let mut guard = last_generated.lock().unwrap();
                    *guard = Some(out_str);
                }
                other => {
                    println!("something else!: {:?}", other)
                }
            }
        }
    });
}

pub fn start() {
    use std::thread;
    let mut body = r#"<svg> <line x1="0px" y1="0px" x2="20px" y2="20px"/> </svg>"#.to_owned();
    let last_generated = Arc::new(Mutex::new(Some(body)));
    run_watcher(last_generated.clone(), "./test.ares");
    run_web(last_generated.clone());
}
