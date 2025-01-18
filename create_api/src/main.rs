use actix_web::{App, HttpServer, HttpResponse, web};
use create_api::backend::data:: {get_fake_trips, get_trips};
use serde::Deserialize;
use log::{ error};
use std::io::{Result};
// use std::env;
// use env_logger::Env;


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

// Get Trips - fake data
async fn trips(query: web::Query<TripsQuery>) -> HttpResponse {
	match get_trips(query.from_ms, query.n_results).await {
		Ok(trips) => HttpResponse::Ok().json(trips),
		Err(e) => HttpResponse::InternalServerError().body(e.to_string())
	}
}

//Get Trips from parquet file 


#[actix_web::main]
async fn main() -> Result<()> {

	// Initialize the logger
	// env_logger::init_from_env(Env::new().default_filter_or("info"))
	// let port = env::var("PORT")
	// 	.unwrap_or_else(|_| "8080".to_string())
	// 	.parse::<u16>()
	// 	.expect("PORT must be a valid number");

	// info!("Starting server on port {}", port)

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
