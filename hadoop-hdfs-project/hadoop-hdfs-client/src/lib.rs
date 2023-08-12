pub mod fs;
pub mod hdfs;

pub mod proto {
    pub mod hadoop {
        use hadoop_common::proto::hadoop::common;
        pub mod hdfs {
            include!(concat!(env!("OUT_DIR"), "/hadoop.hdfs.rs"));
        }
    }
}
