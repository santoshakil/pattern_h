Pattern H - Hexagonal Architecture

Layers (inside out):
- Domain: entities, value objects, ports (trait interfaces), domain services, events
- Application: use cases, composition root, FFI driving adapter
- Infrastructure: driven adapters (sqlite, grpc, etc.)
- Presentation: Flutter UI (pure presentation, zero logic)

Dependency rule: always point inward. Domain knows nothing about infrastructure.

Ports:
- Driving (primary): FFI functions in app_core/ffi_exports.rs — how Flutter drives the domain
- Driven (secondary): trait definitions in domain/ports/ — what domain needs from outside

Adapters:
- Driving: app_core crate translates FFI calls into domain operations
- Driven: sqlite crate implements Repository trait, future crates implement other ports

Data flow:
Flutter UI -> Riverpod -> FfiClient.callFfi() -> protobuf encode -> FFI boundary -> app_core/ffi_exports -> domain service -> port trait -> infrastructure adapter -> response back

Adding infrastructure:
1. Define port trait in domain/src/ports/
2. Create infra crate implementing the trait
3. Wire adapter to domain service in app_core
