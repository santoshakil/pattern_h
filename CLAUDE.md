# Pattern H — Hexagonal Architecture Skeleton

## Architecture
- Hexagonal (Ports & Adapters): domain defines traits, infrastructure implements them
- Flutter: pure UI (zero business logic)
- Rust: all core logic, storage, networking
- Protobuf: single source of truth for cross-language data
- FFI: ByteBuffer zero-copy via Flutter native assets
- Events: Rust→Dart push via irondash_dart_ffi (NativeEventPort + ReceivePort)
- Method Channels: typed PlatformChannel toolkit with ChannelRegistry pattern
- Typed DomainEvent enum (EntityCreated/Updated/Deleted/Custom)
- SQLite migration system (versioned, idempotent)

## Directory Structure
- protos/ — protobuf definitions (single source of truth)
- rust/crates/errors — error type hierarchy (AppError, FfiError, DomainError, StorageError)
- rust/crates/domain — entities, value objects, ports (traits), services, events
- rust/crates/proto — generated protobuf Rust types (prost build.rs)
- rust/crates/ffi — FFI toolkit (ByteBuffer, AppRuntime, HandleRegistry, NativeEventPort, catch_ffi)
- rust/crates/sqlite — SQLite storage adapter + versioned migration system
- rust/crates/app_core — composition root + FFI exports (cdylib, cbindgen header)
- flutter/packages/native_bridge — Dart FFI client + NativeEventReceiver + native asset build hook
- flutter/packages/platform_bridge — typed method channel toolkit (PlatformChannel + error hierarchy)
- flutter/packages/proto_models — generated Dart protobuf classes
- flutter/packages/design_system — Material 3 theme, colors, typography (seed color based)
- flutter/packages/ui_kit — reusable themed widgets (AppButton, AppScaffold)
- flutter/apps/main_app — the Flutter application
- templates/platform/ — platform channel templates for Android/iOS/macOS/Windows/Linux
- tools/cli/ — Rust CLI to create new projects (embeds template at compile time)

## Dependency Flow
Rust:
  app_core → errors, ffi, proto, domain, sqlite (composition root)
  sqlite → errors, tracing
  domain → errors (defines port traits + services)
  ffi → errors (FFI infrastructure)
  proto → prost (generated types)
  NEVER: domain → infrastructure
Flutter:
  main_app → design_system, ui_kit, native_bridge, platform_bridge, proto_models
  ui_kit → flutter SDK only (uses Theme.of(context), NO design_system dep)
  native_bridge → ffi, hooks, code_assets, native_toolchain_rust
  platform_bridge → flutter SDK only

## Commands
just setup — first-time setup (checks tools, installs deps, generates code)
just check — full pipeline: fmt + lint + test
just fmt — format Rust + Dart
just lint — clippy + dart analyze
just test — cargo test + flutter test
just generate — regenerate proto + cbindgen header
just scaffold <name> [--org] [--seed-color] [-o] — create new project
just install-cli — install pattern-h command globally
just clean — clean all build artifacts

## Creating a New Project
Install: cargo install --path tools/cli
Usage: pattern-h my_app --org com.mycompany --seed-color FF6B35 -o ~/projects

What it does:
1. Extracts skeleton (Rust, Dart, protos, scripts, templates)
2. Renames: main_app→my_app, app_core→my_app_core, pattern_h→my_app everywhere
3. Runs flutter create --platforms android,ios,macos,windows,linux
4. Injects platform channel code (ChannelRegistry + DeviceInfoChannel per platform)
5. Adds permissions (INTERNET on Android, network.client on macOS)
6. Updates Xcode projects to include Swift files
7. Patches CMakeLists for Windows/Linux channel sources
8. Initializes git with initial commit

## Adding a New Domain Feature
1. Define messages in protos/services/*.proto
2. Run ./scripts/generate_proto.sh
3. Add domain service trait in rust/crates/domain/src/services/
4. Implement service in domain (pure logic, no infra deps)
5. Add FFI export in rust/crates/app_core/src/ffi_exports.rs
6. Run cargo build -p app_core (regenerates C header)
7. Run dart run ffigen in native_bridge (regenerates @Native bindings)
8. Add typed method in FfiClient
9. Call from Flutter UI via provider

## Adding a New Domain Entity with Storage
1. Define messages in protos/
2. Add entity in rust/crates/domain/src/entities/
3. Define port trait in rust/crates/domain/src/ports/
4. Add migration in sqlite crate (Migration { version, sql })
5. Call run_migrations() from app_core init
6. Implement adapter in rust/crates/sqlite/
7. Follow steps 3-9 from "Adding a New Domain Feature"

## Adding a New Platform Channel
Dart: create lib/platform/nfc.dart (typed wrapper using PlatformChannel)
Android: create channels/NfcChannel.kt, add to ChannelRegistry.registerAll()
iOS: create NfcChannel.swift, add to ChannelRegistry.registerAll()
macOS: create NfcChannel.swift, add to ChannelRegistry.registerAll()
Windows: create nfc_channel.h/.cpp, add to RegisterChannels(), update CMakeLists
Linux: create nfc_channel.h/.cc, add to register_channels(), update CMakeLists

## Rust→Dart Push Events
Rust: EVENT_PORT.send_event(id, vec![DartValue::...])
Dart: NativeEventReceiver().events.listen((e) => ...) or .where(id).listen(...)

## Pinned Versions
- Rust: 1.86.0 (with cross-compilation targets for Android/iOS/macOS)
- prost/prost-build: 0.14
- rusqlite: 0.38 (bundled)
- cbindgen: 0.29
- Dart SDK: ^3.8.0
- Flutter: >=3.32.0
