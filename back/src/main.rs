use actix_web::{delete, get, post, web::{self, Data, Query}, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Book {
    id: usize,
    name: String,
    author: String,
}

#[derive(Clone)]
struct State {
    state: Arc<Mutex<Vec<Book>>>
}

impl State {
    fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(vec![]))
        }
    }

    fn add_book<T: Into<String>>(self: Self, name: T, author: T) -> Self {
        let id = self.state.lock().unwrap().len();
        self.state.lock().unwrap().push(Book { id, name: name.into(), author: author.into() });
        self
    }
}

#[get("/get_books")]
async fn get_books(state: web::Data<State>) -> impl Responder {
    println!("{:?}", "BOOOOOOOKS");
    let date = state.state.lock().unwrap().clone();
    serde_json::to_string(&date)
}

#[derive(Serialize, Deserialize)]
struct Id{
    id: usize
}

#[get("/get_book")]
async fn get_book(state: web::Data<State>, query: Query<Id>) -> impl Responder {
    println!("SUKA");
    if let Some(book) = state.state.lock().unwrap().get(query.0.id) {
        match serde_json::to_string(book) {
            Ok(json) => HttpResponse::Ok().body(json),
            Err(_) => HttpResponse::InternalServerError().body("Error serializing book"),
        }
    } else {
        HttpResponse::NotFound().body("Book not found")
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let state = State::new()
        .add_book("Odin", "Tor")
        .add_book("Anton", "Krest");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .service(get_books)
            .service(get_book)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
