use actix_web::{get, web, HttpResponse, Responder};
use crate::letterboxd;

#[get("/films/{user}")]
async fn hello(path: web::Path<(String,)>) -> impl Responder {
    let username = path.into_inner().0;

    HttpResponse::Ok().json(letterboxd::fetch_movies(username).await.unwrap())
}