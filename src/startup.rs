use crate::routes;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};

pub fn run(
    listener: std::net::TcpListener,
    db_pool: sqlx::PgPool,
) -> std::result::Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
            .app_data(web::Data::clone(&db_pool))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
