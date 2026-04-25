# SPEC — Ich brauche eine (Caffeine Tracker)

## Overview
A personal daily caffeine tracker web app. The user can log coffee and other caffeinated drinks throughout the day, see the running total caffeine intake in mg, and track their history over time. The app ships with a curated list of common drink presets (espresso, cappuccino, latte, filter coffee, energy drink, etc.) and allows the user to add custom drink types. A daily limit indicator (based on the commonly recommended 400 mg/day ceiling) provides at-a-glance feedback on current intake.

## Data Model

- **DrinkType** (id uuid pk, name text not null, caffeine_mg integer not null, emoji text not null default '☕', is_preset boolean not null default false, user_id uuid nullable fk→auth.users)
  - Relationships: user_id is null for global presets; non-null for user-created custom types.
  - Constraint: unique(name, user_id) — a user cannot create two custom types with the same name.

- **DrinkLog** (id uuid pk, user_id uuid not null fk→auth.users, drink_type_id uuid not null fk→DrinkType, consumed_at timestamptz not null default now(), notes text nullable)
  - Relationships: many DrinkLog rows per user per day.

Row-Level Security:
- DrinkType: SELECT allowed for rows where is_preset = true OR user_id = auth.uid(); INSERT/UPDATE/DELETE only where user_id = auth.uid().
- DrinkLog: all operations only where user_id = auth.uid().

Seed data (inserted once as presets, user_id = null):
| name | caffeine_mg | emoji |
|---|---|---|
| Espresso (single) | 63 | ☕ |
| Espresso (double) | 125 | ☕ |
| Cappuccino | 63 | ☕ |
| Latte Macchiato | 63 | 🥛 |
| Filterkaffee (250 ml) | 90 | ☕ |
| Americano | 77 | ☕ |
| Cold Brew (250 ml) | 155 | 🧊 |
| Energy Drink (250 ml) | 80 | ⚡ |
| Schwarztee (200 ml) | 40 | 🍵 |
| Cola (330 ml) | 35 | 🥤 |

## API Surface

- `GET /api/drink-types`
  - Auth: required
  - Returns: all preset DrinkTypes + the authenticated user's custom DrinkTypes
  - Output: `[{id, name, caffeine_mg, emoji, is_preset}]`

- `POST /api/drink-types`
  - Auth: required
  - Input: `{name: text, caffeine_mg: integer, emoji: text}`
  - Output: created DrinkType
  - Errors: 400 if caffeine_mg ≤ 0; 409 if name already exists for this user

- `DELETE /api/drink-types/:id`
  - Auth: required, owner only
  - Errors: 403 if not owner or is preset; 404 if not found

- `GET /api/drinks/today`
  - Auth: required
  - Returns: all DrinkLog rows for today (UTC date) with joined DrinkType data, plus computed `total_caffeine_mg`
  - Output: `{entries: [{id, drink_type: {name, emoji, caffeine_mg}, consumed_at, notes}], total_caffeine_mg: integer, daily_limit_mg: 400}`

- `GET /api/drinks/history?days=30`
  - Auth: required
  - Returns: daily summaries for the last N days (default 30, max 90)
  - Output: `[{date: "YYYY-MM-DD", total_caffeine_mg, drink_count}]`

- `POST /api/drinks`
  - Auth: required
  - Input: `{drink_type_id: uuid, consumed_at?: timestamptz, notes?: text}`
  - Output: created DrinkLog with joined DrinkType
  - Errors: 404 if drink_type_id not found/accessible

- `DELETE /api/drinks/:id`
  - Auth: required, owner only
  - Errors: 403 if not owner; 404 if not found

## UI Pages

- `/` — **Today's Dashboard** (auth required)
  - Headline: current total caffeine in mg with animated counter
  - Progress bar: 0–400 mg; green < 200 mg, yellow 200–350 mg, red > 350 mg
  - Quick-add grid: one button per drink type (preset + custom), tap to log instantly
  - Today's log list: each entry shows emoji, drink name, time, caffeine mg; swipe/button to delete
  - Floating "+ Custom" button to add an ad-hoc custom type
  - No drinks yet state: illustrated empty state with prompt to add first drink

- `/history` — **History View** (auth required)
  - List of past days (most recent first) with daily total and bar chart indicator
  - Tap a day to expand and see individual drinks
  - Shows last 30 days by default; load-more button for up to 90 days

- `/settings` — **Settings** (auth required)
  - List of user's custom drink types with edit/delete
  - Toggle: daily limit mg (default 400, user can customise 200–800)
  - Account section: display email, logout button

- `/login` — **Login** (public)
  - Email + password form
  - Link to register
  - [ASSUMPTION: no OAuth for now — keeps deployment simple for a personal tool]

- `/register` — **Register** (public)
  - Email + password + confirm password
  - On success: redirect to `/`

## Auth
- Provider: Supabase Auth
- Methods: Email + password
- JWT used for all API calls via `Authorization: Bearer <token>` header
- Row-Level Security enforced at database level for all tables
- [ASSUMPTION: single-user personal tool — no org/tenant isolation needed beyond per-user RLS]

## Third-Party Services
- Stripe: no
- Email: no (Supabase handles transactional auth emails via its built-in SMTP)
- Notion: no
- CSV import: no

## Target Server
default (IP: 91.98.146.113)
Live at: https://ich-brauche-eine.mader.xyz

## Open Questions
- [ASSUMPTION: Daily limit is 400 mg — the widely cited safe adult maximum. User can override in Settings.]
- [ASSUMPTION: "Today" is based on the server's UTC date. If user is in a different timezone this could feel off. A future improvement would be to send client timezone.]
- [ASSUMPTION: No push notifications or reminders for now — out of scope for v1.]
- [ASSUMPTION: Mobile-first responsive design since this is a phone-habit-tracking use case.]
- [ASSUMPTION: No social features, sharing, or multi-user households needed.]
