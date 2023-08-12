cargo build --release --target x86_64-unknown-linux-musl 

docker rm receive_canister_metrics_from_fleet_and_push_to_timeseries_database

docker build . --tag receive_canister_metrics_from_fleet_and_push_to_timeseries_database

docker run --env-file .env.docker --detach --publish 3000:3000 --name receive_canister_metrics_from_fleet_and_push_to_timeseries_database --link timescaledb receive_canister_metrics_from_fleet_and_push_to_timeseries_database