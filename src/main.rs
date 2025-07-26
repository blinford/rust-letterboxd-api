mod letterboxd;
mod endpoints;

use actix_cors::Cors;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(endpoints::hello)
            .wrap(Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}


