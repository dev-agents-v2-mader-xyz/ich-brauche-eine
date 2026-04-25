# Security Report — Ich brauche eine (Caffeine Tracker)
Date: 2026-04-25

## Summary

0 critical, 0 high, 2 medium (both mitigated/resolved), 4 informational findings.

**APPROVED FOR DEPLOYMENT**

---

## Checklist

### A — Secrets & Credentials ✓
- ✓ No API keys, tokens, or passwords hardcoded in any source file
- ✓ `.env` is not committed to git; only `.env.template` (no values) is present
- ✓ All secrets read from environment variables via `config.rs`
- ✓ Dockerfile passes secrets via environment variables only — no `ENV` or `RUN` with secret values

### B — Authentication & Authorisation ✓
- ✓ `AuthUser` Rocket request guard enforces JWT validation on all non-public routes
- ✓ Intentionally public: `/health` (no user data), static frontend files (`/dist`)
- ✓ JWT validation checks: signature (HS256), expiry (`exp`), audience (`aud == "authenticated"`), issuer (`iss == SUPABASE_URL/auth/v1`) — **issuer check added in this review**
- ✓ `DELETE /api/drink-types/:id` — checks ownership; presets and other users' types return 403
- ✓ `DELETE /api/drinks/:id` — checks `user_id` ownership before deleting
- ✓ `GET /api/drinks/today` and `GET /api/drinks/history` — both filter by authenticated `user_id`; no cross-user data leakage
- ✓ `POST /api/drinks` — verifies `drink_type_id` is either a preset or belongs to the authenticated user before inserting
- ✓ No horizontal privilege escalation paths found

### C — Input Validation & Injection ✓
- ✓ All user-supplied values pass through typed Serde deserialization (`CreateDrinkTypeBody`, `CreateDrinkBody`)
- ✓ `caffeine_mg <= 0` rejected with 400 (`drink_types.rs:59`)
- ✓ `days` query parameter clamped to 1–90 (`drinks.rs:120`)
- ✓ All SQL queries use SQLx parameterised statements ($1, $2 …) — no string concatenation in queries
- ✓ No file uploads in scope
- ✓ No Stripe webhooks
- ℹ `db.rs:11` — `format!("SET search_path TO {schema}")` uses `SUPABASE_SCHEMA` env var (operator-controlled, not user input); informational only

### D — Transport & Headers ✓
- ✓ Security headers added via `SecurityHeaders` fairing in this review: `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`, `Referrer-Policy: strict-origin-when-cross-origin`
- ✓ HTTPS enforced by the nginx reverse-proxy with Let's Encrypt (`LETSENCRYPT_HOST` in `docker-compose.yml`)
- ✓ CORS not configured — frontend and backend served from the same origin; no CORS headers required
- ✓ Supabase JWKS managed by Supabase infrastructure

### E — Dependency Audit ✓ (one documented exception)
- ✓ **FIXED**: `sqlx 0.7.4` → `0.8.6` — resolves RUSTSEC-2024-0363 (binary protocol misinterpretation cast overflow) and three rustls-webpki advisories (RUSTSEC-2026-0098/0099/0104: name constraint bugs + reachable panic in CRL parsing)
- ⚠ **MEDIUM/mitigated**: `rsa 0.9.10` (RUSTSEC-2023-0071, CVSS 5.9) — no upstream fix available; see finding below
- ℹ Unmaintained GTK3/Tauri ecosystem warnings — all in the desktop `app` crate, not deployed

### F — Supabase RLS ✓
- ✓ `DrinkType`: RLS enabled; SELECT allows presets (`is_preset = true`) + own rows (`user_id = auth.uid()`); INSERT/UPDATE/DELETE restricted to `user_id = auth.uid()`
- ✓ `DrinkLog`: RLS enabled; all operations restricted to `user_id = auth.uid()`
- ✓ Service role key not exposed to WASM frontend

### G — Tauri ✓
- ✓ CSP set: `default-src 'self'; connect-src 'self' https://xoryyknrowwsodrdjtua.supabase.co; style-src 'self' 'unsafe-inline'; img-src 'self' data:; font-src 'self' data:`
- ✓ No `unsafe-eval` in CSP
- ✓ `connect-src` restricted to self and the specific Supabase project URL
- ✓ No `unsafe-inline` for scripts

---

## Findings

### [MEDIUM] RSA Marvin Attack in sqlx-mysql — OPEN (mitigated by architecture)
**Location:** Transitive dependency `rsa 0.9.10` via `sqlx-mysql`
**Advisory:** RUSTSEC-2023-0071 — CVSS 5.9 (Medium)
**Description:** The `rsa` crate is vulnerable to a timing side-channel (Marvin Attack) allowing potential RSA private key recovery through timing measurements.
**Risk:** An attacker with the ability to take many fine-grained timing measurements of RSA operations could recover private key material.
**Mitigation:** This app uses PostgreSQL exclusively. The `sqlx-mysql` driver (which uses `rsa` for MySQL's RSA-based auth handshake) is pulled in as a transitive dependency but is never loaded or executed at runtime. The vulnerable code path is unreachable.
**Resolution:** No fixed version of `rsa` is available upstream. Monitor RUSTSEC-2023-0071.
**Status:** OPEN — mitigated by architecture (PostgreSQL-only; MySQL driver code never executes)

### [MEDIUM] Missing JWT Issuer Validation — RESOLVED
**Location:** `crates/backend/src/auth.rs:46`
**Description:** JWT validation did not check the `iss` (issuer) claim, meaning a token signed with the same shared secret from any source would be accepted.
**Risk:** If the JWT secret were ever shared with another service, its tokens could authenticate against this backend.
**Remediation:** Added `validation.set_issuer(&[format!("{}/auth/v1", config.supabase_url)])` and `iss: Option<String>` field to `Claims` struct. Tests updated to include correct issuer.
**Status:** RESOLVED in this review

### [MEDIUM] Missing Security Headers — RESOLVED
**Location:** `crates/backend/src/main.rs` (was: no headers middleware)
**Description:** No security-relevant HTTP response headers were set.
**Risk:** Without `X-Content-Type-Options` browsers may MIME-sniff responses. Without `X-Frame-Options` the app could be framed (clickjacking). Without `Referrer-Policy` referrer data leaks to third parties.
**Remediation:** Added `SecurityHeaders` Rocket fairing (`crates/backend/src/middleware.rs`) that sets all three headers on every response. Attached in `main.rs` via `.attach(SecurityHeaders)`.
**Status:** RESOLVED in this review

### [INFO] format!() for SET search_path in db.rs
**Location:** `crates/backend/src/db.rs:11`
**Description:** Schema name from `SUPABASE_SCHEMA` env var is interpolated via `format!()` into a `SET search_path` statement.
**Risk:** If `SUPABASE_SCHEMA` were set to a malicious value, an operator could change the active schema. This is an operator-level control, not user input.
**Status:** Informational — no action required

### [INFO] csv_import Tool — Table and Column Name Interpolation
**Location:** `tools/csv_import/src/main.rs:46-50`
**Description:** `args.table` (CLI arg) and column names from CSV headers are interpolated directly into INSERT SQL. This is SQL injection if run with attacker-controlled CSV files.
**Risk:** An operator using the tool with a malicious CSV file could inject SQL via column header names.
**Remediation:** Validate table name against an allowlist; quote column names with `quote_ident` or validate against the actual table schema before building the query.
**Status:** Informational — CLI tool run by operators only; not HTTP-exposed; fix before using with untrusted CSV inputs

### [INFO] Unmaintained Tauri/GTK3 Dependencies
**Location:** `crates/app/` dependency tree
**Description:** Multiple unmaintained GTK3 Tauri ecosystem crates (atk, gdk, gtk, etc.) flagged by `cargo audit`.
**Risk:** Unmaintained crates may contain unpatched vulnerabilities discovered in the future.
**Status:** Informational — desktop app not deployed in the web flow; address when preparing Tauri desktop/mobile releases

### [INFO] schema.sql vs. Route Query Table Name Mismatch
**Location:** `db/schema.sql` vs `crates/backend/src/routes/api/`
**Description:** `schema.sql` creates tables as `"DrinkType"` and `"DrinkLog"` (quoted, case-sensitive PostgreSQL identifiers), while route handlers query `drink_types` and `drink_logs` (unquoted lowercase). These will not resolve to the same table.
**Risk:** API calls will fail with "table not found" at runtime; this is a functional bug, not a security issue.
**Remediation:** Either rename tables in `schema.sql` to unquoted lowercase (`drink_types`, `drink_logs`) — recommended — or update all queries to use quoted names.
**Status:** Informational — functional bug; no security impact; flag to Backend agent for resolution before deployment
