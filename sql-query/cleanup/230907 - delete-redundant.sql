select distinct(date(recorded_at))
from canister_metrics;

select count(*)
from canister_metrics;
-- 3rd - 23104845
-- 2nd - 25598242
-- 1st - 28148557
-- 31st - 30623747
-- 30th - 33085760
-- 29th - 34275801
-- 28th - 34440975
-- 27th - 34607574
-- 26th - 34733575
-- 25th - 34885077
-- 24th - 37073678
-- 23rd - 39669195
-- 22nd - 42262773
-- 21st - 44826377
-- 20th - 47399343
-- 19th - 49989797
-- 18th - 52099736
-- 17th - 52623515
delete from canister_metrics
where date(recorded_at) = '2023-09-02';