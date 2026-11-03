
use axum::extract::State;
use crate::AppState;
// 7. The Handler with the State Extracter
// Notice 'State(state): State<AppState>'. Axum automatically pulls the pool 
// from the router and gives it to this function, this is the rust way to alternative to 
// Global variables
pub async fn health_check(State(state):State<AppState>)-> &'static str{
    // we can now safely use the state.db to query to database here.
    "Ignition Engine is healthy and connected to the DB!"
}