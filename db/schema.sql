-- DrinkType: global presets (user_id = null) and user custom types
create table if not exists public."DrinkType" (
    id uuid primary key default gen_random_uuid(),
    name text not null,
    caffeine_mg integer not null check (caffeine_mg > 0),
    emoji text not null default '☕',
    is_preset boolean not null default false,
    user_id uuid references auth.users(id) on delete cascade,
    unique (name, user_id)
);

alter table public."DrinkType" enable row level security;

create policy "select_drink_types" on public."DrinkType"
    for select using (is_preset = true or user_id = auth.uid());

create policy "insert_drink_types" on public."DrinkType"
    for insert with check (user_id = auth.uid());

create policy "update_drink_types" on public."DrinkType"
    for update using (user_id = auth.uid());

create policy "delete_drink_types" on public."DrinkType"
    for delete using (user_id = auth.uid());

-- DrinkLog: per-user consumption entries
create table if not exists public."DrinkLog" (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references auth.users(id) on delete cascade,
    drink_type_id uuid not null references public."DrinkType"(id) on delete restrict,
    consumed_at timestamptz not null default now(),
    notes text
);

alter table public."DrinkLog" enable row level security;

create policy "select_drink_log" on public."DrinkLog"
    for select using (user_id = auth.uid());

create policy "insert_drink_log" on public."DrinkLog"
    for insert with check (user_id = auth.uid());

create policy "update_drink_log" on public."DrinkLog"
    for update using (user_id = auth.uid());

create policy "delete_drink_log" on public."DrinkLog"
    for delete using (user_id = auth.uid());
