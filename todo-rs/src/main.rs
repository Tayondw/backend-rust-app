#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().max_size(5).build(manager).expect("Failed to create pool.");
    let db_connection = Arc::new(pool);

    let app = Router::new()
        .route("/todos", post(handlers::create_todo))
        .route("/todos", get(handlers::get_todos))
        .route("/todos/:id", get(handlers::get_todo))
        .route("/todos/:id", post(handlers::update_todo))
        .route("/todos/:id", delete(handlers::delete_todo))
        .with_state(db_connection.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let server = axum::serve(listener, app).with_graceful_shutdown(shutdown_signal());

    tokio::spawn(async move {
        println!("Server is running");
    });

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

/*
Line 4-10: Set up the database connection pool

Line 12-18: Define the routes for our API

Line 20-21: Set up the server address

Line 23-30: Log application startup or failure
*/
