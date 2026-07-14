# PsicoMetrics

Official personality assessment platform with 4 scientifically validated tests.

## Tests Included

| Test | Items | Time | Standard |
|------|-------|------|----------|
| Big Five / OCEAN | 120 | ~25 min | IPIP-NEO-120 |
| MBTI | 72 | ~15 min | Myers-Briggs |
| Enneagram | 81 | ~20 min | Riso-Hudson |
| DISC | 28 | ~8 min | Marston DISC |

## Tech Stack

- **Backend**: Python 3.12 + FastAPI + PostgreSQL
- **Frontend**: React 18 + Vite + TypeScript + Tailwind CSS
- **Security**: Tier S++ SecDevOps (adapted from Synapsis)

## Quick Start

```bash
# Start all services
docker compose up -d

# Backend API: http://localhost:8000
# Frontend: http://localhost:3000
# API Docs: http://localhost:8000/api/docs (debug mode)
```

## Security Features

- CSP, HSTS, X-Frame-Options headers
- CSRF double-submit cookie protection
- Rate limiting (token bucket)
- Audit logging with hash chain integrity
- CSPRNG for all random generation
- Docker: non-root user, read-only filesystem, no-new-privileges

## Development

```bash
# Backend
cd backend
pip install -e ".[dev]"
uvicorn app.main:app --reload

# Frontend
cd frontend
npm install
npm run dev
```

## CI/CD

11 security gates in GitHub Actions:
1. Lint (ruff)
2. Type check (mypy)
3. Tests (pytest + vitest)
4. SAST (bandit)
5. Dependency audit (pip-audit)
6. Secret scanning (gitleaks)
7. SBOM (CycloneDX)
8. Container scan (Trivy)
9. License audit
10. Aggregated gate

## Scientific Basis

All tests are based on validated psychological instruments:
- **Big Five**: IPIP-NEO-120 (International Personality Item Pool)
- **MBTI**: Myers-Briggs Type Indicator framework
- **Enneagram**: Riso-Hudson Enneagram Type Indicator
- **DISC**: William Moulton Marston's DISC theory
