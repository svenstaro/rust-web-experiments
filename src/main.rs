#[macro_use] extern crate nickel;
extern crate nickel_mustache;
extern crate ws;

use ws::{ connect, listen, CloseCode, Sender, Handler, Message, Result, Handshake };
use nickel::{Nickel, HttpRouter};
use nickel_mustache::{TemplateSupport, Render};

use std::collections::HashMap;
use std::thread;

fn main() {
    let mut server = Nickel::new();

    server.get("/bar", middleware!("This is the /bar handler"));
    server.get("/user/:herp", middleware! { |req, res|
        let mut data = HashMap::new();
        data.insert("test", req.param("herp"));

        return Render::render(res, "html/index.mustache", &data);
    });

    // server.listen("127.0.0.1:6767");

    struct WebsocketServer {
        out: Sender
    }

    impl Handler for WebsocketServer {
        fn on_message(&mut self, msg: Message) -> Result<()> {
            println!("herp: {}", msg);
            self.out.send(msg)
        }

        fn on_open(&mut self, handshake: Handshake) -> Result<()> {
            println!("connection: {:?}", handshake);
            self.out.send("sup")
        }

        fn on_close(&mut self, code: CloseCode, reason: &str) {
            println!("Closing: ({:?}) {}", code, reason);
            println!("Shutting down");
            self.out.shutdown().unwrap()
        }
    }

    let websocket_server = thread::spawn(move || {
        listen("127.0.0.1:3012", |out| {
            WebsocketServer { out: out }
        }).unwrap()
    });

    websocket_server.join();
}
