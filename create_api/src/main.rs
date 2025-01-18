use actix_web::{App, HttpServer, HttpResponse, web};
use std::io::{Result};
use create_api::backend::data:: {get_fake_trips};
use serde::Deserialize;


#[derive(Deserialize)]
struct TripsQuery {
	from_ms: i64,
	n_results: i64
}

async fn health() -> HttpResponse {
	HttpResponse::Ok().json(serde_json::json!({
		"status": "healthy",
		"timestamp": chrono::Utc::now().to_rfc3339()
	}))
}

async fn trips(query: web::Query<TripsQuery>) -> HttpResponse {
	match get_fake_trips(query.from_ms, query.n_results).await {
		Ok(trips) => HttpResponse::Ok().json(trips),
		Err(e) => HttpResponse::InternalServerError().body(e.to_string())
	}
}

#[actix_web::main]
async fn main() -> Result<()> {
	HttpServer::new(|| {
		App::new()
		.wrap(actix_web::middleware::Logger::default())
		.route("/health", web::get().to(health))
		.route("/trips", web::get().to(trips))
	})
	.bind(("0.0.0.0", 4122))?
	.run()
	.await
	// .map_err(|e| {
	// 	error!("Error starting server: {}", e)
	// })
}
