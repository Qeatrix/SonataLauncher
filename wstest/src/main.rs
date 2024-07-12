use std::net::TcpListener;
use std::thread::spawn;

use async_tungstenite::tungstenite::accept;

fn main() {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();

    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();

            loop {
                let msg = websocket.read().unwrap();

                println!("Received message: {}", msg);

                if msg == "STOP".into() {
                    break;
                }

                if msg.is_binary() || msg.is_text() {
                    websocket.send(msg).unwrap();
                }
            }
        });
    }
}
