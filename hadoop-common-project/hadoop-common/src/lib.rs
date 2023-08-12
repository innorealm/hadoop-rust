pub mod conf;
pub mod fs;
pub mod io;
pub mod ipc;
pub mod net;
pub mod security;
pub mod tracing;
pub mod util;

pub mod proto {
    pub mod hadoop {
        pub mod common {
            include!(concat!(env!("OUT_DIR"), "/hadoop.common.rs"));
        }
    }
}
