use axum::{body::Bytes, extract::State, http::StatusCode};
use candid::Principal;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize, Debug)]
pub struct CanisterStatus {
    pub canister_id: Principal,
    cycle_balance: i64,
    idle_cycles_burned_per_day: i64,
    memory_size: i64,
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
    sqlx::query!(
        r#"
        insert into canister_metrics (canister_id, cycle_balance, idle_cycles_burned_per_day, memory_size)
        VALUES ($1, $2, $3, $4)
        "#,
        canister_status.canister_id.to_text(),
        canister_status.cycle_balance,
        canister_status.idle_cycles_burned_per_day,
        canister_status.memory_size
    ).execute(&*pool).await.map_err(|e| {
        println!("🛑 Failed to insert metrics: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(())
}
