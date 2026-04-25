# Frontend Context

## Status: COMPLETE

## What was built
Full Yew frontend for the caffeine tracker at `crates/ui/`.

## Pages
- `/` → `pages/home.rs` — Today's Dashboard: caffeine total, progress bar, quick-add grid, log list, custom drink modal
- `/history` → `pages/history.rs` — Last 30/90 days summary with expandable rows
- `/settings` → `pages/settings.rs` — Daily limit slider, custom drink management, logout
- `/login` → `pages/login.rs` — Email+password login via Supabase Auth REST API
- `/register` → `pages/register.rs` — Registration with confirm-password validation

## Components
- `components/progress_bar.rs` — Colour-coded bar (safe/warning/danger)
- `components/drink_button.rs` — Grid button for quick-logging a drink type
- `components/log_entry.rs` — Entry row with delete button
- `components/nav.rs` — Fixed bottom navigation (Heute / Verlauf / Einstellungen)
- `components/modal.rs` — Bottom-sheet modal overlay

## Key modules
- `types.rs` — Shared data types: DrinkType, DrinkEntry, TodayResponse, HistoryEntry, AuthResponse
- `utils.rs` — Pure functions: caffeine_color, progress_fraction, format_time, format_date (unit-tested)
- `auth.rs` — AuthState reducer, localStorage read/write (wasm32-guarded)
- `api.rs` — All HTTP calls via gloo-net (wasm32-guarded with native stubs)
- `routes.rs` — Route enum: Home, History, Settings, Login, Register, NotFound

## Auth flow
- Login/Register call Supabase Auth REST directly (POST /auth/v1/token, /auth/v1/signup)
- JWT stored in localStorage under `sb_token`, email under `sb_email`
- AuthState passed via Yew context (UseReducerHandle<AuthState>)
- Daily limit stored in localStorage under `daily_limit_mg` (default 400)

## Build
- `trunk build` in `crates/frontend/` — passes cleanly
- `cargo clippy -p ui --target wasm32-unknown-unknown -- -D warnings` — zero output
- Supabase URL: https://xoryyknrowwsodrdjtua.supabase.co
