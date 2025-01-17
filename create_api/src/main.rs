use actix_web::{App, HttpServer, HttpResponse, web};
use std::io::{Result};


async fn health() -> HttpResponse {
	HttpResponse::Ok().json(serde_json::json!({
		"status": "healthy",
	}))
}

#[actix_web::main]
async fn main() -> Result<()> {
	HttpServer::new(|| {
		App::new().route("/health", web::get().to(health))
	})
	.bind(("0.0.0.0", 4122))?
	.run()
	.await
}
