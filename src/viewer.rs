use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::io::copy;
use std::fs::File;

use notify::{self, PollWatcher};
use hyper::{self, mime, header};
use hyper::server::{Request, Response};

use super::{script, svg, Drawing};

fn run_web(last_generated: Arc<Mutex<Option<String>>>) {
    fn handle_debug(mut response: Response, last_generated: &Arc<Mutex<Option<String>>>) {
        let mimetype =
            mime::Mime(
                mime::TopLevel::Image,
                mime::SubLevel::Ext("svg+xml".into()),
                vec![(mime::Attr::Charset, mime::Value::Utf8)]);

        let guard = last_generated.lock().unwrap();
        if let &Some(ref content) = &*guard {
            response.headers_mut().set(header::ContentType(mimetype));
            response.send(content.as_bytes());
        } else {
            *response.status_mut() = hyper::status::StatusCode::NoContent;
            response.send("oh shit!".as_bytes());
        }
    }

    fn handle_index(mut response: Response) {
        response.headers_mut().set(header::ContentType::html());
        let mut writer = response.start().unwrap();
        let mut index = File::open("./include/viewer.html").unwrap();
        copy(&mut index, &mut writer);
        writer.end();
    }

    let _listening = hyper::Server::http("127.0.0.1:3000").unwrap()
        .handle(move |request: Request, response: Response| {
            println!("{:?}", request.uri);
            if let hyper::uri::RequestUri::AbsolutePath(ref path) = request.uri {
                if path == "/debug.svg" {
                    handle_debug(response, &last_generated);
                } else if path == "/" {
                    handle_index(response);
                }
            }
        });
}

fn run_watcher(last_generated: Arc<Mutex<Option<String>>>, file: &str) {
    use notify::{PollWatcher, Error, Watcher, Event};
    use notify::op::WRITE;
    use std::fs::File;
    use std::sync::mpsc::channel;
    use std::io::Read;
    use std::thread;

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

    let mut watcher = PollWatcher::new_with_delay(tx, 50).unwrap();
    watcher.watch(file).unwrap();
    update(file, &last_generated);

    let file = file.to_owned();
    thread::spawn(move || {
        let _watcher = watcher;
        for change in rx.iter() {
            match change {
                Event { op: Ok(WRITE), .. } => {
                    println!("written! {:?}", change);
                    update(&file, &last_generated);
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
