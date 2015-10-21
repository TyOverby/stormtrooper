use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::io::{Read, Write};
use std::cell::RefCell;

use websocket::{Server, WebSocketStream, Message};
use websocket::server::Connection;
use websocket::server::Sender as WsSender;
use websocket::ws::sender::Sender as WsSenderTrait;
use websocket::result::WebSocketResult;

type Wss = WebSocketStream;

fn get_sender<A: Read, B: Write>(connection: Connection<A, B>) -> WebSocketResult<WsSender<B>> {
    let request = try!(connection.read_request());
    try!(request.validate());
    let response = request.accept();
    let client = try!(response.send());
    let (sender, _) = client.split();
    Ok(sender)
}

pub fn run_websocket(code_changed: Receiver<()>) {
    let (conn_send, conn_recv) = channel::<WsSender<Wss>>();

    thread::spawn(|| {
        let conn_send = conn_send;
        let ws_server = Server::bind("127.0.0.1:2794").unwrap();
        for connection in ws_server {
            if let Ok(c) = connection.map_err(From::from).and_then(get_sender) {
                if let Err(_) = conn_send.send(c) {
                    return;
                }
            }
        }
    });

    thread::spawn(move || {
        let conn_recv = conn_recv;
        let mut senders = vec![];
        loop {
            if let Ok(()) = code_changed.recv() {
                while let Ok(conn) = conn_recv.try_recv() {
                    senders.push(RefCell::new(conn));
                }
                senders.retain(|sender| {
                    let mut sender = sender.borrow_mut();
                    let message = Message::Text("hello_world".to_string());
                    sender.send_message(message).is_ok()
                });
            } else { return }
        }
    });
}
