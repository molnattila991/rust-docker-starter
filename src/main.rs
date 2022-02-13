use mysql::prelude::Queryable;
use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use mysql::Opts;
use mysql::Pool;
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct Item {
    id: u64,
    title: String,
    description: String,
    price: f32,
    category_id: String,
}

pub struct AppState {
    pub pool: mysql::Pool,
}

#[get("/")]
pub async fn say_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/jsonList")]
pub async fn get_list(data: web::Data<AppState>) -> impl Responder {
    let mut conn = data.pool.get_conn().unwrap();

    let res = conn.query_map(
        "select id, title, description, price, category_id from items",
        |(id, title, description, price, category_id)| 
        Item {
                id: id,
                title: title,
                description: description,
                price: price,
                category_id: category_id
            }
    ).expect("Query failed.");
     
    web::Json(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mysql_connection_string = env::var("MYSQL_CONNECTION_STRING");
    let opt = Opts::from_url(&mysql_connection_string.unwrap());
    let pool = Pool::new(opt.unwrap()).unwrap();
    let mut _conn = pool.get_conn().unwrap();

    let app_data = web::Data::new(AppState { pool: pool });
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(say_hello)
            .service(get_list)
    })
    .workers(4)
    .bind("0.0.0.0:12080")?
    .run()
    .await
}