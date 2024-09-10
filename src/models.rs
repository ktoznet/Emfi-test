use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub year: u32,
}

#[derive(Deserialize)]
pub struct Request {
    pub action: String,
    pub id: Option<u32>,
    pub book: Option<Book>,
}
