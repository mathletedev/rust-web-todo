use axum::{extract::State, http::StatusCode, routing::get, Router};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};

#[tokio::main]
async fn main() {
	dotenv().ok();

	let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
	let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

	let pool = PgPoolOptions::new()
		.max_connections(5)
		.connect(&db_url)
		.await
		.expect("Failed to connect to database");

	let app = Router::new()
		.route("/", get(root))
		.route("/hello", get(hello).with_state(pool));

	let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
		.await
		.unwrap();

	println!("Server up and running!");
	axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
	"Hello, World!"
}

async fn hello(State(pool): State<PgPool>) -> Result<String, (StatusCode, String)> {
	sqlx::query_scalar("SELECT 'Hello, world!'")
		.fetch_one(&pool)
		.await
		.map_err(internal_error)
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
	E: std::error::Error,
{
	(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
