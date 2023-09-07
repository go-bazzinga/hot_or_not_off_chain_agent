-- Canister count from daily metrics
select count(distinct(canister_id)) as canister_count,
    date(recorded_at) as recorded_on
from canister_metrics
group by date(recorded_at)
order by date(recorded_at) desc;

-- Get the average of the idle cycles burned per day for each canister
select floor(avg(idle_cycles_burned_per_day) / 1000000),
    date(recorded_at),
    canister_id
from canister_metrics
group by date(recorded_at),
    canister_id
order by date(recorded_at) desc;

-- Get the average of the idle cycles burned per day for all canisters
select round(sum(canister_cycle_burn_per_day) / 1000000, 2) as all_individual_canister_cycle_burn_per_day,
    recorded_on
from (
        select floor(avg(idle_cycles_burned_per_day) / 1000000) as canister_cycle_burn_per_day,
            date(recorded_at) as recorded_on,
            canister_id
        from canister_metrics
        group by date(recorded_at),
            canister_id
        order by date(recorded_at) desc
    ) as canister_cycle_burn_segregated
group by recorded_on;

-- Get the average cycle balance per day for each canister
select floor(avg(cycle_balance) / 1000000),
    date(recorded_at),
    canister_id
from canister_metrics
group by date(recorded_at),
    canister_id
order by date(recorded_at) desc;

-- Get the average cycle balance per day for all canisters
select round(sum(canister_daily_cycle_balance) / 1000000, 2) as all_individual_canister_daily_cycle_balance,
    recorded_on
from (
        select floor(avg(cycle_balance) / 1000000) as canister_daily_cycle_balance,
            date(recorded_at) as recorded_on,
            canister_id
        from canister_metrics
        group by date(recorded_at),
            canister_id
        order by date(recorded_at) desc
    ) as canister_daily_cycle_balance_segregated
group by recorded_on;