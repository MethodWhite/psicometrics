# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 1.0.x   | Yes       |

## Reporting a Vulnerability

If you discover a security vulnerability in PsicoMetrics, please report it responsibly:

- **Email**: methodwhite101@gmail.com
- **Response SLA**: 48 hours acknowledgment
- **Disclosure**: Coordinated disclosure after fix

## Security Architecture (Tier S++)

PsicoMetrics implements security practices adapted from the Synapsis ecosystem:

### Security Headers
- Content-Security-Policy (CSP)
- X-Content-Type-Options: nosniff
- X-Frame-Options: DENY
- Strict-Transport-Security (HSTS)
- Cross-Origin-Embedder-Policy: require-corp
- Cross-Origin-Opener-Policy: same-origin

### Authentication & Sessions
- Anonymous sessions with cryptographically secure IDs (CSPRNG)
- No PII collection
- IP addresses stored as SHA-256 hashes only

### Rate Limiting
- Token bucket algorithm per IP
- 30 requests/minute general
- 5 requests/minute for test submissions
- 10 failed attempts = 1 hour lockout

### CSRF Protection
- Double-submit cookie pattern
- 32-byte cryptographically random tokens
- SameSite=Strict cookies
- Token rotation every 30 minutes

### Audit Logging
- Immutable append-only audit trail
- SHA-256 hash chain integrity
- Records: test start, answers, completion, errors

### Data Privacy
- No user accounts or PII required
- Test results stored in-memory or encrypted database
- No third-party analytics or tracking
- GDPR-compliant by design (data minimization)

### Supply Chain Security
- Dependency scanning (pip-audit)
- Secret scanning (gitleaks)
- SBOM generation (CycloneDX)
- Container scanning (Trivy)
- License auditing (deny.toml pattern)
