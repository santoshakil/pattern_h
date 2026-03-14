pub mod common {
    include!(concat!(env!("OUT_DIR"), "/common.rs"));
}

pub mod services {
    include!(concat!(env!("OUT_DIR"), "/services.rs"));
}
