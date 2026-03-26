# Language-Agnostic Hexagonal Architecture Specification

Extracted from the AgilePlus domain implementation (`crates/agileplus-domain`).
This document is the canonical reference for implementing AgilePlus domain logic
in any language (Go, Python, TypeScript, Java, etc.).

---

## 1. Architecture Overview

The system follows Ports and Adapters (Hexagonal Architecture):

```
+--------------------------------------------------+
|                  Application Layer               |
|   (orchestrates domain, owns use-case flows)     |
+--------------------------------------------------+
         |                         |
         v                         v
+------------------+    +----------------------+
|   Domain Layer   |    |      Port Layer       |
|  Entities, State |    |  Interfaces defining  |
|  Machines, Events|    |  external boundaries  |
+------------------+    +----------------------+
                                   |
                         +---------+---------+
                         |                   |
                    Adapters             Adapters
                  (Storage,           (VCS, Agents,
                  SQLite)            Review, Telemetry)
```

### Layers

| Layer | Responsibility | May Depend On |
|-------|----------------|---------------|
| Domain | Core entities, state machines, domain events, error types | Nothing external |
| Ports | Interface contracts for I/O (storage, VCS, agents, review, observability) | Domain types only |
| Adapters | Concrete implementations of port interfaces | Domain, Ports, external libs |
| Application | Use-case orchestration, command handlers, CLI/API entry points | All layers |

**Dependency Rule**: The domain layer must never import from ports, adapters, or
the application layer. All dependencies point inward.

---

## 2. Core Abstractions

| Pattern | Description | Notes |
|---------|-------------|-------|
| Entity | Has an identity (`id: i64`), mutable, stamped with `created_at` / `updated_at` | `id=0` signals "not yet persisted" |
| AggregateRoot | Entity that owns its consistency boundary; exposes `transition()` for state changes | Feature, WorkPackage, Cycle |
| ValueObject | Immutable; equality by value not identity | `FeatureState`, `WpState`, `CycleState`, `spec_hash` |
| DomainEvent | Immutable record of a state mutation; forms a hash-chain per entity stream | `Event` struct with `prev_hash` / `hash` |
| Port | Language-level interface (trait/interface/protocol) for an external capability | `StoragePort`, `VcsPort`, `AgentPort`, `ReviewPort`, `ContentStoragePort`, `ObservabilityPort` |
| StateMachine | Value type attached to an aggregate; `transition(target)` returns error on illegal move | `FeatureState`, `WpState`, `CycleState` |
| Error | Typed, structured, discriminated error taxonomy; no stringly-typed generic errors | `DomainError` enum with variants |
| AppContext | Wire-up struct injecting concrete adapters into the application layer | Holds references to all port implementations |

---

## 3. Entity Catalog

### 3.1 Feature

The central aggregate root tracking a product feature from ideation to retrospective.

| Field | Type | Description |
|-------|------|-------------|
| id | i64 | Surrogate key; 0 = not yet persisted |
| slug | String | Kebab-case URL-safe identifier derived from `friendly_name` |
| friendly_name | String | Human-readable display name |
| state | FeatureState | Current lifecycle state (see 4.1) |
| spec_hash | [u8; 32] | SHA-256 of the feature spec document; serialized as 64-char hex |
| target_branch | String | Git branch for integration; defaults to "main" |
| plane_issue_id | String? | External Plane.so issue ID (nullable) |
| plane_state_id | String? | External Plane.so state UUID (nullable) |
| labels | String[] | Free-form tags synced with external trackers |
| module_id | i64? | Strict owning module (one feature belongs to one module) |
| project_id | i64? | Owning project for multi-project environments |
| created_at_commit | String? | Git SHA at creation time |
| last_modified_commit | String? | Git SHA of most recent modification |
| created_at | Timestamp | UTC creation time |
| updated_at | Timestamp | UTC last modification time |

**Invariants**:
- `slug` must be globally unique within a project.
- `slug` is derived by lowercasing `friendly_name`, replacing non-alphanumeric characters with `-`, collapsing consecutive dashes.
- `transition(target)` is the only valid way to mutate `state`. Direct assignment is prohibited.
- `spec_hash` is a content-addressable fingerprint; changing the spec document changes the hash.

### 3.2 WorkPackage

A scoped unit of implementation work within a Feature. Multiple WorkPackages decompose a Feature.

| Field | Type | Description |
|-------|------|-------------|
| id | i64 | Surrogate key |
| feature_id | i64 | Owning feature (FK) |
| title | String | Short description of deliverable |
| state | WpState | Current state (see 4.2) |
| sequence | i32 | Ordering within the feature |
| file_scope | String[] | Glob patterns / paths the WP is responsible for |
| acceptance_criteria | String | Prose description of done criteria |
| agent_id | String? | ID of the agent assigned to this WP |
| pr_url | String? | URL of the associated pull request |
| pr_state | PrState? | Open | Review | ChangesRequested | Approved | Merged |
| worktree_path | String? | Filesystem path of the git worktree |
| plane_sub_issue_id | String? | External Plane.so sub-issue ID |
| base_commit | String? | Git SHA at which the worktree was branched from main |
| head_commit | String? | Git SHA of the most recent commit on the WP branch |
| created_at | Timestamp | |
| updated_at | Timestamp | |

**Operations**:
- `transition(target: WpState)`: guards allowed edges (see 4.2).
- `has_file_overlap(other: WorkPackage) -> String[]`: returns files present in both `file_scope` lists.

### 3.3 Cycle

A time-boxed delivery sprint grouping multiple Features.

| Field | Type | Description |
|-------|------|-------------|
| id | i64 | Surrogate key |
| name | String | Human-readable sprint name |
| description | String? | Optional description |
| state | CycleState | Draft | Active | Review | Shipped | Archived |
| start_date | Date | Inclusive start (calendar date, no time) |
| end_date | Date | Exclusive end; must be strictly after start_date |
| module_scope_id | i64? | If set, only features owned by this module may be added |
| created_at | Timestamp | |
| updated_at | Timestamp | |

**Invariants**:
- `end_date > start_date` is enforced at construction time.
- Module-scoped cycles reject features outside that module's ownership.

### 3.4 Module

A logical product area grouping Features hierarchically.

| Field | Type | Description |
|-------|------|-------------|
| id | i64 | Surrogate key |
| slug | String | Kebab-case identifier |
| friendly_name | String | Display name |
| description | String? | Optional description |
| parent_module_id | i64? | Parent module for hierarchy (null = root) |
| created_at | Timestamp | |
| updated_at | Timestamp | |

**Relationships**:
- `ModuleFeatureTag(module_id, feature_id)`: many-to-many tagging join.
- A Feature may have strict ownership (`Feature.module_id`) and/or be tagged to additional modules.
- Circular parent references are forbidden (DomainError::CircularModuleRef).

### 3.5 Event (Domain Event)

An immutable audit record forming a hash-chain per entity stream.

| Field | Type | Description |
|-------|------|-------------|
| id | i64 | Surrogate key |
| entity_type | String | "feature" | "work_package" | "cycle" | etc. |
| entity_id | i64 | ID of the entity this event belongs to |
| event_type | String | Semantic name e.g. "state_transitioned" |
| payload | JSON | Arbitrary structured context |
| actor | String | Agent ID or user identifier who triggered the event |
| timestamp | Timestamp | UTC event time |
| prev_hash | [u8; 32] | Hash of the previous event in this entity's stream |
| hash | [u8; 32] | Content hash of this event (computed by the store) |
| sequence | i64 | Monotonic ordinal within the entity stream |

**Invariants**:
- Events are append-only; no update or delete.
- `prev_hash` of the first event in a stream is `[0u8; 32]`.
- The store is responsible for computing and verifying the hash chain.

---

## 4. State Transition Tables

### 4.1 FeatureState

Ordinal-ordered linear progression. Forward-only; backward transitions are rejected.
Skip transitions (e.g. Created -> Planned) are allowed; skipped states are recorded.

| Ordinal | State | Description |
|---------|-------|-------------|
| 0 | Created | Feature record exists; no spec yet |
| 1 | Specified | Spec document written and hashed |
| 2 | Researched | Technical investigation completed |
| 3 | Planned | Work packages decomposed |
| 4 | Implementing | Active agent development in progress |
| 5 | Validated | All WPs done, acceptance criteria met |
| 6 | Shipped | Merged to target branch and released |
| 7 | Retrospected | Post-ship retrospective recorded |

**Transition rule**: `target.ordinal > current.ordinal` is required. Any pair satisfying
this is valid. Transitions that skip intermediate states record them in `TransitionResult.skipped`.

### 4.2 WpState

Graph-based (not linear). Supports bidirectional edges for review cycles and blocking.

| From | To | Notes |
|------|----|-------|
| Planned | Doing | Work started |
| Planned | Blocked | Dependency not met before start |
| Doing | Review | Agent submitted PR |
| Doing | Blocked | External blocker encountered during work |
| Review | Done | PR approved and merged |
| Review | Doing | Changes requested; agent resumes work |
| Blocked | Planned | Blocker resolved, back to queue |
| Blocked | Doing | Blocker resolved, resume immediately |

All other transitions are invalid.

### 4.3 CycleState

Graph-based. Supports limited reversion to handle replanning.

| From | To | Notes |
|------|----|-------|
| Draft | Active | Cycle officially begins |
| Active | Review | Sprint complete, under review |
| Active | Draft | Cycle reopened for changes |
| Review | Shipped | Gate: all features validated (enforced by service layer) |
| Review | Active | Review rejected, back to active |
| Shipped | Archived | Cycle closed permanently |

Self-to-self transitions return `DomainError::NoOpTransition`.

---

## 5. Port Catalog

Ports are interfaces. The domain defines them; adapters implement them.
All async methods must be non-blocking and thread-safe (Send + Sync in Rust).
Observability methods are synchronous (fire-and-forget).

### 5.1 StoragePort

Persistence abstraction for all domain entities. Primary implementation: SQLite adapter.

See `docs/hexagonal-ports.md` for the complete method catalog.

Purpose: CRUD and query access to Features, WorkPackages, Cycles, Modules, Events,
audit trail, governance contracts, metrics, and sync mappings.

### 5.2 VcsPort

Version control abstraction. Primary implementation: git2 Git adapter.

| Method | Signature | Description |
|--------|-----------|-------------|
| create_worktree | (feature_slug, wp_id) -> Path | Create isolated git worktree for a WP |
| list_worktrees | () -> WorktreeInfo[] | List active worktrees with branch/slug metadata |
| cleanup_worktree | (path) -> () | Remove a worktree when WP completes |
| create_branch | (branch_name, base) -> () | Create branch from base ref |
| checkout_branch | (branch_name) -> () | Switch HEAD to branch |
| merge_to_target | (source, target) -> MergeResult | Merge source branch into target |
| detect_conflicts | (source, target) -> ConflictInfo[] | Dry-run conflict detection |
| read_artifact | (feature_slug, relative_path) -> String | Read feature artifact file |
| write_artifact | (feature_slug, relative_path, content) -> () | Write feature artifact file |
| artifact_exists | (feature_slug, relative_path) -> bool | Check artifact existence |
| scan_feature_artifacts | (feature_slug) -> FeatureArtifacts | Discover meta, audit, evidence files |

### 5.3 AgentPort

AI agent dispatch abstraction. Supports multiple backends (ClaudeCode, Codex).

| Method | Signature | Description |
|--------|-----------|-------------|
| dispatch | (task, config) -> AgentResult | Spawn agent, block until completion |
| dispatch_async | (task, config) -> job_id | Spawn agent, return job handle immediately |
| query_status | (job_id) -> AgentStatus | Poll job status (Pending/Running/WaitingForReview/Completed/Failed) |
| cancel | (job_id) -> () | Cancel a running or pending job |
| send_instruction | (job_id, instruction) -> () | Send mid-execution instruction to agent |

**AgentTask fields**: `wp_id`, `feature_slug`, `prompt_path`, `worktree_path`, `context_files[]`

**AgentConfig fields**: `kind` (ClaudeCode | Codex), `max_review_cycles`, `timeout_secs`, `extra_args[]`

### 5.4 ReviewPort

Code review abstraction. Supports Coderabbit, GitHub API, and manual review adapters.

| Method | Signature | Description |
|--------|-----------|-------------|
| get_review_status | (pr_url) -> ReviewStatus | Current review status of a PR |
| get_review_comments | (pr_url) -> ReviewComment[] | All comments on a PR |
| get_actionable_comments | (pr_url) -> ReviewComment[] | Only comments requiring action |
| get_ci_status | (pr_url) -> CiStatus | CI run status for a PR |
| get_pr_info | (pr_url) -> PrInfo | Full PR summary including state and CI |
| await_review | (pr_url, timeout_secs) -> ReviewStatus | Block until review completes or times out |
| await_ci | (pr_url, timeout_secs) -> CiStatus | Block until CI completes or times out |

**ReviewStatus variants**: Pending | InProgress | Approved | ChangesRequested(comments[]) | Rejected(reason)

**CiStatus variants**: Pending | Running | Passed | Failed(logs_url) | Cancelled

### 5.5 ContentStoragePort

Simplified storage interface focused on content operations (features, backlog, WPs).
Used by content-centric adapters that do not need the full governance/metric surface.

| Method Group | Methods |
|-------------|---------|
| Feature CRUD | create_feature, get_feature_by_slug, get_feature_by_id, update_feature_state, update_feature, list_features_by_state, list_all_features |
| Backlog | create_backlog_item, get_backlog_item, list_backlog_items(filters), update_backlog_status, update_backlog_priority, pop_next_backlog_item |
| WorkPackage CRUD | create_work_package, get_work_package, update_wp_state, update_work_package, list_wps_by_feature |
| WP Dependencies | add_wp_dependency, get_wp_dependencies, get_ready_wps |

### 5.6 ObservabilityPort

Telemetry abstraction. All methods are synchronous; telemetry must never block business logic.
Primary implementation: OpenTelemetry adapter.

| Method Group | Methods |
|-------------|---------|
| Tracing | start_span(name, parent?) -> SpanContext, end_span(ctx), add_span_event(ctx, name, attrs[]), set_span_error(ctx, error) |
| Metrics | record_counter(name, value, labels[]), record_histogram(name, value, labels[]), record_gauge(name, value, labels[]) |
| Logging | log(entry), log_info(message), log_warn(message), log_error(message) |

**MetricValue**: Counter(u64) | Histogram(f64) | Gauge(f64)

**LogLevel**: Trace | Debug | Info | Warn | Error

---

## 6. Error Taxonomy

All errors are typed variants of a single `DomainError` discriminated union. No string-typed generic errors.

| Variant | Fields | Meaning |
|---------|--------|---------|
| NotImplemented | - | Stub not yet filled in |
| InvalidTransition | from, to, reason | State machine rejected the edge |
| NoOpTransition | current_state | Self-to-self transition attempted |
| NotFound | description | Entity lookup returned no result |
| Storage | message | Persistence layer error |
| Vcs | message | Version control error |
| Agent | message | AI agent error |
| Review | message | Code review error |
| Timeout | seconds | Operation exceeded time limit |
| Conflict | message | Concurrent modification conflict |
| Other | message | Uncategorized; use sparingly |
| ModuleNotFound | slug | Module-specific not-found |
| CircularModuleRef | child, ancestor | Hierarchy would form a cycle |
| ModuleHasDependents | description | Cannot delete module with children |
| FeatureNotInModuleScope | feature_slug, module_slug | Feature outside module boundary |
| CycleNotFound | name | Cycle-specific not-found |
| CycleGateNotMet | message | Pre-condition for cycle transition not satisfied |

---

## 7. Language Implementation Notes

### 7.1 Async Semantics

All port methods (except ObservabilityPort) are async. The signature convention:

```
-- Rust:
fn method_name(&self, arg: T) -> impl Future<Output = Result<R, DomainError>> + Send

-- Go:
MethodName(ctx context.Context, arg T) (R, error)

-- Python (asyncio):
async def method_name(self, arg: T) -> R  # raises DomainError subtypes

-- TypeScript:
methodName(arg: T): Promise<R>  // throws or rejects with DomainError
```

ObservabilityPort methods are synchronous in all languages.

### 7.2 Entity Serialization

- Timestamps: UTC ISO 8601 (`2026-01-01T00:00:00Z`).
- Dates (Cycle): `YYYY-MM-DD` calendar date, no time component.
- `spec_hash`: 64-character lowercase hex string (32 bytes).
- Enums: snake_case serialization for WpState and FeatureState; PascalCase for CycleState.
- Nullable fields: omit from serialized form when null/None (do not emit `null`).

### 7.3 Interface Syntax by Language

```
Rust:   pub trait StoragePort: Send + Sync { ... }
Go:     type StoragePort interface { ... }
Python: class StoragePort(Protocol): ...   # or ABC
TS:     interface StoragePort { ... }
Java:   public interface StoragePort { ... }
```

### 7.4 Slug Generation

All entities deriving a slug from a name use this algorithm:
1. Lowercase all characters.
2. Replace any non-alphanumeric character with `-`.
3. Split on `-`, discard empty segments.
4. Re-join with `-`.

Example: `"Hello  World--"` -> `"hello-world"`.

### 7.5 Identity Convention

Entities use integer surrogate keys (`i64`). The value `0` is reserved to mean
"not yet persisted." After a successful `create_*` call, the returned `i64` is
the assigned ID and must be written back to the entity.

### 7.6 Thread Safety

All port implementations must be safe to share across concurrent tasks/goroutines/threads.
In Rust this means `Send + Sync`. In Go, implementations must be goroutine-safe.
In Python (asyncio), implementations must be coroutine-safe.
