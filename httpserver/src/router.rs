use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use std::io::prelude::*;
use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, write_stream: &mut impl Write) -> () {
        match req.method {
            httprequest::Method::Get => match &req.resource {
                httprequest::Resource::Path(s) => {
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        "api" => {
                            let res: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = res.send_response(write_stream);
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
}