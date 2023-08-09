create table canister_metrics (
    canister_id text not null,
    cycle_balance bigint not null,
    idle_cycles_burned_per_day bigint not null,
    memory_size bigint not null,
    recorded_at timestamp with time zone not null default current_timestamp
);

select create_hypertable('canister_metrics', 'recorded_at');

create index index_canister_metrics_indexed_by_recorded_at_time on canister_metrics (canister_id, recorded_at desc);
