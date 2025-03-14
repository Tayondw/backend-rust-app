use std::env;
use std::sync::Arc;
use axum::Router;
use axum::routing::{ delete, get, post };
use diesel::{ PgConnection, r2d2 };
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;
use tokio::signal;

mod models;
mod handlers;
mod schema;

#[tokio::main] // a procedural macro that marks the main function as asynchronous and it runs on the Tokio runtime
async fn main() {
    dotenv().ok(); // calls the dotenv() fxn to load environment variables from a .env file into the process environment

    //fetch the DATABASE_URL environment variable, which will contain the database connection string
    // if DATABASE_URL is not set, it panics with an error message
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Diesel connection manager for psql and then initializes it with the database URL
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    // create a connection pool using r2d2, a thread-safe connection pool manager
    // set the max number of connections
    // if the pool creation fails, panic with "Failed to create pool."
    let pool = r2d2::Pool::builder().max_size(5).build(manager).expect("Failed to create pool.");

    // wrap the connection pool in an Arc (Atomic Reference Counted) smart pointer to allow safe sharing between threads
    let db_connection = Arc::new(pool);

    let app = Router::new() // creates an Axum router
        // define API routes for handling todos using HTTP methods (GET, POST, DELETE)
        .route("/todos", post(handlers::create_todo)) // (POST) calls handlers::create_todo
        .route("/todos", get(handlers::get_todos)) // (GET) calls handlers::get_todos
        .route("/todos/{id}", get(handlers::get_todo)) // (GET) calls handlers::get_todo
        .route("/todos/{id}", post(handlers::update_todo)) // (POST) calls handlers::update_todo
        .route("/todos/{id}", delete(handlers::delete_todo)) // (DELETE) calls handlers::delete_todo
        .with_state(db_connection.clone()); // allows handlers to access the database connection pool

    // create a TCP listener bound to port 8080, listening on our local IP addr
    // ensure if binding fails, the application panics
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();

    // start the Axum server with the given listener and router
    // ensure the server shuts down gracefully when a shutdown signal is received
    let server = axum::serve(listener, app).with_graceful_shutdown(shutdown_signal());

    // spawn an async task that simply prints "Server is running"
    // task will exit immediately since it does not contain an infinite loop or delay
    tokio::spawn(async move {
        println!("Server is running");
    });

    // await the server future
    // if an error occurs while running the server, it prints an error message
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

/*
Line 18-31: Set up the database connection pool
- a pool is a managed set of db connections that can be reused instead of opening a new connection each time one is needed
      pros:
            - efficiency
                  - opening and closing db connections is expensive in terms of time and resources
                  - a pool keeps connections open and allows multiple requests to reuse them
            - concurrency
                  - multiple requests can use connections from the pool instead of waiting for new connections to be created
            - resource management
                  - the pool limits the maximum number of connections to prevent overwhelming the database.
- flow:
      1. the application starts, the pool creates a set number of connections
      2. when the request needs a db connection, it gets one from the pool
      3. after the request is complete, the connection is returned to the pool instead of being closed
      4. if no connections are available, the request waits until one is free or a new one is created

- analogy:
      - Think of a database connection pool like a taxi stand:
            - Instead of calling for a new taxi every time, passengers take taxis from a fixed pool.
            - When a taxi drops off a passenger, it returns to the stand for the next person.
            - If no taxis are available, new ones can be added, but there’s a limit.

r2d2 (short for "Reliable Redis Database Pool" but now generalized) is a connection pool manager for Rust. It helps manage a set of database connections efficiently, preventing the overhead of opening and closing connections frequently.

Line 33-40: Define the routes for our API

Line 42-48: Set up the server address

Line 50-61: Log application startup or failure
*/

// This function waits for a termination signal (Ctrl+C) and then starts a graceful shutdown of the application.
async fn shutdown_signal() {
    // create an async block that listens for Ctrl+C
    let ctrl_c = async {
        // wait until the users presses Ctrl+C
        // if the handler cannot be installed then cause a panic
        signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)] // only compiles the code on UNIX-based systems (Linux or macOS)
    let terminate = async {
        signal::unix
            ::signal(signal::unix::SignalKind::terminate()) // termination signal
            .expect("failed to install signal handler") // panic if it fails
            .recv().await; // wait for termination signal before proceeding
    };

    // if system is not UNIX, it never resolves using the future
    // this means the function only reacts to Ctrl+C on Windows
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! { // waits for whichever event happens first
        _ = ctrl_c => {}, // if Ctrl+C is pressed
        _ = terminate => {}, // if SIGTEM is received (on UNIX)
    }

    println!("signal received, starting graceful shutdown"); // prints a message when a termination signal is received
}
