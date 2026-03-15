# Pattern H Full Audit — 2026-03-14

## Verdict (POST-FIX)
Architecture: SOLID. Implementation: 95% complete. Dependencies: CURRENT. Tooling: COMPLETE.
All bugs fixed. All deps updated. 91 Rust tests + 60 Flutter tests + 10 scaffold tests = 161 total.
Typed EventBus, SQLite migrations, composition root wired. Scaffold tool ready.

---

## 1. BUGS (Must Fix)

### BUG-1: NativeEventReceiver stream controller never closed
File: flutter/packages/native_bridge/lib/src/event_receiver.dart
Lines 51-57 (_onLastListenerGone) and 62-68 (dispose):
Sets `_controller = null` BEFORE calling `_controller?.close()` — the close is a no-op.
Result: StreamController leaks, never fires onDone, memory leak.
Fix: Close before nulling.

### BUG-2: app_send_test_event() exported in release builds
File: rust/crates/app_core/src/lib.rs:75
Debug-only function compiled and exported with `#[no_mangle]` in all builds.
Fix: Wrap in `#[cfg(debug_assertions)]`.

### BUG-3: build.rs silently swallows cbindgen failures
File: rust/crates/app_core/build.rs
`if let Ok(bindings) = ...generate()` — if cbindgen fails, header is stale but build succeeds.
`cbindgen::Config::from_file(...).unwrap_or_default()` — malformed config silently uses defaults.
Fix: `eprintln!` on failure, or `panic!` in build script.

### BUG-4: app_test.dart will always fail
File: flutter/apps/main_app/test/app_test.dart
Pumps `App()` which triggers `ffiClientProvider` → `FfiClient()..init()` → `app_init()` (native FFI).
Native library can't load in unit test context. No mock/override provided.
Fix: Override ffiClientProvider with a mock in tests.

### BUG-5: VERSION file disconnected
File: VERSION contains "0.1.0"
`app_version()` returns hardcoded `1` (integer).
Cargo.toml has version "0.1.0" separately.
Three version sources, none reading from VERSION.
Fix: Wire VERSION into build systems or remove the file.

### BUG-6: VersionRequest/VersionResponse are dead proto messages
File: protos/services/example.proto
Defined but `app_version()` bypasses protobuf entirely (returns u32).
Fix: Either use them or remove them.

---

## 2. OUTDATED DEPENDENCIES

### Rust — Critical Updates Needed

| Crate | Current | Latest | Breaking? |
|---|---|---|---|
| prost | 0.13 | 0.14.3 | YES — Debug no longer supertrait of Message, derive feature renamed, TryFrom changes |
| prost-build | 0.13 | 0.14.3 | YES — matches prost |
| rusqlite | 0.32 | 0.38.0 | YES — execute checks trailing stmts, u64 ToSql disabled by default |
| cbindgen | 0.28 | 0.29.2 | Minor — heck 0.5, unsafe(no_mangle) support |
| rust-toolchain | "stable" (unpinned) | 1.94.0 | Should pin specific version |

tokio, serde, thiserror, tracing, tracing-subscriber, dashmap, parking_lot, uuid, async-trait — all covered by current semver ranges.

### Flutter/Dart — Critical Updates Needed

| Package | Current | Latest | Breaking? |
|---|---|---|---|
| flutter_riverpod | ^2.6.0 | 3.3.1 | YES — major version, deprecated family.overrideWith |
| go_router | ^14.0.0 | 17.1.0 | YES — ShellRoute observer changes |
| protobuf | ^5.0.0 | 6.0.0 | YES — PbList/PbMap constructors hidden, map validation |
| ffigen | ^14.0.0 | 20.1.1 | YES — major rewrite, new FfiGenerate API |
| lints | ^5.0.0 | 6.1.0 | YES — new lint rules |
| melos | (in melos.yaml) | 7.4.0 | YES — config moves to pubspec.yaml, uses pub workspaces |
| Flutter SDK | >=3.24.0 | 3.41.2 | Should update minimum |
| Dart SDK | ^3.6.0 | 3.9 | Should update minimum |

ffi, hooks, code_assets, native_toolchain_rust, fixnum — current ranges are fine.

---

## 3. MISSING PIECES

### 3A. Infrastructure — Critical

- **No CI/CD**: No .github/workflows/ directory. No automated checks on PR. No build verification.
- **No setup/bootstrap script**: Fresh clone requires manual tool installation + code generation. No `scripts/setup.sh`.
- **No Android/iOS/platform directories**: main_app has only lib/ and test/. Cannot run on any device. Needs `flutter create` scaffolding.
- **Generated files don't exist on disk**: proto_models/lib/src/generated/*.pb.dart and native_bridge/lib/src/generated/bindings.dart are all missing. `dart analyze` and tests fail on fresh clone.
- **rust-toolchain.toml unpinned**: `channel = "stable"` is non-deterministic. Should pin version + declare components and targets.

### 3B. Architecture — Important

- **EventBus uses Box<dyn Any + Send>**: Type-erased event publishing. Subscribers must downcast. Should use typed domain event enum.
- **No migration system in sqlite**: Database has no schema versioning, no PRAGMA user_version, no migration runner.
- **sqlite not wired to app_core**: app_core/Cargo.toml doesn't depend on sqlite. No Repository implementation exists.
- **No DI/composition pattern**: DefaultPingService is a static in ffi_exports.rs. No trait-based injection, no composition root builder.
- **proto/src/lib.rs hardcodes module names**: Adding a new proto package requires manually editing lib.rs.

### 3C. Testing — Important

- **ZERO Rust tests**: No #[cfg(test)] modules, no tests/ directories in any crate.
- **ZERO Flutter package tests**: No test/ directories in design_system, ui_kit, native_bridge, platform_bridge, proto_models.
- **app_test.dart is broken**: Can't load native library in test context.

### 3D. Quality — Nice to Have

- **No flutter_lints**: analysis_options.yaml uses lints/recommended.yaml, not flutter_lints. Flutter-specific lints (use_build_context_synchronously, etc.) inactive.
- **No cargo-deny / cargo-audit**: No supply chain security checks.
- **Missing theme components**: No SnackBarThemeData, DialogThemeData, BottomSheetThemeData, NavigationBarThemeData, TabBarThemeData in design_system.
- **AppScaffold minimal**: No bottom nav, drawer, or resizeToAvoidBottomInset support.
- **No async FFI pattern**: All FFI calls are synchronous on calling isolate. Will block UI for slow ops.
- **ffiClientProvider no error handling**: If init() throws, exception propagates uncaught at provider construction.
- **home_screen ping is sync but wrapped in loading state**: Loading spinner can't render before sync call returns.

---

## 4. ARCHITECTURE VERIFICATION

### Dependency Flow — CORRECT
```
errors ← leaf (thiserror only)
domain → errors (defines ports, never touches infra)
proto → prost (generated types, no domain dep)
ffi → errors (FFI infra, no domain dep)
sqlite → errors (infra adapter, no domain dep)
app_core → errors, domain, proto, ffi (composition root)
```

domain NEVER depends on infrastructure. Constraint holds.

### Flutter Dependency Flow — CORRECT
```
ui_kit → flutter SDK only (no design_system dep)
platform_bridge → flutter SDK only
design_system → flutter SDK only
proto_models → protobuf + fixnum only (no flutter dep)
native_bridge → ffi + hooks + code_assets (no flutter dep)
main_app → all packages
```

### What's Done Well
- ByteBuffer zero-copy with ManuallyDrop — correct ownership transfer
- catch_ffi panic boundary — catches panics, maps errors, logs via tracing
- AppRuntime static tokio — guards double-init, blocks calling from within runtime
- HandleRegistry — u64 counter with 0 sentinel, DashMap sharding
- NativeEventPort — AtomicI64 lock-free port, prepends event_id
- Error hierarchy — AppError umbrella with From impls, FfiError with integer codes
- Workspace clippy lints — deny unwrap/expect at workspace level
- Design system — seed color + fromSeed, no hardcoded colors in widgets
- FFI client memory management — nested try/finally for both input and result buffers
- Native asset build hook — RustBuilder integration correct

---

## 5. PROJECT SCAFFOLDING TOOL

### What's Needed
A `pattern_h` CLI tool or script that creates a new project from this skeleton.

### Requirements
1. Replace all "pattern_h" references with new project name
2. Replace package names, import paths, proto package names
3. Replace seed color (design_system)
4. Set initial version
5. Generate platform directories (flutter create)
6. Run code generation (proto + cbindgen + ffigen)
7. Verify everything compiles
8. Initialize git

### Files Requiring Name Replacement
- CLAUDE.md (channel names like com.pattern_h/)
- Cargo.toml (workspace)
- All crate Cargo.toml files
- All pubspec.yaml files
- All .dart import paths referencing package names
- Proto package names
- cbindgen.toml (include guard)
- melos.yaml (project name)
- justfile
- Scripts

### Approach Options
A. Shell script (simplest, portable)
B. Dart CLI tool (type-safe, testable, can use dart:io)
C. Rust CLI tool (fast, but overkill for scaffolding)

Recommendation: Dart CLI tool in tools/scaffold/ — dogfoods the Dart ecosystem, can be run with `dart run`, testable.

---

## 6. IMPROVEMENT PRIORITY

### P0 — Blocks Usage
1. Fix BUG-1 (stream controller leak)
2. Fix BUG-4 (broken test)
3. Update outdated deps (prost 0.14, rusqlite 0.38, flutter_riverpod 3.x, go_router 17.x, protobuf 6.x, ffigen 20.x, melos 7.x)
4. Pin rust-toolchain version
5. Add CI/CD (GitHub Actions)
6. Create setup.sh bootstrap script
7. Fix generated file bootstrapping (generate on first setup)

### P1 — Architecture Completion
8. Type the EventBus (domain event enum)
9. Wire sqlite to app_core with Repository implementation
10. Add migration system to sqlite
11. Add composition root builder pattern
12. Add async FFI pattern (isolate-based)
13. Create project scaffolding tool

### P2 — Quality
14. Fix remaining bugs (BUG-2, 3, 5, 6)
15. Add Rust tests for all crates
16. Add Flutter tests for all packages
17. Switch to flutter_lints
18. Add cargo-deny config
19. Complete theme components
20. Add error handling to ffiClientProvider

---

## 7. DETAILED FILE ISSUES INDEX

### rust/crates/app_core/src/lib.rs
- Line 38: `let _ = ...try_init()` discards tracing init error
- Line 75: app_send_test_event not cfg-gated

### rust/crates/app_core/build.rs
- Line 16: unwrap_or_default silently ignores malformed cbindgen config
- if let Ok: silently ignores cbindgen generation failure
- Missing rerun-if-changed for domain/src/

### rust/crates/ffi/src/handle_registry.rs
- get() returns Ref holding DashMap shard lock — holding across .await = deadlock

### rust/crates/sqlite/src/connection.rs
- No tracing instrumentation
- Single Mutex serializes all access (WAL allows concurrent reads)

### flutter/packages/native_bridge/lib/src/event_receiver.dart
- Lines 51-57: controller nulled before close
- Lines 62-68: same bug in dispose()
- _dartApiInitialized as static on instance class

### flutter/apps/main_app/lib/features/home/home_provider.dart
- ffiClientProvider: no error handling on init()

### flutter/apps/main_app/lib/features/home/home_screen.dart
- _onPing: sync FFI call inside loading state pattern

### flutter/apps/main_app/test/app_test.dart
- Cannot load native library in unit test
