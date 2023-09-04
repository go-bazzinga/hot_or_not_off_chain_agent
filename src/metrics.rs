use std::time::{SystemTime, UNIX_EPOCH};

use axum::{body::Bytes, extract::State, http::StatusCode};
use candid::Principal;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize, Debug)]
pub struct CanisterStatus {
    pub canister_id: Principal,
    pub cycle_balance: i64,
    pub idle_cycles_burned_per_day: i64,
    pub memory_size: i64,
    pub timestamp: SystemTime,
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

    // println!("✅ Received metrics: {:?}", status);

    insert_value_to_database(status, State(pool)).await?;

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

async fn insert_value_to_database(
    canister_status: CanisterStatus,
    pool: State<PgPool>,
) -> Result<(), StatusCode> {
    let microseconds_since_unix_epoch = canister_status
        .timestamp
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get microseconds from canister timestamp")
        .as_micros() as i64;
    let naive_date_time = NaiveDateTime::from_timestamp_micros(microseconds_since_unix_epoch)
        .expect("Failed to convert microseconds to NaiveDateTime");
    let timestamp_date_time = DateTime::<Utc>::from_naive_utc_and_offset(naive_date_time, Utc);

    sqlx::query!(
        r#"
        insert into canister_metrics (canister_id, cycle_balance, idle_cycles_burned_per_day, memory_size, recorded_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        canister_status.canister_id.to_text(),
        canister_status.cycle_balance,
        canister_status.idle_cycles_burned_per_day,
        canister_status.memory_size,
        timestamp_date_time
    ).execute(&*pool).await.map_err(|e| {
        println!("🛑 Failed to insert metrics: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(())
}
