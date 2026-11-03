use axum ::{
    extract:: State,
    routing::get,
    Router,
};
  
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

// 1.Define the Application State
// This struct holds our database pool. Axum will pass this around securely
// In a Production app, you might also add Redis or a Solana RPC client here.
#[derive(Clone)]
struct AppState{
    db: PgPool
}

// handing the control of the main fuction to tokio to execute it asynchronously on 
// multiple CPU cores

#[tokio::main]
async fn main(){
    // 2. Load the .env file
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be in the .env");

    // 3. Create the Database Pool
    // PgPoolOptions allows us to configure max connections.
    // sqlx internally wraps this pool in an Arc, making it thread-safe
    println!("Connecting to database...");

    let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(&db_url)
                .await
                .expect("Failed to connect to Postgres");
    
    println!("Database connected successfully!");

    // 4. Build the App State, pass the acutal db pool to the State
    let state = AppState{ db : pool };

    // 5. The Router with state Injection
    // We attach the state to the router using '.with_state(state)'
    let app = Router::new()
    .route("/health",get(health_check))
    .with_state(state);

    // 6.Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Igniton Engine running on port 3000");
    axum::serve(listener, app).await.unwrap();

}


// 7. The Handler with the State Extracter
// Notice 'State(state): State<AppState>'. Axum automatically pulls the pool 
// from the router and gives it to this function, this is the rust way to alternative to 
// Global variables
async fn health_check(State(state):State<AppState>)-> &'static str{
    // we can now safely use the state.db to query to database here.
    "Ignition Engine is healthy and connected to the DB!"
}