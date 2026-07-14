# AGENTS — SecDevOps Framework

Adapted from Synapsis Tier S++ SecDevOps.

## Core Principles

1. **Shift Left**: Security is everyone's responsibility, not just security team
2. **Zero Trust**: Never trust, always verify — every request, every time
3. **Defense in Depth**: Multiple layers of security controls
4. **Immutable Audit**: Every action is logged and verifiable

## Security Gates (CI/CD)

| # | Gate | Tool | Severity |
|---|------|------|----------|
| 1 | Code Formatting | ruff format | P1 |
| 2 | Linting | ruff check | P1 |
| 3 | Type Checking | mypy | P1 |
| 4 | Unit Tests | pytest / vitest | P0 |
| 5 | SAST | bandit | P0 |
| 6 | Dependency Audit | pip-audit | P0 |
| 7 | Secret Scanning | gitleaks | P0 |
| 8 | SBOM | cyclonedx-bom | P1 |
| 9 | Container Scan | trivy | P0 |
| 10 | License Audit | deny.toml | P1 |
| 11 | Aggregated Gate | ci-status | P0 |

## Branch Strategy

- `main` — production releases only, protected, requires all CI passing
- `develop` — integration branch, requires tests + lint
- `feat/*` — feature branches
- `fix/*` — bug fix branches
- `security/*` — security patches (P0 = immediate merge)

## PR Lifecycle

1. Create PR with description
2. All CI gates must pass
3. <500 lines changed, <20 files
4. No breaking changes without discussion
5. Security changes require review
6. Auto-merge when all gates pass

## Severity Levels

- **P0 (Critical)**: Security vulnerabilities, broken tests, data loss
- **P1 (High)**: Lint failures, type errors, missing tests
- **P2 (Medium)**: Performance issues, code quality
- **P3 (Low)**: Documentation, style, minor improvements
