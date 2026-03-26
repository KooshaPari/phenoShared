# Hexagonal Port Interface Catalog

Complete pseudocode definitions for all ports extracted from the AgilePlus domain layer.
Pseudocode uses a language-neutral notation: `async method(arg: Type) -> ReturnType`.
`?` suffix on a type means optional/nullable. `[]` suffix means list/array.
Errors raise/return `DomainError` unless noted.

---

## StoragePort

Persistence abstraction for all domain entities.
Implementations provide full CRUD access to Features, WorkPackages, Cycles,
Modules, Events, audit trail, governance contracts, metrics, and sync mappings.

```
interface StoragePort {

  // ---- Feature CRUD ----

  // Create a new feature record. Returns the assigned surrogate ID.
  async create_feature(feature: Feature) -> i64

  // Fetch a feature by URL slug. Returns null if not found.
  async get_feature_by_slug(slug: String) -> Feature?

  // Fetch a feature by surrogate ID. Returns null if not found.
  async get_feature_by_id(id: i64) -> Feature?

  // Update only the state field of an existing feature.
  async update_feature_state(id: i64, state: FeatureState) -> ()

  // Return all features currently in a given state.
  async list_features_by_state(state: FeatureState) -> Feature[]

  // Return all feature records regardless of state.
  async list_all_features() -> Feature[]


  // ---- WorkPackage CRUD ----

  // Create a new work package. Returns the assigned surrogate ID.
  async create_work_package(wp: WorkPackage) -> i64

  // Fetch a work package by ID. Returns null if not found.
  async get_work_package(id: i64) -> WorkPackage?

  // Update only the state field of an existing work package.
  async update_wp_state(id: i64, state: WpState) -> ()

  // Return all work packages belonging to a feature, ordered by sequence.
  async list_wps_by_feature(feature_id: i64) -> WorkPackage[]

  // Add a dependency edge between two work packages.
  async add_wp_dependency(dep: WpDependency) -> ()

  // Return all dependency edges for a given work package.
  async get_wp_dependencies(wp_id: i64) -> WpDependency[]

  // Return work packages for a feature whose dependencies are all Done.
  async get_ready_wps(feature_id: i64) -> WorkPackage[]


  // ---- Audit Trail ----

  // Append an immutable audit entry. Returns the assigned surrogate ID.
  async append_audit_entry(entry: AuditEntry) -> i64

  // Return the full audit trail for a feature in chronological order.
  async get_audit_trail(feature_id: i64) -> AuditEntry[]

  // Return the most recent audit entry for a feature. Returns null if none.
  async get_latest_audit_entry(feature_id: i64) -> AuditEntry?


  // ---- Evidence ----

  // Record a governance evidence artifact. Returns the assigned surrogate ID.
  async create_evidence(evidence: Evidence) -> i64

  // Return all evidence records linked to a work package.
  async get_evidence_by_wp(wp_id: i64) -> Evidence[]

  // Return all evidence records linked to a functional requirement ID.
  async get_evidence_by_fr(fr_id: String) -> Evidence[]


  // ---- Policy Rules ----

  // Persist a governance policy rule. Returns the assigned surrogate ID.
  async create_policy_rule(rule: PolicyRule) -> i64

  // Return all currently active policy rules.
  async list_active_policies() -> PolicyRule[]


  // ---- Metrics ----

  // Record a metric data point. Returns the assigned surrogate ID.
  async record_metric(metric: Metric) -> i64

  // Return all metric records for a feature.
  async get_metrics_by_feature(feature_id: i64) -> Metric[]


  // ---- Governance Contracts ----

  // Persist a governance contract snapshot. Returns the assigned surrogate ID.
  async create_governance_contract(contract: GovernanceContract) -> i64

  // Fetch a specific version of a governance contract for a feature.
  // Returns null if no matching record.
  async get_governance_contract(feature_id: i64, version: i32) -> GovernanceContract?

  // Fetch the latest governance contract for a feature. Returns null if none.
  async get_latest_governance_contract(feature_id: i64) -> GovernanceContract?


  // ---- Module CRUD ----

  // Create a new module. Returns the assigned surrogate ID.
  async create_module(module: Module) -> i64

  // Fetch a module by surrogate ID. Returns null if not found.
  async get_module(id: i64) -> Module?

  // Fetch a module by slug. Returns null if not found.
  async get_module_by_slug(slug: String) -> Module?

  // Update module display name and optional description.
  async update_module(id: i64, friendly_name: String, description: String?) -> ()

  // Delete a module. Raises ModuleHasDependents if child modules exist.
  async delete_module(id: i64) -> ()

  // Return all modules with no parent (top-level modules).
  async list_root_modules() -> Module[]

  // Return direct child modules of a given parent.
  async list_child_modules(parent_id: i64) -> Module[]

  // Return a module with its owned features, tagged features, and child modules.
  async get_module_with_features(id: i64) -> ModuleWithFeatures?

  // Create a module-feature tag association.
  async tag_feature_to_module(tag: ModuleFeatureTag) -> ()

  // Remove a module-feature tag association.
  async untag_feature_from_module(module_id: i64, feature_id: i64) -> ()


  // ---- Cycle CRUD ----

  // Create a new cycle. Returns the assigned surrogate ID.
  async create_cycle(cycle: Cycle) -> i64

  // Fetch a cycle by surrogate ID. Returns null if not found.
  async get_cycle(id: i64) -> Cycle?

  // Update only the state field of an existing cycle.
  async update_cycle_state(id: i64, state: CycleState) -> ()

  // Return all cycles in a given state.
  async list_cycles_by_state(state: CycleState) -> Cycle[]

  // Return all cycles scoped to a given module.
  async list_cycles_by_module(module_id: i64) -> Cycle[]

  // Return all cycle records.
  async list_all_cycles() -> Cycle[]

  // Return a cycle with its assigned features and per-feature WP progress.
  async get_cycle_with_features(id: i64) -> CycleWithFeatures?

  // Add a feature to a cycle.
  async add_feature_to_cycle(entry: CycleFeature) -> ()

  // Remove a feature from a cycle.
  async remove_feature_from_cycle(cycle_id: i64, feature_id: i64) -> ()


  // ---- Sync Mappings (Plane.so integration) ----

  // Fetch the sync mapping for an entity. Returns null if not found.
  async get_sync_mapping(entity_type: String, entity_id: i64) -> SyncMapping?

  // Insert or update a sync mapping for an entity.
  async upsert_sync_mapping(mapping: SyncMapping) -> ()

  // Fetch a sync mapping by the external Plane issue ID.
  async get_sync_mapping_by_plane_id(entity_type: String, plane_issue_id: String) -> SyncMapping?

  // Delete a sync mapping for an entity.
  async delete_sync_mapping(entity_type: String, entity_id: i64) -> ()
}
```

---

## VcsPort

Version control system abstraction. Implementations wrap git operations.
Primary implementation uses libgit2.

```
interface VcsPort {

  // Create an isolated git worktree for a work package.
  // Returns the absolute filesystem path of the new worktree.
  async create_worktree(feature_slug: String, wp_id: String) -> Path

  // List all active worktrees with their metadata.
  async list_worktrees() -> WorktreeInfo[]

  // Remove a worktree from disk (prune git metadata).
  async cleanup_worktree(worktree_path: Path) -> ()

  // Create a new branch at the given base ref (commit SHA or branch name).
  async create_branch(branch_name: String, base: String) -> ()

  // Switch HEAD to a branch within the repository.
  async checkout_branch(branch_name: String) -> ()

  // Merge source branch into target branch.
  // Returns MergeResult including conflict info if the merge failed.
  async merge_to_target(source: String, target: String) -> MergeResult

  // Perform a dry-run conflict detection between two branches.
  // Returns conflict info for any files that would conflict.
  async detect_conflicts(source: String, target: String) -> ConflictInfo[]

  // Read a feature artifact file by relative path within the feature directory.
  async read_artifact(feature_slug: String, relative_path: String) -> String

  // Write a feature artifact file by relative path within the feature directory.
  async write_artifact(feature_slug: String, relative_path: String, content: String) -> ()

  // Check whether a feature artifact file exists.
  async artifact_exists(feature_slug: String, relative_path: String) -> bool

  // Scan a feature's directory for known artifact files.
  // Returns paths to meta.json, audit chain, and evidence files.
  async scan_feature_artifacts(feature_slug: String) -> FeatureArtifacts
}
```

### VcsPort Supporting Types

```
struct WorktreeInfo {
  path:          Path    // Absolute filesystem path
  branch:        String  // Branch name checked out in worktree
  feature_slug:  String  // Owning feature
  wp_id:         String  // Owning work package ID
}

struct MergeResult {
  success:        bool           // True if merge completed without conflicts
  conflicts:      ConflictInfo[] // Non-empty when success=false
  merged_commit:  String?        // SHA of the merge commit, if succeeded
}

struct ConflictInfo {
  path:   String   // File path with conflict
  ours:   String?  // Our version snippet (optional)
  theirs: String?  // Their version snippet (optional)
}

struct FeatureArtifacts {
  meta_json:      String?   // Content of meta.json if present
  audit_chain:    String?   // Content of audit chain file if present
  evidence_paths: String[]  // Paths to evidence files
}
```

---

## AgentPort

AI agent dispatch and communication abstraction.
Supports multiple backends. Primary implementations: Claude Code, Codex.

```
interface AgentPort {

  // Spawn an agent and block until it completes or errors.
  async dispatch(task: AgentTask, config: AgentConfig) -> AgentResult

  // Spawn an agent without blocking. Returns an opaque job ID.
  async dispatch_async(task: AgentTask, config: AgentConfig) -> String

  // Query the current status of an async agent job.
  async query_status(job_id: String) -> AgentStatus

  // Cancel a running or pending agent job.
  async cancel(job_id: String) -> ()

  // Send a mid-execution instruction to a running agent job.
  async send_instruction(job_id: String, instruction: String) -> ()
}
```

### AgentPort Supporting Types

```
enum AgentKind {
  ClaudeCode
  Codex
}

struct AgentConfig {
  kind:               AgentKind
  max_review_cycles:  u32      // Maximum review-revise cycles before failure
  timeout_secs:       u64      // Hard timeout for the job
  extra_args:         String[] // Backend-specific CLI arguments
}

struct AgentTask {
  wp_id:          String   // Work package ID
  feature_slug:   String   // Owning feature slug
  prompt_path:    Path     // Path to the task prompt file
  worktree_path:  Path     // Path to the isolated git worktree
  context_files:  Path[]   // Additional context files to pass to the agent
}

struct AgentResult {
  success:    bool     // True if agent exited cleanly
  pr_url:     String?  // URL of the opened pull request (if created)
  commits:    String[] // List of commit SHAs authored by the agent
  stdout:     String   // Captured standard output
  stderr:     String   // Captured standard error
  exit_code:  i32      // Process exit code
}

enum AgentStatus {
  Pending
  Running            { pid: u32 }
  WaitingForReview   { pr_url: String }
  Completed          { result: AgentResult }
  Failed             { error: String }
}
```

---

## ReviewPort

Code review abstraction. Supports Coderabbit, GitHub API, and manual fallback adapters.

```
interface ReviewPort {

  // Get the current review status for a pull request URL.
  async get_review_status(pr_url: String) -> ReviewStatus

  // Fetch all review comments on a pull request.
  async get_review_comments(pr_url: String) -> ReviewComment[]

  // Fetch only comments marked as requiring action (actionable=true).
  async get_actionable_comments(pr_url: String) -> ReviewComment[]

  // Get the CI check status for a pull request.
  async get_ci_status(pr_url: String) -> CiStatus

  // Get the full PR summary (state, review status, CI status).
  async get_pr_info(pr_url: String) -> PrInfo

  // Block until the review reaches a terminal status or times out.
  // Raises DomainError::Timeout if timeout_secs elapses.
  async await_review(pr_url: String, timeout_secs: u64) -> ReviewStatus

  // Block until CI reaches a terminal status or times out.
  // Raises DomainError::Timeout if timeout_secs elapses.
  async await_ci(pr_url: String, timeout_secs: u64) -> CiStatus
}
```

### ReviewPort Supporting Types

```
enum CommentSeverity {
  Critical
  Major
  Minor
  Informational
}

struct ReviewComment {
  author:     String           // Reviewer identifier
  body:       String           // Comment text
  file_path:  String?          // File the comment applies to (null for PR-level)
  line:       u32?             // Line number within the file
  severity:   CommentSeverity
  actionable: bool             // True if the agent must address this comment
}

enum ReviewStatus {
  Pending
  InProgress
  Approved
  ChangesRequested  { comments: ReviewComment[] }
  Rejected          { reason: String }
}

enum CiStatus {
  Pending
  Running
  Passed
  Failed     { logs_url: String }
  Cancelled
}

struct PrInfo {
  url:            String        // PR URL
  number:         u64           // PR number within the repository
  title:          String        // PR title
  state:          String        // GitHub/VCS state: "open" | "closed" | "merged"
  review_status:  ReviewStatus
  ci_status:      CiStatus
}
```

---

## ContentStoragePort

A scoped subset of StoragePort focused on content-centric operations
(features, backlog items, work packages). Used by adapters that do not need
governance, metrics, or cycle management surfaces.

```
interface ContentStoragePort {

  // ---- Feature CRUD ----

  async create_feature(feature: Feature) -> i64
  async get_feature_by_slug(slug: String) -> Feature?
  async get_feature_by_id(id: i64) -> Feature?
  async update_feature_state(id: i64, state: FeatureState) -> ()

  // Full feature update (all mutable fields).
  async update_feature(feature: Feature) -> ()

  async list_features_by_state(state: FeatureState) -> Feature[]
  async list_all_features() -> Feature[]


  // ---- Backlog ----

  // Create a new backlog item. Returns assigned ID.
  async create_backlog_item(item: BacklogItem) -> i64

  // Fetch a backlog item by ID. Returns null if not found.
  async get_backlog_item(id: i64) -> BacklogItem?

  // List backlog items matching the given filter criteria.
  async list_backlog_items(filters: BacklogFilters) -> BacklogItem[]

  // Update the status of a backlog item.
  async update_backlog_status(id: i64, status: BacklogStatus) -> ()

  // Update the priority of a backlog item.
  async update_backlog_priority(id: i64, priority: BacklogPriority) -> ()

  // Atomically retrieve and claim the highest-priority ready backlog item.
  // Returns null if the backlog is empty.
  async pop_next_backlog_item() -> BacklogItem?


  // ---- WorkPackage CRUD ----

  async create_work_package(wp: WorkPackage) -> i64
  async get_work_package(id: i64) -> WorkPackage?
  async update_wp_state(id: i64, state: WpState) -> ()

  // Full work package update (all mutable fields).
  async update_work_package(wp: WorkPackage) -> ()

  async list_wps_by_feature(feature_id: i64) -> WorkPackage[]
  async add_wp_dependency(dep: WpDependency) -> ()
  async get_wp_dependencies(wp_id: i64) -> WpDependency[]
  async get_ready_wps(feature_id: i64) -> WorkPackage[]
}
```

### BacklogItem Supporting Types

```
enum BacklogStatus {
  Pending    // Default; not yet promoted to a Feature
  Promoted   // Converted to a Feature
  Rejected   // Triaged out; will not be implemented
  Archived   // Kept for history but inactive
}

enum BacklogPriority {
  Critical
  High
  Medium
  Low
}

struct BacklogFilters {
  status:    BacklogStatus?   // Filter by status; null = all statuses
  priority:  BacklogPriority? // Filter by priority; null = all priorities
  limit:     u32?             // Maximum number of results; null = unlimited
}
```

---

## ObservabilityPort

Telemetry abstraction for distributed tracing, metrics, and structured logging.
All methods are synchronous. Telemetry must never block business logic.
Primary implementation: OpenTelemetry adapter.

```
interface ObservabilityPort {

  // ---- Distributed Tracing ----

  // Start a new span. If parent is provided, the span is a child of that span.
  // Returns a SpanContext identifying this span.
  start_span(name: String, parent: SpanContext?) -> SpanContext

  // End (close) the span identified by ctx.
  end_span(ctx: SpanContext) -> ()

  // Add a named event to a span with key-value string attributes.
  add_span_event(ctx: SpanContext, name: String, attributes: (String, String)[]) -> ()

  // Mark a span as failed with an error description.
  set_span_error(ctx: SpanContext, error: String) -> ()


  // ---- Metrics ----

  // Increment a named counter by value with labels.
  record_counter(name: String, value: u64, labels: (String, String)[]) -> ()

  // Record a histogram observation (latency, sizes, etc.) with labels.
  record_histogram(name: String, value: f64, labels: (String, String)[]) -> ()

  // Set a gauge to a point-in-time value with labels.
  record_gauge(name: String, value: f64, labels: (String, String)[]) -> ()


  // ---- Structured Logging ----

  // Emit a fully-structured log entry with level, message, fields, and optional span context.
  log(entry: LogEntry) -> ()

  // Convenience: emit a message at INFO level.
  log_info(message: String) -> ()

  // Convenience: emit a message at WARN level.
  log_warn(message: String) -> ()

  // Convenience: emit a message at ERROR level.
  log_error(message: String) -> ()
}
```

### ObservabilityPort Supporting Types

```
struct SpanContext {
  trace_id:       String   // Distributed trace identifier (W3C trace-id format)
  span_id:        String   // This span's identifier
  parent_span_id: String?  // Parent span identifier; null for root spans
}

enum MetricValue {
  Counter(u64)    // Monotonically increasing count
  Histogram(f64)  // Distribution observation
  Gauge(f64)      // Point-in-time absolute value
}

enum LogLevel {
  Trace
  Debug
  Info
  Warn
  Error
}

struct LogEntry {
  level:        LogLevel
  message:      String
  fields:       Map<String, String>  // Structured key-value context
  span_context: SpanContext?         // Correlated span, if any
}
```
