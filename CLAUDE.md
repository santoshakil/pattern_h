# Pattern H — Hexagonal Architecture Skeleton

## Architecture
- Hexagonal (Ports & Adapters): domain defines traits, infrastructure implements them
- Flutter: pure UI (zero business logic)
- Rust: all core logic, storage, networking
- Protobuf: single source of truth for cross-language data
- FFI: ByteBuffer zero-copy via Flutter native assets
- Events: Rust→Dart push via irondash_dart_ffi (NativeEventPort + ReceivePort)

## Directory Structure
- protos/ — protobuf definitions (single source of truth)
- rust/crates/errors — error type hierarchy
- rust/crates/domain — entities, value objects, ports (traits), services, events
- rust/crates/proto — generated protobuf Rust types
- rust/crates/ffi — FFI toolkit (ByteBuffer, runtime, safety, validation, dart_port events)
- rust/crates/sqlite — SQLite storage adapter
- rust/crates/app_core — composition root + FFI exports (cdylib)
- flutter/packages/native_bridge — Dart FFI client + NativeEventReceiver + native asset build hook
- flutter/packages/platform_bridge — typed method channel toolkit for platform APIs
- flutter/packages/proto_models — generated Dart protobuf classes
- flutter/packages/design_system — theme, colors, typography
- flutter/packages/ui_kit — reusable themed widgets
- flutter/apps/main_app — the Flutter application

## Dependency Flow
Rust:
  app_core → errors, ffi, proto, domain (composition root)
  sqlite → errors
  domain → errors (defines port traits + services)
  ffi → errors (FFI infrastructure)
  proto → prost (generated types)
  NEVER: domain → infrastructure
Flutter:
  main_app → design_system, ui_kit, native_bridge, platform_bridge, proto_models
  ui_kit → flutter SDK only (uses Theme.of(context), NO design_system dep)
  native_bridge → ffi, hooks, code_assets, native_toolchain_rust (ffigen for @Native bindings)
  platform_bridge → flutter SDK only

## Build Commands
cargo check --workspace
cargo clippy --workspace -- -D warnings
dart analyze --fatal-infos
./scripts/check.sh
./scripts/generate_proto.sh

## Adding a New Domain Feature
1. Define messages in protos/services/*.proto
2. Run ./scripts/generate_proto.sh (generates Dart + Rust proto types)
3. Add domain service trait in rust/crates/domain/src/services/
4. Implement service in domain (pure logic, no infra deps)
5. Add FFI export in rust/crates/app_core/src/ffi_exports.rs (wire service)
6. Run cargo build -p app_core (regenerates C header via cbindgen)
7. Run dart run ffigen in native_bridge (regenerates @Native bindings)
8. Add typed method in FfiClient (encode proto → callNative → decode proto)
9. Call from Flutter UI via provider

## Adding a New Domain Entity with Storage
1. Define messages in protos/
2. Add entity in rust/crates/domain/src/entities/
3. Define port trait in rust/crates/domain/src/ports/
4. Implement adapter in rust/crates/sqlite/ (or new infra crate)
5. Follow steps 3-9 from "Adding a New Domain Feature"

## Adding a New Infrastructure Adapter
1. Create rust/crates/infra_name/ with Cargo.toml
2. Depend on errors crate (add domain when implementing port traits)
3. Implement the port trait from domain/src/ports/
4. Wire in app_core/src/lib.rs

## Rust→Dart Push Events (irondash)
Rust side:
1. `ffi::define_event_port!(EVENT_PORT)` in app_core (already done)
2. Use `EVENT_PORT.send_event(id, vec![DartValue::...])` from any Rust code
3. FFI exports: `app_init_dart_api`, `app_set_dart_port`, `app_disconnect_dart_port`

Dart side:
1. `final receiver = NativeEventReceiver()`
2. `receiver.events.listen((e) => print(e.id))` — auto-connects on first listen
3. `receiver.where(42).listen(...)` — filter by event ID
4. `receiver.dispose()` on shutdown
5. Auto-cleanup when all listeners detach

## Adding a Platform Feature (Method Channels)
1. Create a PlatformChannel: `final nfc = PlatformChannel('com.pattern_h/nfc')`
2. Call platform methods: `await nfc.call<Map>('readTag', {'timeout': 5000})`
3. Listen to events: `nfc.events('tags').listen((tag) { ... })`
4. Implement platform side in Kotlin/Swift using matching channel names
5. Error codes: UNAVAILABLE, PERMISSION_DENIED, INVALID_ARGUMENT, NOT_FOUND, ALREADY_IN_USE, TIMEOUT, CANCELLED
