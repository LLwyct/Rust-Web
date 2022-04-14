use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use std::net::TcpStream;
use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use std::collections::HashMap;

pub struct Router<'a, F>
where
    F: Fn(HttpRequest, &mut TcpStream)
{
    route_map: HashMap<&'a str, F>,
}

impl<'a, F> Router<'a, F>
where
    F: Fn(HttpRequest, &mut TcpStream)
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
                    // 获取get请求的路径 /xxx/xxx
                    let route: Vec<&str> = s.split("/").collect();
                    // 如果是api请求，交给WebServiceHandler处理
                    if route[1] == "api" {
                        let res: HttpResponse = WebServiceHandler::handle(&req);
                        let _ = res.send_response(write_stream);
                    } else {
                        // 否则判断该path是否在认为规定的路由表中
                        match self.route_map.get(s.as_str()) {
                            // 如果在，则使用提前写好的回调函数处理
                            Some(func) => func(req, write_stream),
                            // 否则交给StaticPageHandler处理
                            None => {
                                let res: HttpResponse = StaticPageHandler::handle(&req);
                                let _ = res.send_response(write_stream);
                            }
                        };
                    }
                }
            },
            _ => {
                let res: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = res.send_response(write_stream);
            }
        }
    }

    pub fn add_route(&mut self, path: &'a str, func: F)
    {
        self.route_map.insert(path, func);
    }
}