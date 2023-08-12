FROM scratch
WORKDIR /app
COPY ./target/x86_64-unknown-linux-musl/release/receive_canister_metrics_from_fleet_and_push_to_timeseries_database .
CMD ["./receive_canister_metrics_from_fleet_and_push_to_timeseries_database"]