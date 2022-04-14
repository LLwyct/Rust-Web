use crate::handler::{WebServiceHandler, Handler};

use super::router::Router;
use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use std::io::prelude::*;
use std::net::TcpListener;
use std::{str};

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &str) -> Server {
        Server { socket_addr }
    }

    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Server is listening {}...", self.socket_addr);
        let mut router = Router::new();

        router.add_route("/myself", |req, stream| {
            let res = match req.method {
                httprequest::Method::Get => HttpResponse::new("200", None, WebServiceHandler::load_file("myself.html")),
                _ => HttpResponse::new("404", None, WebServiceHandler::load_file("404.html"))
            };
            let _ = res.send_response(stream);
        });
        
        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            let mut read_buffer = [0; 1024];
            stream.read(&mut read_buffer).unwrap();
            let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();
            
            router.route(req, &mut stream);
        }
    }
}
