#[macro_use] extern crate nickel;
extern crate handlebars;
extern crate ws;

use ws::{ listen, CloseCode, Sender, Handler, Message, Result, Handshake };
use nickel::{Nickel, HttpRouter};
use handlebars::Handlebars;

use std::collections::HashMap;
use std::thread;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::error::Error;

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

fn load_template(name: &str) -> String {
    let path = Path::new(name);
    let mut file = File::open(path).unwrap();
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't load template '{}': {}", name, Error::description(&why)),
        Ok(_) => s
    }
}

fn main() {
    let mut handlebars = Handlebars::new();

    let t = load_template("templates/index.hbs");
    handlebars.register_template_string("index", t).ok().unwrap();

    let mut server = Nickel::new();

    let websocket_server = thread::spawn(move || {
        listen("127.0.0.1:3012", |out| {
            WebsocketServer { out: out }
        }).unwrap()
    });

    server.get("/bar", middleware!("This is the /bar handler"));
    server.get("/user/:herp", middleware! { |req, res|
        let mut data = HashMap::new();
        data.insert("test".to_string(), req.param("herp").unwrap().to_string());
        handlebars.render("index", &data).ok().unwrap()
    });

    server.listen("127.0.0.1:6767");

    let _ = websocket_server.join();
}
