# Security Notes

## Known Advisories

### RUSTSEC-2023-0071 — rsa 0.9.x Marvin Attack (Medium, 5.9)

- **Crate**: `rsa` 0.9.10 (transitive dependency via `jsonwebtoken`)
- **Description**: Potential key recovery through timing side-channels (Marvin Attack)
- **Impact**: None. This application uses only HMAC-SHA256 (HS256) for JWT signing. No RSA keys are used anywhere. The `rsa` crate is compiled as part of `jsonwebtoken`'s full feature set but is never invoked.
- **Fix**: No safe version of `rsa` that resolves this advisory is currently available. Update `jsonwebtoken` when a version that removes the dependency or patches the flaw is released.
