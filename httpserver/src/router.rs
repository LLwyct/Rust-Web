use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use std::{io::prelude::*, net::{TcpStream, TcpListener}};
use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use std::collections::HashMap;

pub struct Router<'a>
{
    route_map: HashMap<&'a str, Box<dyn Fn(HttpRequest, &mut TcpStream)>>,
}

impl<'a> Router<'a>
{
    pub fn new () -> Self
    {
        Router {
            route_map: HashMap::new()
        }
    }

    pub fn route(&self, req: HttpRequest, write_stream: &mut TcpStream) -> () {
        match req.method {
            httprequest::Method::Get => match &req.resource {
                httprequest::Resource::Path(s) => {
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        "api" => {
                            let res: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = res.send_response(write_stream);
                        },
                        "myself" => {
                            let func = self.route_map.get("myself").unwrap();
                            func(req, write_stream);
                        },
                        _ => {
                            let res: HttpResponse = StaticPageHandler::handle(&req);
                            let _ = res.send_response(write_stream);
                        }
                    }
                }
            },
            _ => {
                let res: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = res.send_response(write_stream);
            }
        }
    }

    pub fn add_route(&mut self, path: &'a str, func: Box<dyn Fn(HttpRequest, &mut TcpStream)>)
    {
        self.route_map.insert(path, func);
    }
}