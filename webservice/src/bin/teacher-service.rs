use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Mutex;

#[path ="../routers.rs"]
mod routers;
#[path ="../handlers.rs"]
mod handlers;
#[path ="../state.rs"]
mod state;
#[path = "../models.rs"]
mod models;

use state::AppState;
use routers::*;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let shared_data = web::Data::new(
        AppState {
            health_check_response: "I am OK.".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        }
    );
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };
    return HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}