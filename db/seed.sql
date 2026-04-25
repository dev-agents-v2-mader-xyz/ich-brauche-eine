-- Preset drink types (user_id = null = global, available to all users)
insert into public."DrinkType" (name, caffeine_mg, emoji, is_preset, user_id)
values
    ('Espresso (single)',    63,  '☕', true, null),
    ('Espresso (double)',   125,  '☕', true, null),
    ('Cappuccino',           63,  '☕', true, null),
    ('Latte Macchiato',      63,  '🥛', true, null),
    ('Filterkaffee (250 ml)', 90, '☕', true, null),
    ('Americano',            77,  '☕', true, null),
    ('Cold Brew (250 ml)',  155,  '🧊', true, null),
    ('Energy Drink (250 ml)', 80, '⚡', true, null),
    ('Schwarztee (200 ml)',  40,  '🍵', true, null),
    ('Cola (330 ml)',        35,  '🥤', true, null)
on conflict (name, user_id) do nothing;
