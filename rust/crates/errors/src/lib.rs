mod domain;
mod ffi;
mod storage;

pub use domain::DomainError;
pub use ffi::FfiError;
pub use storage::StorageError;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Ffi(#[from] FfiError),
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error(transparent)]
    Storage(#[from] StorageError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_not_found_display() {
        let e = DomainError::NotFound {
            entity_type: "User".into(),
            id: "42".into(),
        };
        assert_eq!(e.to_string(), "User not found: 42");
    }

    #[test]
    fn domain_validation_display() {
        let e = DomainError::Validation("bad input".into());
        assert_eq!(e.to_string(), "validation: bad input");
    }

    #[test]
    fn domain_invalid_state_display() {
        let e = DomainError::InvalidState {
            from: "draft".into(),
            to: "archived".into(),
        };
        assert_eq!(e.to_string(), "invalid state: draft -> archived");
    }

    #[test]
    fn domain_permission_denied_display() {
        let e = DomainError::PermissionDenied {
            action: "delete".into(),
            resource: "order".into(),
        };
        assert_eq!(e.to_string(), "permission denied: delete on order");
    }

    #[test]
    fn domain_duplicate_display() {
        let e = DomainError::Duplicate {
            entity_type: "User".into(),
            field: "email".into(),
            value: "a@b.c".into(),
        };
        assert_eq!(e.to_string(), "duplicate User.email = a@b.c");
    }

    #[test]
    fn domain_business_rule_display() {
        let e = DomainError::BusinessRule("limit exceeded".into());
        assert_eq!(e.to_string(), "business rule: limit exceeded");
    }

    #[test]
    fn storage_connection_display() {
        let e = StorageError::Connection("refused".into());
        assert_eq!(e.to_string(), "connection: refused");
    }

    #[test]
    fn storage_query_display() {
        let e = StorageError::Query("syntax err".into());
        assert_eq!(e.to_string(), "query: syntax err");
    }

    #[test]
    fn storage_migration_display() {
        let e = StorageError::Migration {
            version: "3".into(),
            reason: "dup col".into(),
        };
        assert_eq!(e.to_string(), "migration v3: dup col");
    }

    #[test]
    fn storage_optimistic_lock_display() {
        let e = StorageError::OptimisticLock;
        assert_eq!(e.to_string(), "optimistic lock conflict");
    }

    #[test]
    fn storage_transaction_display() {
        let e = StorageError::Transaction("failed".into());
        assert_eq!(e.to_string(), "transaction: failed");
    }

    #[test]
    fn storage_constraint_display() {
        let e = StorageError::Constraint("unique".into());
        assert_eq!(e.to_string(), "constraint: unique");
    }

    #[test]
    fn ffi_null_pointer_display() {
        let e = FfiError::NullPointer;
        assert_eq!(e.to_string(), "null pointer");
    }

    #[test]
    fn ffi_invalid_handle_display() {
        let e = FfiError::InvalidHandle(99);
        assert_eq!(e.to_string(), "invalid handle: 99");
    }

    #[test]
    fn ffi_buffer_overflow_display() {
        let e = FfiError::BufferOverflow {
            capacity: 100,
            required: 200,
        };
        assert_eq!(e.to_string(), "buffer overflow: need 200, have 100");
    }

    #[test]
    fn ffi_invalid_utf8_display() {
        let e = FfiError::InvalidUtf8("bad bytes".into());
        assert_eq!(e.to_string(), "invalid utf8: bad bytes");
    }

    #[test]
    fn ffi_decode_display() {
        let e = FfiError::Decode("proto fail".into());
        assert_eq!(e.to_string(), "decode error: proto fail");
    }

    #[test]
    fn ffi_encode_display() {
        let e = FfiError::Encode("proto fail".into());
        assert_eq!(e.to_string(), "encode error: proto fail");
    }

    #[test]
    fn ffi_not_initialized_display() {
        let e = FfiError::NotInitialized;
        assert_eq!(e.to_string(), "not initialized");
    }

    #[test]
    fn ffi_already_initialized_display() {
        let e = FfiError::AlreadyInitialized;
        assert_eq!(e.to_string(), "already initialized");
    }

    #[test]
    fn ffi_runtime_init_display() {
        let e = FfiError::RuntimeInit("spawn failed".into());
        assert_eq!(e.to_string(), "runtime init failed: spawn failed");
    }

    #[test]
    fn ffi_error_codes() {
        assert_eq!(FfiError::NullPointer.code(), 1);
        assert_eq!(FfiError::InvalidHandle(0).code(), 2);
        assert_eq!(
            FfiError::BufferOverflow {
                capacity: 0,
                required: 0
            }
            .code(),
            3
        );
        assert_eq!(FfiError::InvalidUtf8(String::new()).code(), 4);
        assert_eq!(FfiError::Decode(String::new()).code(), 5);
        assert_eq!(FfiError::Encode(String::new()).code(), 6);
        assert_eq!(FfiError::NotInitialized.code(), 7);
        assert_eq!(FfiError::AlreadyInitialized.code(), 8);
        assert_eq!(FfiError::RuntimeInit(String::new()).code(), 9);
    }

    #[test]
    fn app_error_from_domain() {
        let d = DomainError::Validation("x".into());
        let a: AppError = d.into();
        assert!(matches!(a, AppError::Domain(DomainError::Validation(_))));
    }

    #[test]
    fn app_error_from_storage() {
        let s = StorageError::OptimisticLock;
        let a: AppError = s.into();
        assert!(matches!(a, AppError::Storage(StorageError::OptimisticLock)));
    }

    #[test]
    fn app_error_from_ffi() {
        let f = FfiError::NullPointer;
        let a: AppError = f.into();
        assert!(matches!(a, AppError::Ffi(FfiError::NullPointer)));
    }

    #[test]
    fn app_error_display_transparent() {
        let d = DomainError::BusinessRule("oops".into());
        let a: AppError = d.into();
        assert_eq!(a.to_string(), "business rule: oops");
    }
}
