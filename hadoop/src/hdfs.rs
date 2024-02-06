pub mod fs;
pub mod hdfs;

include!(concat!(env!("OUT_DIR"), "/hadoop.hdfs.rs"));
