use crate::{
    common::{conf::Configuration, fs::FileSystem},
    hdfs::hdfs::DistributedFileSystem,
};
use anyhow::Error;
use iref::Iri;

/// Get a FileSystem for this URI's scheme and authority.
pub fn get(uri: &Iri, conf: &Configuration) -> anyhow::Result<impl FileSystem> {
    // TODO: fallback to default fs, scheme, authority

    // TODO: CACHE

    create_file_system(uri, conf)
}

/// Create and initialize a new instance of a FileSystem.
fn create_file_system(uri: &Iri, conf: &Configuration) -> anyhow::Result<impl FileSystem> {
    match uri.scheme().as_str() {
        "hdfs" => Ok(DistributedFileSystem::new(uri, conf)?),
        _ => Err(Error::msg(format!(
            "Failed to initialize fileystem {}",
            uri
        ))),
    }
}
