drop table if exists day1input;

create table if not exists day1input(
    id SERIAL PRIMARY KEY,
    calories int
);

\copy day1input(calories) from 'day1.txt' (FORMAT CSV);

create sequence if not exists groupid as int;

with grouped as (
    select 
      case when calories is null then nextval('groupid') else currval('groupid') end as group_id,
      calories
    from day1input
)
select group_id, sum(calories) as total
  from grouped
  group by group_id
  order by total desc
  limit 1;
