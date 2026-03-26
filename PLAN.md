# Plan - phenotype-shared

## Phase 1: Core Crates (Complete)

| Task | Description | Status |
|------|-------------|--------|
| P1.1 | Event sourcing crate | Done |
| P1.2 | Cache adapter crate | Done |
| P1.3 | Policy engine crate | Done |
| P1.4 | State machine crate | Done |
| P1.5 | CI workflow | Done |

## Phase 2: Governance and Long-Term Cleanup (In Progress)

| Task | Description | Status |
|------|-------------|--------|
| P2.1 | Governance baseline PR scope | In Progress |
| P2.2 | Wrap-over-hand-roll rules and extraction guidance | In Progress |
| P2.3 | Prioritized shared-extraction backlog | In Progress |
| P2.4 | Repo-structure normalization and worktree placement rules | In Progress |
| P2.5 | ADR promotion path and canonical status source | In Progress |

## Phase 2 Backlog

### P0 — Governance baseline
1. Standardize the `phenotype-shared` governance PR as the canonical baseline for CI, linting, build checks, and policy gates.
2. Keep branch-protection-aligned checks lightweight, deterministic, and repo-local where possible.
3. Add ADR-driven decision records for governance, layout, and extraction policies.

### P1 — Shared extraction
1. Extract pagination primitives and response wrappers first.
2. Extract reusable error-mapping helpers next, without collapsing bounded-context-specific errors.
3. Promote only stable, technology-neutral port contracts into `phenotype-port-interfaces`.
4. Keep adapters and workflow-specific glue local until repetition proves reuse.

### P1 — Repo structure
1. Normalize top-level repo buckets to make intent obvious: `apps/`, `libs/`, `infrastructure/`, `governance/`, `tooling/`, `templates/`.
2. Keep active worktrees under `repos/worktrees/<project>/<category>/<wtree>` and out of canonical repo roots.
3. Prefer shallow, discoverable directories over deeply nested ad hoc layouts.

### P2 — Canonical status and promotion path
1. Keep one active governance status source in `plans/` and treat it as the canonical backlog.
2. Promote ADRs when decisions change repo layout, extraction boundaries, or shared contracts.
3. Revisit the long-term cleanup backlog after each governance PR lands.
