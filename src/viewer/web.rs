use std::io::copy;
use std::fs::File;
use std::sync::{Arc, Mutex};

use hyper::{self, mime, header};
use hyper::server::{Request, Response};

pub fn run_web(last_generated: Arc<Mutex<Option<String>>>) {
    fn handle_debug(mut response: Response, last_generated: &Arc<Mutex<Option<String>>>) {
        let mimetype =
            mime::Mime(
                mime::TopLevel::Image,
                mime::SubLevel::Ext("svg+xml".into()),
                vec![(mime::Attr::Charset, mime::Value::Utf8)]);

        let guard = last_generated.lock().unwrap();
        if let &Some(ref content) = &*guard {
            response.headers_mut().set(header::ContentType(mimetype));
            response.send(content.as_bytes()).unwrap();
        } else {
            *response.status_mut() = hyper::status::StatusCode::NoContent;
            response.send("oh shit!".as_bytes()).unwrap();
        }
    }

    fn handle_index(mut response: Response) {
        response.headers_mut().set(header::ContentType::html());
        let mut writer = response.start().unwrap();
        let mut index = File::open("./include/viewer.html").unwrap();
        copy(&mut index, &mut writer).unwrap();
        writer.end().unwrap();
    }

    fn handle_svg_pan_zoom(mut response: Response) {
        let mimetype =
            mime::Mime(
                mime::TopLevel::Application,
                mime::SubLevel::Javascript,
                vec![(mime::Attr::Charset, mime::Value::Utf8)]);
        response.headers_mut().set(header::ContentType(mimetype));
        let mut writer = response.start().unwrap();
        let mut index = File::open("./include/svg-pan-zoom.min.js").unwrap();
        copy(&mut index, &mut writer).unwrap();
        writer.end().unwrap();
    }

    let _listening = hyper::Server::http("127.0.0.1:3000").unwrap()
        .handle(move |request: Request, response: Response| {
            if let hyper::uri::RequestUri::AbsolutePath(ref path) = request.uri {
                if path.starts_with("/debug.svg") {
                    handle_debug(response, &last_generated);
                } else if path == "/svg-pan-zoom.min.js" {
                    handle_svg_pan_zoom(response);
                } else if path == "/" {
                    handle_index(response);
                }
            }
        });
}
