
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use crate::AppState;


#[derive(Deserialize)]
pub struct SubmitTransactionRequest{
    pub wallet_address: String,
    pub amount : u64,
}

#[derive(Serialize)]
pub struct SubmitTransactionResponse{
    pub status: String,
    pub message:String,
}

pub async fn submit_transaction(
    State(state):State<AppState>,
    Json(payload):Json<SubmitTransactionRequest>
) -> (StatusCode, Json<SubmitTransactionResponse>){
    println!("Received transaction request from: {}",payload.wallet_address);

    let response = SubmitTransactionResponse{
        status: String::from("pending"),
        message: format!("Transaction queued for {} SOL",payload.amount),
    };

    (StatusCode::ACCEPTED, Json(response))
}

