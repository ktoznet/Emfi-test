use actix_web::{web, App, HttpServer, HttpRequest, Error};
use actix_web_actors::ws;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod models;
mod handlers;
use handlers::WsSession;
use models::Book;

struct AppState {
    books: Arc<Mutex<HashMap<u32, Book>>>,
}

fn main() -> std::io::Result<()> {
    let books = Arc::new(Mutex::new(HashMap::new()));

    actix_rt::System::new().block_on(async {
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(AppState {
                    books: books.clone(),
                }))
                .route("/ws/", web::get().to(ws_route))
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
    })
}
async fn ws_route(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<AppState>,
) -> Result<impl actix_web::Responder, Error> {
    ws::start(WsSession { state: state.books.clone() }, &req, stream)
}