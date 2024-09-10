use actix::prelude::*;
use actix_web_actors::ws;
use crate::models::{Book, Request};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde_json;

pub struct WsSession {
    pub state: Arc<Mutex<HashMap<u32, Book>>>,
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Ok(ws::Message::Text(text)) => text,
            _ => return,
        };

        let req: Result<Request, _> = serde_json::from_str(&msg);
        if let Ok(req) = req {
            let response = match req.action.as_str() {
                "get_books" => {
                    let books = self.state.lock().unwrap();
                    serde_json::to_string(&books.values().collect::<Vec<_>>()).unwrap()
                }
                "get_book" => {
                    let books = self.state.lock().unwrap();
                    if let Some(id) = req.id {
                        if let Some(book) = books.get(&id) {
                            serde_json::to_string(book).unwrap()
                        } else {
                            format!("{{\"error\": \"Book with id {} not found\"}}", id)
                        }
                    } else {
                        "{\"error\": \"ID not provided\"}".to_string()
                    }
                }
                "add_book" => {
                    if let Some(book) = req.book {
                        let mut books = self.state.lock().unwrap();
                        books.insert(book.id, book.clone());
                        "{\"status\": \"Book added\"}".to_string()
                    } else {
                        "{\"error\": \"Invalid book data\"}".to_string()
                    }
                }
                "update_book" => {
                    if let Some(id) = req.id {
                        if let Some(book) = req.book {
                            let mut books = self.state.lock().unwrap();
                            if books.contains_key(&id) {
                                books.insert(id, book.clone());
                                "{\"status\": \"Book updated\"}".to_string()
                            } else {
                                format!("{{\"error\": \"Book with id {} not found\"}}", id)
                            }
                        } else {
                            "{\"error\": \"Invalid book data\"}".to_string()
                        }
                    } else {
                        "{\"error\": \"ID not provided\"}".to_string()
                    }
                }
                "delete_book" => {
                    if let Some(id) = req.id {
                        let mut books = self.state.lock().unwrap();
                        if books.remove(&id).is_some() {
                            "{\"status\": \"Book deleted\"}".to_string()
                        } else {
                            format!("{{\"error\": \"Book with id {} not found\"}}", id)
                        }
                    } else {
                        "{\"error\": \"ID not provided\"}".to_string()
                    }
                }
                _ => "{\"error\": \"Unknown action\"}".to_string(),
            };

            ctx.text(response);
        } else {
            ctx.text("{\"error\": \"Invalid request format\"}");
        }
    }
}
