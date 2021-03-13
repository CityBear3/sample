use actix_web::{get, App, HttpResponse, HttpServer};
use actix_web::middleware::Logger;

#[get("/")]
async fn index() -> Result<HttpResponse, actix_web::Error> {
    let body = String::from("Hello, Actix-Web");
    Ok(HttpResponse::Ok().body(body))
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {

    std::env::set_var("RUST_LOG", "info");

    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(index)
    })
    .bind("0.0.0.0:7777")?
    .run()
    .await?;

    Ok(())
}
