use domain::services::ping::{DefaultPingService, PingService};
use ffi::{FfiResult, catch_ffi, validate_ptr};
use prost::Message;

static PING_SERVICE: DefaultPingService = DefaultPingService;

/// # Safety
/// `data` must point to valid protobuf-encoded PingRequest of length `len`.
#[unsafe(no_mangle)]
pub extern "C" fn app_ping(data: *const u8, len: usize) -> FfiResult {
    catch_ffi(|| {
        let bytes = unsafe { validate_ptr(data, len)? };
        let req = proto::services::PingRequest::decode(bytes)
            .map_err(|e| errors::FfiError::Decode(e.to_string()))?;

        let result_msg = PING_SERVICE.process(&req.message)?;

        let response = proto::services::PingResponse {
            message: result_msg,
            timestamp: Some(proto::common::Timestamp {
                seconds: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs() as i64)
                    .unwrap_or(0),
                nanos: 0,
            }),
        };

        Ok(response.encode_to_vec())
    })
}
