# Pattern H

A production-ready project template for building cross-platform apps with **Flutter UI + Rust core logic + Protobuf FFI**. Uses hexagonal architecture to keep domain logic in Rust, UI in Flutter, and communication type-safe via protobuf — all wired through zero-copy FFI with native assets.

Ships as a Rust CLI tool (`pattern-h`) that embeds the entire template at compile time. One command creates a complete project with all platforms configured.

## Quick Start

```bash
# Install the CLI
cargo install --git https://github.com/santoshakil/pattern_h --path tools/cli

# Create a new project
pattern-h my_restaurant_app --org com.mycompany --seed-color FF6B35

# Set up and run
cd my_restaurant_app
./scripts/setup.sh
cd flutter/apps/my_restaurant_app
flutter run
```

## What You Get

A scaffolded project with:

- **6 Rust crates** — errors, domain, proto, ffi, sqlite, app_core (composition root)
- **6 Flutter packages** — native_bridge, platform_bridge, proto_models, design_system, ui_kit, main app
- **3 communication layers** — all wired and tested:
  - Protobuf over FFI (Flutter ↔ Rust, zero-copy ByteBuffer)
  - Rust → Dart push events (irondash, async notifications)
  - Platform method channels (Android/iOS/macOS/Windows/Linux)
- **Platform support** — Android, iOS, macOS, Windows, Linux with channel code injected
- **91 Rust tests** across all crates
- **Material 3 theme** with dynamic seed color
- **SQLite** with versioned migration system
- **Scripts** for setup, code generation, and quality checks

## Architecture

```
Flutter UI (presentation)
    ↓ Riverpod providers
FfiClient / PlatformChannel (driving adapters)
    ↓ protobuf encode → FFI / method channel
app_core (composition root)
    ↓ domain service calls
domain (ports & services)
    ↑ trait implementations
sqlite / infra adapters (driven adapters)
```

**Dependency rule**: always point inward. Domain defines traits. Infrastructure implements them. Domain never imports infrastructure.

### Communication Layers

| Layer | Direction | Mechanism | Use Case |
|-------|-----------|-----------|----------|
| Protobuf FFI | Flutter ↔ Rust | ByteBuffer zero-copy, `@Native` bindings | Business logic, data processing |
| Push Events | Rust → Flutter | irondash NativeEventPort + ReceivePort | Real-time updates, background notifications |
| Method Channels | Flutter ↔ Platform | MethodChannel + ChannelRegistry | Device APIs, sensors, platform features |

## Project Structure

```
my_app/
├── protos/                          # Protobuf definitions (source of truth)
│   ├── common/types.proto
│   └── services/example.proto
├── rust/
│   └── crates/
│       ├── errors/                  # AppError, FfiError, DomainError, StorageError
│       ├── domain/                  # Entities, value objects, port traits, services
│       ├── proto/                   # Generated Rust types (prost)
│       ├── ffi/                     # ByteBuffer, AppRuntime, HandleRegistry, catch_ffi
│       ├── sqlite/                  # Database + versioned migrations
│       └── my_app_core/             # Composition root + FFI exports
├── flutter/
│   ├── packages/
│   │   ├── native_bridge/           # Dart FFI client + event receiver
│   │   ├── platform_bridge/         # Typed method channel toolkit
│   │   ├── proto_models/            # Generated Dart protobuf classes
│   │   ├── design_system/           # Material 3 theme (seed color based)
│   │   └── ui_kit/                  # Reusable themed widgets
│   └── apps/
│       └── my_app/                  # Flutter application
│           ├── lib/platform/        # Typed platform channel wrappers
│           ├── android/.../channels/ # Kotlin: ChannelRegistry + DeviceInfoChannel
│           ├── ios/Runner/          # Swift: ChannelRegistry + DeviceInfoChannel
│           ├── macos/Runner/        # Swift: ChannelRegistry + DeviceInfoChannel
│           ├── windows/runner/      # C++: channel_registry + device_info_channel
│           └── linux/runner/        # C: channel_registry + device_info_channel
├── templates/platform/              # Channel templates with {{placeholders}}
├── scripts/
│   ├── setup.sh                     # First-time setup
│   ├── generate_proto.sh            # Protobuf code generation
│   ├── check.sh                     # Full quality pipeline
│   └── init_platforms.sh            # Platform directory setup
└── justfile                         # Task runner commands
```

## CLI Options

```
pattern-h <name> [options]

Arguments:
  <name>              Project name in snake_case

Options:
  --org <domain>      Organization domain (default: com.example)
  --seed-color <hex>  Material 3 seed color, 6-char hex (default: 1A73E8)
  -o, --output <dir>  Output directory (default: current dir)
```

The CLI embeds the entire template at compile time. No network access or runtime dependencies needed — just a single binary.

## Workflow

```bash
just setup       # First-time: check tools, install deps, generate code
just check       # Full pipeline: format → lint → test
just fmt          # Format Rust + Dart
just lint         # Clippy + dart analyze
just test         # Run all tests
just generate     # Regenerate proto + FFI bindings
just scaffold ... # Create new project (wraps pattern-h CLI)
```

## Adding Features

### New Domain Feature (Rust ↔ Flutter)

1. Define messages in `protos/services/your_feature.proto`
2. Run `just generate`
3. Add service trait in `rust/crates/domain/src/services/`
4. Add FFI export in `rust/crates/app_core/src/ffi_exports.rs`
5. Run `cargo build -p app_core && dart run ffigen` in native_bridge
6. Add typed method in `FfiClient`
7. Call from Flutter UI

### New Platform Channel

1. **Dart**: Create `lib/platform/nfc.dart` with typed wrapper
2. **Android**: Create `channels/NfcChannel.kt`, add to `ChannelRegistry`
3. **iOS/macOS**: Create `NfcChannel.swift`, add to `ChannelRegistry`
4. **Windows**: Create `nfc_channel.h/.cpp`, add to `RegisterChannels()`, update CMakeLists
5. **Linux**: Create `nfc_channel.h/.cc`, add to `register_channels()`, update CMakeLists

### New Domain Entity with Storage

1. Define proto messages
2. Add entity in `domain/src/entities/`
3. Define port trait in `domain/src/ports/`
4. Add migration: `Migration { version: N, sql: "CREATE TABLE ..." }`
5. Implement adapter in `sqlite/` crate
6. Wire in `app_core`

## Tech Stack

### Rust
| Crate | Version | Purpose |
|-------|---------|---------|
| tokio | 1.x | Async runtime (multi-thread, static) |
| prost | 0.14 | Protobuf encode/decode |
| rusqlite | 0.38 | SQLite (bundled) |
| thiserror | 2.x | Error types |
| tracing | 0.1 | Structured logging |
| irondash_dart_ffi | 0.2 | Rust → Dart push events |
| dashmap | 6.x | Concurrent handle registry |
| cbindgen | 0.29 | C header generation |

### Flutter/Dart
| Package | Version | Purpose |
|---------|---------|---------|
| flutter_riverpod | ^3.3.0 | State management |
| go_router | ^17.0.0 | Routing |
| protobuf | ^6.0.0 | Protobuf runtime |
| native_toolchain_rust | ^1.0.3 | Native asset build hook |
| ffigen | ^20.0.0 | @Native binding generation |

### Toolchain
- Rust 1.86.0 (pinned, with Android/iOS/macOS cross-compilation targets)
- Dart SDK ^3.8.0
- Flutter >=3.32.0

## Requirements

- Rust (via [rustup](https://rustup.rs))
- Flutter SDK ([flutter.dev](https://flutter.dev))
- protoc ([protobuf compiler](https://github.com/protocolbuffers/protobuf/releases))
- protoc-gen-dart (`dart pub global activate protoc_plugin`)
- [just](https://github.com/casey/just) (optional, recommended)

## Tests

91 Rust tests across all crates covering:
- Error types, display messages, code mappings, From conversions
- EntityId generation, validation, display
- PingService with valid/invalid input
- DomainEvent enum variants and Clone
- ByteBuffer ownership, FfiResult encoding, free safety
- HandleRegistry insert/get/remove/contains
- FFI validation (null pointer, buffer overflow, UTF-8, handle)
- SQLite connection, execute, transactions, rollback
- Migration system (create table, apply order, skip applied, version tracking)

Verified on device:
- Android (SM-X115, ARM64, API 35) — all pass
- iOS (iPhone 17 Pro simulator, iOS 26.2) — all pass

## License

MIT
