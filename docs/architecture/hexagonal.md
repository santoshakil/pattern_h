Pattern H - Hexagonal Architecture

Layers (inside→out):
1. Domain — entities, value objects, port traits, domain services, typed DomainEvent enum
2. Application — composition root (app_core), FFI driving adapter, protobuf encode/decode
3. Infrastructure — driven adapters: sqlite (Repository), future crates (grpc, http, etc.)
4. Presentation — Flutter UI (pure presentation, zero logic, Riverpod state)

Dependency rule: always point inward. Domain knows nothing about infrastructure.

Ports (trait interfaces in domain/src/ports/):
- Repository<T, Id> — async CRUD operations, implemented by sqlite adapter
- EventBus — typed DomainEvent publishing (EntityCreated/Updated/Deleted/Custom)
- PingService — example domain service trait

Adapters:
- Driving (primary): app_core/ffi_exports.rs — FFI functions that translate external calls into domain ops
- Driven (secondary): sqlite crate implements port traits, future infra crates do the same

Communication layers:
1. Protobuf over FFI — Flutter→Rust→Flutter (request/response, zero-copy ByteBuffer)
2. Rust→Dart push events — irondash NativeEventPort (fire-and-forget, async notifications)
3. Platform method channels — Flutter→Kotlin/Swift/C++ (device APIs, sensors, platform features)

Data flow (FFI):
Flutter UI → Riverpod provider → FfiClient.callNative() → protobuf encode → malloc+copy → FFI boundary → app_core/ffi_exports → validate_ptr → protobuf decode → domain service → port trait → infra adapter → response encode → FfiResult → Dart extract+free → protobuf decode → UI update

Data flow (Method Channel):
Flutter → PlatformChannel.call() → MethodChannel → platform handler (Kotlin/Swift/C++) → result → Dart

Data flow (Push Events):
Rust domain event → EVENT_PORT.send_event() → irondash DartPort → ReceivePort → NativeEventReceiver.events stream → UI

Adding infrastructure:
1. Define port trait in domain/src/ports/
2. Create new crate implementing the trait (depend on errors + domain)
3. Wire adapter in app_core composition root
4. Domain never depends on the new crate — only app_core does

Error propagation:
Rust: DomainError/StorageError/FfiError → AppError (umbrella) → catch_ffi → FfiResult (code + message bytes)
Dart: FfiResult → FfiException.fromCode() → sealed class hierarchy matching Rust error codes
Platform: PlatformException → PlatformBridgeException.from() → typed exception hierarchy
