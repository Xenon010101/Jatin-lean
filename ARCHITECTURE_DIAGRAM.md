# jatin-lean v2.0 Architecture Diagram

## Current Architecture (v0.5.1)

```
┌─────────────────────────────────────────────────────────────┐
│                      jatin-lean CLI                         │
│                    (Flat Structure)                         │
└─────────────────────────────────────────────────────────────┘
                              │
                              ├─ [path] (default scan)
                              │
                              ├─ dedup
                              ├─ health
                              ├─ deps
                              ├─ compress
                              ├─ treeshake
                              ├─ audit
                              ├─ analyze
                              ├─ watch
                              ├─ policy
                              ├─ visualize
                              ├─ cache
                              ├─ snapshots
                              ├─ analytics
                              ├─ plugins
                              ├─ bench
                              ├─ io
                              ├─ xdp          ← Confusing!
                              ├─ ipc          ← Hidden!
                              ├─ serde        ← What?
                              ├─ coalesce     ← Unclear!
                              ├─ engine       ← ???
                              ├─ gateway
                              ├─ simd-json
                              ├─ arena
                              ├─ maglev
                              ├─ io-uring
                              ├─ cpu-cache
                              ├─ optimize
                              ├─ bpf
                              ├─ pcie
                              ├─ hedge
                              ├─ mmap-ipc
                              └─ static-dispatch
                              
                    32+ commands in flat list!
                    Users get lost and confused
```

---

## New Architecture (v2.0.0)

```
┌─────────────────────────────────────────────────────────────┐
│                      jatin-lean CLI                         │
│                 (Hierarchical Structure)                    │
└─────────────────────────────────────────────────────────────┘
                              │
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
        ▼                     ▼                     ▼
   ┌─────────┐          ┌─────────┐          ┌─────────┐
   │  NODE   │          │ SYSTEM  │          │ NETWORK │
   └─────────┘          └─────────┘          └─────────┘
        │                     │                     │
        ├─ scan               ├─ optimize           ├─ xdp
        ├─ prune              ├─ cpu-cache          ├─ bpf
        ├─ health             └─ io                 └─ gateway
        ├─ dedup
        ├─ deps               ┌─────────┐          ┌─────────┐
        ├─ compress           │ MEMORY  │          │  BENCH  │
        ├─ treeshake          └─────────┘          └─────────┘
        ├─ audit                   │                     │
        ├─ analyze                 ├─ ipc                ├─ all
        ├─ watch                   ├─ mmap               ├─ simd
        ├─ policy                  ├─ arena              ├─ serde
        └─ visualize               └─ pcie               ├─ json
                                                         ├─ io-uring
                              ┌─────────┐                ├─ coalesce
                              │ ANALYZE │                ├─ hedge
                              └─────────┘                ├─ maglev
                                   │                     └─ dispatch
                                   ├─ project
                                   ├─ cache
                                   ├─ dist-cache
                                   ├─ engine
                                   ├─ snapshots
                                   ├─ analytics
                                   ├─ undo
                                   └─ restore

                    Clear organization!
                    Easy to discover features
```

---

## Command Flow Diagram

### Old Flow (v0.5.1)

```
User Input: "jatin-lean health ."
     │
     ▼
┌─────────────────┐
│   main.rs       │
│  parse_args()   │
└─────────────────┘
     │
     ▼
┌─────────────────┐
│ match command   │
│ Commands::      │
│   Health {...}  │
└─────────────────┘
     │
     ▼
┌─────────────────┐
│ health::check() │
│ health::print() │
└─────────────────┘
     │
     ▼
  Terminal Output
  (Human-readable only)
```

### New Flow (v2.0.0)

```
User Input: "jatin-lean node health --json"
     │
     ▼
┌─────────────────────────┐
│   main.rs               │
│  parse_args()           │
│  + global flags (json)  │
└─────────────────────────┘
     │
     ▼
┌─────────────────────────┐
│ match command           │
│ Commands::Node {        │
│   NodeCommands::        │
│     Health {...}        │
│ }                       │
└─────────────────────────┘
     │
     ▼
┌─────────────────────────┐
│ cli::node::             │
│   handle_health()       │
└─────────────────────────┘
     │
     ├─────────────────────┐
     │                     │
     ▼                     ▼
┌──────────────┐    ┌──────────────┐
│ health::     │    │ output::     │
│   check()    │    │   json()     │
└──────────────┘    └──────────────┘
     │                     │
     ▼                     ▼
  Terminal Output    JSON Output
  (Human-readable)   (Machine-readable)
```

---

## Module Structure

### Current (v0.5.1)

```
src/
├── main.rs (2650 lines!)
│   ├── CLI definitions
│   ├── Command handlers
│   ├── All logic mixed
│   └── Hard to maintain
│
├── [30+ feature modules]
│   ├── health.rs
│   ├── dedup.rs
│   ├── xdp_middleware.rs
│   └── ...
│
└── No organization
```

### New (v2.0.0)

```
src/
├── main.rs (200 lines)
│   ├── CLI structure
│   ├── Global flags
│   └── Route to handlers
│
├── cli/
│   ├── mod.rs
│   ├── node.rs       ← Node commands
│   ├── system.rs     ← System commands
│   ├── network.rs    ← Network commands
│   ├── memory.rs     ← Memory commands
│   ├── bench.rs      ← Bench commands
│   ├── analyze.rs    ← Analyze commands
│   └── legacy.rs     ← Legacy support
│
├── output/
│   ├── mod.rs
│   ├── json.rs       ← JSON output
│   ├── csv.rs        ← CSV export
│   └── html.rs       ← HTML export
│
├── ai_context.rs     ← AI helper
│
└── [30+ feature modules]
    ├── health.rs     ← Unchanged
    ├── dedup.rs      ← Unchanged
    └── ...           ← All preserved
```

---

## Data Flow: Health Check Example

### Human-Readable Output

```
┌──────────────┐
│ User Command │
│ node health  │
└──────────────┘
       │
       ▼
┌──────────────────────┐
│ cli::node::          │
│   handle_health()    │
│                      │
│ if ctx.json {        │
│   // JSON path       │
│ } else {             │
│   // Human path  ←── │
│ }                    │
└──────────────────────┘
       │
       ▼
┌──────────────────────┐
│ health::             │
│   check_health()     │
│                      │
│ Returns:             │
│   HealthReport {     │
│     grade: "B",      │
│     score: 83,       │
│     issues: [...]    │
│   }                  │
└──────────────────────┘
       │
       ▼
┌──────────────────────┐
│ health::             │
│   print_report()     │
│                      │
│ Formats with:        │
│   - Colors           │
│   - Tables           │
│   - Icons            │
└──────────────────────┘
       │
       ▼
┌──────────────────────┐
│ Terminal Output      │
│                      │
│ ╭─ Health Check ─╮  │
│ │ Grade: B (83)   │  │
│ │ Issues: 10      │  │
│ ╰─────────────────╯  │
└──────────────────────┘
```

### JSON Output

```
┌──────────────┐
│ User Command │
│ node health  │
│   --json     │
└──────────────┘
       │
       ▼
┌──────────────────────┐
│ cli::node::          │
│   handle_health()    │
│                      │
│ if ctx.json {    ←── │
│   // JSON path       │
│ }                    │
└──────────────────────┘
       │
       ▼
┌──────────────────────┐
│ health::             │
│   check_health()     │
│                      │
│ Returns:             │
│   HealthReport       │
└──────────────────────┘
       │
       ▼
┌──────────────────────┐
│ Serialize to JSON    │
│                      │
│ HealthOutput {       │
│   grade: String,     │
│   score: u32,        │
│   issues: Vec<...>   │
│ }                    │
└──────────────────────┘
       │
       ▼
┌──────────────────────┐
│ output::             │
│   output_result()    │
│                      │
│ Wraps in:            │
│   JsonOutput {       │
│     command,         │
│     timestamp,       │
│     result           │
│   }                  │
└──────────────────────┘
       │
       ▼
┌──────────────────────┐
│ JSON Output          │
│                      │
│ {                    │
│   "command": "...",  │
│   "timestamp": "...",│
│   "result": {...}    │
│ }                    │
└──────────────────────┘
```

---

## Legacy Command Support

```
┌──────────────────────┐
│ User runs old syntax │
│ "jatin-lean health"  │
└──────────────────────┘
       │
       ▼
┌──────────────────────┐
│ main.rs              │
│ Matches:             │
│   Commands::Legacy(  │
│     LegacyCommands:: │
│       Health         │
│   )                  │
└──────────────────────┘
       │
       ▼
┌──────────────────────┐
│ cli::legacy::        │
│   handle_command()   │
│                      │
│ if !json {           │
│   show_deprecation() │ ← Shows warning
│ }                    │
└──────────────────────┘
       │
       ▼
┌──────────────────────┐
│ Route to new handler │
│                      │
│ cli::node::          │
│   handle_health()    │
└──────────────────────┘
       │
       ▼
┌──────────────────────┐
│ Works exactly like   │
│ new command!         │
└──────────────────────┘
```

---

## AI Context Command Flow

```
┌──────────────────────┐
│ AI Assistant runs:   │
│ jatin-lean           │
│   ai-context --json  │
└──────────────────────┘
       │
       ▼
┌──────────────────────┐
│ ai_context::         │
│   handle()           │
│                      │
│ Detects:             │
│ - Project context    │
│ - System context     │
│ - Tool capabilities  │
└──────────────────────┘
       │
       ├─────────────────────┐
       │                     │
       ▼                     ▼
┌──────────────┐    ┌──────────────┐
│ Scan project │    │ Detect system│
│ - node_modules│    │ - OS, arch   │
│ - frameworks  │    │ - CPU, SIMD  │
│ - packages    │    │ - Memory     │
└──────────────┘    └──────────────┘
       │                     │
       └─────────┬───────────┘
                 ▼
┌──────────────────────────────┐
│ Generate AiContext {         │
│   tool: "jatin-lean",        │
│   capabilities: [...],       │
│   quick_commands: {...},     │
│   project_context: {...},    │
│   system_context: {...}      │
│ }                            │
└──────────────────────────────┘
       │
       ▼
┌──────────────────────────────┐
│ JSON Output                  │
│                              │
│ AI can now:                  │
│ - Understand tool            │
│ - Know project state         │
│ - Make smart suggestions     │
└──────────────────────────────┘
```

---

## Backward Compatibility Strategy

```
┌─────────────────────────────────────────────────────────┐
│                    v2.0.0 Release                       │
│                                                         │
│  ┌─────────────┐              ┌─────────────┐         │
│  │ New Syntax  │              │ Old Syntax  │         │
│  │ (Primary)   │              │ (Supported) │         │
│  └─────────────┘              └─────────────┘         │
│        │                             │                 │
│        │                             │                 │
│        ▼                             ▼                 │
│  ┌─────────────┐              ┌─────────────┐         │
│  │ Direct      │              │ Legacy      │         │
│  │ Handler     │              │ Handler     │         │
│  └─────────────┘              └─────────────┘         │
│        │                             │                 │
│        │                             ├─ Show warning   │
│        │                             │                 │
│        └─────────────┬───────────────┘                 │
│                      ▼                                 │
│              ┌─────────────┐                           │
│              │ Same Result │                           │
│              └─────────────┘                           │
└─────────────────────────────────────────────────────────┘

Timeline:
├─ v2.0.0 (Week 6)   : Both work, warnings shown
├─ v2.1.0 (Month 3)  : Warnings more prominent
├─ v2.5.0 (Month 6)  : Final warning
└─ v3.0.0 (Month 12) : Old syntax removed
```

---

## Testing Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Test Suite                           │
└─────────────────────────────────────────────────────────┘
                      │
        ┌─────────────┼─────────────┐
        │             │             │
        ▼             ▼             ▼
┌─────────────┐ ┌─────────────┐ ┌─────────────┐
│ Unit Tests  │ │ Integration │ │ E2E Tests   │
└─────────────┘ └─────────────┘ └─────────────┘
        │             │             │
        │             │             │
        ▼             ▼             ▼
┌─────────────┐ ┌─────────────┐ ┌─────────────┐
│ - CLI parse │ │ - Commands  │ │ - Real usage│
│ - Routing   │ │ - JSON out  │ │ - npm pkg   │
│ - JSON ser  │ │ - Legacy    │ │ - Binary    │
└─────────────┘ └─────────────┘ └─────────────┘

┌─────────────────────────────────────────────────────────┐
│              Performance Benchmarks                     │
│                                                         │
│  Before (v0.5.1)  vs  After (v2.0.0)                   │
│                                                         │
│  - Scan speed                                           │
│  - Memory usage                                         │
│  - Binary size                                          │
│  - JSON serialization overhead                          │
└─────────────────────────────────────────────────────────┘
```

---

## Deployment Pipeline

```
┌─────────────┐
│ Development │
└─────────────┘
       │
       ├─ Commit
       ├─ Push
       │
       ▼
┌─────────────┐
│ CI/CD       │
│ - Tests     │
│ - Lint      │
│ - Build     │
└─────────────┘
       │
       ├─ All pass?
       │
       ▼
┌─────────────┐
│ Beta Release│
│ v2.0.0-beta │
└─────────────┘
       │
       ├─ Feedback
       ├─ Bug fixes
       │
       ▼
┌─────────────┐
│ Stable      │
│ v2.0.0      │
└─────────────┘
       │
       ├─ npm publish
       ├─ crates.io
       ├─ GitHub release
       │
       ▼
┌─────────────┐
│ Users       │
│ - npm       │
│ - cargo     │
│ - binary    │
└─────────────┘
```

---

This architecture ensures:
- ✅ Clear organization
- ✅ Easy maintenance
- ✅ Backward compatibility
- ✅ AI-friendly
- ✅ Scalable for future features
