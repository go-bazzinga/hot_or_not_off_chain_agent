use axum::{
    body::Bytes,
    extract::State,
    http::{status, StatusCode},
};
use candid::Principal;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize, Debug)]
pub struct CanisterStatus {
    pub canister_id: Principal,
    cycle_balance: u128,
    idle_cycles_burned_per_day: u128,
    memory_consumed: u128,
}

#[derive(Deserialize, Debug)]
pub struct CanisterStatusError {
    pub canister_id: Principal,
    pub error_message: String,
}

pub async fn receive_metrics(State(pool): State<PgPool>, body: Bytes) -> Result<(), StatusCode> {
    // TODO: deduplicate metrics
    // TODO: verify metric coming from a fleet canister

    let status = parse_metrics(body)?;

    Ok(())
}

fn parse_metrics(body: Bytes) -> Result<CanisterStatus, StatusCode> {
    let body = rmp_serde::from_slice::<Result<CanisterStatus, CanisterStatusError>>(&body)
        .map_err(|e| {
            println!("🛑 Failed to deserialize body: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let status = body.map_err(|e| {
        println!(
            "🛑 Canister returned an error: {}: {}",
            e.canister_id.to_text(),
            e.error_message
        );
        StatusCode::BAD_REQUEST
    })?;

    Ok(status)
}

fn insert_values_to_database(canister_status: CanisterStatus) {}
